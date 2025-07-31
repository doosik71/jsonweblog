use crate::{ui::get_static_file, JsonLogParser, LogEntry, LogFilter, LogLevel, schema::{Schema, TableConfig, ColumnConfig}};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{any, get},
    Json, Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::{
    sync::{broadcast, RwLock},
    // time::{interval, Duration},
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct AppState {
    pub logs: Arc<RwLock<Vec<LogEntry>>>,
    pub log_tx: broadcast::Sender<LogEntry>,
    pub connection_count: Arc<AtomicU64>,
    pub schema: Arc<RwLock<Schema>>,
    pub table_config: Arc<RwLock<Option<TableConfig>>>,
}

impl AppState {
    pub fn new() -> Self {
        let (log_tx, _) = broadcast::channel(1000);
        
        // Try to load existing table configuration
        let settings_path = TableConfig::get_settings_path();
        let table_config = TableConfig::load_from_file(&settings_path)
            .map(Some)
            .unwrap_or(None);
        
        Self {
            logs: Arc::new(RwLock::new(Vec::new())),
            log_tx,
            connection_count: Arc::new(AtomicU64::new(0)),
            schema: Arc::new(RwLock::new(Schema::new())),
            table_config: Arc::new(RwLock::new(table_config)),
        }
    }

    pub async fn add_log(&self, entry: LogEntry) {
        {
            let mut logs = self.logs.write().await;
            logs.push(entry.clone());
            
            // Keep only last 100,000 entries to prevent memory issues
            if logs.len() > 100_000 {
                let len = logs.len();
                logs.drain(0..len - 100_000);
            }
        }
        
        // Initialize schema from first entry
        {
            let mut schema = self.schema.write().await;
            schema.initialize_from_first_entry(&entry.raw_fields);
            
            // Auto-generate table config if none exists
            if self.table_config.read().await.is_none() && schema.initialized {
                let default_columns = schema.get_default_columns();
                let config = TableConfig {
                    columns: default_columns,
                };
                *self.table_config.write().await = Some(config);
            }
        }
        
        // Broadcast to all connected clients
        if let Err(_e) = self.log_tx.send(entry) {
            // warn!("Failed to broadcast log entry: {}", e);
        }
    }

    pub async fn get_logs(&self, filter: Option<LogFilter>) -> Vec<LogEntry> {
        let logs = self.logs.read().await;
        
        if let Some(filter) = filter {
            logs.iter()
                .filter(|entry| filter.matches(entry))
                .cloned()
                .collect()
        } else {
            logs.clone()
        }
    }

    pub async fn clear_logs(&self) {
        let mut logs = self.logs.write().await;
        logs.clear();
    }
}

#[derive(Debug, Deserialize)]
pub struct LogQueryParams {
    level: Option<String>,
    search: Option<String>,
    logger: Option<String>,
    module: Option<String>,
    limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct LogResponse {
    logs: Vec<LogEntry>,
    total_count: usize,
    filtered_count: usize,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    total_logs: usize,
    active_connections: u64,
    uptime_seconds: u64,
}

pub struct WebServer {
    state: AppState,
    port: u16,
}

impl WebServer {
    pub fn new(port: u16) -> Self {
        Self {
            state: AppState::new(),
            port,
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let app = self.create_router();
        
        // Try to find an available port starting from the requested port
        let actual_port = self.find_available_port().await?;
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", actual_port)).await?;
        // info!("Web server listening on http://0.0.0.0:{}", actual_port);
        info!("Web interface available at http://localhost:{}", actual_port);
        
        // Start stdin parser task
        let parser_state = self.state.clone();
        tokio::spawn(async move {
            Self::stdin_parser_task(parser_state).await;
        });

        // // Start stats task
        // let stats_state = self.state.clone();
        // tokio::spawn(async move {
        //     Self::stats_task(stats_state).await;
        // });

        axum::serve(listener, app).await?;
        
        Ok(())
    }

    pub async fn find_available_port_for_new(port: u16) -> anyhow::Result<u16> {
        let mut current_port = port;
        const MAX_PORT_ATTEMPTS: u16 = 100;
        
        for _ in 0..MAX_PORT_ATTEMPTS {
            match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", current_port)).await {
                Ok(_) => {
                    if current_port != port {
                        info!("Port {} is not available, will use port {} instead", port, current_port);
                    }
                    return Ok(current_port);
                }
                Err(_) => {
                    warn!("Port {} is not available, trying {}", current_port, current_port + 1);
                    if current_port == 65535 {
                        return Err(anyhow::anyhow!("No available ports found in range"));
                    }
                    current_port += 1;
                }
            }
        }
        
        Err(anyhow::anyhow!("Could not find an available port after {} attempts", MAX_PORT_ATTEMPTS))
    }

    async fn find_available_port(&self) -> anyhow::Result<u16> {
        let mut port = self.port;
        const MAX_PORT_ATTEMPTS: u16 = 100;
        
        for _ in 0..MAX_PORT_ATTEMPTS {
            match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
                Ok(_) => {
                    if port != self.port {
                        info!("Port {} is not available, using port {} instead", self.port, port);
                    }
                    return Ok(port);
                }
                Err(_) => {
                    warn!("Port {} is not available, trying {}", port, port + 1);
                    if port == 65535 {
                        return Err(anyhow::anyhow!("No available ports found in range"));
                    }
                    port += 1;
                }
            }
        }
        
        Err(anyhow::anyhow!("Could not find an available port after {} attempts", MAX_PORT_ATTEMPTS))
    }

    fn create_router(&self) -> Router {
        Router::new()
            // Static file routes
            .route("/", get(serve_index))
            .route("/index.html", get(serve_index))
            .route("/style.css", get(serve_css))
            .route("/app.js", get(serve_js))
            // API routes
            .route("/api/logs", get(get_logs_handler))
            .route("/api/logs/clear", axum::routing::post(clear_logs_handler))
            .route("/api/stats", get(get_stats_handler))
            .route("/api/schema", get(get_schema_handler))
            .route("/api/schema/columns", get(get_columns_handler))
            .route("/api/schema/columns", axum::routing::post(set_columns_handler))
            // WebSocket route
            .route("/ws", any(websocket_handler))
            // Catch-all for static files
            .fallback(serve_static)
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::permissive())
            )
            .with_state(self.state.clone())
    }

    async fn stdin_parser_task(state: AppState) {
        let mut parser = JsonLogParser::new();
        let mut stream = parser.parse_stdin().await;
        
        // info!("Started stdin parser task");
        
        while let Some(result) = stream.next().await {
            match result {
                Ok(entry) => {
                    state.add_log(entry).await;
                }
                Err(e) => {
                    warn!("Failed to parse log line: {}", e);
                }
            }
        }
        
        // info!("Stdin parser task ended");
    }

    // async fn stats_task(state: AppState) {
    //     let mut interval = interval(Duration::from_secs(30));
        
    //     loop {
    //         interval.tick().await;
            
    //         let _log_count = state.logs.read().await.len();
    //         let _connection_count = state.connection_count.load(Ordering::Relaxed);
            
    //         // info!("Stats: {} logs, {} connections", log_count, connection_count);
    //     }
    // }
}

