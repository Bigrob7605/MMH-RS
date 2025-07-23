# MMH-RS V1.2.0 User Guide

## Quick Start Guide

### Launching the System
```bash
# Windows (PowerShell)
cargo build --release
./target/release/mmh.exe

# Windows (Universal Launcher)
mmh_universal.bat

# Linux/macOS
cargo build --release
./target/release/mmh

# Linux/macOS (Universal Launcher)
./mmh.sh
```

### Benchmark Menu Options
1. **Smoketest** (~50 MiB) - Quick validation
2. **Toasty** (2 GiB) - Standard testing
3. **Scorched** (8 GiB) - Performance testing
4. **Nuked** (32 GiB) - Extended testing (Baseline)
5. **Total Annihilation** (128 GiB) - Stress testing
6. **Memory Madness** (256 GiB) - Extreme testing
7. **Swapocalypse** (512 GiB) - System testing
8. **RAMpocalypse** (1 TiB) - Ultimate testing

---

## Performance Baseline

### Validation System Performance
- **System**: Intel i7-13620H + RTX 4070 + 64GB RAM
- **OS**: Windows 11 Home (24H2) with WSL
- **32GB Benchmark Score**: 83/100 (High-end laptop tier)
- **Compression Ratio**: 2.15x (CPU-only implementation)
- **Pack Speed**: 54.0 MB/s
- **Unpack Speed**: 47.7 MB/s
- **Total Time**: 20.6 minutes for 32GB test

### Performance Expectations by System Tier
- **Entry Level**: 20-40 MB/s, 1.5-2.0x compression
- **Mainstream**: 30-50 MB/s, 1.8-2.2x compression
- **High End**: 45-60 MB/s, 2.0-2.5x compression
- **Enterprise**: 50-80 MB/s, 2.2-3.0x compression
- **Development**: 60+ MB/s, 2.5+ compression

---

## V1.2.0 Features

### Data Integrity
- **Bit-for-bit verification**: SHA-256 + Merkle tree validation
- **Extension preservation**: Original file extensions perfectly maintained
- **Deterministic output**: Consistent compression results every time
- **Self-healing**: RaptorQ FEC corruption recovery

### Real-Time Progress Monitoring
- **Speed Displays**: MB/s, GB/s, TB/s in real-time
- **File Count**: Shows files created during generation
- **Phase Indicators**: Clear status for each operation
- **Progress Bars**: Visual feedback for all operations
- **Spinning Indicators**: Continuous visual feedback in menus

### Abort Functionality
- **Ctrl+C**: Immediately stops any operation
- **Phase Awareness**: Knows which phase was aborted
- **Clean Exit**: No hanging processes
- **Status Reports**: Proper aborted report generation

### Size-Prefixed Logging
- **Unique Filenames**: `32G-test_results_2025-07-22_19-29-49.txt`
- **Multiple Formats**: Text, JSON, and log outputs
- **Organized Storage**: Easy to find and compare results
- **Timestamped**: Automatic timestamp generation

### Universal Format
- **Open CBOR "seed pack" format**: Interoperable and extensible
- **128-bit "Digital DNA"**: Unique identifier for each compressed file
- **Cross-platform compatibility**: Windows, Linux, macOS

---

## Testing Protocol

### Recommended Testing Sequence

#### **Step 1: Smoketest Validation**
```bash
Select tier: 1
# Validates system is working correctly
# ~50 MiB test, completes quickly
# Expected: 1-2 minutes, 1.5-2.0x compression
```

#### **Step 2: Standard Performance Test**
```bash
Select tier: 2
# 2 GiB test for baseline performance
# Expected: 5-10 minutes, 1.8-2.2x compression
```

#### **Step 3: Gold Standard Test**
```bash
Select tier: 4 (Nuked - 32 GiB)
# ⭐ RECOMMENDED: Gold standard baseline
# Expected: 15-30 minutes, 2.0-2.5x compression
# Target score: 80+ for high-end systems
```

