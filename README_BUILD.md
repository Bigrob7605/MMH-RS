# MMH-RS Build Guide

**ORCID: 0009-0008-4352-6842**

## 🚀 **Quick Start**

### **Prerequisites**
- **Rust** 1.70+ ([rustup.rs](https://rustup.rs))
- **Windows 10/11, Linux, or macOS**
- **Git** for cloning the repository

### **Windows Build**
```powershell
# Clone the repository
git clone https://github.com/Bigrob7605/MMH-RS
cd MMH-RS

# Build the project
cargo build --release

# Run the human launcher
.\mmh_human.bat

# Or run the agent launcher (automated testing)
.\mmh_agent.bat
```

### **Linux/macOS Build**
```bash
# Clone the repository
git clone https://github.com/Bigrob7605/MMH-RS
cd MMH-RS

# Build the project
cargo build --release

# Run the universal launcher
./mmh.sh

# Or run directly
./target/release/mmh
```

### **Documentation Build (Optional)**
```bash
# Install LaTeX (if not already installed)
# Windows: Install MiKTeX
# Linux: sudo apt-get install texlive-full
# macOS: brew install --cask mactex

# Build the PDF documentation
cd overleaf
pdflatex mmh-rs-overleaf.tex
cd ..
copy overleaf\mmh-rs-overleaf.pdf main.pdf
```

---

## 📁 **Project Structure**

```
MMH-RS/
├── src/                    # Rust source code
│   ├── main.rs            # Main entry point
│   ├── lib.rs             # Library interface
│   ├── cli.rs             # CLI implementation
│   ├── bench.rs           # Benchmark system
│   └── cli/
│       └── agent.rs       # Automated testing agent
├── target/release/         # Compiled binary
│   └── mmh.exe            # Production executable
├── benchmarks/             # Benchmark results
│   ├── agent_2GB_result.txt
│   └── human_2GB_result.txt
├── examples/               # Code examples
├── docs/                   # Documentation
├── tests/                  # Test suite
├── overleaf/               # LaTeX documentation
│   ├── mmh-rs-overleaf.tex
│   └── mmh-rs-overleaf.pdf
├── mmh_human.bat          # Human launcher (Windows)
├── mmh_agent.bat          # Agent launcher (Windows)
├── mmh.sh                 # Universal launcher (Unix)
├── mmh_menu.ps1           # PowerShell menu
├── mmh_cmdmenu.bat        # CMD menu
├── Cargo.toml             # Rust project configuration
├── Cargo.lock             # Dependency lock file
├── README.md              # Main documentation
├── CHANGELOG.md           # Release notes
├── LAUNCHER_GUIDE.md      # Launcher documentation
├── BENCHMARKS.md          # Benchmark documentation
└── main.pdf               # Complete technical specification
```

---

## 🔧 **Build Process**

### **1. Rust Compilation**
```bash
# Development build (faster, includes debug info)
cargo build

# Release build (optimized, production-ready)
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### **2. Binary Location**
After building, the executable is located at:
- **Windows:** `target\release\mmh.exe`
- **Linux/macOS:** `target/release/mmh`

### **3. Verification**
```bash
# Check version
./target/release/mmh --version

# Run self-test
./target/release/mmh selftest

# Show help
./target/release/mmh --help
```

---

## 📊 **V1.0 Build Status**

### **✅ Completed Components**
- [x] **Rust Core** - Complete compression engine
- [x] **CLI Interface** - Full command-line interface
- [x] **Benchmark System** - 9 performance tiers
- [x] **Testing Suite** - Automated validation
- [x] **Launcher System** - Universal launchers
- [x] **Documentation** - Complete user guides
- [x] **PDF Specification** - Technical documentation
- [x] **Examples** - Code examples and tutorials

### **✅ Build Features**
- [x] **Cross-Platform** - Windows, Linux, macOS
- [x] **Release Builds** - Optimized production binaries
- [x] **Dependency Management** - Cargo.lock for reproducibility
- [x] **Error Handling** - Robust error recovery
- [x] **Logging System** - Comprehensive logging
- [x] **Testing Framework** - Automated test suite

---

## 🧪 **Testing**

### **Automated Testing**
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration

# Run with output
cargo test -- --nocapture
```

### **Manual Testing**
```bash
# Run the agent launcher (automated testing)
.\mmh_agent.bat

# Run the human launcher (manual testing)
.\mmh_human.bat

# Test specific features
./target/release/mmh selftest
./target/release/mmh pack test.txt test.mmh
./target/release/mmh unpack test.mmh restored.txt
./target/release/mmh verify test.txt restored.txt
```

---

## 🐛 **Troubleshooting**

### **Common Issues**

#### **Build Errors**
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release

# Check dependencies
cargo check
```

#### **Missing Dependencies**
```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install build tools (Windows)
# Install Visual Studio Build Tools

# Install build tools (Linux)
sudo apt-get install build-essential

# Install build tools (macOS)
xcode-select --install
```

#### **Permission Issues (Linux/macOS)**
```bash
# Make launcher executable
chmod +x mmh.sh

# Make binary executable
chmod +x target/release/mmh
```

#### **LaTeX Build Issues**
```bash
# Install LaTeX packages
# Windows: MiKTeX Console
# Linux: sudo apt-get install texlive-full
# macOS: brew install --cask mactex

# Clean LaTeX build
rm -f overleaf/*.aux overleaf/*.log overleaf/*.out overleaf/*.toc
```

---

## 📈 **Performance**

### **Build Performance**
- **Development Build:** ~30 seconds
- **Release Build:** ~2 minutes
- **Test Suite:** ~10 seconds
- **Documentation Build:** ~1 minute

### **Runtime Performance**
- **Compression Speed:** 121.59 MB/s
- **Decompression Speed:** 572.20 MB/s
- **Memory Usage:** ~50MB
- **Startup Time:** <100ms

---

## 🔄 **Development Workflow**

### **1. Make Changes**
```bash
# Edit source code
code src/

# Edit documentation
code README.md
code docs/
```

### **2. Test Changes**
```bash
# Run tests
cargo test

# Run agent testing
.\mmh_agent.bat

# Manual testing
.\mmh_human.bat
```

### **3. Build and Deploy**
```bash
# Build release
cargo build --release

# Update documentation
cd overleaf
pdflatex mmh-rs-overleaf.tex
cd ..
copy overleaf\mmh-rs-overleaf.pdf main.pdf

# Commit changes
git add .
git commit -m "Description of changes"
git push
```

---

## 📚 **Documentation**

### **User Documentation**
- [README.md](README.md) - Main project overview
- [LAUNCHER_GUIDE.md](LAUNCHER_GUIDE.md) - Launcher system guide
- [BENCHMARKS.md](BENCHMARKS.md) - Performance testing guide
- [CHANGELOG.md](CHANGELOG.md) - Release notes

### **Technical Documentation**
- [main.pdf](main.pdf) - Complete technical specification
- [docs/future.md](docs/future.md) - V2.0 and V3.0 roadmap
- [examples/](examples/) - Code examples and tutorials

### **API Documentation**
```bash
# Generate API documentation
cargo doc --open
```

---

## 🤝 **Contributing**

### **Development Setup**
1. Fork the repository
2. Clone your fork
3. Create a feature branch
4. Make your changes
5. Run tests: `cargo test`
6. Run agent: `.\mmh_agent.bat`
7. Submit a pull request

### **Code Style**
- Follow Rust conventions
- Run `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Add tests for new features
- Update documentation

---

**MMH-RS V1.0 is production-ready! The build process is streamlined and reliable. 🚀** 