// Static file handlers
async fn serve_index() -> Response {
    match get_static_file("/") {
        Ok(response) => response,
        Err(status) => status.into_response(),
    }
}

async fn serve_css() -> Response {
    match get_static_file("/style.css") {
        Ok(response) => response,
        Err(status) => status.into_response(),
    }
}

async fn serve_js() -> Response {
    match get_static_file("/app.js") {
        Ok(response) => response,
        Err(status) => status.into_response(),
    }
}

async fn serve_static(uri: axum::http::Uri) -> Response {
    let path = uri.path();
    match get_static_file(path) {
        Ok(response) => response,
        Err(status) => status.into_response(),
    }
}

// API handlers
async fn get_logs_handler(
    State(state): State<AppState>,
    Query(params): Query<LogQueryParams>,
) -> Json<LogResponse> {
    let mut filter = LogFilter::new();
    
    if let Some(level) = params.level {
        if !level.is_empty() {
            filter = filter.with_level(LogLevel::from_str(&level));
        }
    }
    
    if let Some(search) = params.search {
        if !search.is_empty() {
            filter = filter.with_search_text(search);
        }
    }
    
    if let Some(logger) = params.logger {
        if !logger.is_empty() {
            filter = filter.with_logger(logger);
        }
    }
    
    if let Some(module) = params.module {
        if !module.is_empty() {
            filter = filter.with_module(module);
        }
    }
    
    let all_logs = state.logs.read().await;
    let total_count = all_logs.len();
    
    let filtered_logs = if filter.is_empty() {
        all_logs.clone()
    } else {
        all_logs.iter()
            .filter(|entry| filter.matches(entry))
            .cloned()
            .collect()
    };
    
    let filtered_count = filtered_logs.len();
    
    // Apply limit
    let logs = if let Some(limit) = params.limit {
        filtered_logs.into_iter().rev().take(limit).collect::<Vec<_>>().into_iter().rev().collect()
    } else {
        filtered_logs
    };
    
    Json(LogResponse {
        logs,
        total_count,
        filtered_count,
    })
}

