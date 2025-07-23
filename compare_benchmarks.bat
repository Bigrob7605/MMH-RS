@echo off
title MMH-RS V1.2.0 - Benchmark Comparison Tool
color 0B

echo.
echo ================================================================================
echo                    MMH-RS V1.2.0 - BENCHMARK COMPARISON TOOL
echo                           SIDE-BY-SIDE ANALYSIS
echo ================================================================================
echo.
echo This tool will compare two benchmark reports side by side.
echo.
echo Available comparison methods:
echo   1. Compare two compact reports (recommended)
echo   2. Compare two detailed reports
echo   3. Compare with your latest benchmark
echo.
set /p choice="Select comparison method (1-3): "

if "%choice%"=="1" (
    echo.
    echo Please provide the paths to two compact_report.txt files:
    set /p file1="First compact report: "
    set /p file2="Second compact report: "
    echo.
    echo Comparing %file1% vs %file2%...
    echo.
    echo ================================================================================
    echo                                COMPARISON RESULTS
    echo ================================================================================
    echo.
    echo FIRST REPORT:
    echo ----------------------------------------
    type "%file1%"
    echo.
    echo SECOND REPORT:
    echo ----------------------------------------
    type "%file2%"
    echo.
    echo ================================================================================
    echo                           COMPARISON COMPLETE
    echo ================================================================================
) else if "%choice%"=="2" (
    echo.
    echo Please provide the paths to two detailed report.txt files:
    set /p file1="First detailed report: "
    set /p file2="Second detailed report: "
    echo.
    echo Comparing %file1% vs %file2%...
    echo.
    echo ================================================================================
    echo                                COMPARISON RESULTS
    echo ================================================================================
    echo.
    echo FIRST REPORT:
    echo ----------------------------------------
    type "%file1%"
    echo.
    echo SECOND REPORT:
    echo ----------------------------------------
    type "%file2%"
    echo.
    echo ================================================================================
    echo                           COMPARISON COMPLETE
    echo ================================================================================
) else if "%choice%"=="3" (
    echo.
    echo Finding your latest benchmark...
    for /f "delims=" %%i in ('dir /b /od bench_reports\*\compact_report.txt 2^>nul') do set latest=%%i
    if defined latest (
        echo Latest benchmark found: %latest%
        echo.
        echo Please provide the path to another compact report to compare with:
        set /p file2="Other compact report: "
        echo.
        echo Comparing %latest% vs %file2%...
        echo.
        echo ================================================================================
        echo                                COMPARISON RESULTS
        echo ================================================================================
        echo.
        echo YOUR LATEST BENCHMARK:
        echo ----------------------------------------
        type "%latest%"
        echo.
        echo OTHER BENCHMARK:
        echo ----------------------------------------
        type "%file2%"
        echo.
        echo ================================================================================
        echo                           COMPARISON COMPLETE
        echo ================================================================================
    ) else (
        echo No benchmark reports found. Run a benchmark first!
    )
) else (
    echo Invalid choice. Please run the script again.
)

echo.
pause 