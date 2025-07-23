# MMH-RS User Guide

**📚 Full Documentation:** [Master Roadmap](MMH-RS_ROADMAP_COMPLETE.pdf) | [Technical Specification](MMH-RS_TECHNICAL_COMPLETE.pdf) | [Project Status](PROJECT_STATUS.md) | [Development History](DEVELOPMENT_HISTORY.md) | [Changelog](CHANGELOG.md) | [RGIG Integration](RGIG_INTEGRATION_COMPLETE.pdf) | [Kai Core Integration](KAI_CORE_INTEGRATION_COMPLETE.pdf)

## 🚧 V2 GPU/Quantum Features in Active Development—Community Contributors Wanted!

**For the full V2 roadmap and latest development milestones, see [MMH-RS_ROADMAP_COMPLETE.pdf](MMH-RS_ROADMAP_COMPLETE.pdf).**

## 🚀 Why MMH-RS for AI Storage?

**Built for the AI age—deterministic, self-healing, quantum-ready, GPU-accelerated, and 100% open. Don't settle for legacy—upgrade your storage and future-proof your data today.**

MMH-RS represents the next evolution in compression technology, designed specifically for AI workloads, quantum computing, and the data-intensive future. With perfect data integrity, GPU acceleration, and seamless AI integration, MMH-RS is the foundation for next-generation storage systems.

## 🔥 Coming in V2.0: GPU Acceleration & Quantum Security

**Coming Soon:** GPU acceleration, quantum encryption, directory support, blazing-fast benchmarks, and true multi-GPU scaling.

**Full roadmap:** [MMH-RS_ROADMAP_COMPLETE.pdf](MMH-RS_ROADMAP_COMPLETE.pdf)

### V2.0 Features Preview
- **GPU Acceleration**: 10-100x performance improvement
- **Directory Support**: Native recursive compression
- **Quantum Encryption**: Post-quantum cryptographic algorithms
- **Multi-GPU Scaling**: Distributed processing across multiple GPUs
- **Advanced CLI**: Enhanced command-line interface with progress tracking

## 🚀 Quick Start Guide

### 🟢 Run This First

```bash
# Windows
git clone https://github.com/Bigrob7605/MMH-RS
cd MMH-RS
cargo build --release
.\mmh_human.bat

# Linux/macOS
git clone https://github.com/Bigrob7605/MMH-RS
cd MMH-RS
cargo build --release
./mmh_human.sh
```

**🟢 You should see:** Interactive MMH-RS menu with compression, verification, and extraction options.

## 🎯 Getting Started for Different Audiences

### **For AI Researchers & Data Scientists**
*"I need to compress large datasets and neural network models with perfect integrity."*

**Best Practices:**
- Use `--verify` after every compression operation
- Run benchmarks on your specific hardware: `cargo run --release -- bench 1gb`
- Prepare for V2.0 GPU acceleration (Q4 2025)
- Monitor memory usage for large files (>10GB)

**GPU Tips (V2.0):**
- Install latest CUDA/ROCm drivers
- Use `--gpu` flag for 10-100x speed improvement
- Multi-GPU support coming in V2.1

**Example Workflow:**
```bash
# Compress model files with verification
cargo run --release -- pack model.pt model.mmh
cargo run --release -- verify model.mmh

# Batch process multiple models
for model in models/*.pt; do
    cargo run --release -- pack "$model" "${model%.pt}.mmh"
done
```

### **For IT Professionals & Backup Managers**
*"I need reliable, self-healing compression for enterprise backup systems."*

**Best Practices:**
- Always use `--verify` for integrity checking
- Test recovery procedures regularly
- Use `--smoketest` for comprehensive validation
- Monitor for corruption with `--verify` on restored files

**Enterprise Integration:**
- Script integration with existing backup systems
- Use deterministic output for reproducible backups
- Prepare for V2.0 directory support and metadata preservation

**Example Workflow:**
```bash
# Create verified backup with self-healing
cargo run --release -- pack critical_data.tar critical_data.mmh
cargo run --release -- verify critical_data.mmh

# Restore and verify integrity
cargo run --release -- unpack critical_data.mmh
cargo run --release -- verify critical_data.mmh
```

