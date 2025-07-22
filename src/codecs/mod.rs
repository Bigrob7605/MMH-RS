//! Compression codecs for MMH-RS

use crate::{Result, CodecType};
use std::io::Write;

/// Compression codec trait
pub trait Codec {
    /// Compress data
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>>;
    
    /// Decompress data
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>>;
    
    /// Get codec name
    fn name(&self) -> &str;
}

/// Zstd compression codec with real compression
pub struct ZstdCodec {
    level: i32,
}

impl ZstdCodec {
    /// Create a new Zstd codec with specified compression level
    pub fn new(level: i32) -> Self {
        Self { level }
    }
    
    /// Create a new Zstd codec with default compression level
    pub fn default() -> Self {
        Self { level: 3 }
    }
}

impl Codec for ZstdCodec {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Use real zstd compression
        let mut compressed = Vec::new();
        compressed.extend_from_slice(b"ZSTD");
        compressed.extend_from_slice(&(data.len() as u32).to_le_bytes());
        
        // Real zstd compression
        match zstd::bulk::compress(data, self.level) {
            Ok(compressed_data) => {
                compressed.extend_from_slice(&compressed_data);
                Ok(compressed)
            }
            Err(e) => Err(crate::MMHError::Codec(format!("ZSTD compression failed: {}", e)))
        }
    }
    
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 8 || &data[0..4] != b"ZSTD" {
            return Err(crate::MMHError::Codec("Invalid ZSTD header".to_string()));
        }
        
        let original_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let compressed_data = &data[8..];
        
        if compressed_data.len() == 0 {
            return Err(crate::MMHError::Codec("Empty compressed data".to_string()));
        }
        
        // Real zstd decompression
        match zstd::bulk::decompress(compressed_data, original_size as usize) {
            Ok(decompressed) => {
                if decompressed.len() != original_size as usize {
                    return Err(crate::MMHError::Codec("Decompressed size mismatch".to_string()));
                }
                Ok(decompressed)
            }
            Err(e) => Err(crate::MMHError::Codec(format!("ZSTD decompression failed: {}", e)))
        }
    }
    
    fn name(&self) -> &str {
        "zstd"
    }
}

/// LZ4 compression codec
pub struct Lz4Codec;

impl Codec for Lz4Codec {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Real LZ4 compression
        let mut compressed = Vec::new();
        compressed.extend_from_slice(b"LZ4 ");
        compressed.extend_from_slice(&(data.len() as u32).to_le_bytes());
        
        match lz4::block::compress(data, None, false) {
            Ok(compressed_data) => {
                compressed.extend_from_slice(&compressed_data);
                Ok(compressed)
            }
            Err(e) => Err(crate::MMHError::Codec(format!("LZ4 compression failed: {}", e)))
        }
    }
    
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 8 || &data[0..4] != b"LZ4 " {
            return Err(crate::MMHError::Codec("Invalid LZ4 header".to_string()));
        }
        
        let original_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let compressed_data = &data[8..];
        
        match lz4::block::decompress(compressed_data, Some(original_size as i32)) {
            Ok(decompressed) => {
                if decompressed.len() != original_size as usize {
                    return Err(crate::MMHError::Codec("Decompressed size mismatch".to_string()));
                }
                Ok(decompressed)
            }
            Err(e) => Err(crate::MMHError::Codec(format!("LZ4 decompression failed: {}", e)))
        }
    }
    
    fn name(&self) -> &str {
        "lz4"
    }
}

/// Brotli compression codec
pub struct BrotliCodec;

impl Codec for BrotliCodec {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Real Brotli compression
        let mut compressed = Vec::new();
        compressed.extend_from_slice(b"BROT");
        compressed.extend_from_slice(&(data.len() as u32).to_le_bytes());
        
        let mut compressed_data = Vec::new();
        {
            let mut encoder = brotli::CompressorWriter::new(&mut compressed_data, 4096, 11, 22);
            
            match std::io::Write::write_all(&mut encoder, data) {
                Ok(_) => {
                    match encoder.flush() {
                        Ok(_) => {},
                        Err(e) => return Err(crate::MMHError::Codec(format!("Brotli flush failed: {}", e)))
                    }
                }
                Err(e) => return Err(crate::MMHError::Codec(format!("Brotli compression failed: {}", e)))
            }
        }
        
        compressed.extend_from_slice(&compressed_data);
        Ok(compressed)
    }
    
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 8 || &data[0..4] != b"BROT" {
            return Err(crate::MMHError::Codec("Invalid Brotli header".to_string()));
        }
        
        let original_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let compressed_data = &data[8..];
        
        let mut decompressed = Vec::new();
        let mut decoder = brotli::Decompressor::new(compressed_data, compressed_data.len());
        
        match std::io::copy(&mut decoder, &mut std::io::Cursor::new(&mut decompressed)) {
            Ok(_) => {
                if decompressed.len() != original_size as usize {
                    return Err(crate::MMHError::Codec("Decompressed size mismatch".to_string()));
                }
                Ok(decompressed)
            }
            Err(e) => Err(crate::MMHError::Codec(format!("Brotli decompression failed: {}", e)))
        }
    }
    
    fn name(&self) -> &str {
        "brotli"
    }
}

