//! Utility functions for MMH-RS

use std::path::Path;

/// Simple hash function (placeholder for SHA-256)
pub fn hash(data: &[u8]) -> [u8; 32] {
    let mut result = [0u8; 32];
    for (i, &byte) in data.iter().enumerate() {
        result[i % 32] ^= byte;
    }
    result
}

/// Hash data and return hex string
pub fn hash_hex(data: &[u8]) -> String {
    let hash = hash(data);
    hex_encode(&hash)
}

/// Convert seed to hex string
pub fn seed_to_hex(seed: &[u8; 16]) -> String {
    hex_encode(seed)
}

/// Convert hex string to seed
pub fn hex_to_seed(hex: &str) -> Result<[u8; 16], String> {
    let bytes = hex_decode(hex)
        .map_err(|e| format!("Invalid hex string: {}", e))?;
    
    if bytes.len() != 16 {
        return Err("Hex string must be exactly 32 characters (16 bytes)".to_string());
    }
    
    let mut seed = [0u8; 16];
    seed.copy_from_slice(&bytes);
    Ok(seed)
}

/// Simple hex encoding
fn hex_encode(data: &[u8]) -> String {
    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    let mut result = String::with_capacity(data.len() * 2);
    for &byte in data {
        result.push(HEX_CHARS[(byte >> 4) as usize] as char);
        result.push(HEX_CHARS[(byte & 0xf) as usize] as char);
    }
    result
}

/// Simple hex decoding
fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }
    
    let mut result = Vec::with_capacity(hex.len() / 2);
    let mut chars = hex.chars();
    
    while let (Some(a), Some(b)) = (chars.next(), chars.next()) {
        let high = hex_char_to_byte(a)?;
        let low = hex_char_to_byte(b)?;
        result.push((high << 4) | low);
    }
    
    Ok(result)
}

fn hex_char_to_byte(c: char) -> Result<u8, String> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        'A'..='F' => Ok(c as u8 - b'A' + 10),
        _ => Err(format!("Invalid hex character: {}", c)),
    }
}

/// Format bytes as human-readable string
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

/// Calculate compression ratio
pub fn compression_ratio(original: u64, compressed: u64) -> f64 {
    if original == 0 {
        0.0
    } else {
        original as f64 / compressed as f64
    }
}

/// Calculate deduplication ratio
pub fn dedup_ratio(original: u64, deduplicated: u64) -> f64 {
    if original == 0 {
        0.0
    } else {
        original as f64 / deduplicated as f64
    }
}

/// Get file size
pub fn get_file_size(path: &Path) -> Result<u64, std::io::Error> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}

/// Get directory size recursively
pub fn get_dir_size(path: &Path) -> Result<u64, std::io::Error> {
    let mut total_size = 0u64;
    
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            total_size += entry.metadata()?.len();
        }
    }
    
    Ok(total_size)
}

/// Generate a random seed for testing
pub fn random_seed() -> [u8; 16] {
    let mut seed = [0u8; 16];
    for i in 0..16 {
        seed[i] = (i * 7) as u8;
    }
    seed
}

/// Validate seed format
pub fn validate_seed(seed: &[u8]) -> bool {
    seed.len() == 16
}

/// Compare two files
pub fn compare_files(path1: &Path, path2: &Path) -> Result<bool, std::io::Error> {
    let data1 = std::fs::read(path1)?;
    let data2 = std::fs::read(path2)?;
    Ok(data1 == data2)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash() {
        let data = b"Hello, world!";
        let hash = hash(data);
        assert_eq!(hash.len(), 32);
        
        let hash2 = hash(data);
        assert_eq!(hash, hash2); // Deterministic
    }
    
    #[test]
    fn test_hash_hex() {
        let data = b"Hello, world!";
        let hex = hash_hex(data);
        assert_eq!(hex.len(), 64); // 32 bytes = 64 hex chars
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_seed_conversion() {
        let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let hex = seed_to_hex(&seed);
        let decoded = hex_to_seed(&hex).unwrap();
        assert_eq!(seed, decoded);
    }
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }
    
    #[test]
    fn test_compression_ratio() {
        assert_eq!(compression_ratio(100, 50), 2.0);
        assert_eq!(compression_ratio(100, 100), 1.0);
        assert_eq!(compression_ratio(100, 200), 0.5);
    }
    
    #[test]
    fn test_dedup_ratio() {
        assert_eq!(dedup_ratio(100, 50), 2.0);
        assert_eq!(dedup_ratio(100, 100), 1.0);
        assert_eq!(dedup_ratio(100, 200), 0.5);
    }
    
    #[test]
    fn test_validate_seed() {
        assert!(validate_seed(&[0u8; 16]));
        assert!(!validate_seed(&[0u8; 15]));
        assert!(!validate_seed(&[0u8; 17]));
    }
} 