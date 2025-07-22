@echo off
REM MMH-RS Human Launcher - Simple and Clean
setlocal

REM Force working directory to the script's location
cd /d "%~dp0"

echo.
echo ===================================================
echo           MMH-RS V1 - Human Launcher
echo ===================================================
echo.
echo Starting MMH-RS...
echo.

REM Check if we have the binary available
if exist "target\release\mmh.exe" (
    echo Found MMH-RS binary - starting...
    target\release\mmh.exe
    goto :end
)

REM If no binary, try to build
echo MMH-RS binary not found. Attempting to build...
echo.
echo Building MMH-RS...
cargo build --release
if exist "target\release\mmh.exe" (
    echo Build successful! Starting MMH-RS...
    target\release\mmh.exe
) else (
    echo.
    echo ========================================
    echo           BUILD FAILED
    echo ========================================
    echo.
    echo Please ensure you have Rust installed:
    echo Visit: https://rustup.rs/
    echo.
    echo If Rust is installed, try running:
    echo cargo build --release
    echo.
    echo For help, visit: https://github.com/Bigrob7605/MMH-RS
    echo.
    pause
)

:end
echo.
echo ========================================
echo        MMH-RS V1 COMPLETE
echo ========================================
echo.
echo Thank you for using MMH-RS V1!
echo.
pause 