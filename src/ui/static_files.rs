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
                <div class="theme-selector">
                    <select id="theme-selector" class="btn btn-secondary">
                        <option value="dark">Dark</option>
                        <option value="light">Light</option>
                        <option value="solarized-dark">Solarized Dark</option>
                        <option value="monokai-pro">Monokai Pro</option>
                    </select>
                </div>
                <button id="columns-btn" class="btn btn-secondary">컬럼 설정</button>
                <div class="auto-scroll-control">
                    <input type="checkbox" id="auto-scroll-checkbox">
                    <label for="auto-scroll-checkbox">자동 스크롤</label>
                </div>
                <button id="clear-btn" class="btn btn-danger">지우기</button>
            </div>
        </header>

        <div class="filter-panel">
            <div class="filter-group">
                <select id="filter-column">
                    <option value="">컬럼 선택</option>
                </select>
            </div>

            <div class="filter-group">
                <input type="text" id="filter-value" placeholder="검색할 값을 입력하세요...">
            </div>

            <div class="filter-group">
                <button id="clear-filters-btn" class="btn btn-secondary">필터 지우기</button>
            </div>
        </div>

        <!-- Column Configuration Panel -->
        <div id="column-config-panel" class="config-panel" style="display: none;">
            <div class="config-header">
                <h3>컬럼 설정</h3>
                <button id="close-config-btn" class="btn-close">&times;</button>
            </div>
            <div class="config-content">
                <div class="config-section">
                    <h4>컬럼 가시성</h4>
                    <div id="column-visibility-list" class="column-list">
                        <div class="loading">컬럼 정보를 불러오는 중...</div>
                    </div>
                </div>
                <div class="config-actions">
                    <button id="apply-columns-btn" class="btn btn-primary">적용</button>
                    <button id="show-all-btn" class="btn btn-secondary">모두 보기</button>
                </div>
            </div>
        </div>

        <div class="log-container">

            <div class="log-table-container" id="log-table-container">
                <div class="log-table-header">
                    <div class="log-header-row" id="log-header-row">
                        <!-- 동적 컬럼 헤더들이 여기에 렌더링됩니다 -->
                    </div>
                </div>
                <div class="virtual-scroll-container" id="virtual-scroll-container">
                    <div class="virtual-scroll-content" id="virtual-scroll-content">
                        <div class="virtual-scroll-spacer-top" id="virtual-scroll-spacer-top"></div>
                        <div class="virtual-scroll-viewport" id="virtual-scroll-viewport">
                            <!-- 가상 스크롤 행들이 여기에 렌더링됩니다 -->
                        </div>
                        <div class="virtual-scroll-spacer-bottom" id="virtual-scroll-spacer-bottom"></div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <script src="/app.js"></script>
</body>
</html>"#;

const CSS_STYLES: &str = r#"/* --- THEME DEFINITIONS --- */
:root,
html[data-theme="dark"] {
    --bg-primary: #1a1a1a;
    --bg-secondary: #252525;
    --bg-tertiary: #303030;
    --bg-header: #2d2d2d;
    --bg-panel: #2d2d2d;
    --bg-panel-header: #383838;
    --bg-input: #1a1a1a;
    --bg-row-hover: #2a2a2a;

    --text-primary: #e0e0e0;
    --text-secondary: #b0b0b0;
    --text-accent: #61dafb;
    --text-inverted: #1a1a1a;

    --border-primary: #404040;
    --border-secondary: #333;
    --border-accent: #61dafb;
    --border-resizer: rgba(255, 255, 255, 0.15);
    --border-resizer-hover: rgba(97, 218, 251, 0.25);

    --btn-secondary-bg: #404040;
    --btn-secondary-bg-hover: #505050;
    --btn-secondary-text: #e0e0e0;
    --btn-danger-bg: #dc3545;
    --btn-danger-bg-hover: #c82333;
    --btn-danger-text: white;
    --btn-primary-bg: #61dafb;
    --btn-primary-bg-hover: #4fa8c5;
    --btn-primary-text: #1a1a1a;

    --level-trace: #6B7280;
    --level-debug: #3B82F6;
    --level-info: #10B981;
    --level-warn: #F59E0B;
    --level-error: #EF4444;
    --level-fatal: #DC2626;

    --bg-level-trace: rgba(107, 114, 128, 0.05); /* Subtle background for trace */
    --bg-level-debug: rgba(59, 130, 246, 0.05); /* Subtle background for debug */
    --bg-level-info: rgba(16, 185, 129, 0.05); /* Subtle background for info */
    --bg-level-warn: rgba(245, 158, 11, 0.08);  /* Slightly more visible for warn */
    --bg-level-error: rgba(239, 68, 68, 0.1);   /* More visible for error */
    --bg-level-fatal: rgba(220, 38, 38, 0.15);  /* Most visible for fatal */
}

html[data-theme="light"] {
    --bg-primary: #ffffff;
    --bg-secondary: #f8f9fa;
    --bg-tertiary: #e9ecef;
    --bg-header: #f8f9fa;
    --bg-panel: #ffffff;
    --bg-panel-header: #f1f3f5;
    --bg-input: #ffffff;
    --bg-row-hover: #e9ecef;

    --text-primary: #212529;
    --text-secondary: #495057;
    --text-accent: #007bff;
    --text-inverted: #ffffff;

    --border-primary: #dee2e6;
    --border-secondary: #e9ecef;
    --border-accent: #007bff;
    --border-resizer: rgba(0, 0, 0, 0.15);
    --border-resizer-hover: rgba(0, 123, 255, 0.25);

    --btn-secondary-bg: #e9ecef;
    --btn-secondary-bg-hover: #d3d9df;
    --btn-secondary-text: #212529;
    --btn-danger-bg: #dc3545;
    --btn-danger-bg-hover: #c82333;
    --btn-danger-text: white;
    --btn-primary-bg: #007bff;
    --btn-primary-bg-hover: #0069d9;
    --btn-primary-text: #ffffff;
}

