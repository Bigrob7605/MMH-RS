@echo off
title MMH-RS V1.2.0 - Generate Shareable Benchmark
color 0A

echo.
echo ================================================================================
echo                    MMH-RS V1.2.0 - SHAREABLE BENCHMARK GENERATOR
echo                           CROSS-SYSTEM COMPARISON READY
echo ================================================================================
echo.
echo This will generate a compact, shareable benchmark report for cross-system comparison.
echo.
echo Available test sizes:
echo   1. Smoketest (~50 MiB) - Quick validation
echo   2. Standard (2 GB) - Recommended for comparison
echo   3. Extended (32 GB) - Gold standard baseline
echo   4. Custom size
echo.
set /p choice="Select test size (1-4): "

if "%choice%"=="1" goto smoketest
if "%choice%"=="2" goto standard
if "%choice%"=="3" goto extended
if "%choice%"=="4" goto custom
goto standard

:smoketest
echo.
echo Running Smoketest (~50 MiB)...
target\release\mmh.exe goldbench --size 0 --format compact
goto end

:standard
echo.
echo Running Standard Test (2 GB)...
target\release\mmh.exe goldbench --size 2 --format compact
goto end

:extended
echo.
echo Running Extended Test (32 GB) - This may take 15-30 minutes...
target\release\mmh.exe goldbench --size 32 --format compact
goto end

:custom
echo.
set /p custom_size="Enter custom size in GB: "
echo Running Custom Test (%custom_size% GB)...
target\release\mmh.exe goldbench --size %custom_size% --format compact
goto end

:end

echo.
echo ================================================================================
echo                           BENCHMARK COMPLETE!
echo ================================================================================
echo.
echo Your shareable benchmark report has been generated!
echo.
echo Files created:
echo   - compact_report.txt (for sharing)
echo   - report.txt (detailed version)
echo   - report.json (machine-readable)
echo.
echo Share the compact_report.txt file to compare with other users!
echo.
pause 