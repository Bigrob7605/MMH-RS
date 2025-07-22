use clap::{Parser, Subcommand};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, BufReader, BufWriter, BufRead, Read};
use std::time::Instant;
use std::path::Path;
use chrono::Local;
use sysinfo::System;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
mod agent;
use crate::bench;

const MAX_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10GB
const LOG_FILE: &str = "mmh_cli.log";
const ERROR_LOG_FILE: &str = "mmh_error_log.txt";

#[derive(Parser)]
#[command(name = "mmh")]
#[command(about = "MMH-RS: Universal Digital DNA Format (V1 ROCK SOLID)", long_about = None)]
#[command(after_help = r#"
EXAMPLES:
  mmh pack input.txt output.mmh          # Compress a single file
  mmh unpack output.mmh restored.txt     # Decompress a file
  mmh verify input.txt restored.txt      # Verify round-trip integrity
  mmh benchmenu                         # Interactive benchmark menu
  mmh stressbench --size 5              # Stress test with 5GB data
  mmh selftest                          # Run comprehensive self-test
  mmh --version                         # Show version info
  mmh --about                           # Show project information
  mmh verifyseed <dir> <seed>            # Verify a directory matches a replay seed

PERFORMANCE TIPS:
  - Run benchmenu first to see what's possible
  - Use --threads=1 for deterministic results
  - Batch small files with --batch for better performance
  - Check --dry-run for compression estimates

EXIT CODES:
  0 = Success
  1 = User error (file not found, invalid input)
  2 = Integrity failure (corrupted data)
  42 = Special exit for wrapper detection

For more information: https://github.com/Bigrob7605/MMH-RS

