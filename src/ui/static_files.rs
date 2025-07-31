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
                <button id="columns-btn" class="btn btn-secondary">컬럼 설정</button>
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
    height: 100vh;
    overflow: hidden; /* Prevent body scroll */
}

#app {
    height: 100vh;
    display: flex;
    flex-direction: column;
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
    background-color: #2d2d2d;
    border-bottom: 2px solid #404040;
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
    border-bottom: 1px solid #333;
    align-items: center;
}

.virtual-log-row:hover {
    background-color: #2a2a2a;
}


.log-table-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

.log-table-header {
    background-color: #2d2d2d;
    border-bottom: 1px solid #404040;
    position: sticky;
    top: 0;
    z-index: 10;
}

.log-header-row {
    display: flex;
    color: #e0e0e0;
    font-size: 0.875rem;
    font-weight: 600;
}

.log-header-row > div {
    padding: 0.75rem 0.5rem;
    text-align: left;
    border-right: 1px solid #404040;
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
    border-bottom: 1px solid #333;
    height: 2.5rem;
    align-items: center;
    position: relative;
    width: 100%;
    background-color: #1a1a1a;
    box-sizing: border-box;
}

.virtual-log-row:hover {
    background-color: #2a2a2a;
}

.virtual-log-row > div {
    padding: 0.5rem;
    border-right: 1px solid #333;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
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
    background-color: #2d2d2d;
    border-left: 1px solid #404040;
    box-shadow: -2px 0 8px rgba(0, 0, 0, 0.3);
    z-index: 1000;
    display: flex;
    flex-direction: column;
}

.config-header {
    padding: 1rem;
    border-bottom: 1px solid #404040;
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: #383838;
}

.config-header h3 {
    color: #61dafb;
    font-size: 1.2rem;
    margin: 0;
}

.btn-close {
    background: none;
    border: none;
    color: #b0b0b0;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 4px;
    transition: all 0.2s;
}

.btn-close:hover {
    background-color: #505050;
    color: #e0e0e0;
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
    color: #b0b0b0;
    font-size: 0.875rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 0.75rem;
    border-bottom: 1px solid #404040;
    padding-bottom: 0.5rem;
}

.field-list,
.column-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    height: calc(100vh - 300px);
    min-height: 400px;
    max-height: calc(100vh - 200px);
    overflow-y: auto;
    padding: 0.5rem;
    border: 1px solid #404040;
    border-radius: 4px;
    background-color: #1a1a1a;
}

.field-item,
.column-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem;
    border-radius: 4px;
    background-color: #252525;
    border: 1px solid #404040;
    transition: all 0.2s;
}

.field-item:hover,
.column-item:hover {
    background-color: #303030;
    border-color: #505050;
}

.field-item input[type="checkbox"] {
    margin: 0;
}

.field-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.field-name {
    font-weight: 500;
    color: #e0e0e0;
}

.field-details {
    font-size: 0.75rem;
    color: #b0b0b0;
    display: flex;
    gap: 1rem;
}

.field-type {
    color: #61dafb;
    font-weight: 500;
}

.column-item {
    justify-content: space-between;
    cursor: grab;
}

.column-item:active {
    cursor: grabbing;
}

.column-controls {
    display: flex;
    gap: 0.5rem;
    align-items: center;
}

.btn-small {
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
}

.btn-remove {
    background-color: #dc3545;
    color: white;
}

.btn-remove:hover {
    background-color: #c82333;
}

.drag-handle {
    color: #b0b0b0;
    cursor: grab;
    padding: 0.25rem;
}

.drag-handle:hover {
    color: #61dafb;
}

.config-actions {
    display: flex;
    gap: 0.75rem;
    padding-top: 1rem;
    border-top: 1px solid #404040;
    margin-top: auto;
}

.btn-primary {
    background-color: #61dafb;
    color: #1a1a1a;
    font-weight: 500;
}

.btn-primary:hover {
    background-color: #4fa8c5;
}

.loading {
    text-align: center;
    color: #b0b0b0;
    padding: 2rem;
    font-style: italic;
}

/* Dynamic column styles */
.dynamic-column {
    flex: 0 0 auto;
    padding: 0.5rem;
    border-right: 1px solid #404040;
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
    right: -2px;
    top: 0;
    width: 4px;
    height: 100%;
    cursor: col-resize;
    background-color: transparent;
    transition: background-color 0.2s;
    z-index: 10;
}

.column-resizer:hover {
    background-color: #61dafb;
}

.column-resizer.resizing {
    background-color: #61dafb;
}

/* Column visibility styles */
.column-visibility-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    border-radius: 4px;
    background-color: #252525;
    border: 1px solid #404040;
    transition: all 0.2s;
}

.column-visibility-item:hover {
    background-color: #303030;
    border-color: #505050;
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
    color: #e0e0e0;
}

.column-width {
    font-size: 0.875rem;
    color: #b0b0b0;
}

/* Drag and drop styles */
.column-visibility-item[draggable="true"] {
    cursor: move;
}

.column-visibility-item:hover {
    background-color: #303030;
    border-color: #505050;
}

.column-visibility-item.dragging {
    opacity: 0.5;
    transform: scale(1.02);
}