### **For Developers & System Administrators**
*"I need to integrate MMH-RS into my applications and automation scripts."*

**API Integration:**
- Use command-line interface for scripting
- Parse JSON output for programmatic access
- Integrate with CI/CD pipelines
- Use exit codes for error handling

**Automation Examples:**
```bash
# Automated backup script
#!/bin/bash
BACKUP_FILE="backup_$(date +%Y%m%d_%H%M%S).mmh"
cargo run --release -- pack "$1" "$BACKUP_FILE"
if cargo run --release -- verify "$BACKUP_FILE"; then
    echo "Backup successful: $BACKUP_FILE"
else
    echo "Backup verification failed!"
    exit 1
fi
```

**CI/CD Integration:**
```yaml
# GitHub Actions example
- name: Test MMH-RS
  run: |
    cargo build --release
    cargo run --release -- smoketest test_data/
    cargo run --release -- bench 100mb
```

### **For Home Users & Content Creators**
*"I want to compress my photos, videos, and documents with perfect integrity."*

**Best Practices:**
- Use interactive menu for simple operations
- Verify files after compression
- Keep original files until verification passes
- Use descriptive filenames for easy identification

**Simple Workflow:**
```bash
# Interactive mode (recommended for beginners)
cargo run --release

# Or direct commands
cargo run --release -- pack family_photos.zip photos.mmh
cargo run --release -- verify photos.mmh
cargo run --release -- unpack photos.mmh
```

### **For Security Professionals**
*"I need cryptographic integrity and audit trails for sensitive data."*

**Security Features:**
- SHA-256 + Merkle tree verification
- Deterministic output (same input = same output)
- No data collection or telemetry
- Local processing only

**Audit Workflow:**
```bash
# Create compressed archive with integrity
cargo run --release -- pack sensitive_data.tar secure.mmh

# Verify integrity and generate audit log
cargo run --release -- verify secure.mmh > audit.log

# Restore and re-verify
cargo run --release -- unpack secure.mmh
cargo run --release -- verify secure.mmh >> audit.log
```

### Prerequisites
- **V1.2.0**: Rust 1.70+, 4GB+ RAM, Windows 10+/Ubuntu 20.04+/macOS 11+
- **V2.0**: GPU support (NVIDIA GTX 1060+/AMD RX 580+/Apple M1+), 8GB+ RAM

### Installation
```bash
# Clone the repository
git clone https://github.com/Bigrob7605/MMH-RS.git
cd MMH-RS

# Build the project
cargo build --release

# Install (optional)
cargo install --path .
```

## 📋 V1.2.0 Usage (Current Production Version)

### Basic Commands
```bash
# Compress a file
cargo run --release -- compress input_file.txt output_file.mmh

# Extract a file
cargo run --release -- extract input_file.mmh output_file.txt

# Verify integrity
cargo run --release -- verify input_file.mmh

# Show file information
cargo run --release -- info input_file.mmh
```

### Interactive Menu
```bash
# Launch interactive menu
cargo run --release
```

### Batch Processing
```bash
# Process multiple files
for file in *.txt; do
    cargo run --release -- compress "$file" "${file%.txt}.mmh"
done
```

### Benchmarking
```bash
# Run performance benchmark
cargo run --release -- bench 1gb

# Generate test data
cargo run --release -- gentest 100mb test_data.bin
```

## 🚀 V2.0 Features (Upcoming)

### GPU Acceleration
```bash
# GPU-accelerated compression (V2.0)
cargo run --release -- compress --gpu input_file.txt output_file.mmh

# Multi-GPU processing (V2.1+)
cargo run --release -- compress --gpu --multi-gpu input_file.txt output_file.mmh

# GPU selection
cargo run --release -- compress --gpu-id 0 input_file.txt output_file.mmh
```

### Directory Support
```bash
# Compress entire directory (V2.0)
cargo run --release -- compress-dir input_directory/ output_file.mmh

# Extract directory with metadata preservation
cargo run --release -- extract-dir input_file.mmh output_directory/

# Directory verification
cargo run --release -- verify-dir input_file.mmh
```

