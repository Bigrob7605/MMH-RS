# MMH-RS V1.0.2 — OFFICIAL RELEASE 🚀

**ORCID: 0009-0008-4352-6842**

## 🏆 **Final Gold Standard Basemarks: 50MB: 2.01x | 1GB: 2.17x | 2GB: 2.15x**

**MMH-RS V1.0.2 is a world-class, deterministic file compression engine with legendary CLI/UX and unmatched transparency. All benchmarks use realistic AI/user data mix—not synthetic or random-only data.**

**MMH-RS V1 is a world-class, deterministic file compression engine with legendary CLI/UX and unmatched transparency.**

---

## 🚀 **Quick Start**

**TL;DR:** Download, run `cargo build --release`, then use the universal launchers.

### **Windows (Recommended)**
```powershell
# Clone and build
git clone https://github.com/Bigrob7605/MMH-RS
cd MMH-RS
cargo build --release

# Run the human launcher (recommended)
.\mmh_human.bat

# Or run the agent launcher (automated testing)
.\mmh_agent.bat

# Or direct CLI
.\target\release\mmh.exe

# Alternative menu systems
.\mmh_menu.ps1      # PowerShell menu
.\mmh_cmdmenu.bat   # CMD menu
```

### **Linux/macOS**
```bash
# Clone and build
git clone https://github.com/Bigrob7605/MMH-RS
cd MMH-RS
cargo build --release

# Run the universal launcher
./mmh.sh

# Or direct CLI
./target/release/mmh
```

### **Prebuilt Binaries (Coming Soon)**
**Windows:** Download `mmh.exe` from releases  
**Linux:** Download AppImage from releases  
**macOS:** Download `.dmg` from releases

### **Quick Test**
Try the included sample file:
```bash
# Test compression
mmh pack sample_data.txt sample_data.mmh

# Test decompression  
mmh unpack sample_data.mmh restored.txt

# Verify integrity
mmh verify sample_data.txt restored.txt
```

### **Universal Launcher System**

**Windows:**
- `mmh_human.bat` - **Human user launcher** (recommended for new users)
- `mmh_agent.bat` - **Automated testing launcher** (for developers)
- `mmh_menu.ps1` - Full PowerShell menu system
- `mmh_cmdmenu.bat` - CMD menu system

**macOS/Linux:**
- `mmh.sh` - **Universal launcher** with platform detection

**All Platforms:**
- `mmh.exe` / `mmh` - Direct CLI binary

---

## 🎯 **Main Menu System**

**MMH-RS V1 features a comprehensive menu system:**

### **Main Menu (Clean & Simple):**
1. **Benchmark Menu (Try MMH File System)** - Performance testing with 9 tiers
2. **Generate test data** - Create test datasets
3. **Fold a file** - Compress single files
4. **Unfold a file** - Decompress files
5. **Advanced Features** - Development & testing tools
6. **Full MMH-RS CLI** - Direct command-line access

### **Benchmark Menu (9 Performance Tiers):**
1. **Smoketest (1MB)** - Quick validation
2. **Toasty (2GB)** - Standard testing
3. **Hot (5GB)** - Performance validation
4. **Blazing (10GB)** - Stress testing
5. **Inferno (25GB)** - High-performance testing
6. **Plasma (50GB)** - Enterprise testing
7. **Fusion (100GB)** - Data center testing
8. **Nova (250GB)** - Extreme testing
9. **RAMpocalypse (500GB)** - Maximum stress testing

### **Advanced Features Menu (Development Tools):**
1. **Run Automated Tests** - Complete test suite execution
2. **Self-Test** - System validation
3. **Clear Test Data** - Clean up test files
4. **Rebuild** - Compile from source

---

## 📊 **Performance Benchmarks**

### **Final Gold Standard Benchmark Results (RTX 4070 8GB, 64GB RAM, 4TB SSD, Win 11):**

- [50MB Realistic Test](benchmarks/realistic_50MB_benchmark_result.txt) - 2.01x compression ratio
- [1GB Realistic Test](benchmarks/realistic_1GB_benchmark_result.txt) - 2.17x compression ratio
- [2GB Realistic Test](benchmarks/realistic_2GB_benchmark_result.txt) - 2.15x compression ratio

**Key Performance Metrics:**
- **Compression Speed:** 121.59 MB/s
- **Decompression Speed:** 572.20 MB/s
- **Compression Ratio:** 2.01-2.17x (realistic AI/user data mix)
- **Integrity Check:** PASS
- **System:** Windows 11, RTX 4070, 64GB RAM, 4TB SSD