html[data-theme="solarized-dark"] {
    --bg-primary: #002b36;
    --bg-secondary: #073642;
    --bg-tertiary: #586e75;
    --bg-header: #073642;
    --bg-panel: #002b36;
    --bg-panel-header: #073642;
    --bg-input: #002b36;
    --bg-row-hover: #073642;

    --text-primary: #839496;
    --text-secondary: #586e75;
    --text-accent: #268bd2;
    --text-inverted: #002b36;

    --border-primary: #073642;
    --border-secondary: #586e75;
    --border-accent: #268bd2;
    --border-resizer: rgba(88, 110, 117, 0.5);
    --border-resizer-hover: rgba(38, 139, 210, 0.5);

    --btn-secondary-bg: #586e75;
    --btn-secondary-bg-hover: #657b83;
    --btn-secondary-text: #002b36;
    --btn-danger-bg: #dc322f;
    --btn-danger-bg-hover: #cb4b16;
    --btn-danger-text: #fdf6e3;
    --btn-primary-bg: #268bd2;
    --btn-primary-bg-hover: #2aa198;
    --btn-primary-text: #fdf6e3;

    --level-trace: #586e75;
    --level-debug: #268bd2;
    --level-info: #859900;
    --level-warn: #b58900;
    --level-error: #dc322f;
    --level-fatal: #d33682;
}

html[data-theme="monokai-pro"] {
    --bg-primary: #2D2A2E;
    --bg-secondary: #3E3B3F;
    --bg-tertiary: #4E4A4F;
    --bg-header: #3E3B3F;
    --bg-panel: #2D2A2E;
    --bg-panel-header: #3E3B3F;
    --bg-input: #2D2A2E;
    --bg-row-hover: #4E4A4F;

    --text-primary: #FCFCFA;
    --text-secondary: #727072;
    --text-accent: #FFD866;
    --text-inverted: #2D2A2E;

    --border-primary: #4E4A4F;
    --border-secondary: #727072;
    --border-accent: #FFD866;
    --border-resizer: rgba(114, 112, 114, 0.5);
    --border-resizer-hover: rgba(255, 216, 102, 0.5);

    --btn-secondary-bg: #4E4A4F;
    --btn-secondary-bg-hover: #727072;
    --btn-secondary-text: #FCFCFA;
    --btn-danger-bg: #FF6188;
    --btn-danger-bg-hover: #fc7ca0;
    --btn-danger-text: #2D2A2E;
    --btn-primary-bg: #FFD866;
    --btn-primary-bg-hover: #ffe187;
    --btn-primary-text: #2D2A2E;

    --level-trace: #727072;
    --level-debug: #AB9DF2;
    --level-info: #A9DC76;
    --level-warn: #FFD866;
    --level-error: #FF6188;
    --level-fatal: #FF6188;
}

/* --- BASE STYLES --- */
* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

body {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    background-color: var(--bg-primary);
    color: var(--text-primary);
    line-height: 1.4;
    height: 100vh;
    overflow: hidden; /* Prevent body scroll */
    transition: background-color 0.2s, color 0.2s;
}

#app {
    height: 100vh;
    display: flex;
    flex-direction: column;
}

.header {
    background-color: var(--bg-header);
    padding: 1rem 2rem;
    border-bottom: 1px solid var(--border-primary);
    display: flex;
    justify-content: space-between;
    align-items: center;
    transition: background-color 0.2s, border-bottom 0.2s;
}

.header h1 {
    color: var(--text-accent);
    font-size: 1.5rem;
    font-weight: 600;
    transition: color 0.2s;
}

.header-controls {
    display: flex;
    gap: 0.5rem;
    align-items: center;
}

.btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    transition: background-color 0.2s, color 0.2s;
}

.btn-secondary {
    background-color: var(--btn-secondary-bg);
    color: var(--btn-secondary-text);
}

.btn-secondary:hover {
    background-color: var(--btn-secondary-bg-hover);
}

.btn-danger {
    background-color: var(--btn-danger-bg);
    color: var(--btn-danger-text);
}

.btn-danger:hover {
    background-color: var(--btn-danger-bg-hover);
}

.filter-panel {
    background-color: var(--bg-secondary);
    padding: 1rem 2rem;
    border-bottom: 1px solid var(--border-primary);
    display: flex;
    gap: 1.5rem;
    flex-wrap: wrap;
    align-items: end;
    transition: background-color 0.2s, border-bottom 0.2s;
}

