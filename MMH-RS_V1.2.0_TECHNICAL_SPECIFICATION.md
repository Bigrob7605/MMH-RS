# MMH-RS V1.2.0 ELITE TIER - TECHNICAL SPECIFICATION

## 🔧 **TECHNICAL ARCHITECTURE**

**Version**: V1.2.0 Elite Tier  
**Language**: Rust  
**Compression**: zstd  
**Platform**: Cross-platform (Windows, Linux, macOS)

---

## 📦 **MMH FILE FORMAT SPECIFICATION**

### **Header Structure (40 bytes)**
```rust
#[repr(C, packed)]
pub struct MMHHeader {
    pub magic: [u8; 4],           // "MMH\0" (0x4D, 0x4D, 0x48, 0x00)
    pub version: u8,              // Version number (currently 0x02)
    pub flags: u8,                // Feature flags
    pub original_extension: [u8; 16], // Original file extension (null-padded)
    pub reserved: [u8; 16],       // Reserved for future extensions
}
```

### **File Format Layout**
```
┌─────────────────┬─────────────────┬─────────────────┬─────────────────┐
│   MMH Header    │   zstd Frame    │   zstd Frame    │   zstd Frame    │
│   (40 bytes)    │   (variable)    │   (variable)    │   (variable)    │
└─────────────────┴─────────────────┴─────────────────┴─────────────────┘
```

### **Extension Storage Format**
- **Field**: `original_extension[16]`
- **Format**: ASCII string, null-padded
- **Example**: `"pdf\0\0\0\0\0\0\0\0\0\0\0\0\0"` for PDF files
- **Restoration**: Automatic during unpack operation

---

## 🚀 **CORE ALGORITHMS**

### **Compression Algorithm (zstd)**
```rust
// Compression parameters
const ZSTD_COMPRESSION_LEVEL: i32 = 3;  // Balanced speed/size
const ZSTD_DICT_SIZE: usize = 32 * 1024; // 32KB dictionary
const ZSTD_WORKSPACE_SIZE: usize = 1024 * 1024; // 1MB workspace
```

### **File Processing Pipeline**
1. **Input Validation** - Check file existence and permissions
2. **Header Generation** - Create MMH header with extension
3. **zstd Compression** - Compress file data in chunks
4. **Output Writing** - Write header + compressed data
5. **Integrity Check** - Verify output file integrity

### **Decompression Pipeline**
1. **Header Reading** - Read and validate MMH header
2. **Extension Extraction** - Extract original file extension
3. **zstd Decompression** - Decompress data in chunks
4. **File Reconstruction** - Reconstruct original file with extension
5. **Integrity Verification** - Verify bit-for-bit integrity

---

## 🎯 **EXTENSION RESTORATION IMPLEMENTATION**

### **Problem Analysis**
```rust
// Previous faulty logic
let output_name = if let Some(ext) = path.extension() {
    format!("{}.{}", base_name, ext.to_string_lossy())
} else {
    base_name.to_string()
};
```

### **Solution Implementation**
```rust
// Fixed extension restoration
pub fn unpack(input_path: &Path, output_base: Option<&str>) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(input_path)?;
    
    // Read MMH header
    let mut header = [0u8; 40];
    file.read_exact(&mut header)?;
    
    // Extract original extension from header
    let original_ext = extract_extension_from_header(&header);
    
    // Construct output filename with original extension
    let output_name = if let Some(ext) = original_ext {
        format!("{}.{}", base_name, ext)
    } else {
        base_name.to_string()
    };
    
    // Decompress and write file
    // ...
}
```

### **Header Extension Extraction**
```rust
fn extract_extension_from_header(header: &[u8; 40]) -> Option<&str> {
    let ext_slice = &header[6..22]; // original_extension field
    
    // Find null terminator
    if let Some(null_pos) = ext_slice.iter().position(|&b| b == 0) {
        if null_pos > 0 {
            std::str::from_utf8(&ext_slice[..null_pos]).ok()
        } else {
            None
        }
    } else {
        None
    }
}
```