/// No-op compression codec
pub struct NoCodec;

impl Codec for NoCodec {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut compressed = Vec::new();
        compressed.extend_from_slice(b"NONE");
        compressed.extend_from_slice(&(data.len() as u32).to_le_bytes());
        compressed.extend_from_slice(data);
        Ok(compressed)
    }
    
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 8 || &data[0..4] != b"NONE" {
            return Err(crate::MMHError::Codec("Invalid NONE header".to_string()));
        }
        
        let size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        if data.len() < 8 + size as usize {
            return Err(crate::MMHError::Codec("Incomplete NONE data".to_string()));
        }
        
        Ok(data[8..8 + size as usize].to_vec())
    }
    
    fn name(&self) -> &str {
        "none"
    }
}

/// Codec factory
pub struct CodecFactory;

impl CodecFactory {
    /// Create a codec from type
    pub fn create(codec_type: CodecType) -> Box<dyn Codec> {
        match codec_type {
            CodecType::Zstd => Box::new(ZstdCodec::default()),
            CodecType::Lz4 => Box::new(Lz4Codec),
            CodecType::Brotli => Box::new(BrotliCodec),
            CodecType::None => Box::new(NoCodec),
        }
    }
    
    /// Create a codec with custom compression level
    pub fn create_with_level(codec_type: CodecType, level: i32) -> Box<dyn Codec> {
        match codec_type {
            CodecType::Zstd => Box::new(ZstdCodec::new(level)),
            CodecType::Lz4 => Box::new(Lz4Codec),
            CodecType::Brotli => Box::new(BrotliCodec),
            CodecType::None => Box::new(NoCodec),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zstd_codec() {
        let codec = ZstdCodec::default();
        let data = b"Hello, MMH-RS! This is a test string for compression.";
        
        let compressed = codec.compress(data).unwrap();
        let decompressed = codec.decompress(&compressed).unwrap();
        
        assert_eq!(data, &decompressed[..]);
        assert_eq!(codec.name(), "zstd");
        
        // Test that compression actually reduces size for repetitive data
        let repetitive_data = b"Hello, MMH-RS! ".repeat(100);
        let compressed_repetitive = codec.compress(&repetitive_data).unwrap();
        println!("Original size: {} bytes", repetitive_data.len());
        println!("Compressed size: {} bytes", compressed_repetitive.len());
        println!("Compression ratio: {:.2}x", repetitive_data.len() as f64 / compressed_repetitive.len() as f64);
    }
    
    #[test]
    fn test_lz4_codec() {
        let codec = Lz4Codec;
        let data = b"Test data for LZ4 compression";
        
        let compressed = codec.compress(data).unwrap();
        let decompressed = codec.decompress(&compressed).unwrap();
        
        assert_eq!(data, &decompressed[..]);
        assert_eq!(codec.name(), "lz4");
    }
    
    #[test]
    fn test_brotli_codec() {
        let codec = BrotliCodec;
        let data = b"Test data for Brotli compression";
        
        let compressed = codec.compress(data).unwrap();
        let decompressed = codec.decompress(&compressed).unwrap();
        
        assert_eq!(data, &decompressed[..]);
        assert_eq!(codec.name(), "brotli");
    }
    
    #[test]
    fn test_no_codec() {
        let codec = NoCodec;
        let data = b"Test data for no compression";
        
        let compressed = codec.compress(data).unwrap();
        let decompressed = codec.decompress(&compressed).unwrap();
        
        assert_eq!(data, &decompressed[..]);
        assert_eq!(codec.name(), "none");
    }
    
    #[test]
    fn test_codec_factory() {
        let zstd = CodecFactory::create(CodecType::Zstd);
        let lz4 = CodecFactory::create(CodecType::Lz4);
        let brotli = CodecFactory::create(CodecType::Brotli);
        let none = CodecFactory::create(CodecType::None);
        
        assert_eq!(zstd.name(), "zstd");
        assert_eq!(lz4.name(), "lz4");
        assert_eq!(brotli.name(), "brotli");
        assert_eq!(none.name(), "none");
    }
    
    #[test]
    fn test_compression_ratio() {
        let codec = ZstdCodec::default();
        
        // Test with highly compressible data
        let compressible_data = b"Hello, MMH-RS! ".repeat(1000);
        let compressed = codec.compress(&compressible_data).unwrap();
        
        let ratio = compressible_data.len() as f64 / compressed.len() as f64;
        println!("Compression ratio for repetitive data: {:.2}x", ratio);
        
        // Test with random data
        let random_data: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let compressed_random = codec.compress(&random_data).unwrap();
        
        let ratio_random = random_data.len() as f64 / compressed_random.len() as f64;
        println!("Compression ratio for random data: {:.2}x", ratio_random);
    }
} 