**Data Recipe:** All benchmarks use realistic AI model weights (15%), text documents (15%), source code (15%), JSON configs (15%), images (15%), logs (10%), and mixed data (15%) - reflecting actual user experience, not synthetic random data.

### **Benchmark Features:**
- **9 Performance Tiers** - From 1MB to 500GB
- **Realistic Data Generation** - AI models, text docs, code, JSON, images, logs (not random noise)
- **Abort Support** - Ctrl+C to stop any benchmark
- **Result Saving** - Save to TXT, JSON, or LOG formats
- **Integrity Verification** - SHA-256 hash validation
- **System Metrics** - CPU, memory, and storage monitoring

---

## 🔧 **Core Features**

### **Compression Engine**
- **Deterministic compression** - Same input always produces same output
- **Zstd integration** - Industry-leading compression algorithm
- **File tax optimization** - Efficient handling of small files
- **Cross-platform** - Windows, Linux, macOS support

### **CLI Interface**
- **Interactive menus** - User-friendly navigation
- **Direct commands** - Full CLI access
- **Abort support** - Graceful interruption handling
- **Progress indicators** - Real-time operation feedback

### **Testing & Validation**
- **Automated test suite** - Comprehensive validation
- **Self-test system** - Built-in diagnostics
- **Benchmark system** - Performance measurement
- **Integrity verification** - SHA-256 hash checking

### **File Operations**
- **Pack/Unpack** - Compress and decompress files
- **Directory support** - Handle entire directories
- **Seed generation** - Deterministic file identification
- **Verification** - Ensure data integrity

---

## 📚 **Documentation**

### **User Guides**
- [Launcher Guide](LAUNCHER_GUIDE.md) - How to use the launchers
- [Build Instructions](README_BUILD.md) - Compilation guide
- [Benchmarks](BENCHMARKS.md) - Performance testing guide

### **Technical Documentation**
- [CHANGELOG](CHANGELOG.md) - Release notes and updates
- [Examples](examples/) - Code examples and tutorials
- [Python Bindings](python/) - Python integration

---

## 🛠 **Development**

### **Building from Source**
```bash
# Clone repository
git clone https://github.com/Bigrob7605/MMH-RS
cd MMH-RS

# Build release version
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### **Dependencies**
- **Rust** 1.70+ (for compilation)
- **Zstd** (for compression)
- **Windows:** PowerShell 5.1+ (for menus)
- **Linux/macOS:** POSIX shell

### **Project Structure**
```
MMH-RS/
├── src/                    # Rust source code
├── target/release/         # Compiled binary
├── benchmarks/             # Benchmark results
├── examples/               # Code examples
├── docs/                   # Documentation
├── tests/                  # Test suite
├── mmh_human.bat          # Human launcher (Windows)
├── mmh_agent.bat          # Agent launcher (Windows)
├── mmh.sh                 # Universal launcher (Unix)
└── README.md              # This file
```

---

## 🎯 **V1 Core Deliverables**

**MMH-RS V1 focuses on 3 core deliverables:**

1. **✅ Benchmark System** - Complete performance testing with 9 tiers
2. **✅ 10GB MMH File System Demo** - Showcase compression capabilities
3. **✅ Full CLI Commands** - Complete command-line interface

**All other features are planned for V2+**

---

## 🤝 **Contributing**

### **Getting Started**
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run the test suite: `.\mmh_agent.bat`
5. Submit a pull request

### **Testing**
- **Automated tests:** `.\mmh_agent.bat`
- **Manual tests:** `.\mmh_human.bat`
- **Benchmarks:** Use the benchmark menu

### **Code Style**
- Follow Rust conventions
- Add tests for new features
- Update documentation
- Run `cargo fmt` and `cargo clippy`

---

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 **Acknowledgments**

- **Zstd team** - For the excellent compression library
- **Rust community** - For the amazing language and ecosystem
- **Open source contributors** - For inspiration and tools

---

## 📞 **Contact**

- **Author:** Robert Long
- **Email:** Screwball7605@aol.com
- **GitHub:** [Bigrob7605](https://github.com/Bigrob7605)
- **ORCID:** 0009-0008-4352-6842

---

**MMH-RS V1 - Precision Compression Engine**  
**Ready for production use! 🚀**
