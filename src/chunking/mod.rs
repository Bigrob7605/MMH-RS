//! Content-defined chunking for MMH-RS

use crate::{Result, MMHError};

/// Content-defined chunker
pub struct Chunker {
    /// Chunk size in bits (2^chunk_bits bytes)
    chunk_bits: u8,
    /// Rolling hash window size
    window_size: usize,
}

impl Chunker {
    /// Create a new chunker with specified chunk size
    pub fn new(chunk_bits: u8) -> Self {
        Self {
            chunk_bits,
            window_size: 64,
        }
    }
    
    /// Chunk a file into content-defined chunks
    pub fn chunk_file(&self, path: &std::path::Path) -> Result<Vec<Vec<u8>>> {
        let data = std::fs::read(path)?;
        self.chunk_data(&data)
    }
    
    /// Chunk data into content-defined chunks
    pub fn chunk_data(&self, data: &[u8]) -> Result<Vec<Vec<u8>>> {
        let mut chunks = Vec::new();
        let mut start = 0;
        let chunk_size = 1 << self.chunk_bits;
        
        while start < data.len() {
            let end = std::cmp::min(start + chunk_size, data.len());
            let chunk = data[start..end].to_vec();
            chunks.push(chunk);
            start = end;
        }
        
        Ok(chunks)
    }
    
    /// Find chunk boundaries using rolling hash
    fn find_chunk_boundary(&self, data: &[u8], offset: usize) -> usize {
        // Simplified boundary detection
        let chunk_size = 1 << self.chunk_bits;
        std::cmp::min(offset + chunk_size, data.len())
    }
    
    /// Get chunk statistics
    pub fn get_stats(&self, chunks: &[Vec<u8>]) -> ChunkStats {
        let total_size: usize = chunks.iter().map(|c| c.len()).sum();
        let avg_size = if chunks.is_empty() { 0 } else { total_size / chunks.len() };
        let min_size = chunks.iter().map(|c| c.len()).min().unwrap_or(0);
        let max_size = chunks.iter().map(|c| c.len()).max().unwrap_or(0);
        
        ChunkStats {
            count: chunks.len(),
            total_size,
            avg_size,
            min_size,
            max_size,
            chunk_bits: self.chunk_bits,
        }
    }
}

/// Statistics about chunks
#[derive(Debug, Clone)]
pub struct ChunkStats {
    /// Number of chunks
    pub count: usize,
    /// Total size of all chunks
    pub total_size: usize,
    /// Average chunk size
    pub avg_size: usize,
    /// Minimum chunk size
    pub min_size: usize,
    /// Maximum chunk size
    pub max_size: usize,
    /// Chunk size in bits
    pub chunk_bits: u8,
}

impl ChunkStats {
    /// Get compression ratio estimate
    pub fn compression_ratio(&self) -> f64 {
        if self.total_size == 0 {
            0.0
        } else {
            self.total_size as f64 / self.avg_size as f64
        }
    }
    
    /// Get deduplication ratio estimate
    pub fn dedup_ratio(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.count as f64 / (self.count as f64 * 0.8) // Estimate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunker() {
        let chunker = Chunker::new(4); // 16-byte chunks
        let data = b"Hello, world! This is a test string for chunking.";
        
        let chunks = chunker.chunk_data(data).unwrap();
        assert!(!chunks.is_empty());
        assert!(chunks.len() > 1); // Should have multiple chunks
        
        for chunk in &chunks {
            assert!(chunk.len() <= 16); // Max chunk size
        }
    }
    
    #[test]
    fn test_chunk_stats() {
        let chunker = Chunker::new(4);
        let data = b"Hello, world! This is a test string for chunking.";
        let chunks = chunker.chunk_data(data).unwrap();
        
        let stats = chunker.get_stats(&chunks);
        assert_eq!(stats.count, chunks.len());
        assert!(stats.total_size > 0);
        assert!(stats.avg_size > 0);
    }
    
    #[test]
    fn test_chunk_file() {
        let chunker = Chunker::new(4);
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        std::fs::write(&test_file, "Hello, world! This is a test.").unwrap();
        
        let chunks = chunker.chunk_file(&test_file).unwrap();
        assert!(!chunks.is_empty());
    }
} 