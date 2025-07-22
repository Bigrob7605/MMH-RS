@echo off
echo Building MMH-RS Technical Specification...
echo.

REM Check if main.tex exists
if not exist main.tex (
    echo ERROR: main.tex not found!
    pause
    exit /b 1
)

REM Build the document
echo Running pdflatex...
pdflatex main.tex
if %errorlevel% neq 0 (
    echo ERROR: pdflatex failed!
    pause
    exit /b 1
)

REM Run bibtex if references.bib exists
if exist references.bib (
    echo Running bibtex...
    bibtex main
)

REM Run pdflatex again for references
echo Running pdflatex again for references...
pdflatex main.tex

echo.
echo Build completed successfully!
echo Output: main.pdf
echo.
pause 