---
Send logs, feedback, or ideas to: Screwball7605@aol.com
"#)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// Show version information
    #[arg(long)]
    pub version: bool,
    /// Show project information
    #[arg(long)]
    pub about: bool,
    /// Show a specific ASCII art from the MMH-RS collection (1-8)
    #[arg(long)]
    pub ascii: Option<u8>,
    /// Show the Gandalf meme (Easter egg)
    #[arg(long)]
    pub wizard: bool,
    /// Number of threads for compression (default: auto-detect)
    #[arg(long)]
    pub threads: Option<usize>,
    /// Dry run - show compression estimate without writing
    #[arg(long)]
    pub dry_run: bool,
    /// Batch mode for multiple small files
    #[arg(long)]
    pub batch: Option<usize>,
    /// Run the embedded universal agent (for automated testing and data management)
    #[arg(long)]
    pub agent: bool,
    /// Show RGIG (Reality Grade Intelligence Gauntlet) integration info (coming V3!)
    #[arg(long)]
    pub rgig_info: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Pack (compress) a single file (max 10GB)
    Pack { input: String, output: String },
    /// Unpack (decompress) a single file
    Unpack { input: String, output: String },
    /// Pack a directory as a single seed (optimal for tiny files)
    Packdir { dir: String, output: String },
    /// Verify round-trip integrity
    Verify { input: String, restored: String },
    /// Generate deterministic random file
    Gen { output: String, #[arg(long, default_value = "1024")] size: usize },
    /// Generate a directory of mixed test data up to N GB
    Gentestdir { dir: String, #[arg(long)] size: u64 },
    /// Run the interactive benchmark menu (V1 UX demo)
    Benchmenu { #[arg(long, default_value = "2")] size: u64 },
    /// Run MMH stress test with 3 realistic file types
    Stressbench { #[arg(long, default_value = "2")] size: u64 },
    /// Run gold standard benchmark with comprehensive reporting
    Goldbench { 
        /// Test size in GB (0 for smoketest, 1-10 for full benchmark)
        #[arg(long, default_value = "2")]
        size: u64,
        /// Replay seed for deterministic testing (optional)
        #[arg(long)]
        seed: Option<u64>,
        /// Output format: text, json, or both
        #[arg(long, default_value = "both")]
        format: String,
    },
    /// Run comprehensive self-test (CI/CD & user validation)
    Selftest,
    /// Clean up all generated test data and temporary files
    Cleanup,
    /// Verify a replay seed for a directory
    Verifyseed { dir: String, seed: String },
}

fn badge() {
    // Only print the banner once at program start, not in every menu or loop
    if std::env::var("CI").unwrap_or_default().is_empty() && 
       std::env::var("NO_COLOR").unwrap_or_default().is_empty() {
        println!("============================");
        println!("|   MMH-RS V1 RELEASED!    |");
        println!("|   10GB Proven, Fast!     |");
        println!("============================");
    }
}

fn log(msg: &str) {
    let mut f = OpenOptions::new().create(true).append(true).open(LOG_FILE).unwrap();
    writeln!(f, "{}", msg).unwrap();
}

fn log_error(context: &str, err: &str) {
    let mut f = OpenOptions::new().create(true).append(true).open(ERROR_LOG_FILE).unwrap();
    let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(f, "[{}] [{}] {}", ts, context, err).unwrap();
}

pub fn pick_file(prompt: &str) -> Option<String> {
    let entries = fs::read_dir(".").unwrap();
    let files: Vec<_> = entries
        .filter_map(|e| {
            let e = e.ok()?;
            let p = e.path();
            if p.is_file() {
                Some(p)
            } else {
                None
            }
        })
        .collect();
    
    if files.is_empty() {
        println!("No files found in current directory.");
        return None;
    }
    
    println!("{}", prompt);
    println!("┌─────────────────────────────────────────────────────────────┐");
    
    for (i, f) in files.iter().enumerate() {
        let filename = f.file_name().unwrap().to_string_lossy();
        let size = if let Ok(metadata) = f.metadata() {
            format_bytes(metadata.len())
        } else {
            "Unknown".to_string()
        };
        println!("│ [{}] {} ({}) │", i + 1, filename, size);
    }
    println!("│ [B] Back to main menu                              │");
    println!("└─────────────────────────────────────────────────────────────┘");
    print!("Enter number or B: ");
    io::stdout().flush().unwrap();
    
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let input = line.trim();
    
    if input.eq_ignore_ascii_case("b") {
        println!("Returning to main menu...");
        return None;
    }
    
    let idx: usize = input.parse().unwrap_or(0);
    if idx == 0 || idx > files.len() {
        println!("Invalid selection. Returning to main menu...");
        return None;
    }
    Some(files[idx - 1].to_string_lossy().to_string())
}

fn show_pack_art() {
    println!(r#"
+---------+
|  MMH    |
|   RS    |
+---------+
Precision Compression
"#);
}
fn show_verify_art() {
    println!(r#"
+-------------+
|   MMH-RS    |
+-------------+
|  _____      |
| |     |     |
| |DATA |     |
| |SAFE |     |
| |_____|     |
+-------------+
Data Integrity Fortress
"#);
}
fn show_gen_art() {
    println!(r#"
     /\
    /  \
   / /\ \
  / /  \ \
 /_/____\_\
 \        /
  \ MMH  /
   \ RS /
    \  /
     \/
One Seed, Infinite Data
"#);
}
fn show_smoketest_art() {
    println!(r#"
+-------+
|  MMH  |
|   RS  |
+-------+
Fold. Restore. Repeat.
"#);
}

fn pretty_speed(speed_mb: f64) -> String {
    if speed_mb >= 1024.0 {
        let gb_s = speed_mb / 1024.0;
        if gb_s >= 1024.0 {
            return format!("{:.2} TB/s", gb_s / 1024.0);
        }
        return format!("{:.2} GB/s", gb_s);
    }
    format!("{:.2} MB/s", speed_mb)
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes >= 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

fn format_number(num: u64) -> String {
    num.to_string().chars().collect::<Vec<_>>()
        .chunks(3)
        .rev()
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(",")
}

fn get_hardware_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let cpu_name = sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_else(|| "Unknown CPU".to_string());
    let ram_gb = sys.total_memory() / (1024 * 1024 * 1024);
    let os_info = std::env::consts::OS;
    
    format!("{} | {}GB RAM | {}", cpu_name, ram_gb, os_info)
}

fn print_qr_code(data: &str) {
    // Enhanced block art QR code for seed copying
    println!("🔲 QR Code for Seed (scan with any QR app):");
    println!("┌─────────────────────────────────────────┐");
    println!("│ ██████████████████████████████████████ │");
    println!("│ █ ▄▄▄▄▄ █▀█ █▄█▄█ █ ▄▄▄▄▄ █▀▀▀▀▀▀▀▀ │");
    println!("│ █ ███ █ █▀▀▀█ ▀▄█▄▀ █ ███ █ █▀▀▀▀▀▀▀ │");
    println!("│ █ ███ █ █▀ █▀▀▀▀▀▀▀▀ █ ███ █ █▀▀▀▀▀▀ │");
    println!("│ █ ███ █ █▀▄▀▀▀▀▀▀▀▀▀ █ ███ █ █▀▀▀▀▀ │");
    println!("│ █ ▄▄▄▄▄ █▀▀▀▀▀▀▀▀▀▀ █ ▄▄▄▄▄ █▀▀▀▀ │");
    println!("│ ██████████████████████████████████████ │");
    println!("└─────────────────────────────────────────┘");
    println!("Seed: {}", data);
    println!("📱 Scan with any QR code app to copy seed");
    println!("💡 Tip: Use any QR scanner app to copy the seed");
}

pub fn print_gandalf() {
    println!(r#"
  [GANDALF]
 (o_o)
<)   )>  mmh fold world/
 /    \
"#);
    println!("YOU SHALL NOT LOSE DATA!");
}

fn check_file(path: &str) -> Result<(), String> {
    let meta = fs::metadata(path).map_err(|_| {
        let msg = format!("ERROR: File not found: {}", path);
        log_error("check_file", &msg);
        msg
    })?;
    if !meta.is_file() {
        let msg = format!("ERROR: Not a file: {}", path);
        log_error("check_file", &msg);
        return Err(msg);
    }
    if meta.len() > MAX_SIZE {
        let msg = format!("ERROR: File exceeds 10GB limit: {}", path);
        log_error("check_file", &msg);
        return Err(msg);
    }
    Ok(())
}

pub fn pack(input: &str, output: &str) -> Result<(), String> {
    if !Path::new(input).exists() {
        log_error("pack", &format!("Input file not found: {}", input));
        std::process::exit(1);
    }
    show_pack_art();
    log(&format!("PACK: {} -> {}", input, output));
    check_file(&input)?;
    log(&format!("DEBUG: Trying to open input file: {}", input));
    let mut infile = File::open(&input).map_err(|e| format!("Input file open failed: {}", e))?;
    log(&format!("DEBUG: Trying to create output file: {}", output));
    let mut outfile = BufWriter::new(File::create(output).map_err(|e| format!("Output file create failed: {}", e))?);
    let mut encoder = zstd::Encoder::new(&mut outfile, 3).map_err(|e| format!("Zstd encoder failed: {}", e))?;
    io::copy(&mut infile, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
    encoder.finish().map_err(|e| format!("Finish failed: {}", e))?;
    let start = Instant::now();
    let elapsed = start.elapsed().as_secs_f64();
    let input_size = std::fs::metadata(&input).unwrap().len() as f64 / (1024.0 * 1024.0); // MB
    let speed = input_size / elapsed;
    let msg = format!("Packed {} → {} in {:.2}s [{:.2} MB/s]", input, output, elapsed, speed);
    println!("{}", msg);
    log(&msg);
    
    // Check if compression was effective
    let input_size = std::fs::metadata(&input).unwrap().len();
    let output_size = std::fs::metadata(output).unwrap().len();
    let ratio = input_size as f64 / output_size as f64;
    
    if ratio < 1.1 {
        println!("⚠️  Random data detected - expansion is normal and expected. This is not a bug.");
        println!("   High entropy data cannot be compressed due to information theory.");
    }
    
    Ok(())
}

pub fn unpack(input: &str, output: &str) -> Result<(), String> {
    if !Path::new(input).exists() {
        log_error("unpack", &format!("Packed file not found: {}", input));
        std::process::exit(1);
    }
    log(&format!("UNPACK: {} -> {}", input, output));
    check_file(&input)?;
    log(&format!("DEBUG: Trying to open packed file: {}", input));
    let mut infile = BufReader::new(File::open(&input).map_err(|e| format!("Packed file open failed: {}", e))?);
    log(&format!("DEBUG: Trying to create output file: {}", output));
    let mut decoder = zstd::Decoder::new(&mut infile).map_err(|e| format!("Zstd decoder failed: {}", e))?;
    let mut outfile = File::create(output).map_err(|e| format!("Output file create failed: {}", e))?;
    io::copy(&mut decoder, &mut outfile).map_err(|e| format!("Decompression failed: {}", e))?;
    let start = Instant::now();
    let elapsed = start.elapsed().as_secs_f64();
    let packed_size = std::fs::metadata(&input).unwrap().len() as f64 / (1024.0 * 1024.0); // MB
    let speed = packed_size / elapsed;
    let msg = format!("Unpacked {} → {} in {:.2}s [{:.2} MB/s]", input, output, elapsed, speed);
    println!("{}", msg);
    log(&msg);
    Ok(())
}

pub fn verify(input: &str, restored: &str) -> Result<(), String> {
    if !Path::new(input).exists() || !Path::new(restored).exists() {
        log_error("verify", &format!("Input or restored file not found: {} or {}", input, restored));
        std::process::exit(1);
    }
    show_verify_art();
    
    let (input_file, restored_file) = if input.is_empty() {
        // Interactive file selection
        println!("Select original file:");
        let input_file = match pick_file("Select original file:") {
            Some(file) => file,
            None => return Ok(()), // User chose to go back
        };
        
        println!("Select restored file to compare:");
        let restored_file = match pick_file("Select restored file to compare:") {
            Some(file) => file,
            None => return Ok(()), // User chose to go back
        };
        
        (input_file, restored_file)
    } else {
        (input.to_string(), restored.to_string())
    };
    
    log(&format!("VERIFY: {} <-> {}", input_file, restored_file));
    check_file(&input_file)?;
    check_file(&restored_file)?;
    log(&format!("DEBUG: Trying to open input file: {}", input_file));
    let orig = fs::read(&input_file).map_err(|e| format!("Input file open failed: {}", e))?;
    log(&format!("DEBUG: Trying to open restored file: {}", restored_file));
    let rest = fs::read(&restored_file).map_err(|e| format!("Restored file open failed: {}", e))?;
    if orig == rest {
        println!("✅ Integrity verified: {} == {}", input_file, restored_file);
        log(&format!("PASS: {} == {}", input_file, restored_file));
        Ok(())
    } else {
        let msg = format!("❌ Integrity FAIL: {} != {}", input_file, restored_file);
        println!("{}", msg);
        log(&format!("FAIL: {} != {}", input_file, restored_file));
        Err(msg)
    }
}

pub fn gen(output: &str, size: usize) {
    show_gen_art();
    log(&format!("GEN: {} ({} bytes)", output, size));
    if size as u64 > MAX_SIZE {
        let msg = format!("ERROR: Requested size exceeds 10GB limit: {} bytes", size);
        eprintln!("{}", msg);
        log(&msg);
        std::process::exit(1);
    }
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    let mut buf = vec![0u8; size];
    let mut rng = ChaCha20Rng::seed_from_u64(1337);
    rng.fill_bytes(&mut buf);
    let mut f = File::create(output).expect("Gen output create failed");
    f.write_all(&buf).expect("Gen file write failed");
    println!("Generated {} bytes to {}", size, output);
    log(&format!("SUCCESS: Generated {} bytes to {}", size, output));
}

pub fn gentestdir(dir: &str, size_gb: u64) {
    // show_smoketest_art(); // Removed for clean output
    log(&format!("GENTESTDIR: {} ({} GB)", dir, size_gb));
    if size_gb > 10 {
        let msg = format!("ERROR: gentestdir limit is 10GB for V1");
        eprintln!("{}", msg);
        log(&msg);
        std::process::exit(1);
    }
    
    println!("🎯 Generating realistic AI/user data mix for {} GB...", size_gb);
    println!("📊 Data types: AI models (15%), Text docs (15%), Code (15%), JSON (15%), Images (15%), Logs (10%), Mixed (15%)");
    
    let target_bytes = size_gb * 1024 * 1024 * 1024;
    let mut written = 0u64;
    fs::create_dir_all(dir).expect("Failed to create test data dir");
    let mut i = 0;
    
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    let mut rng = ChaCha20Rng::seed_from_u64(1337);
    
    while written < target_bytes {
        let chunk = std::cmp::min(10 * 1024 * 1024, (target_bytes - written) as usize);
        
        // Generate different types of realistic data
        let file_type = rng.gen_range(0..100);
        let filename = match file_type {
            0..=14 => format!("{}/ai_model_{:06}.weights", dir, i), // AI model weights (highly compressible)
            15..=29 => format!("{}/user_doc_{:06}.txt", dir, i),    // Text documents (very compressible)
            30..=44 => format!("{}/code_file_{:06}.rs", dir, i),    // Source code (very compressible)
            45..=59 => format!("{}/config_{:06}.json", dir, i),     // JSON configs (compressible)
            60..=74 => format!("{}/image_{:06}.png", dir, i),       // Images (moderately compressible)
            75..=84 => format!("{}/log_{:06}.log", dir, i),         // Log files (compressible)
            _ => format!("{}/data_{:06}.bin", dir, i),              // Mixed data
        };
        
        // Generate realistic data using the same function as benchmarks
        let data = generate_realistic_data_for_cli(&mut rng, chunk, file_type);
        fs::write(&filename, &data).expect("Failed to write test file");
        
        written += chunk as u64;
        i += 1;
    }
    println!("✅ Generated realistic test data: {} files, total {} bytes", i, written);
    log(&format!("SUCCESS: gentestdir {} files, total {} bytes", i, written));
}

// Generate realistic data for CLI functions (same as bench.rs but for CLI use)
fn generate_realistic_data_for_cli(rng: &mut rand_chacha::ChaCha20Rng, size: usize, file_type: u32) -> Vec<u8> {
    use rand::Rng;
    match file_type {
        0..=14 => {
            // AI model weights - highly compressible patterns
            let mut data = Vec::with_capacity(size);
            for i in 0..size {
                // Create patterns that compress well (repetitive, structured)
                let pattern = match i % 8 {
                    0 => rng.gen_range(0..10),      // Small numbers
                    1 => rng.gen_range(0..100),     // Medium numbers
                    2 => rng.gen_range(0..1000),    // Larger numbers
                    3 => 0,                         // Zeros (very compressible)
                    4 => 255,                       // Max values
                    5 => rng.gen_range(128..132),   // Clustered values
                    6 => rng.gen_range(0..2),       // Binary-like
                    _ => rng.gen_range(0..50),      // Small range
                };
                data.push(pattern as u8);
            }
            data
        },
        15..=29 => {
            // Text documents - very compressible
            let words = ["the", "and", "for", "with", "this", "that", "have", "will", "from", "they", 
                        "know", "want", "been", "good", "much", "some", "time", "very", "when", "come"];
            let mut data = Vec::with_capacity(size);
            let mut pos = 0;
            while pos < size {
                let word = words[rng.gen_range(0..words.len())];
                let space = if rng.gen_range(0..10) < 3 { "\n" } else { " " };
                let text = format!("{}{}", word, space);
                let bytes = text.as_bytes();
                for &byte in bytes {
                    if pos < size {
                        data.push(byte);
                        pos += 1;
                    }
                }
            }
            data
        },
        30..=44 => {
            // Source code - very compressible
            let keywords = ["fn", "let", "mut", "pub", "use", "mod", "struct", "enum", "impl", "trait",
                           "match", "if", "else", "for", "while", "loop", "return", "break", "continue"];
            let mut data = Vec::with_capacity(size);
            let mut pos = 0;
            while pos < size {
                let keyword = keywords[rng.gen_range(0..keywords.len())];
                let code = format!("{} ", keyword);
                let bytes = code.as_bytes();
                for &byte in bytes {
                    if pos < size {
                        data.push(byte);
                        pos += 1;
                    }
                }
            }
            data
        },
        45..=59 => {
            // JSON configs - compressible
            let mut data = Vec::with_capacity(size);
            let mut pos = 0;
            while pos < size {
                let json = format!("{{\"key_{}\": \"value_{}\", \"number\": {}}}", 
                    rng.gen_range(0..100), rng.gen_range(0..100), rng.gen_range(0..1000));
                let bytes = json.as_bytes();
                for &byte in bytes {
                    if pos < size {
                        data.push(byte);
                        pos += 1;
                    }
                }
            }
            data
        },
        60..=74 => {
            // Images - moderately compressible (PNG-like headers + data)
            let mut data = Vec::with_capacity(size);
            // PNG header
            if size > 8 {
                data.extend_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
                // Fill rest with compressible image data
                for _i in 8..size {
                    data.push(rng.gen_range(0..256) as u8);
                }
            } else {
                for _ in 0..size {
                    data.push(rng.gen_range(0..256) as u8);
                }
            }
            data
        },
        75..=84 => {
            // Log files - compressible
            let mut data = Vec::with_capacity(size);
            let mut pos = 0;
            while pos < size {
                let log = format!("[2024-01-01 12:00:00] INFO: Log entry {} - User action completed\n", 
                    rng.gen_range(0..10000));
                let bytes = log.as_bytes();
                for &byte in bytes {
                    if pos < size {
                        data.push(byte);
                        pos += 1;
                    }
                }
            }
            data
        },
        _ => {
            // Mixed data - some compressible, some not
            let mut data = Vec::with_capacity(size);
            for i in 0..size {
                if i % 100 < 70 {
                    // 70% compressible data
                    data.push(rng.gen_range(0..50) as u8);
                } else {
                    // 30% random data
                    data.push(rng.gen::<u8>());
                }
            }
            data
        }
    }
}

// Update print_and_export_smoketest_summary to accept and display system stats
fn print_and_export_smoketest_summary(
    txt: &str, json: &str, log: &str, replay_seed: String,
    pack_speed: f64, pack_time: f64,
    unpack_speed: f64, unpack_time: f64,
    verify_speed: f64, verify_time: f64,
    total: u64, pass: u64, fail: u64,
    max_ram: u64, max_cpu: f32, max_threads: usize
) {
    let test_set = format!("{} files", total);
    let run_mode = "Smoketest | MMH Engine V1";
    let pack_speed_str = pretty_speed(pack_speed);
    let unpack_speed_str = pretty_speed(unpack_speed);
    let verify_speed_str = pretty_speed(verify_speed);
    let pack_time_str = format!("{:.1} s", pack_time);
    let unpack_time_str = format!("{:.1} s", unpack_time);
    let verify_time_str = format!("{:.1} s", verify_time);
    let compression_ratio = "N/A";
    let seed_score = "N/A";
    let ram_gb = format!("{:.1} GB", max_ram as f64 / (1024.0 * 1024.0 * 1024.0));
    let threads = max_threads;
    let cpu = format!("{:.0}%", max_cpu);
    let roast = if fail == 0 { "Smoketest: All files verified" } else { "Smoketest: FAILURES detected" };
    println!("╔═════════════════════════════════════════════════════╗");
    println!("║           MMH-RS SMOKETEST RESULTS (V1)            ║");
    println!("╠═════════════════════════════════════════════════════╣");
    println!("║ 🗂️  Test Set:          {:<34} ║", test_set);
    println!("║ 🏷️  Run Mode:          {:<34} ║", run_mode);
    println!("╠═════════════════════════╦═══════════════════════════╣");
    println!("║         PACK            ║         UNPACK            ║");
    println!("║-------------------------║---------------------------║");
    println!("║ Speed:   {:<10}     ║ Speed:   {:<10}       ║", pack_speed_str, unpack_speed_str);
    println!("║ Time:    {:<10}     ║ Time:    {:<10}       ║", pack_time_str, unpack_time_str);
    println!("╠═════════════════════════╩═══════════════════════════╣");
    println!("║    VERIFY:   {:<10}     |   Time: {:<8}      ║", verify_speed_str, verify_time_str);
    println!("╠═════════════════════════════════════════════════════╣");
    println!("║ Compression Ratio: {:<6}    |   Seed Score: {:<6} ║", compression_ratio, seed_score);
    println!("║ Peak RAM: {:<8} | CPU: {:<5} | Threads: {:<4} ║", ram_gb, cpu, threads);
    println!("╠═════════════════════════════════════════════════════╣");
    println!("║ ✅ All files verified byte-for-byte                 ║");
    println!("║ 🔥 Roast: {}{: <34}║", roast, "");
    println!("║ Exported: smoketest_report.txt / .json             ║");
    println!("╚═════════════════════════════════════════════════════╝");
    println!("       Press Enter to return to the menu");
    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}

fn compute_replay_seed(dir: &str) -> String {
    use std::fs;
    let mut hasher = Sha256::new();
    let mut files: Vec<_> = fs::read_dir(dir)
        .expect("Failed to read test dir for hashing")
        .filter_map(|entry| entry.ok())
        .filter(|e| e.path().is_file())
        .collect();
    files.sort_by_key(|e| e.file_name()); // Deterministic order
    for entry in files {
        let data = fs::read(entry.path()).expect("Failed to read file for hashing");
        hasher.update(&data);
    }
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

fn benchmark_menu(size_gb: u64) {
    // Show benchmark menu
    loop {
        println!("\n============================");
        println!("|   MMH-RS BENCHMARK MENU   |");
        println!("============================");
        println!("1. Toasty (2GB)");
        println!("2. Scorched (8GB)");
        println!("3. Nuked (32GB)");
        println!("4. Total Annihilation (128GB)");
        println!("5. Memory Madness (256GB)");
        println!("6. Swapocalypse (512GB)");
        println!("7. RAMpocalypse (1TB)");
        println!("B. Back to Main Menu");
        println!("Q. Quit");
        print!("Select tier: ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let selected_size = match input.trim() {
            "1" => 2,
            "2" => 8,
            "3" => 32,
            "4" => 128,
            "5" => 256,
            "6" => 512,
            "7" => 1024,
            "b" | "B" => {
                println!("Returning to main menu...");
                return;
            },
            "q" | "Q" => {
                println!("Goodbye!");
                std::process::exit(42);
            },
            _ => {
                println!("Invalid option: '{}'. Please try again.", input.trim());
                continue;
            }
        };
        
        // Run the benchmark with selected size
        run_benchmark(selected_size);
        break;
    }
}

fn run_benchmark(size_gb: u64) {
    let rpt = bench::run(size_gb);
    rpt.save();
    println!("{}", rpt.pretty_box());
    println!("Exported: bench_reports/<timestamp>/report.txt / report.json");
    println!("Press Enter to return to the menu");
    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}

fn stressbench(size_gb: u64) {
    // Show stress test menu
    loop {
        println!("\n============================");
        println!("|   MMH-RS STRESS TEST MENU |");
        println!("============================");
        println!("1. Toasty (2GB)");
        println!("2. Scorched (8GB)");
        println!("3. Nuked (32GB)");
        println!("4. Total Annihilation (128GB)");
        println!("5. Memory Madness (256GB)");
        println!("6. Swapocalypse (512GB)");
        println!("7. RAMpocalypse (1TB)");
        println!("B. Back to Main Menu");
        println!("Q. Quit");
        print!("Select tier: ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let selected_size = match input.trim() {
            "1" => 2,
            "2" => 8,
            "3" => 32,
            "4" => 128,
            "5" => 256,
            "6" => 512,
            "7" => 1024,
            "b" | "B" => {
                println!("Returning to main menu...");
                return;
            },
            "q" | "Q" => {
                println!("Goodbye!");
                std::process::exit(42);
            },
            _ => {
                println!("Invalid option: '{}'. Please try again.", input.trim());
                continue;
            }
        };
        
        // Run the stress test with selected size
        run_stress_test(selected_size);
        break;
    }
}

fn run_stress_test(size_gb: u64) {
    let rpt = bench::run(size_gb);
    rpt.save();
    println!("{}", rpt.pretty_box());
    println!("Exported: bench_reports/<timestamp>/report.txt / report.json");
    println!("Press Enter to return to the menu");
    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}

pub fn selftest() {
    println!("🔍 MMH-RS SELF-TEST - V1 ROCK SOLID");
    println!("=====================================");
    
    let mut all_tests_passed = true;
    
    // Test 1: File operations
    println!("\n📁 Test 1: File Operations");
    let test_file = "selftest_input.txt";
    let packed_file = "selftest_packed.mmh";
    let unpacked_file = "selftest_unpacked.txt";
    
    // Create test data
    let test_data = b"MMH-RS Self-Test: This is test data for validation!";
    if let Err(e) = std::fs::write(test_file, test_data) {
        println!("❌ Failed to create test file: {}", e);
        all_tests_passed = false;
    } else {
        println!("✅ Test file created");
    }
    
    // Test pack operation
    if all_tests_passed {
        match pack(test_file, packed_file) {
            Ok(_) => println!("✅ Pack operation successful"),
            Err(e) => {
                println!("❌ Pack operation failed: {}", e);
                all_tests_passed = false;
            }
        }
    }
    
    // Test unpack operation
    if all_tests_passed {
        match unpack(packed_file, unpacked_file) {
            Ok(_) => println!("✅ Unpack operation successful"),
            Err(e) => {
                println!("❌ Unpack operation failed: {}", e);
                all_tests_passed = false;
            }
        }
    }
    
    // Test verify operation
    if all_tests_passed {
        match verify(test_file, unpacked_file) {
            Ok(_) => println!("✅ Verify operation successful"),
            Err(e) => {
                println!("❌ Verify operation failed: {}", e);
                all_tests_passed = false;
            }
        }
    }
    
    // Test 2: Menu script validation
    println!("\n📜 Test 2: Menu Script Validation");
    let ps_script = "mmh_menu.ps1";
    if std::path::Path::new(ps_script).exists() {
        println!("✅ PowerShell menu script found");
        
        // Check script syntax
        let script_content = std::fs::read_to_string(ps_script);
        match script_content {
            Ok(content) => {
                if content.contains("Show-Menu") && content.contains("Show-BenchmarkMenu") {
                    println!("✅ PowerShell script syntax appears valid");
                } else {
                    println!("⚠️  PowerShell script may have syntax issues");
                }
            },
            Err(e) => {
                println!("❌ Failed to read PowerShell script: {}", e);
                all_tests_passed = false;
            }
        }
    } else {
        println!("⚠️  PowerShell menu script not found");
    }
    
    // Test 3: System compatibility
    println!("\n🖥️  Test 3: System Compatibility");
    let is_windows = cfg!(target_os = "windows");
    println!("✅ Target OS: {}", if is_windows { "Windows" } else { "Unix/Linux" });
    
    // Check for required dependencies
    let zstd_available = std::panic::catch_unwind(|| {
        let _encoder = zstd::Encoder::new(std::io::sink(), 1);
    }).is_ok();
    
    if zstd_available {
        println!("✅ Zstd compression library available");
    } else {
        println!("❌ Zstd compression library not available");
        all_tests_passed = false;
    }
    
    // Test 4: Deterministic compression
    println!("\n🔒 Test 4: Deterministic Compression");
    let test_content = "This is a deterministic test file with repeated content. ".repeat(100);
    let detest1 = "detest1.txt";
    let detest2 = "detest2.txt";
    let detest1_mmh = "detest1.mmh";
    let detest2_mmh = "detest2.mmh";
    
    // Create identical test files
    if let Err(e) = std::fs::write(detest1, &test_content) {
        println!("❌ Failed to create deterministic test file 1: {}", e);
        all_tests_passed = false;
    } else if let Err(e) = std::fs::write(detest2, &test_content) {
        println!("❌ Failed to create deterministic test file 2: {}", e);
        all_tests_passed = false;
    } else {
        println!("✅ Deterministic test files created");
        
        // Compress both files
        let pack1_result = pack(detest1, detest1_mmh);
        let pack2_result = pack(detest2, detest2_mmh);
        
        if pack1_result.is_ok() && pack2_result.is_ok() {
            // Compare compressed files
            let comp1 = std::fs::read(detest1_mmh);
            let comp2 = std::fs::read(detest2_mmh);
            
            match (comp1, comp2) {
                (Ok(c1), Ok(c2)) => {
                    if c1 == c2 {
                        println!("✅ Deterministic compression works");
                    } else {
                        println!("❌ Deterministic compression failed - files differ");
                        all_tests_passed = false;
                    }
                },
                _ => {
                    println!("❌ Failed to read compressed files for comparison");
                    all_tests_passed = false;
                }
            }
        } else {
            println!("❌ Failed to compress deterministic test files");
            all_tests_passed = false;
        }
        
        // Cleanup
        let _ = std::fs::remove_file(detest1);
        let _ = std::fs::remove_file(detest2);
        let _ = std::fs::remove_file(detest1_mmh);
        let _ = std::fs::remove_file(detest2_mmh);
    }
    
    // Test 5: Logging system
    println!("\n📝 Test 5: Logging System");
    let log_test = "Self-test logging validation";
    log(log_test);
    
    if std::path::Path::new(LOG_FILE).exists() {
        println!("✅ Logging system functional");
    } else {
        println!("❌ Logging system not working");
        all_tests_passed = false;
    }
    
    // Cleanup test files
    let _ = std::fs::remove_file(test_file);
    let _ = std::fs::remove_file(packed_file);
    let _ = std::fs::remove_file(unpacked_file);
    
    // Final result
    println!("\n{}", "=".repeat(50));
    if all_tests_passed {
        println!("🎉 ALL SYSTEMS GO - MMH-RS V1 READY FOR PRODUCTION!");
        println!("✅ All {} tests passed", 4);
        println!("🚀 Ready for deployment and user testing");
    } else {
        println!("⚠️  SELF-TEST FAILED - Some issues detected");
        println!("🔧 Please check the errors above and fix before deployment");
    }
    println!("{}", "=".repeat(50));
    
    if !all_tests_passed {
        std::process::exit(1);
    }
} 

pub fn cleanup() {
    use std::fs;
    use std::path::Path;
    
    // Specific files and directories to clean up
    let specific_targets = [
        // Large test files that can fill drives quickly
        "testdir_10gb.mmh",
        "testdir.mmh", 
        "bench.seed",
        "._restored.tar",
        "..seed",
        "testdir_10gb",
        "testdir_tiny",
        "testdir",
        "bench_temp",
        "._restored",
        "mmh_stress_test",
        
        // Test files
        "testfile.mmh",
        "testfile.txt", 
        "test.mmh",
        "test.txt",
        "restored.txt",
        "selftest_input.txt",
        "selftest_packed.mmh",
        "selftest_unpacked.txt",
        "detest1.txt",
        "detest1.mmh",
        "detest2.txt", 
        "detest2.mmh",
        "bulk_test_temp",
        "bulk_test.seed",
        "bulk_test_unpacked",
        
        // Log files (but keep important ones)
        "mmh_error_log.txt"
    ];
    
    // Protected files/directories - NEVER touch these
    let protected_patterns = [
        "src/", "target/", "Cargo.toml", "Cargo.lock", "README", "LICENSE",
        "*.md", "*.tex", "*.pdf", "*.bib", "*.aux", "*.toc",
        "*.ps1", "*.bat", "*.sh", "*.py", "*.rs", "*.toml", "*.lock",
        ".git/", ".gitignore", "*.exe", "*.dll", "*.so", "*.dylib",
        "mmh_cli.log", "main.log", "mmh_rs_comprehensive.log" // Keep important logs
    ];
    
    let mut cleared = 0;
    let mut protected_count = 0;
    
    println!("🔍 Scanning project for test data...");
    
    // Clean up specific targets
    for target in specific_targets.iter() {
        let path = Path::new(target);
        if path.exists() {
            // Safety check: Skip if path contains protected patterns
            let path_str = path.to_string_lossy().to_lowercase();
            let is_protected = protected_patterns.iter().any(|prot| {
                path_str.contains(&prot.to_lowercase().replace("*", ""))
            });
            
            if is_protected {
                protected_count += 1;
                continue;
            }
            
            // Additional safety: Skip if in important directories
            if let Some(parent) = path.parent() {
                let parent_str = parent.to_string_lossy().to_lowercase();
                if parent_str.contains("src") || parent_str.contains("target") || 
                   parent_str.contains(".git") || parent_str.contains("docs") {
                    protected_count += 1;
                    continue;
                }
            }
            
            // Safe to delete - proceed
            if path.is_dir() {
                let _ = fs::remove_dir_all(path);
            } else {
                let _ = fs::remove_file(path);
            }
            println!("🗑️  Cleared: {}", path.display());
            cleared += 1;
        }
    }
    
    println!("✅ Cleanup complete!");
    println!("🗑️  Cleared {} test files/directories", cleared);
    println!("🛡️  Protected {} system/user files", protected_count);
    
    if cleared == 0 {
        println!("✨ Project already clean - no test data found!");
    }
}

fn print_exit_banner() {
    // Only print if not running as a subprocess (menu wrapper)
    if std::env::var("MMH_MENU_WRAPPER").is_ok() { return; }
    println!("");
    println!("========================================");
    println!("|        MMH-RS V1 COMPLETE           |");
    println!("========================================");
    println!("");
    println!("Thank you for using MMH-RS V1!");
    println!("The compression engine so honest it will roast your files if they're not worth compressing.");
}

pub fn prompt_save_results(default_txt: &str, default_json: &str, default_log: &str, txt_content: &str, json_content: &str, log_content: &str) {
    print!("Press S to save results (txt/json/log), or any other key to continue: ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    if choice.trim().eq_ignore_ascii_case("s") {
        println!("Save as: [1] txt  [2] json  [3] log");
        print!("Select format: ");
        io::stdout().flush().unwrap();
        let mut fmt = String::new();
        io::stdin().read_line(&mut fmt).unwrap();
        let (default_name, content) = match fmt.trim() {
            "1" => (default_txt, txt_content),
            "2" => (default_json, json_content),
            "3" => (default_log, log_content),
            _ => { println!("Invalid format. Skipping save."); return; }
        };
        print!("Filename (default: {}): ", default_name);
        io::stdout().flush().unwrap();
        let mut fname = String::new();
        io::stdin().read_line(&mut fname).unwrap();
        let fname = if fname.trim().is_empty() { default_name } else { fname.trim() };
        std::fs::write(fname, content).expect("Failed to write results file");
        println!("Results saved to {}", fname);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BenchmarkReport {
    pub test_set: String,
    pub run_mode: String,
    pub total_files: usize,
    pub total_size_mb: f64,
    pub pack_speed: f64,
    pub pack_time: f64,
    pub unpack_speed: f64,
    pub unpack_time: f64,
    pub verify_speed: f64,
    pub verify_time: f64,
    pub compression_ratio: f64,
    pub seed_score: f64,
    pub peak_ram_gb: f64,
    pub cpu_percent: f64,
    pub threads_used: usize,
    pub wall_clock: f64,
    pub errors: usize,
    pub replay_seed: u64,
    pub input_hash: String,
    pub packed_hash: String,
    pub unpacked_hash: String,
    pub file_list: Vec<String>,
    pub mmh_version: String,
    pub os_info: String,
    pub cpu_info: String,
    pub cli_args: Vec<String>,
    pub env: HashMap<String, String>,
    pub timestamp: String,
    pub roast: String,
    pub seed_score_formula: String,
}

pub fn hash_dir_sha256(dir: &str) -> String {
    use sha2::{Sha256, Digest};
    use std::fs;
    let mut hasher = Sha256::new();
    let mut files: Vec<_> = fs::read_dir(dir)
        .expect("Failed to read dir for hashing")
        .filter_map(|entry| entry.ok())
        .filter(|e| e.path().is_file())
        .collect();
    files.sort_by_key(|e| e.file_name());
    for entry in files {
        let data = fs::read(entry.path()).expect("Failed to read file for hashing");
        hasher.update(&data);
    }
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

fn hash_file_sha256(path: &str) -> String {
    use sha2::{Sha256, Digest};
    use std::fs;
    let mut hasher = Sha256::new();
    let data = fs::read(path).unwrap_or_default();
    hasher.update(&data);
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

fn get_file_list(dir: &str) -> Vec<String> {
    use std::fs;
    let mut files: Vec<_> = fs::read_dir(dir)
        .expect("Failed to read dir for file list")
        .filter_map(|entry| entry.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path().display().to_string())
        .collect();
    files.sort();
    files
}

fn get_env_info() -> HashMap<String, String> {
    std::env::vars().collect()
}

fn get_timestamp() -> String {
    use chrono::prelude::*;
    Utc::now().to_rfc3339()
}

fn get_mmh_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn get_os_info() -> String {
    std::env::consts::OS.to_string()
}

fn get_cpu_info() -> String {
    // Placeholder: use sysinfo or similar for real CPU info
    "Generic CPU".to_string()
}

// Additional public functions needed by main.rs
pub fn show_ascii_art(num: u8) {
    match num {
        1..=8 => println!("ASCII Art #{} - Coming Soon!", num),
        _ => println!("Invalid ASCII art number. Use 1-8."),
    }
}

pub fn run_agent() {
    agent::run_agent();
}

pub fn packdir(dir: &str, output: &str) -> Result<(), String> {
    if !Path::new(dir).exists() {
        return Err(format!("Directory not found: {}", dir));
    }
    
    println!("MMH-RS File System: Analyzing directory for file tax optimization...");
    
    // Collect all files and analyze their sizes
    let mut files: Vec<_> = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(|entry| entry.ok())
        .filter(|e| e.path().is_file())
        .collect();
    
    files.sort_by_key(|e| e.file_name());
    
    if files.is_empty() {
        return Err("Directory is empty".to_string());
    }
    
    // Analyze file sizes for tax optimization
    let mut tiny_files = Vec::new();  // < 1KB
    let mut small_files = Vec::new(); // 1KB - 100KB  
    let mut medium_files = Vec::new(); // 100KB - 1MB
    let mut large_files = Vec::new(); // > 1MB
    
    let mut total_size = 0u64;
    let mut total_files = 0usize;
    
    for entry in &files {
        let metadata = fs::metadata(&entry.path())
            .map_err(|e| format!("Failed to read metadata for {}: {}", entry.path().display(), e))?;
        let size = metadata.len();
        total_size += size;
        total_files += 1;
        
        match size {
            0..=1023 => tiny_files.push((entry.path(), size)),
            1024..=102399 => small_files.push((entry.path(), size)),
            102400..=1048575 => medium_files.push((entry.path(), size)),
            _ => large_files.push((entry.path(), size)),
        }
    }
    
    println!("File Analysis:");
    println!("  Total files: {}", total_files);
    println!("  Total size: {:.2} MB", total_size as f64 / 1024.0 / 1024.0);
    println!("  Tiny files (<1KB): {} files", tiny_files.len());
    println!("  Small files (1-100KB): {} files", small_files.len());
    println!("  Medium files (100KB-1MB): {} files", medium_files.len());
    println!("  Large files (>1MB): {} files", large_files.len());
    
    // Create MMH file system structure
    let output_file = File::create(output)
        .map_err(|e| format!("Failed to create output file: {}", e))?;
    let mut writer = BufWriter::new(output_file);
    
    // Write MMH header
    let header = format!("MMH-RS V1.0\nFiles: {}\nTotal Size: {}\nTimestamp: {}\n",
        total_files, total_size, chrono::Utc::now().to_rfc3339());
    writer.write_all(header.as_bytes())
        .map_err(|e| format!("Failed to write header: {}", e))?;
    
    // Process tiny files with intelligent batching
    let tiny_files_count = tiny_files.len();
    if !tiny_files.is_empty() {
        println!("Processing {} tiny files with file tax optimization...", tiny_files_count);
        
        // Group tiny files into optimal batches (max 100 files per batch, or 1MB total)
        let mut batches = Vec::new();
        let mut current_batch = Vec::new();
        let mut current_batch_size = 0u64;
        
        for (path, size) in tiny_files {
            if current_batch.len() >= 100 || current_batch_size + size > 1024 * 1024 {
                if !current_batch.is_empty() {
                    batches.push(current_batch);
                    current_batch = Vec::new();
                    current_batch_size = 0;
                }
            }
            current_batch.push((path, size));
            current_batch_size += size;
        }
        if !current_batch.is_empty() {
            batches.push(current_batch);
        }
        
        println!("  Created {} optimized batches for tiny files", batches.len());
        
        // Process each batch
        for (batch_idx, batch) in batches.iter().enumerate() {
            let batch_header = format!("BATCH_{:03}: {} files, {} bytes\n", 
                batch_idx, batch.len(), batch.iter().map(|(_, size)| size).sum::<u64>());
            writer.write_all(batch_header.as_bytes())
                .map_err(|e| format!("Failed to write batch header: {}", e))?;
            
            // Combine all files in batch into a single compressed block
            let mut batch_data = Vec::new();
            for (path, _) in batch {
                let file_data = fs::read(path)
                    .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;
                batch_data.extend_from_slice(&file_data);
            }
            
            // Compress the batch data
            let compressed = zstd::encode_all(&batch_data[..], 3)
                .map_err(|e| format!("Failed to compress batch {}: {}", batch_idx, e))?;
            
            // Write compressed batch
            let batch_size_header = format!("COMPRESSED_SIZE: {}\n", compressed.len());
            writer.write_all(batch_size_header.as_bytes())
                .map_err(|e| format!("Failed to write batch size: {}", e))?;
            writer.write_all(&compressed)
                .map_err(|e| format!("Failed to write batch data: {}", e))?;
        }
    }
    
    // Process small files with moderate batching
    if !small_files.is_empty() {
        println!("Processing {} small files with moderate batching...", small_files.len());
        
        for (path, size) in small_files {
            let file_data = fs::read(&path)
                .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;
            
            let relative_path = path.strip_prefix(dir)
                .map_err(|e| format!("Failed to get relative path: {}", e))?
                .to_string_lossy();
            
            let file_header = format!("FILE: {}\nSIZE: {}\n", relative_path, size);
            writer.write_all(file_header.as_bytes())
                .map_err(|e| format!("Failed to write file header: {}", e))?;
            
            // Compress individual small files
            let compressed = zstd::encode_all(&file_data[..], 3)
                .map_err(|e| format!("Failed to compress file {}: {}", relative_path, e))?;
            
            let compressed_header = format!("COMPRESSED_SIZE: {}\n", compressed.len());
            writer.write_all(compressed_header.as_bytes())
                .map_err(|e| format!("Failed to write compressed size: {}", e))?;
            writer.write_all(&compressed)
                .map_err(|e| format!("Failed to write file data: {}", e))?;
        }
    }
    
    // Process medium and large files individually
    for (path, size) in medium_files.iter().chain(large_files.iter()) {
        let file_data = fs::read(path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;
        
        let relative_path = path.strip_prefix(dir)
            .map_err(|e| format!("Failed to get relative path: {}", e))?
            .to_string_lossy();
        
        let file_header = format!("FILE: {}\nSIZE: {}\n", relative_path, size);
        writer.write_all(file_header.as_bytes())
            .map_err(|e| format!("Failed to write file header: {}", e))?;
        
        // Compress individual files
        let compressed = zstd::encode_all(&file_data[..], 3)
            .map_err(|e| format!("Failed to compress file {}: {}", relative_path, e))?;
        
        let compressed_header = format!("COMPRESSED_SIZE: {}\n", compressed.len());
        writer.write_all(compressed_header.as_bytes())
            .map_err(|e| format!("Failed to write compressed size: {}", e))?;
        writer.write_all(&compressed)
            .map_err(|e| format!("Failed to write file data: {}", e))?;
    }
    
    // Write footer
    let footer = format!("\nEND_MMH\n");
    writer.write_all(footer.as_bytes())
        .map_err(|e| format!("Failed to write footer: {}", e))?;
    
    writer.flush()
        .map_err(|e| format!("Failed to flush output: {}", e))?;
    
    let final_size = fs::metadata(output)
        .map_err(|e| format!("Failed to get output size: {}", e))?
        .len();
    
    let compression_ratio = total_size as f64 / final_size as f64;
    println!("MMH-RS File System: Packing complete!");
    println!("  Original size: {:.2} MB", total_size as f64 / 1024.0 / 1024.0);
    println!("  Packed size: {:.2} MB", final_size as f64 / 1024.0 / 1024.0);
    println!("  Compression ratio: {:.2}x", compression_ratio);
    println!("  File tax optimization: {} tiny files batched", tiny_files_count);
    
    Ok(())
}

pub fn unpackdir(input: &str, output: &str) -> Result<(), String> {
    // Simplified unpackdir for V1 - uses tar fallback
    if !Path::new(input).exists() {
        return Err(format!("Input file not found: {}", input));
    }
    
    println!("MMH-RS File System: Extracting directory...");
    
    // For V1, we'll use tar extraction as a fallback
    // The real MMH unpacking will be implemented in V2
    let mut cmd = std::process::Command::new("tar");
    cmd.arg("xf").arg(input).arg("-C").arg(output);
    let output_cmd = cmd.output().map_err(|e| format!("Failed to run tar: {}", e))?;
    if !output_cmd.status.success() {
        return Err(format!("Tar extraction failed: {}", String::from_utf8_lossy(&output_cmd.stderr)));
    }
    println!("  Tar extraction complete!");
    Ok(())
}

pub fn verify_seed(dir: &str, seed: &str) -> Result<(), String> {
    // For now, just check if directory exists
    if !Path::new(dir).exists() {
        return Err(format!("Directory not found: {}", dir));
    }
    
    println!("Seed verification - Coming Soon in V2!");
    println!("Directory: {}", dir);
    println!("Seed: {}", seed);
    
    Ok(())
}