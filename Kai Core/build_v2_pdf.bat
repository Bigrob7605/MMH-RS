@echo off
echo ======================================================================
echo Kai Core V2.0 - Self-Auditing Master Document Builder
echo ======================================================================
echo.

echo 🚀 Starting Kai Core V2.0 PDF compilation...
echo.

REM Check if LaTeX is installed
where pdflatex >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ ERROR: pdflatex not found. Please install LaTeX distribution.
    echo    Recommended: MiKTeX or TeX Live
    pause
    exit /b 1
)

echo ✅ LaTeX found. Proceeding with compilation...
echo.

REM Create output directory
if not exist "output" mkdir output

REM First compilation pass
echo 📄 First compilation pass...
pdflatex -interaction=nonstopmode -output-directory=output KAI_CORE_V2_MASTER.tex
if %errorlevel% neq 0 (
    echo ❌ First compilation failed
    pause
    exit /b 1
)

REM Generate glossary
echo 📚 Generating glossary...
makeglossaries -d output KAI_CORE_V2_MASTER
if %errorlevel% neq 0 (
    echo ⚠️ Glossary generation failed, continuing...
)

REM Generate index
echo 📇 Generating index...
makeindex -s output/KAI_CORE_V2_MASTER.idx -o output/KAI_CORE_V2_MASTER.ind
if %errorlevel% neq 0 (
    echo ⚠️ Index generation failed, continuing...
)

REM Second compilation pass
echo 📄 Second compilation pass...
pdflatex -interaction=nonstopmode -output-directory=output KAI_CORE_V2_MASTER.tex
if %errorlevel% neq 0 (
    echo ❌ Second compilation failed
    pause
    exit /b 1
)

REM Third compilation pass (for final references)
echo 📄 Third compilation pass (final references)...
pdflatex -interaction=nonstopmode -output-directory=output KAI_CORE_V2_MASTER.tex
if %errorlevel% neq 0 (
    echo ❌ Third compilation failed
    pause
    exit /b 1
)

REM Check if PDF was created
if exist "output\KAI_CORE_V2_MASTER.pdf" (
    echo.
    echo ✅ SUCCESS: Kai Core V2.0 PDF created successfully!
    echo.
    echo 📁 Output location: output\KAI_CORE_V2_MASTER.pdf
    echo.
    
    REM Get file size
    for %%A in ("output\KAI_CORE_V2_MASTER.pdf") do (
        echo 📊 File size: %%~zA bytes
    )
    
    echo.
    echo 🎉 Kai Core V2.0 - Self-Auditing Master Document is ready!
    echo.
    echo 🚀 Features included:
    echo    ✅ Recursive Intelligence Language (RIL v7)
    echo    ✅ Meta-Memory Hologram (MMH) integration
    echo    ✅ Paradox resolution system
    echo    ✅ Observer pattern monitoring
    echo    ✅ GPU acceleration examples
    echo    ✅ Self-auditing capabilities
    echo    ✅ Performance benchmarks
    echo    ✅ Troubleshooting guides
    echo    ✅ Future roadmap
    echo    ✅ Interactive code examples
    echo.
    
    REM Open the PDF if possible
    echo 🔍 Opening PDF...
    start "" "output\KAI_CORE_V2_MASTER.pdf"
    
) else (
    echo ❌ ERROR: PDF was not created
    echo.
    echo 🔍 Check the log files in the output directory for details
    pause
    exit /b 1
)

echo.
echo ======================================================================
echo Kai Core V2.0 compilation complete!
echo ======================================================================
pause 