.column-visibility-item.drag-over {
    border-color: #61dafb;
    background-color: #2a4a5a;
}
}"#;

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
        this.initializeColumns();
    }

    initializeElements() {
        this.elements = {
            filterColumn: document.getElementById('filter-column'),
            filterValue: document.getElementById('filter-value'),
            clearFiltersBtn: document.getElementById('clear-filters-btn'),
            clearBtn: document.getElementById('clear-btn'),
            columnsBtn: document.getElementById('columns-btn'),
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
        
        // Column configuration event listeners
        this.elements.closeConfigBtn.addEventListener('click', () => this.hideColumnConfig());
        this.elements.applyColumnsBtn.addEventListener('click', () => this.applyColumnConfiguration());
        this.elements.showAllBtn.addEventListener('click', () => this.showAllColumns());
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
        
        console.log('=== DIMENSIONS CALCULATED ===');
        console.log('Container height:', this.virtualScroll.containerHeight + 'px');
        console.log('Measured Row height:', this.virtualScroll.rowHeight + 'px'); // Log the measured height
        console.log('Calculated visible rows:', this.virtualScroll.visibleRows);
        console.log('=============================');
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
    }

    handleResize() {
        this.calculateContainerDimensions();
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
        
        console.log(`[vScroll] Range: ${startIndex}-${endIndex}, Viewport Offset: ${viewportOffset}px`);
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
            console.log(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`);
            setTimeout(() => this.connectWebSocket(), delay);
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
        if (wasAtBottom && this.filteredLogs.length > 0) {
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
        
        console.log('=== RENDER DEBUG ===');
        console.log(`렌더링된 행: ${actualRowsRendered}개 (${firstRenderedIndex}번부터 ${lastRenderedIndex}번까지)`);
        console.log(`실제 화면에 보이는 메시지: ${firstVisibleIndex + 1}번부터 ${lastVisibleIndex + 1}번까지`);
        console.log('====================');
    }

    scrollToBottom() {
        const maxScroll = this.virtualScroll.contentHeight - this.virtualScroll.containerHeight;
        if (maxScroll > 0) {
            this.elements.virtualScrollContainer.scrollTop = maxScroll;
        }
    }

    createVirtualLogRow(log, virtualIndex) {
        const row = document.createElement('div');
        row.className = 'virtual-log-row';
        
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
        
        // Debug: log the line number vs array index for first few rows
        if (virtualIndex < 5 || virtualIndex % 100 === 0) {
            console.log(`Row ${virtualIndex}: log.line=${log.line}, arrayIndex=${virtualIndex + 1}`);
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
        
        console.log('Initialized columns from first log:', this.columns);
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

    async initializeColumns() {
        // Load initial column configuration
        try {
            const response = await fetch('/api/schema/columns');
            const tableConfig = await response.json();
            
            if (tableConfig && tableConfig.columns) {
                this.columns = tableConfig.columns;
                this.renderTableHeaders();
            }
        } catch (error) {
            console.error('Failed to initialize columns:', error);
            // Will get columns from first log entry
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
                <div class="column-visibility-item" draggable="true" data-column-index="${originalIndex}">
                    <div class="drag-handle">⋮⋮</div>
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
            <div class="dynamic-column" style="width: ${column.width}px; flex: 0 0 ${column.width}px;">
                <div class="column-header">
                    ${column.field_name}
                    ${index < visibleColumns.length - 1 ? '<div class="column-resizer" data-column="' + index + '"></div>' : ''}
                </div>
            </div>
        `).join('');
        
        this.elements.logHeaderRow.innerHTML = html;
        this.setupColumnResizers();
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
        
        console.log(`컬럼 "${this.columns[this.resizing.columnIndex].field_name}" 폭 변경: ${newWidth}px`);
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
        this.saveColumnConfiguration();
        
        this.resizing = null;
    }

    async saveColumnConfiguration() {
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
                console.log('컬럼 설정이 자동으로 저장되었습니다.');
            } else {
                console.warn('컬럼 설정 저장에 실패했습니다.');
            }
        } catch (error) {
            console.error('컬럼 설정 저장 중 오류:', error);
        }
    }

    setupColumnDragAndDrop() {
        const items = this.elements.columnVisibilityList.querySelectorAll('.column-visibility-item');
        let draggedElement = null;
        let draggedIndex = null;

        items.forEach(item => {
            item.addEventListener('dragstart', (e) => {
                draggedElement = item;
                draggedIndex = parseInt(item.dataset.columnIndex);
                item.style.opacity = '0.5';
                e.dataTransfer.effectAllowed = 'move';
            });

            item.addEventListener('dragend', (e) => {
                item.style.opacity = '';
                draggedElement = null;
                draggedIndex = null;
            });

            item.addEventListener('dragover', (e) => {
                e.preventDefault();
                e.dataTransfer.dropEffect = 'move';
            });

            item.addEventListener('drop', (e) => {
                e.preventDefault();
                if (draggedElement && draggedElement !== item) {
                    const targetIndex = parseInt(item.dataset.columnIndex);
                    this.reorderColumns(draggedIndex, targetIndex);
                }
            });
        });
    }

    reorderColumns(fromIndex, toIndex) {
        // Get the current orders
        const fromColumn = this.columns[fromIndex];
        const toColumn = this.columns[toIndex];
        
        if (!fromColumn || !toColumn) return;

        const fromOrder = fromColumn.order || 0;
        const toOrder = toColumn.order || 0;

        // Swap the orders
        fromColumn.order = toOrder;
        toColumn.order = fromOrder;

        console.log(`컬럼 순서 변경: "${fromColumn.field_name}"와 "${toColumn.field_name}" 위치 교환`);

        // Re-render the visibility list and headers
        this.renderColumnVisibilityList();
        this.renderTableHeaders();
        this.updateDisplay();
        
        // Auto-save column configuration
        this.saveColumnConfiguration();
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
    window.app = new JsonWebLogApp();
});"#;