@echo off
setlocal enabledelayedexpansion

REM Set port number (default: 3000)
set "PORT=%~1"
if "%PORT%"=="" set "PORT=3000"

echo ==========================================
echo JsonWebLog Test Script
echo Requested Port: %PORT%
echo ==========================================
echo.

REM Build check
echo [1/2] Building project...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)
echo Build completed successfully
echo.

echo.
echo [2/2] Starting test log data streaming...
echo.
echo ==========================================
echo Test Features:
echo - Check web interface in browser
echo - Verify real-time log updates
echo - Test filtering (level, search, logger, module)
echo - Check WebSocket connection status
echo ==========================================
echo.

REM Stream test logs
echo Streaming test logs... (Press Ctrl+C to stop)
echo.
type temp\sample.log | cargo run --release %PORT%

echo.
echo ==========================================
echo Test completed!
echo ==========================================
echo.
