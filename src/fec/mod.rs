//! Forward Error Correction (FEC) for MMH-RS

use crate::{Result, MMHError, FECType};

/// Trait for FEC codecs
pub trait FECCodec: Send + Sync {
    /// Encode data with FEC
    fn encode(&self, data: &[Vec<u8>]) -> Result<Vec<Vec<u8>>>;
    
    /// Decode data with FEC
    fn decode(&self, data: &[Vec<u8>]) -> Result<Vec<Vec<u8>>>;
    
    /// Get FEC codec name
    fn name(&self) -> &str;
    
    /// Get redundancy level
    fn redundancy(&self) -> f64;
}

/// RaptorQ FEC implementation
pub struct RaptorQCodec {
    redundancy: f64,
}

impl RaptorQCodec {
    pub fn new(redundancy: f64) -> Self {
        Self { redundancy }
    }
    
    pub fn default() -> Self {
        Self { redundancy: 1.5 }
    }
}

impl FECCodec for RaptorQCodec {
    fn encode(&self, data: &[Vec<u8>]) -> Result<Vec<Vec<u8>>> {
        // Simplified RaptorQ implementation
        // In a real implementation, this would use a proper RaptorQ library
        let mut encoded = Vec::new();
        
        // Add original data
        encoded.extend_from_slice(data);
        
        // Add redundancy blocks
        let redundancy_count = (data.len() as f64 * (self.redundancy - 1.0)) as usize;
        for i in 0..redundancy_count {
            let mut redundancy_block = Vec::new();
            for (j, chunk) in data.iter().enumerate() {
                let coefficient = (i * j) % 256;
                for &byte in chunk {
                    redundancy_block.push(byte ^ coefficient as u8);
                }
            }
            encoded.push(redundancy_block);
        }
        
        Ok(encoded)
    }
    
    fn decode(&self, data: &[Vec<u8>]) -> Result<Vec<Vec<u8>>> {
        // Simplified decoding - just return the first N chunks
        // In a real implementation, this would perform proper RaptorQ decoding
        let original_count = (data.len() as f64 / self.redundancy) as usize;
        Ok(data[..original_count].to_vec())
    }
    
    fn name(&self) -> &str {
        "raptorq"
    }
    
    fn redundancy(&self) -> f64 {
        self.redundancy
    }
}

/// Reed-Solomon FEC implementation
pub struct ReedSolomonCodec {
    redundancy: f64,
}

impl ReedSolomonCodec {
    pub fn new(redundancy: f64) -> Self {
        Self { redundancy }
    }
    
    pub fn default() -> Self {
        Self { redundancy: 1.3 }
    }
}

impl FECCodec for ReedSolomonCodec {
    fn encode(&self, data: &[Vec<u8>]) -> Result<Vec<Vec<u8>>> {
        // Simplified Reed-Solomon implementation
        let mut encoded = Vec::new();
        
        // Add original data
        encoded.extend_from_slice(data);
        
        // Add parity blocks
        let parity_count = (data.len() as f64 * (self.redundancy - 1.0)) as usize;
        for i in 0..parity_count {
            let mut parity_block = Vec::new();
            for chunk in data {
                let mut parity_byte = 0u8;
                for (j, &byte) in chunk.iter().enumerate() {
                    parity_byte ^= byte.rotate_left((i + j) as u32);
                }
                parity_block.push(parity_byte);
            }
            encoded.push(parity_block);
        }
        
        Ok(encoded)
    }
    
    fn decode(&self, data: &[Vec<u8>]) -> Result<Vec<Vec<u8>>> {
        // Simplified decoding
        let original_count = (data.len() as f64 / self.redundancy) as usize;
        Ok(data[..original_count].to_vec())
    }
    
    fn name(&self) -> &str {
        "reed-solomon"
    }
    
    fn redundancy(&self) -> f64 {
        self.redundancy
    }
}

/// No FEC implementation
pub struct NoFECCodec;

impl FECCodec for NoFECCodec {
    fn encode(&self, data: &[Vec<u8>]) -> Result<Vec<Vec<u8>>> {
        Ok(data.to_vec())
    }
    
    fn decode(&self, data: &[Vec<u8>]) -> Result<Vec<Vec<u8>>> {
        Ok(data.to_vec())
    }
    
    fn name(&self) -> &str {
        "none"
    }
    
    fn redundancy(&self) -> f64 {
        1.0
    }
}

/// Factory for creating FEC codecs
pub struct FECFactory;

impl FECFactory {
    /// Create a new FEC codec
    pub fn new(fec_type: &FECType) -> Result<Box<dyn FECCodec>> {
        match fec_type {
            FECType::RaptorQ => Ok(Box::new(RaptorQCodec::default())),
            FECType::ReedSolomon => Ok(Box::new(ReedSolomonCodec::default())),
            FECType::None => Ok(Box::new(NoFECCodec)),
        }
    }
    
    /// Create a new FEC codec with custom redundancy
    pub fn new_with_redundancy(fec_type: &FECType, redundancy: f64) -> Result<Box<dyn FECCodec>> {
        match fec_type {
            FECType::RaptorQ => Ok(Box::new(RaptorQCodec::new(redundancy))),
            FECType::ReedSolomon => Ok(Box::new(ReedSolomonCodec::new(redundancy))),
            FECType::None => Ok(Box::new(NoFECCodec)),
        }
    }
    
    /// List available FEC codecs
    pub fn list_available() -> Vec<FECType> {
        vec![
            FECType::RaptorQ,
            FECType::ReedSolomon,
            FECType::None,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_raptorq_codec() {
        let codec = RaptorQCodec::default();
        let data = vec![
            b"chunk1".to_vec(),
            b"chunk2".to_vec(),
            b"chunk3".to_vec(),
        ];
        
        let encoded = codec.encode(&data).unwrap();
        let decoded = codec.decode(&encoded).unwrap();
        
        assert_eq!(data, decoded);
        assert!(encoded.len() > data.len()); // Should have redundancy
    }
    
    #[test]
    fn test_reed_solomon_codec() {
        let codec = ReedSolomonCodec::default();
        let data = vec![
            b"chunk1".to_vec(),
            b"chunk2".to_vec(),
            b"chunk3".to_vec(),
        ];
        
        let encoded = codec.encode(&data).unwrap();
        let decoded = codec.decode(&encoded).unwrap();
        
        assert_eq!(data, decoded);
        assert!(encoded.len() > data.len()); // Should have redundancy
    }
    
    #[test]
    fn test_no_fec_codec() {
        let codec = NoFECCodec;
        let data = vec![
            b"chunk1".to_vec(),
            b"chunk2".to_vec(),
        ];
        
        let encoded = codec.encode(&data).unwrap();
        let decoded = codec.decode(&encoded).unwrap();
        
        assert_eq!(data, decoded);
        assert_eq!(encoded.len(), data.len()); // No redundancy
    }
    
    #[test]
    fn test_fec_factory() {
        let codec = FECFactory::new(&FECType::RaptorQ).unwrap();
        assert_eq!(codec.name(), "raptorq");
        
        let codec = FECFactory::new(&FECType::ReedSolomon).unwrap();
        assert_eq!(codec.name(), "reed-solomon");
        
        let codec = FECFactory::new(&FECType::None).unwrap();
        assert_eq!(codec.name(), "none");
    }
} 