### Advanced Security (V2.0)
```bash
# Encrypted compression
cargo run --release -- compress --encrypt input_file.txt output_file.mmh

# Key management
cargo run --release -- keygen --output key.pem
cargo run --release -- compress --key key.pem input_file.txt output_file.mmh

# Quantum-resistant encryption (V2.1+)
cargo run --release -- compress --quantum-safe input_file.txt output_file.mmh
```

### Real-time Verification
```bash
# Continuous integrity checking (V2.0)
cargo run --release -- compress --verify-continuous input_file.txt output_file.mmh

# Progress monitoring
cargo run --release -- compress --progress input_file.txt output_file.mmh
```

## 🎯 Performance Optimization

### V1.2.0 Optimization
```bash
# Memory optimization
cargo run --release -- compress --memory-limit 4gb input_file.txt output_file.mmh

# Thread optimization
cargo run --release -- compress --threads 8 input_file.txt output_file.mmh

# Block size optimization
cargo run --release -- compress --block-size 1mb input_file.txt output_file.mmh
```

### V2.0 GPU Optimization
```bash
# GPU memory management
cargo run --release -- compress --gpu-memory 8gb input_file.txt output_file.mmh

# Kernel optimization
cargo run --release -- compress --gpu-kernel optimized input_file.txt output_file.mmh

# Load balancing
cargo run --release -- compress --gpu-balance auto input_file.txt output_file.mmh
```

## 🔧 Configuration

### V1.2.0 Configuration
```toml
# config.toml
[compression]
algorithm = "lz77_huffman"
block_size = "1mb"
threads = 8
memory_limit = "4gb"

[verification]
hash_algorithm = "sha256"
merkle_tree = true
fec_enabled = true
```

### V2.0 Configuration
```toml
# config_v2.toml
[gpu]
enabled = true
cuda = true
rocm = true
metal = true
memory_pool = "8gb"
multi_gpu = true

[security]
encryption = "aes256_gcm"
quantum_safe = true
key_rotation = "30d"

[directory]
preserve_metadata = true
symlink_handling = "preserve"
cross_platform = true
```

## 🧪 Testing & Validation

### V1.2.0 Testing
```bash
# Run comprehensive tests
cargo test

# Run benchmarks
cargo bench

# Smoke test
cargo run --release -- smoketest test_data/

# Integrity validation
cargo run --release -- validate-all test_data/
```

### V2.0 Testing
```bash
# GPU compatibility test
cargo run --release -- gpu-test

# Performance benchmarking
cargo run --release -- bench-gpu 10gb

# Security validation
cargo run --release -- security-audit

# Cross-platform validation
cargo run --release -- cross-platform-test
```

## 🔍 Troubleshooting

### Common Issues

#### V1.2.0 Issues
```bash
# Memory issues
cargo run --release -- compress --memory-limit 2gb input_file.txt output_file.mmh

# Corrupted file recovery
cargo run --release -- repair input_file.mmh

# Verification failures
cargo run --release -- diagnose input_file.mmh
```

#### V2.0 Issues
```bash
# GPU detection issues
cargo run --release -- gpu-info

# Driver compatibility
cargo run --release -- gpu-check

# Memory allocation errors
cargo run --release -- compress --gpu-memory 4gb input_file.txt output_file.mmh
```

### Error Codes
- `E001`: File not found
- `E002`: Insufficient memory
- `E003`: GPU not available (V2.0)
- `E004`: Encryption key required
- `E005`: Directory access denied (V2.0)

## 📊 Performance Monitoring

### V1.2.0 Metrics
```bash
# Performance statistics
cargo run --release -- stats input_file.mmh

# Memory usage
cargo run --release -- memory-usage

# Compression ratio analysis
cargo run --release -- analyze input_file.mmh
```

### V2.0 Metrics
```bash
# GPU utilization
cargo run --release -- gpu-stats

# Performance dashboard
cargo run --release -- dashboard

# Real-time monitoring
cargo run --release -- monitor --continuous
```

## 🔗 Integration Examples

### Python Integration (V2.0)
```python
import mmh_rs

# Basic compression
mmh_rs.compress("input.txt", "output.mmh")

# GPU acceleration
mmh_rs.compress_gpu("input.txt", "output.mmh", gpu_id=0)

# Directory processing
mmh_rs.compress_directory("input_dir/", "output.mmh")
```

