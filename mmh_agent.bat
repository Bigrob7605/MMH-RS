@echo off
REM MMH-RS Agent Launcher - For Automated Testing
setlocal

REM Force working directory to the script's location
cd /d "%~dp0"

echo.
echo ===================================================
echo           MMH-RS V1 - Agent Launcher
echo ===================================================
echo.
echo Running MMH-RS Agent...
echo.

REM Check if we have the binary available
if exist "target\release\mmh.exe" (
    echo Found MMH-RS binary - running agent...
    target\release\mmh.exe --agent
    goto :end
)

REM If no binary, try to build
echo MMH-RS binary not found. Attempting to build...
echo.
echo Building MMH-RS...
cargo build --release
if exist "target\release\mmh.exe" (
    echo Build successful! Running agent...
    target\release\mmh.exe --agent
) else (
    echo.
    echo ========================================
    echo           BUILD FAILED
    echo ========================================
    echo.
    echo Agent cannot run without MMH-RS binary.
    echo Please ensure Rust is installed and build succeeds.
    echo.
    exit /b 1
)

:end
echo.
echo ========================================
echo        MMH-RS AGENT COMPLETE
echo ========================================
echo. 