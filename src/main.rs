use jsonweblog::WebServer;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting JsonWebLog server...");

    // Parse command line arguments
    let requested_port = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);

    info!("Requested port: {}", requested_port);
    info!("Send JSONL formatted logs to stdin to see them in the web interface");

    // Find an available port starting from the requested port
    let actual_port = WebServer::find_available_port_for_new(requested_port).await?;
    
    if actual_port != requested_port {
        info!("Port {} was not available, using port {} instead", requested_port, actual_port);
    }

    // Create and start the server
    let server = WebServer::new(actual_port);
    server.start().await?;

    Ok(())
}