### JavaScript Integration (V2.0)
```javascript
const mmh = require('mmh-rs');

// Basic compression
mmh.compress('input.txt', 'output.mmh');

// GPU acceleration
mmh.compressGPU('input.txt', 'output.mmh', { gpuId: 0 });

// Directory processing
mmh.compressDirectory('input_dir/', 'output.mmh');
```

## 📚 Advanced Usage

### Scripting Examples
```bash
#!/bin/bash
# Automated backup script (V2.0)

# Compress with encryption
cargo run --release -- compress --encrypt --key backup.key \
    /home/user/documents/ backup_$(date +%Y%m%d).mmh

# Verify backup integrity
cargo run --release -- verify backup_$(date +%Y%m%d).mmh

# Upload to cloud (V2.1+)
cargo run --release -- upload --cloud aws backup_$(date +%Y%m%d).mmh
```

### CI/CD Integration
```yaml
# .github/workflows/test.yml
name: MMH-RS Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: cargo run --release -- smoketest test_data/
```

## 🔄 V2 Upgrade Guide

**When V2 releases, simply:**

1. **Pull the latest code**
   ```bash
   git pull origin main
   ```

2. **Build with GPU support**
   ```bash
   cargo build --release --features gpu
   ```

3. **Use the new GPU and directory features**
   ```bash
   # GPU-accelerated compression
   cargo run --release -- compress --gpu input.txt output.mmh
   
   # Directory compression
   cargo run --release -- compress-dir input_directory/ output.mmh
   ```

**No migration needed—MMH-RS seeds are forward-compatible.**

## 🔧 Troubleshooting FAQ

### Common Build Issues
**Q: "error: failed to compile"**
A: Ensure you have Rust 1.70+ installed: `rustup update stable`

**Q: "crate not found"**
A: Run `cargo clean && cargo build --release`

**Q: "permission denied"**
A: On Linux/macOS, ensure execute permissions: `chmod +x target/release/mmh-rs`

### Common Usage Issues
**Q: "file not found"**
A: Check file paths and ensure files exist in the specified location

**Q: "out of memory"**
A: For large files (>16GB), ensure you have sufficient RAM (8GB+ recommended)

**Q: "compression failed"**
A: Check file permissions and ensure the file isn't corrupted

### Performance Issues
**Q: "compression is slow"**
A: V1.2.0 is CPU-only. V2.0 will add GPU acceleration for 10-100x improvement

**Q: "high memory usage"**
A: Normal for large files. V2.0 will optimize memory usage by 50%

## 🤝 Community Support

**We want your feedback—join Discord, open issues, and help shape V2+!**

### Getting Help
- **GitHub Issues**: Bug reports and feature requests
- **Discord**: Community discussions and support
- **Documentation**: Complete guides in Project White Papers
- **Email**: Direct support at Screwball7605@aol.com

### Contributing
- **Testing**: Help test V2 features on your hardware
- **Benchmarking**: Run performance tests and share results
- **Documentation**: Improve guides and examples
- **Code**: Submit pull requests for improvements

---

## See Also

- **[Master Roadmap](MMH-RS_ROADMAP_COMPLETE.pdf)** - Complete V1-V5 development roadmap
- **[Technical Specification](MMH-RS_TECHNICAL_COMPLETE.pdf)** - Detailed implementation documentation
- **[Project Status](PROJECT_STATUS.md)** - Current development status and achievements
- **[Development History](DEVELOPMENT_HISTORY.md)** - Complete timeline and milestones
- **[Changelog](CHANGELOG.md)** - Version-by-version change history
- **[RGIG Integration](RGIG_INTEGRATION_COMPLETE.pdf)** - Reality-Grade Intelligence Gauntlet integration
- **[Kai Core Integration](KAI_CORE_INTEGRATION_COMPLETE.pdf)** - AI bootstrap and neural processing

---

**For detailed technical specifications and implementation details, see [MMH-RS_TECHNICAL_COMPLETE.pdf](MMH-RS_TECHNICAL_COMPLETE.pdf).** 