async fn clear_logs_handler(State(state): State<AppState>) -> StatusCode {
    state.clear_logs().await;
    info!("Logs cleared via API");
    StatusCode::OK
}

async fn get_stats_handler(State(state): State<AppState>) -> Json<StatsResponse> {
    let log_count = state.logs.read().await.len();
    let connection_count = state.connection_count.load(Ordering::Relaxed);
    
    Json(StatsResponse {
        total_logs: log_count,
        active_connections: connection_count,
        uptime_seconds: 0, // TODO: Track actual uptime
    })
}

// WebSocket handler
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

async fn websocket_connection(socket: WebSocket, state: AppState) {
    // let connection_id = state.connection_count.fetch_add(1, Ordering::Relaxed) + 1;
    // info!("WebSocket connection {} established", connection_id);
    
    let (mut sender, mut receiver) = socket.split();
    let mut log_rx = state.log_tx.subscribe();
    
    // Send existing logs to the new client
    tokio::spawn(async move {
        let logs = state.logs.read().await;
        let recent_logs = logs.iter().rev().take(100_000).rev().cloned().collect::<Vec<_>>();
        drop(logs);
        
        for log_entry in recent_logs {
            if let Ok(json) = serde_json::to_string(&log_entry) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
        
        // Forward new log entries
        while let Ok(log_entry) = log_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&log_entry) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Handle incoming messages (for potential future features)
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(_)) => {
                // Handle text messages if needed
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
    
    state.connection_count.fetch_sub(1, Ordering::Relaxed);
    // info!("WebSocket connection {} closed", connection_id);
}

// Schema API handlers
async fn get_schema_handler(State(state): State<AppState>) -> Json<Schema> {
    let schema = state.schema.read().await;
    Json(schema.clone())
}

async fn get_columns_handler(State(state): State<AppState>) -> Json<Option<TableConfig>> {
    let config = state.table_config.read().await;
    Json(config.clone())
}

#[derive(Debug, Deserialize)]
struct SetColumnsRequest {
    columns: Vec<ColumnConfig>,
}

async fn set_columns_handler(
    State(state): State<AppState>,
    Json(request): Json<SetColumnsRequest>,
) -> Json<TableConfig> {
    let config = TableConfig {
        columns: request.columns,
    };
    
    // Save to file
    let settings_path = TableConfig::get_settings_path();
    if let Err(e) = config.save_to_file(&settings_path) {
        warn!("Failed to save table configuration: {}", e);
    } else {
        info!("Saved table configuration to {:?}", settings_path);
    }
    
    *state.table_config.write().await = Some(config.clone());
    info!("Updated table configuration with {} columns", config.columns.len());
    
    Json(config)
}