.filter-group {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.filter-group input,
.filter-group select {
    padding: 0.5rem;
    border: 1px solid var(--border-primary);
    border-radius: 4px;
    background-color: var(--bg-input);
    color: var(--text-primary);
    font-size: 0.875rem;
    min-width: 150px;
    transition: background-color 0.2s, color 0.2s, border-color 0.2s;
}

.filter-group input:focus,
.filter-group select:focus {
    outline: none;
    border-color: var(--border-accent);
    box-shadow: 0 0 0 1px var(--border-accent);
}

.log-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

.log-table-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

.log-table-header {
    flex-shrink: 0;
    background-color: var(--bg-header);
    border-bottom: 2px solid var(--border-primary);
    transition: background-color 0.2s, border-bottom-color 0.2s;
}

.log-header-row {
    display: flex;
    align-items: center;
    height: 40px;
    padding: 0;
    /* Reserve space for scrollbar */
    padding-right: 17px;
}

.virtual-scroll-container {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
}

.virtual-scroll-content {
    position: relative;
}

.virtual-scroll-viewport {
    display: flex;
    flex-direction: column;
}

.virtual-log-row {
    display: flex;
    min-height: 40px;
    border-bottom: 1px solid var(--border-secondary);
    align-items: center;
    transition: background-color 0.2s, border-bottom-color 0.2s;
}

.virtual-log-row:hover {
    background-color: var(--bg-row-hover);
}

.log-table-header {
    background-color: var(--bg-header);
    border-bottom: 1px solid var(--border-primary);
    position: sticky;
    top: 0;
    z-index: 10;
}

.log-header-row {
    display: flex;
    color: var(--text-primary);
    font-size: 0.875rem;
    font-weight: 600;
}

.log-header-row > div {
    padding: 0.75rem 0.5rem;
    text-align: left;
    border-right: 1px solid var(--border-primary);
}

.virtual-scroll-container {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    position: relative;
    height: 100%;
    outline: none;
}

.virtual-scroll-content {
    position: relative;
    width: 100%;
    /* This element now acts as the main spacer for total height */
}

.virtual-scroll-viewport {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    /* This element will be moved with transform: translateY() */
}

/* Spacers are no longer needed and will be removed from the DOM via JS */
.virtual-scroll-spacer-top,
.virtual-scroll-spacer-bottom {
    display: none;
}

.virtual-log-row {
    display: flex;
    font-size: 0.875rem;
    border-bottom: 1px solid var(--border-secondary);
    height: 2.5rem;
    align-items: center;
    position: relative;
    width: 100%;
    background-color: var(--bg-primary);
    box-sizing: border-box;
}

.virtual-log-row:hover {
    background-color: var(--bg-row-hover);
}

.virtual-log-row > div {
    padding: 0.5rem;
    border-right: 1px solid var(--border-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.level-trace { color: var(--level-trace); }
.level-debug { color: var(--level-debug); }
.level-info { color: var(--level-info); }
.level-warn { color: var(--level-warn); }
.level-error { color: var(--level-error); }
.level-fatal { color: var(--level-fatal); font-weight: bold; }

/* Row background colors based on level */
.virtual-log-row.row-level-trace { background-color: var(--bg-level-trace); }
.virtual-log-row.row-level-debug { background-color: var(--bg-level-debug); }
.virtual-log-row.row-level-info { background-color: var(--bg-level-info); }
.virtual-log-row.row-level-warn { background-color: var(--bg-level-warn); }
.virtual-log-row.row-level-error { background-color: var(--bg-level-error); }
.virtual-log-row.row-level-fatal { background-color: var(--bg-level-fatal); }

.log-time {
    font-family: 'Courier New', monospace;
    color: var(--text-secondary);
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
    
    .log-table {
        font-size: 0.75rem;
    }
    
    .col-module,
    .col-function {
        display: none;
    }
}

/* Column Configuration Panel */
.config-panel {
    position: fixed;
    top: 0;
    right: 0;
    width: 400px;
    height: 100vh;
    background-color: var(--bg-panel);
    border-left: 1px solid var(--border-primary);
    box-shadow: -2px 0 8px rgba(0, 0, 0, 0.3);
    z-index: 1000;
    display: flex;
    flex-direction: column;
    transition: background-color 0.2s, border-left-color 0.2s;
}

.config-header {
    padding: 1rem;
    border-bottom: 1px solid var(--border-primary);
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: var(--bg-panel-header);
    transition: background-color 0.2s, border-bottom-color 0.2s;
}

.config-header h3 {
    color: var(--text-accent);
    font-size: 1.2rem;
    margin: 0;
}

.btn-close {
    background: none;
    border: none;
    color: var(--text-secondary);
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 4px;
    transition: all 0.2s;
}

.btn-close:hover {
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
}

.config-content {
    flex: 1;
    padding: 1rem;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.config-section h4 {
    color: var(--text-secondary);
    font-size: 0.875rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 0.75rem;
    border-bottom: 1px solid var(--border-primary);
    padding-bottom: 0.5rem;
}

.column-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    height: calc(100vh - 300px);
    min-height: 400px;
    max-height: calc(100vh - 200px);
    overflow-y: auto;
    padding: 0.5rem;
    border: 1px solid var(--border-primary);
    border-radius: 4px;
    background-color: var(--bg-primary);
}

.column-visibility-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    border-radius: 4px;
    background-color: var(--bg-secondary);
    border: 1px solid var(--border-primary);
    transition: all 0.2s;
}

.column-visibility-item:hover {
    background-color: var(--bg-tertiary);
    border-color: var(--border-secondary);
}

.column-visibility-item input[type="checkbox"] {
    margin: 0;
}

.column-info {
    flex: 1;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.column-name {
    font-weight: 500;
    color: var(--text-primary);
}

.column-width {
    font-size: 0.875rem;
    color: var(--text-secondary);
}

.config-actions {
    display: flex;
    gap: 0.75rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-primary);
    margin-top: auto;
}

.btn-primary {
    background-color: var(--btn-primary-bg);
    color: var(--btn-primary-text);
    font-weight: 500;
}

.btn-primary:hover {
    background-color: var(--btn-primary-bg-hover);
}

.loading {
    text-align: center;
    color: var(--text-secondary);
    padding: 2rem;
    font-style: italic;
}

/* Dynamic column styles */
.dynamic-column {
    flex: 0 0 auto;
    padding: 0.5rem;
    border-right: 1px solid var(--border-primary);
    word-break: break-word;
    position: relative;
}

.dynamic-column:last-child {
    border-right: none;
}

.column-header {
    position: relative;
    display: flex;
    align-items: center;
    height: 100%;
}

.column-resizer {
    position: absolute;
    right: 0;
    top: 0;
    width: 6px; /* Make it easier to grab */
    height: 100%;
    cursor: col-resize;
    background-color: transparent;
    transition: background-color 0.2s;
    z-index: 10;
    border-right: 2px dotted var(--border-resizer);
}

.column-resizer:hover {
    background-color: var(--border-resizer-hover);
}

.column-resizer.resizing {
    background-color: var(--border-accent);
}

/* Styles for header drag & drop */
.dynamic-column.dragging {
    opacity: 0.5;
    background-color: var(--bg-tertiary);
}

.dynamic-column.drag-over {
    border-left: 2px solid var(--border-accent);
}

/* Column visibility styles */
.column-visibility-item:hover {
    background-color: var(--bg-tertiary);
    border-color: var(--border-secondary);
}
"#;

const JS_APP: &str = r#"class JsonWebLogApp {
        constructor() {
        this.logs = [];
        this.filteredLogs = [];
        this.filters = {
            column: '',
            value: ''
        };
        this.ws = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.scrollUpdateQueued = false; // Flag for requestAnimationFrame
        this.currentTheme = 'dark'; // Default theme
        this.autoScrollEnabled = true; // Default auto-scroll to true
        
        // Column configuration
        this.columns = []; // Simple list of column configs from server
        this.isConfigPanelOpen = false;
        this.resizing = null; // For column resizing
        
        // Virtual scrolling properties
        this.virtualScroll = {
            rowHeight: 40, // Default fallback, will be measured from DOM
            visibleRows: 20,
            bufferSize: 5,
            startIndex: 0,
            endIndex: 0,
            scrollTop: 0,
            containerHeight: 0,
            contentHeight: 0,
            initialized: false
        };
        
        this.initializeElements();
        this.setupEventListeners();
        this.setupVirtualScrolling();
        this.connectWebSocket();
        this.initializeSettings();
        this.throttledApplyFilters = this.throttle(this.applyFilters.bind(this), 50, { 'maxWait': 500 });
    }

    initializeElements() {
        this.elements = {
            filterColumn: document.getElementById('filter-column'),
            filterValue: document.getElementById('filter-value'),
            clearFiltersBtn: document.getElementById('clear-filters-btn'),
            clearBtn: document.getElementById('clear-btn'),
            columnsBtn: document.getElementById('columns-btn'),
            themeSelector: document.getElementById('theme-selector'),
            autoScrollCheckbox: document.getElementById('auto-scroll-checkbox'),
            columnConfigPanel: document.getElementById('column-config-panel'),
            closeConfigBtn: document.getElementById('close-config-btn'),
            columnVisibilityList: document.getElementById('column-visibility-list'),
            applyColumnsBtn: document.getElementById('apply-columns-btn'),
            showAllBtn: document.getElementById('show-all-btn'),
            logHeaderRow: document.getElementById('log-header-row'),
            virtualScrollContainer: document.getElementById('virtual-scroll-container'),
            virtualScrollContent: document.getElementById('virtual-scroll-content'),
            virtualScrollViewport: document.getElementById('virtual-scroll-viewport'),
            virtualScrollSpacerTop: document.getElementById('virtual-scroll-spacer-top'),
            virtualScrollSpacerBottom: document.getElementById('virtual-scroll-spacer-bottom')
        };
    }

    setupEventListeners() {
        // Filter event listeners
        this.elements.filterColumn.addEventListener('change', () => this.updateFilter('column', this.elements.filterColumn.value));
        this.elements.filterValue.addEventListener('input', this.debounce(() => this.updateFilter('value', this.elements.filterValue.value), 300));
        
        // Button event listeners
        this.elements.clearFiltersBtn.addEventListener('click', () => this.clearFilters());
        this.elements.clearBtn.addEventListener('click', () => this.clearLogs());
        this.elements.columnsBtn.addEventListener('click', () => this.showColumnConfig());
        this.elements.themeSelector.addEventListener('change', (e) => this.applyTheme(e.target.value));
        this.elements.autoScrollCheckbox.addEventListener('change', (e) => this.toggleAutoScroll(e.target.checked));
        
        // Column configuration event listeners
        this.elements.closeConfigBtn.addEventListener('click', () => this.hideColumnConfig());
        this.elements.applyColumnsBtn.addEventListener('click', () => this.applyColumnConfiguration());
        this.elements.showAllBtn.addEventListener('click', () => this.showAllColumns());

        // Global keydown listener for ESC key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && this.isConfigPanelOpen) {
                this.hideColumnConfig();
            }
        });
    }

    setupVirtualScrolling() {
        // Wait for DOM to be ready then calculate dimensions
        setTimeout(() => {
            this.calculateDimensions();
            this.updateVisibleRange();
            this.renderVisibleRows();
        }, 100);
        
        // Add scroll event listener
        this.elements.virtualScrollContainer.addEventListener('scroll', () => this.requestScrollUpdate());
        
        // Handle resize
        window.addEventListener('resize', 
            this.debounce(() => this.handleResize(), 250));
    }

    calculateDimensions() {
        // 1. Measure the actual row height from the DOM
        const dummyRow = document.createElement('div');
        dummyRow.className = 'virtual-log-row';
        dummyRow.style.position = 'absolute';
        dummyRow.style.top = '-9999px'; // Move it off-screen
        dummyRow.style.left = '-9999px';
        // Add content to ensure height is calculated correctly, including borders/padding
        dummyRow.innerHTML = '<span>&nbsp;</span>';
        document.body.appendChild(dummyRow);
        
        const measuredRowHeight = dummyRow.offsetHeight;
        document.body.removeChild(dummyRow);

        // Use the measured height, or fallback to 40 if measurement fails
        this.virtualScroll.rowHeight = measuredRowHeight > 0 ? measuredRowHeight : 40;

        // 2. Get actual container height
        const containerRect = this.elements.virtualScrollContainer.getBoundingClientRect();
        this.virtualScroll.containerHeight = containerRect.height;
        
        // 3. Calculate visible rows based on the *actual* measured height
        this.virtualScroll.visibleRows = Math.ceil(this.virtualScroll.containerHeight / this.virtualScroll.rowHeight) + 2; // +2 for buffer
        this.virtualScroll.initialized = true;
        
        
    }

    requestScrollUpdate() {
        if (!this.scrollUpdateQueued) {
            this.scrollUpdateQueued = true;
            window.requestAnimationFrame(() => this.handleScroll());
        }
    }

    handleScroll() {
        this.scrollUpdateQueued = false; // Allow next frame to be queued

        const scrollTop = this.elements.virtualScrollContainer.scrollTop;
        this.virtualScroll.scrollTop = scrollTop;
        
        this.updateVisibleRange();
        this.renderVisibleRows();

        // If auto-scroll was enabled and user scrolled up, disable it and update UI
        if (this.autoScrollEnabled && !this.isAtBottom()) {
            this.autoScrollEnabled = false;
            this.elements.autoScrollCheckbox.checked = false;
            this.saveSettings(); // Persist the change
        }
    }

    handleResize() {
        this.calculateDimensions();
        this.updateVisibleRange();
        this.renderVisibleRows();
    }

        updateVisibleRange() {
        // Don't update if not initialized yet
        if (!this.virtualScroll.initialized) {
            return;
        }

        const { rowHeight, bufferSize, scrollTop, containerHeight } = this.virtualScroll;
        const totalRows = this.filteredLogs.length;

        // 1. Update the total height of the scrollable content area
        this.virtualScroll.contentHeight = totalRows * rowHeight;
        this.elements.virtualScrollContent.style.height = `${this.virtualScroll.contentHeight}px`;

        // 2. Determine the range of items to render
        const firstVisibleIndex = Math.floor(scrollTop / rowHeight);
        const startIndex = Math.max(0, firstVisibleIndex - bufferSize);
        
        const visibleRowCount = Math.ceil(containerHeight / rowHeight);
        const rowsToRender = visibleRowCount + (bufferSize * 2);
        const endIndex = Math.min(totalRows, startIndex + rowsToRender);

        this.virtualScroll.startIndex = startIndex;
        this.virtualScroll.endIndex = endIndex;

        // 3. Calculate the Y offset for the viewport
        // This is the crucial part: we move the viewport itself, not a spacer.
        const viewportOffset = startIndex * rowHeight;

        // 4. Apply the transform to the viewport
        this.elements.virtualScrollViewport.style.transform = `translateY(${viewportOffset}px)`;
        
        
    }

    connectWebSocket() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws`;
        
        this.ws = new WebSocket(wsUrl);
        
        this.ws.onopen = () => {
            console.log('WebSocket connected');
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
            this.scheduleReconnect();
        };
        
        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
        };
    }

    scheduleReconnect() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);
            setTimeout(() => this.connectWebSocket(), delay);
        }
    }

    addLogEntry(logEntry) {
        this.logs.push(logEntry);
        
        // Keep only last 100,000 entries to prevent memory issues
        if (this.logs.length > 100000) {
            this.logs = this.logs.slice(-100000);
        }
        
        this.throttledApplyFilters();
    }

    updateFilter(filterType, value) {
        this.filters[filterType] = value;
        this.applyFilters();
    }

    applyFilters() {
        // If no filter is applied, just use the raw logs
        if (!this.filters.column && !this.filters.value) {
            this.filteredLogs = this.logs.slice(); // Create a shallow copy
            this.updateDisplay();
            return;
        }

        this.filteredLogs = this.logs.filter(log => {
            // Column-based filter
            if (this.filters.column && this.filters.value) {
                const filterValue = this.filters.value.toLowerCase();
                const columnValue = this.getFieldValue(log, this.filters.column);
                const displayValue = this.formatValue(columnValue).toLowerCase();
                
                if (!displayValue.includes(filterValue)) {
                    return false;
                }
            }
            
            return true;
        });
        
        this.updateDisplay();
    }

    updateDisplay() {
        // Check if user was at bottom before update
        const wasAtBottom = this.isAtBottom();
        
        this.updateVisibleRange();
        this.renderVisibleRows();
        
        // Auto-scroll to bottom for new logs only if user was already at bottom
        if (this.autoScrollEnabled && wasAtBottom && this.filteredLogs.length > 0) {
            this.scrollToBottom();
        }
    }

    isAtBottom() {
        const { scrollTop, scrollHeight, clientHeight } = this.elements.virtualScrollContainer;
        return scrollTop + clientHeight >= scrollHeight - 10; // 10px threshold
    }

    renderVisibleRows() {
        const viewport = this.elements.virtualScrollViewport;
        const { startIndex, endIndex, rowHeight } = this.virtualScroll;
        
        // Clear existing rows
        viewport.innerHTML = '';
        
        let actualRowsRendered = 0;
        let firstRenderedIndex = -1;
        let lastRenderedIndex = -1;
        
        // Render visible rows with consistent height
        for (let i = startIndex; i < endIndex; i++) {
            if (i >= this.filteredLogs.length) break;
            
            const log = this.filteredLogs[i];
            const row = this.createVirtualLogRow(log, i);
            // Ensure consistent row height
            row.style.height = rowHeight + 'px';
            row.style.minHeight = rowHeight + 'px';
            row.style.maxHeight = rowHeight + 'px';
            viewport.appendChild(row);
            
            if (actualRowsRendered === 0) {
                firstRenderedIndex = i + 1; // 1-based indexing for display
            }
            lastRenderedIndex = i + 1; // 1-based indexing for display
            actualRowsRendered++;
        }
        
        // Calculate actual visible range (without buffer)
        const { scrollTop, containerHeight } = this.virtualScroll;
        const firstVisibleIndex = Math.floor(scrollTop / rowHeight);
        const lastVisibleIndex = Math.min(this.filteredLogs.length - 1, firstVisibleIndex + Math.ceil(containerHeight / rowHeight) - 1);
        
        
    }

    scrollToBottom() {
        if (!this.autoScrollEnabled) return; // Only scroll if auto-scroll is enabled
        const maxScroll = this.virtualScroll.contentHeight - this.virtualScroll.containerHeight;
        if (maxScroll > 0) {
            this.elements.virtualScrollContainer.scrollTop = maxScroll;
        }
    }

    createVirtualLogRow(log, virtualIndex) {
        const row = document.createElement('div');
        row.className = 'virtual-log-row';
        
        // Add level-specific class for background color
        if (log.level) {
            row.classList.add(`row-level-${log.level.toLowerCase()}`);
        }
        
        // Initialize columns from first log if not already done
        if (this.columns.length === 0 && log.raw_fields) {
            this.initializeColumnsFromLog(log);
        }
        
        if (this.columns.length === 0) {
            // Fallback: show raw JSON if no columns configured
            row.innerHTML = `<div class="dynamic-column" style="flex: 1;">${this.escapeHtml(JSON.stringify(log.raw_fields))}</div>`;
        } else {
            // Render using configured columns, sorted by order
            const visibleColumns = this.columns
                .sort((a, b) => (a.order || 0) - (b.order || 0))
                .filter(col => col.visible);
            const html = visibleColumns.map(column => {
                const value = this.getFieldValue(log, column.field_name);
                const formattedValue = this.formatValue(value);
                
                return `<div class="dynamic-column" style="width: ${column.width}px; flex: 0 0 ${column.width}px;">${formattedValue}</div>`;
            }).join('');
            
            row.innerHTML = html;
        }
        
        
        
        return row;
    }

    initializeColumnsFromLog(log) {
        // Always start with line number column
        this.columns = [{
            field_name: '#',
            width: 80,
            visible: true
        }];
        
        // Add columns from first log entry's keys
        const fieldNames = Object.keys(log.raw_fields || {});
        const logColumns = fieldNames.map(fieldName => ({
            field_name: fieldName,
            width: 150,
            visible: true
        }));
        
        this.columns = this.columns.concat(logColumns);
        
        // Render headers immediately
        this.renderTableHeaders();
        
        
    }

    getFieldValue(log, fieldName) {
        // Handle special line number column
        if (fieldName === '#') {
            return log.line || '';
        }
        
        // Get value from raw_fields
        if (log.raw_fields && log.raw_fields.hasOwnProperty(fieldName)) {
            return log.raw_fields[fieldName];
        }
        return '';
    }

    formatValue(value) {
        if (value === null || value === undefined) {
            return '';
        }
        
        if (typeof value === 'object') {
            if (Array.isArray(value)) {
                return `[${value.length}]`;
            } else {
                return JSON.stringify(value);
            }
        } else if (typeof value === 'boolean') {
            return value ? 'true' : 'false';
        } else {
            return this.escapeHtml(String(value));
        }
    }

    clearFilters() {
        this.filters = {
            column: '',
            value: ''
        };
        
        this.elements.filterColumn.value = '';
        this.elements.filterValue.value = '';
        
        this.applyFilters();
    }

    clearLogs() {
        if (confirm('모든 로그를 지우시겠습니까?')) {
            this.logs = [];
            this.filteredLogs = [];
            this.updateDisplay();
        }
    }

    async initializeSettings() {
        // Load initial configuration for columns and theme
        try {
            const response = await fetch('/api/schema/columns');
            if (!response.ok) return;

            const tableConfig = await response.json();
            
            if (tableConfig) {
                if (tableConfig.columns) {
                    this.columns = tableConfig.columns;
                    this.renderTableHeaders();
                }
                if (tableConfig.theme) {
                    this.applyTheme(tableConfig.theme, false); // Apply theme without saving
                }
                // Initialize auto-scroll setting
                if (typeof tableConfig.auto_scroll !== 'undefined') {
                    this.autoScrollEnabled = tableConfig.auto_scroll;
                    this.elements.autoScrollCheckbox.checked = tableConfig.auto_scroll;
                }
            }
        } catch (error) {
            console.error('Failed to initialize settings:', error);
        }
    }

    showColumnConfig() {
        this.isConfigPanelOpen = true;
        this.elements.columnConfigPanel.style.display = 'flex';
        this.renderColumnVisibilityList();
    }

    hideColumnConfig() {
        this.isConfigPanelOpen = false;
        this.elements.columnConfigPanel.style.display = 'none';
    }

    renderColumnVisibilityList() {
        if (!this.columns || this.columns.length === 0) {
            this.elements.columnVisibilityList.innerHTML = '<div class="loading">컬럼 정보가 없습니다.</div>';
            return;
        }

        // Sort columns by order for display
        const sortedColumns = [...this.columns].sort((a, b) => (a.order || 0) - (b.order || 0));

        const html = sortedColumns.map((column, index) => {
            const originalIndex = this.columns.indexOf(column);
            return `
                <div class="column-visibility-item" data-column-index="${originalIndex}">
                    <input type="checkbox" id="col-${originalIndex}" ${column.visible ? 'checked' : ''} 
                           onchange="app.toggleColumnVisibility(${originalIndex}, this.checked)">
                    <div class="column-info">
                        <span class="column-name">${column.field_name}</span>
                        <span class="column-width">${column.width}px</span>
                    </div>
                </div>
            `;
        }).join('');

        this.elements.columnVisibilityList.innerHTML = html;
        this.setupColumnDragAndDrop();
    }

    toggleColumnVisibility(index, visible) {
        if (this.columns[index]) {
            this.columns[index].visible = visible;
        }
    }

    showAllColumns() {
        this.columns.forEach(column => column.visible = true);
        this.renderColumnVisibilityList();
    }

    async applyColumnConfiguration() {
        try {
            const response = await fetch('/api/schema/columns', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    columns: this.columns
                })
            });
            
            if (response.ok) {
                this.renderTableHeaders();
                this.updateDisplay(); // Re-render rows with new columns
                this.hideColumnConfig();
            } else {
                alert('컬럼 설정을 저장하는데 실패했습니다.');
            }
        } catch (error) {
            console.error('Failed to apply column configuration:', error);
            alert('컬럼 설정을 적용하는데 실패했습니다.');
        }
    }

    renderTableHeaders() {
        if (!this.columns || this.columns.length === 0) return;
        
        // Sort columns by order, then filter by visibility
        const visibleColumns = this.columns
            .sort((a, b) => (a.order || 0) - (b.order || 0))
            .filter(col => col.visible);
            
        const html = visibleColumns.map((column, index) => `
            <div class="dynamic-column" 
                 style="width: ${column.width}px; flex: 0 0 ${column.width}px;" 
                 draggable="true" 
                 data-field-name="${column.field_name}">
                <div class="column-header">
                    ${column.field_name}
                    ${index < visibleColumns.length - 1 ? '<div class="column-resizer" data-column="' + index + '"></div>' : ''}
                </div>
            </div>
        `).join('');
        
        this.elements.logHeaderRow.innerHTML = html;
        this.setupColumnResizers();
        this.setupHeaderDragAndDrop(); // Add this call
        this.updateFilterColumnOptions();
    }

    updateFilterColumnOptions() {
        if (!this.columns || this.columns.length === 0) return;
        
        const currentValue = this.elements.filterColumn.value;
        const options = ['<option value="">컬럼 선택</option>'];
        
        this.columns.forEach(column => {
            const selected = column.field_name === currentValue ? 'selected' : '';
            options.push(`<option value="${column.field_name}" ${selected}>${column.field_name}</option>`);
        });
        
        this.elements.filterColumn.innerHTML = options.join('');
    }

    setupColumnResizers() {
        const resizers = this.elements.logHeaderRow.querySelectorAll('.column-resizer');
        resizers.forEach(resizer => {
            resizer.addEventListener('mousedown', (e) => this.startColumnResize(e));
        });
    }

    startColumnResize(e) {
        e.preventDefault();
        const columnIndex = parseInt(e.target.dataset.column);
        const visibleColumns = this.columns.filter(col => col.visible);
        
        this.resizing = {
            columnIndex: this.columns.indexOf(visibleColumns[columnIndex]),
            startX: e.clientX,
            startWidth: visibleColumns[columnIndex].width
        };

        document.addEventListener('mousemove', this.handleColumnResize.bind(this));
        document.addEventListener('mouseup', this.endColumnResize.bind(this));
        e.target.classList.add('resizing');
    }

    handleColumnResize(e) {
        if (!this.resizing) return;
        
        const deltaX = e.clientX - this.resizing.startX;
        const newWidth = Math.max(50, this.resizing.startWidth + deltaX); // Minimum 50px
        
        this.columns[this.resizing.columnIndex].width = newWidth;
        this.renderTableHeaders();
        this.updateDisplay(); // Re-render rows with new widths
        
        
    }

    endColumnResize(e) {
        if (!this.resizing) return;
        
        document.removeEventListener('mousemove', this.handleColumnResize.bind(this));
        document.removeEventListener('mouseup', this.endColumnResize.bind(this));
        
        const resizer = document.querySelector('.column-resizer.resizing');
        if (resizer) {
            resizer.classList.remove('resizing');
        }
        
        // Auto-save column configuration after resize
        this.saveSettings();
        
        this.resizing = null;
    }

    async saveSettings() {
        try {
            const settings = {
                theme: this.currentTheme,
                columns: this.columns,
                auto_scroll: this.autoScrollEnabled
            };

            const response = await fetch('/api/schema/columns', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(settings)
            });
            
            if (response.ok) {
            } else {
            }
        } catch (error) {
            console.error('Error saving settings:', error);
        }
    }

    applyTheme(themeName, save = true) {
        document.documentElement.setAttribute('data-theme', themeName);
        this.currentTheme = themeName;
        this.elements.themeSelector.value = themeName;
        if (save) {
            this.saveSettings();
        }
    }

    toggleAutoScroll(enabled) {
        this.autoScrollEnabled = enabled;
        this.saveSettings();
        if (enabled) {
            this.scrollToBottom(); // Scroll to bottom immediately if enabled
        }
    }

    setupColumnDragAndDrop() {
        // This function is now intentionally left blank as we drag headers directly.
    }

    setupHeaderDragAndDrop() {
        const headers = this.elements.logHeaderRow.querySelectorAll('.dynamic-column');
        let draggedElement = null;

        headers.forEach(header => {
            header.addEventListener('dragstart', (e) => {
                draggedElement = header;
                e.dataTransfer.effectAllowed = 'move';
                e.dataTransfer.setData('text/plain', header.dataset.fieldName);
                header.classList.add('dragging');
            });

            header.addEventListener('dragend', (e) => {
                header.classList.remove('dragging');
                headers.forEach(h => h.classList.remove('drag-over'));
                draggedElement = null;
            });

            header.addEventListener('dragover', (e) => {
                e.preventDefault();
                if (header !== draggedElement) {
                    header.classList.add('drag-over');
                }
            });

            header.addEventListener('dragleave', (e) => {
                header.classList.remove('drag-over');
            });

            header.addEventListener('drop', (e) => {
                e.preventDefault();
                header.classList.remove('drag-over');
                if (header !== draggedElement) {
                    const fromFieldName = e.dataTransfer.getData('text/plain');
                    const toFieldName = header.dataset.fieldName;
                    this.reorderColumnsByFieldName(fromFieldName, toFieldName);
                }
            });
        });
    }

    reorderColumnsByFieldName(fromField, toField) {
        const fromIndex = this.columns.findIndex(c => c.field_name === fromField);
        const toIndex = this.columns.findIndex(c => c.field_name === toField);

        if (fromIndex === -1 || toIndex === -1) return;

        // Move the dragged item in the array
        const [movedItem] = this.columns.splice(fromIndex, 1);
        this.columns.splice(toIndex, 0, movedItem);

        // Update the 'order' property for all columns to reflect the new sequence
        this.columns.forEach((col, index) => {
            col.order = index;
        });

        

        // Re-render and save
        this.renderTableHeaders();
        this.updateDisplay();
        this.saveSettings();
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

    throttle(func, wait, options) {
        let timeoutId;
        let lastArgs;
        let lastThis;
        let lastResult;
        let lastCallTime = 0;
        let lastInvokeTime = 0;
        let leading = true;
        let trailing = true;
        let maxWait = null;

        if (typeof options === 'object') {
            leading = 'leading' in options ? !!options.leading : leading;
            trailing = 'trailing' in options ? !!options.trailing : trailing;
            maxWait = 'maxWait' in options ? Math.max(wait, options.maxWait) : maxWait;
        }

        const invokeFunc = (time) => {
            lastResult = func.apply(lastThis, lastArgs);
            lastInvokeTime = time;
            clearTimeout(timeoutId);
            timeoutId = null;
        };

        const leadingEdge = (time) => {
            lastInvokeTime = time;
            timeoutId = setTimeout(timerExpired, wait);
            return func.apply(lastThis, lastArgs);
        };

        const timerExpired = () => {
            const time = Date.now();
            if (trailing && lastArgs && (lastCallTime - lastInvokeTime >= wait)) {
                invokeFunc(time);
            } else {
                timeoutId = setTimeout(timerExpired, lastInvokeTime + wait - time);
            }
        };

        const throttled = function(...args) {
            const time = Date.now();
            lastArgs = args;
            lastThis = this;

            const isInvoking = (time - lastCallTime >= wait) || (maxWait && (time - lastInvokeTime >= maxWait));
            lastCallTime = time;

            if (isInvoking) {
                if (timeoutId === null && leading) {
                    return leadingEdge(time);
                }
                if (maxWait && (time - lastInvokeTime >= maxWait)) {
                    invokeFunc(time);
                }
            }

            if (timeoutId === null) {
                timeoutId = setTimeout(timerExpired, wait);
            }

            return lastResult;
        };

        throttled.cancel = () => {
            clearTimeout(timeoutId);
            timeoutId = null;
            lastCallTime = 0;
            lastInvokeTime = 0;
        };

        return throttled;
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
    window.app = new JsonWebLogApp();
});"#;