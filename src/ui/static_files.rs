use axum::response::Response;
use axum::{body::Body, http::StatusCode};

pub fn get_static_file(path: &str) -> Result<Response<Body>, StatusCode> {
    match path {
        "/" | "/index.html" => Ok(create_html_response(HTML_INDEX)),
        "/style.css" => Ok(create_css_response(CSS_STYLES)),
        "/app.js" => Ok(create_js_response(JS_APP)),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

fn create_html_response(content: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .body(Body::from(content))
        .unwrap()
}

fn create_css_response(content: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/css; charset=utf-8")
        .body(Body::from(content))
        .unwrap()
}

fn create_js_response(content: &'static str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/javascript; charset=utf-8")
        .body(Body::from(content))
        .unwrap()
}

const HTML_INDEX: &str = r#"<!DOCTYPE html>
<html lang="ko">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>JsonWebLog - 실시간 로그 모니터</title>
    <link rel="stylesheet" href="/style.css">
</head>
<body>
    <div id="app">
        <header class="header">
            <h1>JsonWebLog</h1>
            <div class="header-controls">
                <button id="settings-btn" class="btn btn-secondary">설정</button>
                <button id="clear-btn" class="btn btn-danger">지우기</button>
            </div>
        </header>

        <div class="filter-panel">
            <div class="filter-group">
                <label for="level-filter">레벨:</label>
                <select id="level-filter">
                    <option value="">전체</option>
                    <option value="TRACE">TRACE</option>
                    <option value="DEBUG">DEBUG</option>
                    <option value="INFO">INFO</option>
                    <option value="WARN">WARN</option>
                    <option value="ERROR">ERROR</option>
                    <option value="FATAL">FATAL</option>
                </select>
            </div>

            <div class="filter-group">
                <label for="search-filter">검색:</label>
                <input type="text" id="search-filter" placeholder="메시지, 로거, 모듈 검색...">
            </div>

            <div class="filter-group">
                <label for="logger-filter">로거:</label>
                <input type="text" id="logger-filter" placeholder="로거 이름...">
            </div>

            <div class="filter-group">
                <label for="module-filter">모듈:</label>
                <input type="text" id="module-filter" placeholder="모듈 이름...">
            </div>

            <div class="filter-group">
                <button id="clear-filters-btn" class="btn btn-secondary">필터 지우기</button>
            </div>
        </div>

        <div class="log-container">
            <div class="log-stats">
                <span>총 로그: <span id="total-count">0</span></span>
                <span>필터된 로그: <span id="filtered-count">0</span></span>
                <span>연결 상태: <span id="connection-status" class="status-disconnected">연결 안됨</span></span>
            </div>

            <div class="log-table-container">
                <table class="log-table">
                    <thead>
                        <tr>
                            <th class="col-line">라인</th>
                            <th class="col-time">시간</th>
                            <th class="col-level">레벨</th>
                            <th class="col-logger">로거</th>
                            <th class="col-message">메시지</th>
                            <th class="col-module">모듈</th>
                            <th class="col-function">함수</th>
                        </tr>
                    </thead>
                    <tbody id="log-table-body">
                        <!-- 로그 엔트리들이 여기에 추가됩니다 -->
                    </tbody>
                </table>
            </div>
        </div>
    </div>

    <script src="/app.js"></script>
</body>
</html>"#;

const CSS_STYLES: &str = r#"* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

body {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    background-color: #1a1a1a;
    color: #e0e0e0;
    line-height: 1.4;
}

.header {
    background-color: #2d2d2d;
    padding: 1rem 2rem;
    border-bottom: 1px solid #404040;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.header h1 {
    color: #61dafb;
    font-size: 1.5rem;
    font-weight: 600;
}

.header-controls {
    display: flex;
    gap: 0.5rem;
}

.btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    transition: background-color 0.2s;
}

.btn-secondary {
    background-color: #404040;
    color: #e0e0e0;
}

.btn-secondary:hover {
    background-color: #505050;
}

.btn-danger {
    background-color: #dc3545;
    color: white;
}

.btn-danger:hover {
    background-color: #c82333;
}

.filter-panel {
    background-color: #252525;
    padding: 1rem 2rem;
    border-bottom: 1px solid #404040;
    display: flex;
    gap: 1.5rem;
    flex-wrap: wrap;
    align-items: end;
}

.filter-group {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.filter-group label {
    font-size: 0.75rem;
    color: #b0b0b0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.filter-group input,
.filter-group select {
    padding: 0.5rem;
    border: 1px solid #404040;
    border-radius: 4px;
    background-color: #1a1a1a;
    color: #e0e0e0;
    font-size: 0.875rem;
    min-width: 150px;
}

.filter-group input:focus,
.filter-group select:focus {
    outline: none;
    border-color: #61dafb;
    box-shadow: 0 0 0 1px #61dafb;
}

.log-container {
    display: flex;
    flex-direction: column;
    height: calc(100vh - 140px);
}

.log-stats {
    background-color: #2d2d2d;
    padding: 0.75rem 2rem;
    border-bottom: 1px solid #404040;
    display: flex;
    gap: 2rem;
    font-size: 0.875rem;
}

.log-stats span {
    color: #b0b0b0;
}

.status-connected {
    color: #28a745 !important;
}

.status-disconnected {
    color: #dc3545 !important;
}

.log-table-container {
    flex: 1;
    overflow: auto;
}

.log-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
}

.log-table th {
    background-color: #2d2d2d;
    color: #e0e0e0;
    padding: 0.75rem 0.5rem;
    text-align: left;
    border-bottom: 1px solid #404040;
    position: sticky;
    top: 0;
    z-index: 10;
}

.log-table td {
    padding: 0.5rem;
    border-bottom: 1px solid #333;
    vertical-align: top;
}

.log-table tbody tr:hover {
    background-color: #2a2a2a;
}

.col-line {
    width: 60px;
    text-align: center;
}

.col-time {
    width: 100px;
    font-family: 'Courier New', monospace;
}

.col-level {
    width: 80px;
    text-align: center;
}

.col-logger {
    width: 120px;
}

.col-message {
    min-width: 300px;
}

.col-module {
    width: 120px;
}

.col-function {
    width: 120px;
}

.level-trace { color: #6B7280; }
.level-debug { color: #3B82F6; }
.level-info { color: #10B981; }
.level-warn { color: #F59E0B; }
.level-error { color: #EF4444; }
.level-fatal { color: #DC2626; font-weight: bold; }

.log-message {
    word-break: break-word;
}

.log-time {
    font-family: 'Courier New', monospace;
    color: #b0b0b0;
}

@media (max-width: 768px) {
    .filter-panel {
        flex-direction: column;
        gap: 1rem;
    }
    
    .filter-group {
        width: 100%;
    }
    
    .filter-group input,
    .filter-group select {
        min-width: auto;
        width: 100%;
    }
    
    .log-stats {
        flex-direction: column;
        gap: 0.5rem;
    }
    
    .log-table {
        font-size: 0.75rem;
    }
    
    .col-module,
    .col-function {
        display: none;
    }
}"#;

const JS_APP: &str = r#"class JsonWebLogApp {
    constructor() {
        this.logs = [];
        this.filteredLogs = [];
        this.filters = {
            level: '',
            search: '',
            logger: '',
            module: ''
        };
        this.ws = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        
        this.initializeElements();
        this.setupEventListeners();
        this.connectWebSocket();
    }

    initializeElements() {
        this.elements = {
            levelFilter: document.getElementById('level-filter'),
            searchFilter: document.getElementById('search-filter'),
            loggerFilter: document.getElementById('logger-filter'),
            moduleFilter: document.getElementById('module-filter'),
            clearFiltersBtn: document.getElementById('clear-filters-btn'),
            clearBtn: document.getElementById('clear-btn'),
            settingsBtn: document.getElementById('settings-btn'),
            totalCount: document.getElementById('total-count'),
            filteredCount: document.getElementById('filtered-count'),
            connectionStatus: document.getElementById('connection-status'),
            logTableBody: document.getElementById('log-table-body')
        };
    }

    setupEventListeners() {
        // Filter event listeners
        this.elements.levelFilter.addEventListener('change', () => this.updateFilter('level', this.elements.levelFilter.value));
        this.elements.searchFilter.addEventListener('input', this.debounce(() => this.updateFilter('search', this.elements.searchFilter.value), 300));
        this.elements.loggerFilter.addEventListener('input', this.debounce(() => this.updateFilter('logger', this.elements.loggerFilter.value), 300));
        this.elements.moduleFilter.addEventListener('input', this.debounce(() => this.updateFilter('module', this.elements.moduleFilter.value), 300));
        
        // Button event listeners
        this.elements.clearFiltersBtn.addEventListener('click', () => this.clearFilters());
        this.elements.clearBtn.addEventListener('click', () => this.clearLogs());
        this.elements.settingsBtn.addEventListener('click', () => this.showSettings());
    }

    connectWebSocket() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws`;
        
        this.ws = new WebSocket(wsUrl);
        
        this.ws.onopen = () => {
            console.log('WebSocket connected');
            this.updateConnectionStatus(true);
            this.reconnectAttempts = 0;
        };
        
        this.ws.onmessage = (event) => {
            try {
                const logEntry = JSON.parse(event.data);
                this.addLogEntry(logEntry);
            } catch (error) {
                console.error('Failed to parse log entry:', error);
            }
        };
        
        this.ws.onclose = () => {
            console.log('WebSocket disconnected');
            this.updateConnectionStatus(false);
            this.scheduleReconnect();
        };
        
        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
            this.updateConnectionStatus(false);
        };
    }

    scheduleReconnect() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);
            console.log(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`);
            setTimeout(() => this.connectWebSocket(), delay);
        }
    }

    updateConnectionStatus(connected) {
        const status = this.elements.connectionStatus;
        if (connected) {
            status.textContent = '연결됨';
            status.className = 'status-connected';
        } else {
            status.textContent = '연결 안됨';
            status.className = 'status-disconnected';
        }
    }

    addLogEntry(logEntry) {
        this.logs.push(logEntry);
        
        // Keep only last 100,000 entries to prevent memory issues
        if (this.logs.length > 100000) {
            this.logs = this.logs.slice(-100000);
        }
        
        this.applyFilters();
    }

    updateFilter(filterType, value) {
        this.filters[filterType] = value;
        this.applyFilters();
    }

    applyFilters() {
        this.filteredLogs = this.logs.filter(log => {
            // Level filter
            if (this.filters.level && log.level !== this.filters.level) {
                return false;
            }
            
            // Search filter
            if (this.filters.search) {
                const searchTerm = this.filters.search.toLowerCase();
                const message = (log.message || '').toLowerCase();
                const logger = (log.logger || '').toLowerCase();
                const module = (log.module || '').toLowerCase();
                const func = (log.function || '').toLowerCase();
                
                if (!message.includes(searchTerm) && 
                    !logger.includes(searchTerm) && 
                    !module.includes(searchTerm) && 
                    !func.includes(searchTerm)) {
                    return false;
                }
            }
            
            // Logger filter
            if (this.filters.logger && !(log.logger || '').toLowerCase().includes(this.filters.logger.toLowerCase())) {
                return false;
            }
            
            // Module filter
            if (this.filters.module && !(log.module || '').toLowerCase().includes(this.filters.module.toLowerCase())) {
                return false;
            }
            
            return true;
        });
        
        this.updateDisplay();
    }

    updateDisplay() {
        this.elements.totalCount.textContent = this.logs.length.toLocaleString();
        this.elements.filteredCount.textContent = this.filteredLogs.length.toLocaleString();
        
        this.renderLogTable();
    }

    renderLogTable() {
        const tbody = this.elements.logTableBody;
        tbody.innerHTML = '';
        
        // Show only last 1000 entries for performance
        const logsToShow = this.filteredLogs.slice(-1000);
        
        logsToShow.forEach(log => {
            const row = this.createLogRow(log);
            tbody.appendChild(row);
        });
        
        // Auto-scroll to bottom
        tbody.scrollTop = tbody.scrollHeight;
    }

    createLogRow(log) {
        const row = document.createElement('tr');
        
        const timestamp = new Date(log.timestamp);
        const timeStr = timestamp.toLocaleTimeString('ko-KR', { 
            hour12: false,
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit'
        });
        
        row.innerHTML = `
            <td class="col-line">${log.line}</td>
            <td class="col-time log-time">${timeStr}</td>
            <td class="col-level level-${log.level.toLowerCase()}">${log.level}</td>
            <td class="col-logger">${this.escapeHtml(log.logger || '')}</td>
            <td class="col-message log-message">${this.escapeHtml(log.message || '')}</td>
            <td class="col-module">${this.escapeHtml(log.module || '')}</td>
            <td class="col-function">${this.escapeHtml(log.function || '')}</td>
        `;
        
        return row;
    }

    clearFilters() {
        this.filters = {
            level: '',
            search: '',
            logger: '',
            module: ''
        };
        
        this.elements.levelFilter.value = '';
        this.elements.searchFilter.value = '';
        this.elements.loggerFilter.value = '';
        this.elements.moduleFilter.value = '';
        
        this.applyFilters();
    }

    clearLogs() {
        if (confirm('모든 로그를 지우시겠습니까?')) {
            this.logs = [];
            this.filteredLogs = [];
            this.updateDisplay();
        }
    }

    showSettings() {
        alert('설정 기능은 아직 구현되지 않았습니다.');
    }

    debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }

    escapeHtml(text) {
        const map = {
            '&': '&amp;',
            '<': '&lt;',
            '>': '&gt;',
            '"': '&quot;',
            "'": '&#039;'
        };
        return text.replace(/[&<>"']/g, (m) => map[m]);
    }
}

// Initialize the application when the DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    new JsonWebLogApp();
});"#;