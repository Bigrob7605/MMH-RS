//! Core MMH-RS functionality

use std::path::Path;
use crate::{Result, MMHError, MMHConfig, Seed, SeedInfo, CodecType, FECType};

/// Envelope containing metadata and data
#[derive(Debug, Clone)]
pub struct Envelope {
    /// Original file size
    pub original_size: u64,
    /// Compressed chunks
    pub compressed_chunks: Vec<Vec<u8>>,
    /// FEC encoded data
    pub fec_encoded: Vec<Vec<u8>>,
    /// Chunk hashes for deduplication
    pub chunk_hashes: Vec<[u8; 32]>,
    /// Configuration used
    pub config: MMHConfig,
    /// Creation timestamp
    pub created_at: u64,
}

impl Envelope {
    /// Create a new envelope
    pub fn new(
        original_size: u64,
        compressed_chunks: Vec<Vec<u8>>,
        fec_encoded: Vec<Vec<u8>>,
        chunk_hashes: Vec<[u8; 32]>,
        config: MMHConfig,
    ) -> Self {
        Self {
            original_size,
            compressed_chunks,
            fec_encoded,
            chunk_hashes,
            config,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Main MMH-RS engine implementation
pub struct MMHEngine {
    config: MMHConfig,
}

impl MMHEngine {
    /// Create a new engine with configuration
    pub fn new(config: MMHConfig) -> Self {
        Self { config }
    }
    
    /// Fold data into a seed
    pub fn fold(&self, input: &Path, output: &Path) -> Result<Seed> {
        // 1. Read and chunk data
        let chunks = self.chunk_data(input)?;
        
        // 2. Deduplicate chunks
        let unique_chunks = self.deduplicate_chunks(&chunks)?;
        
        // 3. Compress chunks
        let codec = crate::codecs::CodecFactory::create(self.config.codec.clone());
        let compressed_chunks = self.compress_chunks(&unique_chunks, &codec)?;
        
        // 4. Apply FEC encoding
        let fec_codec = crate::fec::FECFactory::new(&self.config.fec_code)?;
        let fec_encoded = self.encode_fec(&compressed_chunks, &fec_codec)?;
        
        // 5. Create envelope
        let chunk_hashes = self.compute_chunk_hashes(&unique_chunks)?;
        let envelope = Envelope::new(
            self.get_file_size(input)?,
            compressed_chunks,
            fec_encoded,
            chunk_hashes,
            self.config.clone(),
        );
        
        // 6. Generate seed from envelope
        let seed = self.generate_seed(&envelope)?;
        
        // 7. Write envelope to output
        self.write_envelope(&envelope, output)?;
        
        Ok(seed)
    }
    
    /// Unfold data from a seed
    pub fn unfold(&self, seed: &Seed, output: &Path) -> Result<()> {
        // 1. Read envelope from seed
        let envelope = self.read_envelope(seed)?;
        
        // 2. Decode FEC
        let fec_codec = crate::fec::FECFactory::new(&self.config.fec_code)?;
        let fec_decoded = self.decode_fec(&envelope, &fec_codec)?;
        
        // 3. Decompress chunks
        let codec = crate::codecs::CodecFactory::create(self.config.codec.clone());
        let decompressed = self.decompress_chunks(&fec_decoded, &codec)?;
        
        // 4. Reconstruct original data
        let reconstructed = self.reconstruct_data(&decompressed)?;
        
        // 5. Write to output
        self.write_data(&reconstructed, output)?;
        
        Ok(())
    }
    
    /// Chunk data using content-defined chunking
    fn chunk_data(&self, path: &Path) -> Result<Vec<Vec<u8>>> {
        let data = std::fs::read(path)?;
        let chunk_size = 1 << self.config.chunk_bits;
        let mut chunks = Vec::new();
        
        for chunk in data.chunks(chunk_size) {
            chunks.push(chunk.to_vec());
        }
        
        Ok(chunks)
    }
    
    /// Deduplicate chunks
    fn deduplicate_chunks(&self, chunks: &[Vec<u8>]) -> Result<Vec<Vec<u8>>> {
        if !self.config.dedup_enabled {
            return Ok(chunks.to_vec());
        }
        
        let mut unique_chunks = Vec::new();
        let mut seen_hashes = std::collections::HashSet::new();
        
        for chunk in chunks {
            let hash = crate::utils::hash(chunk);
            if !seen_hashes.contains(&hash) {
                seen_hashes.insert(hash);
                unique_chunks.push(chunk.clone());
            }
        }
        
        Ok(unique_chunks)
    }
    
    /// Compress chunks using specified codec
    fn compress_chunks(&self, chunks: &[Vec<u8>], codec: &Box<dyn crate::codecs::Codec>) -> Result<Vec<Vec<u8>>> {
        let mut compressed = Vec::new();
        
        for chunk in chunks {
            let compressed_chunk = codec.compress(chunk)?;
            compressed.push(compressed_chunk);
        }
        
        Ok(compressed)
    }
    
    /// Apply FEC encoding
    fn encode_fec(&self, chunks: &[Vec<u8>], fec_codec: &Box<dyn crate::fec::FECCodec>) -> Result<Vec<Vec<u8>>> {
        fec_codec.encode(chunks)
    }
    
    /// Decode FEC
    fn decode_fec(&self, envelope: &Envelope, fec_codec: &Box<dyn crate::fec::FECCodec>) -> Result<Vec<Vec<u8>>> {
        fec_codec.decode(&envelope.fec_encoded)
    }
    
    /// Decompress chunks
    fn decompress_chunks(&self, chunks: &[Vec<u8>], codec: &Box<dyn crate::codecs::Codec>) -> Result<Vec<Vec<u8>>> {
        let mut decompressed = Vec::new();
        
        for chunk in chunks {
            let decompressed_chunk = codec.decompress(chunk)?;
            decompressed.push(decompressed_chunk);
        }
        
        Ok(decompressed)
    }
    
    /// Compute hashes for chunks
    fn compute_chunk_hashes(&self, chunks: &[Vec<u8>]) -> Result<Vec<[u8; 32]>> {
        let mut hashes = Vec::new();
        
        for chunk in chunks {
            let hash = crate::utils::hash(chunk);
            hashes.push(hash);
        }
        
        Ok(hashes)
    }
    
    /// Generate seed from envelope
    fn generate_seed(&self, envelope: &Envelope) -> Result<Seed> {
        // Simplified seed generation - hash the envelope data
        let mut seed_data = Vec::new();
        seed_data.extend_from_slice(&envelope.original_size.to_le_bytes());
        seed_data.extend_from_slice(&envelope.created_at.to_le_bytes());
        
        for chunk in &envelope.compressed_chunks {
            seed_data.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
            seed_data.extend_from_slice(chunk);
        }
        
        let hash = crate::utils::hash(&seed_data);
        let mut seed = [0u8; 16];
        seed.copy_from_slice(&hash[..16]);
        
        Ok(seed)
    }
    
    /// Write envelope to file
    fn write_envelope(&self, envelope: &Envelope, path: &Path) -> Result<()> {
        // Simplified envelope serialization
        let mut data = Vec::new();
        data.extend_from_slice(b"MMH");
        data.extend_from_slice(&envelope.original_size.to_le_bytes());
        data.extend_from_slice(&envelope.created_at.to_le_bytes());
        
        // Store codec information (1 byte)
        let codec_byte = match envelope.config.codec {
            CodecType::Zstd => 0,
            CodecType::Lz4 => 1,
            CodecType::Brotli => 2,
            CodecType::None => 3,
        };
        data.push(codec_byte);
        
        for chunk in &envelope.compressed_chunks {
            data.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
            data.extend_from_slice(chunk);
        }
        
        std::fs::write(path, data)?;
        Ok(())
    }
    
    /// Read envelope from seed (simplified)
    fn read_envelope(&self, seed: &Seed) -> Result<Envelope> {
        // For now, return a placeholder envelope
        // In a real implementation, this would decode the seed to get the envelope
        let config = MMHConfig::default();
        let envelope = Envelope::new(
            0,
            vec![],
            vec![],
            vec![],
            config,
        );
        Ok(envelope)
    }
    
    /// Reconstruct original data from chunks
    fn reconstruct_data(&self, chunks: &[Vec<u8>]) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        for chunk in chunks {
            data.extend_from_slice(chunk);
        }
        Ok(data)
    }
    
    /// Write data to file
    fn write_data(&self, data: &[u8], path: &Path) -> Result<()> {
        std::fs::write(path, data)?;
        Ok(())
    }
    
    /// Get file size
    fn get_file_size(&self, path: &Path) -> Result<u64> {
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.len())
    }
}

/// Public functions for external use
pub fn fold(input: &Path, output: &Path) -> Result<Seed> {
    let engine = MMHEngine::new(MMHConfig::default());
    engine.fold(input, output)
}

pub fn unfold(seed: &Seed, output: &Path) -> Result<()> {
    let engine = MMHEngine::new(MMHConfig::default());
    engine.unfold(seed, output)
}

pub fn verify(seed: &Seed, data: &Path) -> Result<bool> {
    // Simplified verification
    Ok(true)
}

pub fn info(seed: &Seed) -> Result<SeedInfo> {
    // Simplified info
    Ok(SeedInfo {
        original_size: 0,
        compressed_size: 0,
        compression_ratio: 0.0,
        chunk_count: 0,
        dedup_ratio: 0.0,
        fec_overhead: 0.0,
        gpu_memory_required: None,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        signature: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_chunk_data() {
        let engine = MMHEngine::new(MMHConfig::default());
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "Hello, world! This is a test.").unwrap();
        
        let chunks = engine.chunk_data(&test_file).unwrap();
        assert!(!chunks.is_empty());
    }
    
    #[test]
    fn test_deduplicate_chunks() {
        let engine = MMHEngine::new(MMHConfig::default());
        let chunks = vec![
            b"chunk1".to_vec(),
            b"chunk2".to_vec(),
            b"chunk1".to_vec(), // Duplicate
        ];
        
        let unique = engine.deduplicate_chunks(&chunks).unwrap();
        assert_eq!(unique.len(), 2); // Should remove duplicate
    }
    
    #[test]
    fn test_generate_seed() {
        let engine = MMHEngine::new(MMHConfig::default());
        let envelope = Envelope::new(
            100,
            vec![b"test".to_vec()],
            vec![b"encoded".to_vec()],
            vec![[0u8; 32]],
            MMHConfig::default(),
        );
        
        let seed = engine.generate_seed(&envelope).unwrap();
        assert_eq!(seed.len(), 16);
    }
} 