#### **Step 4: Stress Testing (Optional)**
```bash
Select tier: 5+ (128 GiB+)
# For systems with 32GB+ RAM
# Extended testing for performance validation
```

---

## 🚀 **V1.2.0 ELITE TIER CAPABILITIES**

### **✅ WHAT MMH-RS V1.2.0 DOES BEST**
- **Text files**: .txt, .md, .json, .csv, .xml, .html (2-4x compression)
- **Log files**: Application logs, system logs, debug output
- **Code files**: Source code, scripts, configuration files
- **Raw images**: .bmp, .tiff, uncompressed formats
- **AI model weights**: Neural network parameters, training data
- **Databases**: SQL dumps, data exports
- **Any file type**: Perfect integrity preservation

### **⚠️ LIMITATIONS (Expected Behavior)**
- **Already-compressed files**: .mp4, .jpg, .png, .mp3, .zip (may expand slightly)
- **Encrypted files**: Random data that can't be compressed
- **Binary executables**: Limited compression potential

### **🔍 Understanding "Random Data Detected"**
When you see this message:
```
Random data detected - expansion is normal and expected. This is not a bug.
```

**This is NOT an error!** It means:
- Your file is already highly compressed or contains random data
- MMH-RS cannot compress it further (mathematically impossible)
- The file might grow slightly due to metadata overhead
- **This is normal behavior for all compression tools**

---

## 🛠 **TROUBLESHOOTING**

### **✅ COMMON ISSUES & SOLUTIONS**

#### **Memory Issues**
- **Problem**: "Not enough memory" for large tests
- **Solution**: Use smaller tiers (1-3) or increase system RAM
- **Note**: V1.2.0 is CPU-focused, V2 will add GPU acceleration

#### **Windows 11/WSL Compatibility**
- **Problem**: Memory reporting differences
- **Solution**: Use Windows native mode for most accurate results
- **Note**: WSL mode works but may show different memory usage

#### **Performance Expectations**
- **Problem**: Slower than expected compression
- **Solution**: V1.2.0 is CPU-only, V2 will add GPU acceleration
- **Note**: 50-60 MB/s is excellent for CPU-only compression

#### **File Type Limitations**
- **Problem**: Some files don't compress well
- **Solution**: This is expected for already-compressed files
- **Note**: MMH-RS still provides perfect integrity preservation

---

## 🎯 **AFTER RUNNING BENCHMARKS**

### **✅ WHAT TO DO WITH YOUR RESULTS**

#### **Save Your Log Files**
- **Location**: Automatically saved in project directory
- **Format**: `[SIZE]-test_results_[TIMESTAMP].txt`
- **Content**: Complete benchmark results and system info

#### **Compare to Gold Standard**
- **Baseline**: 83/100 on 32GB test (UniversalTruth system)
- **Your Score**: Compare your results to the baseline
- **Hardware**: Consider your system specifications

#### **Share Results**
- **GitHub**: Submit results for community comparison
- **Documentation**: Help improve performance expectations
- **Feedback**: Report any issues or improvements

---

## 🚧 **ROADMAP PREVIEW**

### **🎯 V2 (Q3 2025): GPU Acceleration**
- **GPU Support**: NVIDIA, AMD, Apple M-series
- **Speed Boost**: 10x–50x faster compression/decompression
- **Directory Support**: Full directory compression
- **Real-time Monitoring**: Heat/throttle monitoring

### **🎯 V3 (Q4 2025+): AI Integration**
- **AI Model Folding**: Compress entire AI models
- **RGIG Integration**: Reality-Grade Intelligence Gauntlet
- **Quantum Codecs**: Advanced compression algorithms
- **Universal AI File System**: Complete AI ecosystem

---

## 📞 **SUPPORT & CONTACT**

- **Email**: Screwball7605@aol.com
- **GitHub**: [Bigrob7605/MMH-RS](https://github.com/Bigrob7605/MMH-RS)
- **Documentation**: Complete technical and user guides available

---

**MMH-RS V1.2.0 ELITE TIER - Production-ready with perfect bit-for-bit integrity!** 