---

## 📊 **PERFORMANCE OPTIMIZATIONS**

### **Memory Management**
- **Streaming Processing** - Process files in chunks to minimize memory usage
- **Buffer Pooling** - Reuse compression/decompression buffers
- **Lazy Loading** - Load only necessary data into memory

### **I/O Optimization**
- **Buffered I/O** - Use buffered readers/writers for better performance
- **Direct I/O** - Bypass OS cache for large files
- **Parallel Processing** - Multi-threaded compression for large files

### **Compression Tuning**
```rust
// Optimized zstd parameters for different file types
pub enum CompressionProfile {
    Fast,      // Level 1-3, high speed
    Balanced,  // Level 3-6, balanced
    Maximum,   // Level 7-22, high compression
}
```

---

## 🧪 **TESTING FRAMEWORK**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extension_preservation() {
        let test_files = vec!["test.pdf", "document.docx", "image.png"];
        
        for file in test_files {
            let packed = pack_file(file).unwrap();
            let unpacked = unpack_file(&packed).unwrap();
            
            assert_eq!(file, unpacked);
        }
    }
}
```

### **Integration Tests**
- **File Integrity Tests** - Verify bit-for-bit reconstruction
- **Extension Tests** - Verify extension preservation
- **Performance Tests** - Benchmark compression/decompression
- **Stress Tests** - Test with pathological data

### **Benchmark Suite**
```rust
#[bench]
fn bench_compression(b: &mut Bencher) {
    let test_data = generate_test_data(1024 * 1024); // 1MB
    
    b.iter(|| {
        compress_data(&test_data)
    });
}
```

---

## 🔒 **SECURITY CONSIDERATIONS**

### **Input Validation**
- **Path Traversal Protection** - Prevent directory traversal attacks
- **File Size Limits** - Prevent memory exhaustion attacks
- **Extension Validation** - Validate file extensions

### **Error Handling**
```rust
pub enum MMHError {
    InvalidHeader,
    UnsupportedVersion,
    CompressionFailed,
    DecompressionFailed,
    ExtensionRestorationFailed,
    IntegrityCheckFailed,
}
```

---

## 📈 **MONITORING & LOGGING**

### **Debug Logging**
```rust
#[derive(Debug)]
pub struct OperationLog {
    pub operation: String,
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub compression_ratio: f64,
    pub processing_time: Duration,
    pub extension_restored: Option<String>,
}
```

### **Performance Metrics**
- **Compression Ratio** - Space savings percentage
- **Processing Speed** - MB/s throughput
- **Memory Usage** - Peak memory consumption
- **Error Rates** - Failure frequency

---

## 🔧 **BUILD CONFIGURATION**

### **Cargo.toml Dependencies**
```toml
[dependencies]
zstd = "0.12"
clap = { version = "4.0", features = ["derive"] }
indicatif = "0.17"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### **Feature Flags**
```toml
[features]
default = ["cli", "benchmark"]
cli = ["clap"]
benchmark = ["criterion"]
test-utils = ["tempfile"]
```

### **Build Profiles**
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

---

## 🚀 **DEPLOYMENT**

### **Release Build**
```bash
# Optimized release build
cargo build --release

# Binary location
./target/release/mmh.exe
```

### **Cross-Platform Compilation**
```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS
cargo build --release --target x86_64-apple-darwin
```

---

## 📋 **API REFERENCE**

### **Core Functions**
```rust
pub fn pack(input: &Path, output: &Path) -> Result<(), MMHError>
pub fn unpack(input: &Path, output_base: Option<&str>) -> Result<(), MMHError>
pub fn verify(original: &Path, restored: &Path) -> Result<bool, MMHError>
```

### **CLI Commands**
```bash
mmh pack <input> <output>     # Pack file
mmh unpack <input> [output]   # Unpack file
mmh verify <file1> <file2>    # Verify integrity
mmh benchmark                 # Run benchmarks
```

---

**🔧 This technical specification provides complete implementation details for MMH-RS V1.2.0 Elite Tier.** 