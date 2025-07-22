// V1 NOTE: Integration tests are disabled for release. Restore for V2+ with proper dependencies.
// #[cfg(test)]
// mod integration {
//     use super::*;
//     //! Integration tests for MMH-RS
//     //! 
//     //! TODO: @next-release Add comprehensive CLI testing using assert_cmd and predicates
//     //! for full end-to-end testing including real command execution in CI.

//     use mmh_rs::{MMH, MMHConfig, CodecType, FECType};
//     use std::fs;
//     use tempfile::tempdir;

//     #[test]
//     fn test_basic_fold_unfold() {
//         let temp_dir = tempdir().unwrap();
//         let input_file = temp_dir.path().join("input.txt");
//         let output_file = temp_dir.path().join("output.mmh");
//         let restored_file = temp_dir.path().join("restored.txt");
        
//         // Create test data
//         let test_data = "Hello, MMH-RS! This is a test of the fold/unfold functionality.";
//         fs::write(&input_file, test_data).unwrap();
        
//         // Configure MMH
//         let config = MMHConfig {
//             chunk_bits: 10,
//             codec: CodecType::Zstd,
//             fec_code: FECType::RaptorQ,
//             gpu_enabled: false,
//             batch_size: 1024,
//             dedup_enabled: true,
//             signature_key: None,
//         };
        
//         let mmh = MMH::with_config(config);
        
//         // Fold the data
//         let seed = mmh.fold(&input_file, &output_file).unwrap();
//         println!("Generated seed: 0x{}", mmh_rs::utils::seed_to_hex(&seed));
        
//         // Verify the output file exists
//         assert!(output_file.exists());
        
//         // Unfold the data
//         mmh.unfold(&seed, &restored_file).unwrap();
        
//         // Verify the restored data matches
//         let restored_data = fs::read_to_string(&restored_file).unwrap();
//         assert_eq!(test_data, restored_data);
//     }

//     #[test]
//     fn test_chunking() {
//         use mmh_rs::chunking::Chunker;
        
//         let chunker = Chunker::new(10); // 1KB chunks
//         let data = vec![0u8; 5000];
        
//         let chunks = chunker.chunk_data(&data).unwrap();
//         assert!(!chunks.is_empty());
        
//         let stats = chunker.get_stats(&chunks);
//         assert_eq!(stats.total_size, 5000);
//         assert!(stats.chunk_count > 1);
//     }

//     #[test]
//     fn test_codecs() {
//         use mmh_rs::codecs::{CodecRegistry, CodecType};
        
//         let codec = CodecRegistry::get(&CodecType::Zstd).unwrap();
//         let data = b"Hello, world! This is a test string for compression.";
        
//         let compressed = codec.compress(data).unwrap();
//         let decompressed = codec.decompress(&compressed).unwrap();
        
//         assert_eq!(data, &decompressed[..]);
//         assert!(compressed.len() < data.len());
//     }

//     #[test]
//     fn test_fec() {
//         use mmh_rs::fec::{FECCodec, FECType};
        
//         let fec_codec = FECCodec::new(&FECType::RaptorQ).unwrap();
//         let data = vec![
//             b"chunk1".to_vec(),
//             b"chunk2".to_vec(),
//             b"chunk3".to_vec(),
//         ];
        
//         let encoded = fec_codec.encode(&data).unwrap();
//         let decoded = fec_codec.decode(&encoded).unwrap();
        
//         assert_eq!(data, decoded);
//         assert!(encoded.len() > data.len()); // Should have redundancy
//     }

//     #[test]
//     fn test_utils() {
//         use mmh_rs::utils;
        
//         let data = b"Hello, world!";
//         let hash = utils::hash(data);
//         assert_eq!(hash.len(), 32);
        
//         let hex = utils::hash_hex(data);
//         assert_eq!(hex.len(), 64);
        
//         let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
//         let seed_hex = utils::seed_to_hex(&seed);
//         let decoded_seed = utils::hex_to_seed(&seed_hex).unwrap();
//         assert_eq!(seed, decoded_seed);
//     }
// } 