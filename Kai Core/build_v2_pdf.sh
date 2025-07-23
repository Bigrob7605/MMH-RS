#!/bin/bash

echo "======================================================================"
echo "Kai Core V2.0 - Self-Auditing Master Document Builder"
echo "======================================================================"
echo

echo "🚀 Starting Kai Core V2.0 PDF compilation..."
echo

# Check if LaTeX is installed
if ! command -v pdflatex &> /dev/null; then
    echo "❌ ERROR: pdflatex not found. Please install LaTeX distribution."
    echo "   Recommended: TeX Live or MiKTeX"
    exit 1
fi

echo "✅ LaTeX found. Proceeding with compilation..."
echo

# Create output directory
mkdir -p output

# First compilation pass
echo "📄 First compilation pass..."
pdflatex -interaction=nonstopmode -output-directory=output KAI_CORE_V2_MASTER.tex
if [ $? -ne 0 ]; then
    echo "❌ First compilation failed"
    exit 1
fi

# Generate glossary
echo "📚 Generating glossary..."
makeglossaries -d output KAI_CORE_V2_MASTER
if [ $? -ne 0 ]; then
    echo "⚠️ Glossary generation failed, continuing..."
fi

# Generate index
echo "📇 Generating index..."
makeindex -s output/KAI_CORE_V2_MASTER.idx -o output/KAI_CORE_V2_MASTER.ind
if [ $? -ne 0 ]; then
    echo "⚠️ Index generation failed, continuing..."
fi

# Second compilation pass
echo "📄 Second compilation pass..."
pdflatex -interaction=nonstopmode -output-directory=output KAI_CORE_V2_MASTER.tex
if [ $? -ne 0 ]; then
    echo "❌ Second compilation failed"
    exit 1
fi

# Third compilation pass (for final references)
echo "📄 Third compilation pass (final references)..."
pdflatex -interaction=nonstopmode -output-directory=output KAI_CORE_V2_MASTER.tex
if [ $? -ne 0 ]; then
    echo "❌ Third compilation failed"
    exit 1
fi

# Check if PDF was created
if [ -f "output/KAI_CORE_V2_MASTER.pdf" ]; then
    echo
    echo "✅ SUCCESS: Kai Core V2.0 PDF created successfully!"
    echo
    echo "📁 Output location: output/KAI_CORE_V2_MASTER.pdf"
    echo
    
    # Get file size
    file_size=$(stat -c%s "output/KAI_CORE_V2_MASTER.pdf" 2>/dev/null || stat -f%z "output/KAI_CORE_V2_MASTER.pdf" 2>/dev/null)
    echo "📊 File size: $file_size bytes"
    
    echo
    echo "🎉 Kai Core V2.0 - Self-Auditing Master Document is ready!"
    echo
    echo "🚀 Features included:"
    echo "   ✅ Recursive Intelligence Language (RIL v7)"
    echo "   ✅ Meta-Memory Hologram (MMH) integration"
    echo "   ✅ Paradox resolution system"
    echo "   ✅ Observer pattern monitoring"
    echo "   ✅ GPU acceleration examples"
    echo "   ✅ Self-auditing capabilities"
    echo "   ✅ Performance benchmarks"
    echo "   ✅ Troubleshooting guides"
    echo "   ✅ Future roadmap"
    echo "   ✅ Interactive code examples"
    echo
    
    # Open the PDF if possible
    echo "🔍 Opening PDF..."
    if command -v xdg-open &> /dev/null; then
        xdg-open "output/KAI_CORE_V2_MASTER.pdf" &
    elif command -v open &> /dev/null; then
        open "output/KAI_CORE_V2_MASTER.pdf" &
    else
        echo "📖 PDF ready for manual opening"
    fi
    
else
    echo "❌ ERROR: PDF was not created"
    echo
    echo "🔍 Check the log files in the output directory for details"
    exit 1
fi

echo
echo "======================================================================"
echo "Kai Core V2.0 compilation complete!"
echo "======================================================================" 