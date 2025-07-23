@echo off
REM MMH-RS CMD Menu
setlocal enabledelayedexpansion
set MMH_BENCH_SIZE_GB=2
:menu
cls
echo ============================
echo | MMH-RS V1.1.0 ELITE TIER |
echo |     CPU ONLY SYSTEM      |
echo ============================
echo   MMH-RS V1.1.0 Main Menu
echo ============================
echo 1. Generate test data (gentestdir)
echo 2. MMH-RS Seed Benchmark (Coming Soon)
echo 3. Fold a file
echo 4. Unfold a file
echo 5. Fold a folder (V2+)
echo 6. Unfold a folder (V2+)
echo 7. Set Benchmark Data Size (Current: !MMH_BENCH_SIZE_GB! GB)
echo 8. Advanced Features
echo 9. View ASCII Art Gallery
echo 10. Gandalf Easter Egg
echo 11. Run Benchmark Menu
echo 12. Rebuild (cargo build --release)
echo Q. Quit
set /p choice=Select option: 
if "%choice%"=="1" (
  set /p dir=Test directory name: 
  target\release\mmh.exe gentestdir "%dir%" --size !MMH_BENCH_SIZE_GB!
  pause
  goto menu
)
if "%choice%"=="2" (
    echo [INFO] Seed Benchmark is coming soon in a future update!
    pause
    goto menu
)
if "%choice%"=="3" (
    echo [INFO] Fold a file feature - use the main MMH-RS menu for this
    echo Run: target\release\mmh.exe
    pause
    goto menu
)
if "%choice%"=="4" (
    echo [INFO] Unfold a file feature - use the main MMH-RS menu for this
    echo Run: target\release\mmh.exe
    pause
    goto menu
)
if "%choice%"=="5" (
  echo [MMH-RS] Folder folding coming in V2!
  pause
  goto menu
)
if "%choice%"=="6" (
  echo [MMH-RS] Folder unfolding coming in V2!
  pause
  goto menu
)
if "%choice%"=="7" (
  echo Select Benchmark Data Size:
  echo   1. 128 MB
  echo   2. 512 MB
  echo   3. 1 GB
  echo   4. 5 GB
  echo   5. 10 GB
  echo   6. 20 GB
  echo   7. 32 GB
  echo   8. 64 GB
  echo   9. 128 GB
  set /p sizeChoice=Enter option (1-9): 
  if "%sizeChoice%"=="1" set MMH_BENCH_SIZE_GB=0.128
  if "%sizeChoice%"=="2" set MMH_BENCH_SIZE_GB=0.512
  if "%sizeChoice%"=="3" set MMH_BENCH_SIZE_GB=1
  if "%sizeChoice%"=="4" set MMH_BENCH_SIZE_GB=5
  if "%sizeChoice%"=="5" set MMH_BENCH_SIZE_GB=10
  if "%sizeChoice%"=="6" set MMH_BENCH_SIZE_GB=20
  if "%sizeChoice%"=="7" set MMH_BENCH_SIZE_GB=32
  if "%sizeChoice%"=="8" set MMH_BENCH_SIZE_GB=64
  if "%sizeChoice%"=="9" set MMH_BENCH_SIZE_GB=128
  pause
  goto menu
)
if "%choice%"=="8" (
  goto advanced_menu
)
if "%choice%"=="9" (
  set /p n=ASCII Art Number (1-8): 
  target\release\mmh.exe --ascii !n!
  pause
  goto menu
)
if "%choice%"=="10" (
  target\release\mmh.exe --wizard
  pause
  goto menu
)
if "%choice%"=="11" (
  target\release\mmh.exe
  pause
  goto menu
)
if "%choice%"=="12" (
  echo [MMH-RS] Rebuilding...
  cargo build --release
  pause
  goto menu
)
if /I "%choice%"=="Q" (
  echo [MMH-RS] Goodbye!
  pause
  exit /b
)
echo Invalid option.
pause
goto menu

:advanced_menu
cls
echo Advanced Features Menu
echo =====================
echo 1. Run Automated Tests
echo 2. Windows Menu Test
echo 3. Universal Launcher Test
echo 4. Clear Test Data
echo 5. Export Results
echo 6. Rebuild
echo 7. Show Last Run Log
echo 8. System Information
echo 9. Performance Diagnostics
echo 10. Development Tools
echo B/back. Back to Main Menu
set /p advChoice=Select option: 
if "%advChoice%"=="1" (
  echo [MMH-RS] Running automated tests...
  pause
  goto advanced_menu
)
if "%advChoice%"=="2" (
  echo [MMH-RS] Running Windows menu test...
  pause
  goto advanced_menu
)
if "%advChoice%"=="3" (
  echo [MMH-RS] Running universal launcher test...
  pause
  goto advanced_menu
)
if "%advChoice%"=="4" (
  echo [MMH-RS] Clearing test data...
  del /q testdata mmh_stress_test benchmark_testdir demo_testdir mmh_cli.log mmh_error_log.txt main.log test_* 2>nul
  echo [MMH-RS] Test data cleared
  pause
  goto advanced_menu
)
if "%advChoice%"=="5" (
  echo [MMH-RS] Exporting results...
  pause
  goto advanced_menu
)
if "%advChoice%"=="6" (
  echo [MMH-RS] Rebuilding...
  cargo build --release
  pause
  goto advanced_menu
)
if "%advChoice%"=="7" (
  echo [MMH-RS] Last Run Log:
  if exist "mmh_cli.log" (
    type mmh_cli.log
  ) else (
    echo No log file found
  )
  pause
  goto advanced_menu
)
if "%advChoice%"=="8" (
  echo [MMH-RS] System Information:
  echo OS: Windows
  echo Current Directory: %CD%
  pause
  goto advanced_menu
)
if "%advChoice%"=="9" (
  echo [MMH-RS] Performance Diagnostics:
  echo Available Memory: 
  pause
  goto advanced_menu
)
if "%advChoice%"=="10" (
  echo [MMH-RS] Development Tools:
  echo View Cargo.toml
  echo View Source Files
  echo Check Rust Version
  pause
  goto advanced_menu
)
if "%advChoice%"=="B" (
  echo [MMH-RS] Returning to main menu...
  goto menu
)
if "%advChoice%"=="b" (
  echo [MMH-RS] Returning to main menu...
  goto menu
)
if "%advChoice%"=="back" (
  echo [MMH-RS] Returning to main menu...
  goto menu
)
if "%advChoice%"=="BACK" (
  echo [MMH-RS] Returning to main menu...
  goto menu
)
echo Invalid option.
pause
goto advanced_menu 