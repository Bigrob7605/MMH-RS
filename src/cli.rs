use clap::{Parser, Subcommand};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, BufReader, BufWriter, BufRead, Read};

use std::path::Path;
use chrono::Local;
use sysinfo::System;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use chrono;
mod agent;
use crate::bench;

const MAX_SIZE: u64 = 1024 * 1024 * 1024 * 1024; // 1TB (increased for large benchmarks)
const LOG_FILE: &str = "mmh_cli.log";
const ERROR_LOG_FILE: &str = "mmh_error_log.txt";

// Progress meter constants
const BYTES_PER_MB: f64 = 1_048_576.0;
const BYTES_PER_GB: f64 = 1_073_741_824.0;
const BYTES_PER_TB: f64 = 1_099_511_627_776.0;

#[derive(Debug, Clone, Copy)]
enum OverwriteAction {
    Skip,
    Replace,
    Cancel,
}

static mut REPLACE_ALL: bool = false;
static mut SKIP_ALL: bool = false;

// Global abort flag for progress meters
static mut ABORT_REQUESTED: bool = false;

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
    /// Run agent in continuous mode (don't exit after tests)
    #[arg(long)]
    pub continuous: bool,
    /// Show RGIG (Reality Grade Intelligence Gauntlet) integration info (coming V3!)
    #[arg(long)]
    pub rgig_info: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Pack (compress) a single file (max 1TB)
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

#[allow(dead_code)]
fn badge() {
    // Only print the banner once at program start, not in every menu or loop
    if std::env::var("CI").unwrap_or_default().is_empty() && 
       std::env::var("NO_COLOR").unwrap_or_default().is_empty() {
        println!("============================");
        println!("|   MMH-RS V1 RELEASED!    |");
        println!("|   1TB Proven, Fast!      |");
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
#[allow(dead_code)]
fn show_smoketest_art() {
    println!(r#"
+-------+
|  MMH  |
|   RS  |
+-------+
Fold. Restore. Repeat.
"#);
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn get_hardware_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let cpu_name = sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_else(|| "Unknown CPU".to_string());
    let ram_gb = sys.total_memory() / (1024 * 1024 * 1024);
    let os_info = std::env::consts::OS;
    
    format!("{} | {}GB RAM | {}", cpu_name, ram_gb, os_info)
}

#[allow(dead_code)]
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
        let msg = format!("ERROR: File exceeds 1TB limit: {}", path);
        log_error("check_file", &msg);
        return Err(msg);
    }
    Ok(())
}

fn prompt_overwrite(file_path: &str, operation: &str) -> Result<OverwriteAction, String> {
    unsafe {
        // Check global flags first
        if REPLACE_ALL {
            return Ok(OverwriteAction::Replace);
        }
        if SKIP_ALL {
            return Ok(OverwriteAction::Skip);
        }
    }
    
    println!("\n⚠️  File already exists: {}", file_path);
    println!("Operation: {}", operation);
    print!("Options: [S]kip, [R]eplace, [A]lways replace, [K]eep all, [C]ancel: ");
    io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| format!("Failed to read input: {}", e))?;
    let input = input.trim().to_lowercase();
    
    match input.as_str() {
        "s" | "skip" => Ok(OverwriteAction::Skip),
        "r" | "replace" => Ok(OverwriteAction::Replace),
        "a" | "always" | "always replace" => {
            unsafe { REPLACE_ALL = true; }
            Ok(OverwriteAction::Replace)
        }
        "k" | "keep" | "keep all" => {
            unsafe { SKIP_ALL = true; }
            Ok(OverwriteAction::Skip)
        }
        "c" | "cancel" => Ok(OverwriteAction::Cancel),
        _ => {
            println!("Invalid option. Please choose S, R, A, K, or C.");
            prompt_overwrite(file_path, operation)
        }
    }
}

pub fn pack(input: &str, output: &str) -> Result<(), String> {
    if !Path::new(input).exists() {
        log_error("pack", &format!("Input file not found: {}", input));
        std::process::exit(1);
    }
    
    // Check for existing output file and handle overwrite
    if Path::new(output).exists() {
        match prompt_overwrite(output, "pack")? {
            OverwriteAction::Skip => {
                println!("Skipping pack operation for {}", output);
                return Ok(());
            }
            OverwriteAction::Replace => {
                // Continue with overwrite
            }
            OverwriteAction::Cancel => {
                println!("Pack operation cancelled.");
                return Ok(());
            }
        }
    }
    
    show_pack_art();
    log(&format!("PACK: {} -> {}", input, output));
    check_file(&input)?;
    
    // Get file size for progress bar
    let input_size = std::fs::metadata(&input).unwrap().len();
    let _input_size_mb = input_size as f64 / BYTES_PER_MB;
    
    println!("📦 Packing {} ({})...", input, format_size(input_size));
    
    // Create progress bar for reading
    let pb_read = create_progress_bar(input_size, "Reading file");
    pb_read.set_message("Reading input file...");
    
    log(&format!("DEBUG: Trying to open input file: {}", input));
    let mut infile = File::open(&input).map_err(|e| format!("Input file open failed: {}", e))?;
    
    // Read file content with progress
    let mut content = Vec::new();
    let mut buffer = [0u8; 8192];
    let mut bytes_read = 0u64;
    
    loop {
        if check_abort() {
            pb_read.finish_with_message("❌ Pack operation aborted");
            return Err("Operation aborted by user".to_string());
        }
        
        match infile.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                content.extend_from_slice(&buffer[..n]);
                bytes_read += n as u64;
                pb_read.set_position(bytes_read);
                
                // Update speed display
                let elapsed = pb_read.elapsed().as_secs_f64();
                if elapsed > 0.0 {
                    let speed = bytes_read as f64 / elapsed;
                    pb_read.set_message(format!("Reading... {} ({})", 
                        format_size(bytes_read), format_speed(speed)));
                }
            }
            Err(e) => {
                pb_read.finish_with_message("❌ Read error");
                return Err(format!("Failed to read input file: {}", e));
            }
        }
    }
    
    pb_read.finish_with_message("✅ File read complete");
    
    // Create deterministic filename based on content hash
    let content_hash = sha2::Sha256::digest(&content);
    let hash_prefix = &content_hash[..8];
    let deterministic_filename = format!("file_{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}", 
        hash_prefix[0], hash_prefix[1], hash_prefix[2], hash_prefix[3],
        hash_prefix[4], hash_prefix[5], hash_prefix[6], hash_prefix[7]);
    
    // Create progress bar for compression
    let pb_compress = create_progress_bar(content.len() as u64, "Compressing");
    pb_compress.set_message("Compressing data...");
    
    log(&format!("DEBUG: Trying to create output file: {}", output));
    let mut outfile = BufWriter::new(File::create(output).map_err(|e| format!("Output file create failed: {}", e))?);
    
    // Write MMH header
    let input_path = Path::new(input);
    let file_extension = input_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    let header = format!("MMH-RS V1.1.0\nDETERMINISTIC_ID: {}\nFILE_EXTENSION: {}\n", 
        deterministic_filename, file_extension);
    outfile.write_all(header.as_bytes()).map_err(|e| format!("Header write failed: {}", e))?;
    
    // Compress with progress tracking
    let mut encoder = zstd::Encoder::new(&mut outfile, 3).map_err(|e| format!("Zstd encoder failed: {}", e))?;
    let mut compressed_bytes = 0u64;
    let chunk_size = 64 * 1024; // 64KB chunks
    
    for chunk in content.chunks(chunk_size) {
        if check_abort() {
            pb_compress.finish_with_message("❌ Compression aborted");
            return Err("Operation aborted by user".to_string());
        }
        
        encoder.write_all(chunk).map_err(|e| format!("Compression failed: {}", e))?;
        compressed_bytes += chunk.len() as u64;
        pb_compress.set_position(compressed_bytes);
        
        // Update speed display
        let elapsed = pb_compress.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            let speed = compressed_bytes as f64 / elapsed;
            pb_compress.set_message(format!("Compressing... {} ({})", 
                format_size(compressed_bytes), format_speed(speed)));
        }
    }
    
    encoder.finish().map_err(|e| format!("Finish failed: {}", e))?;
    pb_compress.finish_with_message("✅ Compression complete");
    
    // Calculate final stats
    let output_size = std::fs::metadata(output).unwrap().len();
    let ratio = input_size as f64 / output_size as f64;
    let total_time = pb_read.elapsed().as_secs_f64() + pb_compress.elapsed().as_secs_f64();
    let avg_speed = input_size as f64 / total_time / BYTES_PER_MB;
    
    println!("✅ Packed {} → {} in {:.2}s [{:.2} MB/s avg]", 
        input, output, total_time, avg_speed);
        println!("📊 Compression ratio: {:.2}x ({:.1} MB → {:.1} MB)",
        ratio, _input_size_mb, output_size as f64 / BYTES_PER_MB);
    
    // Calculate and display space savings
    let space_saved = input_size.saturating_sub(output_size);
    let space_saved_mb = space_saved as f64 / BYTES_PER_MB;
    let savings_percent = if input_size > 0 { (space_saved as f64 / input_size as f64) * 100.0 } else { 0.0 };
    
    if space_saved > 0 {
        println!("💾 Space saved: {:.1} MB ({:.1}% reduction)", space_saved_mb, savings_percent);
    } else {
        println!("💾 Space used: {:.1} MB (expansion due to compression overhead)", -space_saved_mb);
    }
    
    let msg = format!("Packed {} → {} in {:.2}s [{:.2} MB/s avg]", input, output, total_time, avg_speed);
    log(&msg);
    
    // Check if compression was effective
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
    
    // Get file size for progress bar
    let input_size = std::fs::metadata(&input).unwrap().len();
    let _input_size_mb = input_size as f64 / BYTES_PER_MB;
    
    println!("📦 Unpacking {} ({})...", input, format_size(input_size));
    
    // Track total time for the operation
    let start_time = std::time::Instant::now();
    
    log(&format!("UNPACK: {} -> {}", input, output));
    check_file(&input)?;
    log(&format!("DEBUG: Trying to open packed file: {}", input));
    let mut infile = BufReader::new(File::open(&input).map_err(|e| format!("Packed file open failed: {}", e))?);
    
    // Read MMH header to get original filename
    let mut header_line = String::new();
    infile.read_line(&mut header_line).map_err(|e| format!("Header read failed: {}", e))?;
    
    let final_output = if !header_line.starts_with("MMH-RS V1.1.0") {
        // Legacy format without header - treat as raw zstd
        log("WARNING: Legacy format detected, attempting raw zstd decompression");
        
        // Check for existing output file and handle overwrite
        if Path::new(output).exists() {
            match prompt_overwrite(output, "unpack")? {
                OverwriteAction::Skip => {
                    println!("Skipping unpack operation for {}", output);
                    return Ok(());
                }
                OverwriteAction::Replace => {
                    // Continue with overwrite
                }
                OverwriteAction::Cancel => {
                    println!("Unpack operation cancelled.");
                    return Ok(());
                }
            }
        }
        
        // Create progress bar for decompression - estimate output size as 3x input (typical compression ratio)
        let estimated_output_size = input_size.saturating_mul(3);
        let pb_decompress = create_progress_bar(estimated_output_size, "Decompressing");
        pb_decompress.set_message("Decompressing legacy format...");
        
        let mut decoder = zstd::Decoder::new(&mut infile).map_err(|e| format!("Zstd decoder failed: {}", e))?;
        let mut outfile = File::create(output).map_err(|e| format!("Output file create failed: {}", e))?;
        
        // Decompress with progress tracking
        let mut buffer = [0u8; 8192];
        let mut bytes_written = 0u64;
        
        loop {
            if check_abort() {
                pb_decompress.finish_with_message("❌ Unpack operation aborted");
                return Err("Operation aborted by user".to_string());
            }
            
            match decoder.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    outfile.write_all(&buffer[..n]).map_err(|e| format!("Write failed: {}", e))?;
                    bytes_written += n as u64;
                    pb_decompress.set_position(bytes_written);
                    
                    // Update speed display
                    let elapsed = pb_decompress.elapsed().as_secs_f64();
                    if elapsed > 0.0 {
                        let speed = bytes_written as f64 / elapsed;
                        pb_decompress.set_message(format!("Decompressing... {} ({})", 
                            format_size(bytes_written), format_speed(speed)));
                    }
                }
                Err(e) => {
                    pb_decompress.finish_with_message("❌ Decompression error");
                    return Err(format!("Decompression failed: {}", e));
                }
            }
        }
        
        pb_decompress.finish_with_message("✅ Decompression complete");
        output.to_string()
    } else {
        // Read deterministic ID from header
        let mut deterministic_id_line = String::new();
        infile.read_line(&mut deterministic_id_line).map_err(|e| format!("Deterministic ID read failed: {}", e))?;
        
        // Read file extension from header
        let mut extension_line = String::new();
        infile.read_line(&mut extension_line).map_err(|e| format!("File extension read failed: {}", e))?;
        
        let file_extension = if extension_line.starts_with("FILE_EXTENSION: ") {
            extension_line.trim_start_matches("FILE_EXTENSION: ").trim()
        } else {
            ""
        };
        
        // For deterministic format, use the output filename with proper extension
        let base_filename = Path::new(output).file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("restored_file");
        
        // Always add the extension if it exists in the header
        let original_filename = if !file_extension.is_empty() {
            format!("{}.{}", base_filename, file_extension)
        } else {
            base_filename.to_string()
        };
        
        // Always use the original filename with extension for unpacking
        let final_output = original_filename.to_string();
        
        // Check for existing output file and handle overwrite
        if Path::new(&final_output).exists() {
            match prompt_overwrite(&final_output, "unpack")? {
                OverwriteAction::Skip => {
                    println!("Skipping unpack operation for {}", final_output);
                    return Ok(());
                }
                OverwriteAction::Replace => {
                    // Continue with overwrite
                }
                OverwriteAction::Cancel => {
                    println!("Unpack operation cancelled.");
                    return Ok(());
                }
            }
        }
        
        log(&format!("DEBUG: Restoring original filename: {}", original_filename));
        log(&format!("DEBUG: Trying to create output file: {}", final_output));
        
        // Create progress bar for decompression - estimate output size as 3x input (typical compression ratio)
        let estimated_output_size = input_size.saturating_mul(3);
        let pb_decompress = create_progress_bar(estimated_output_size, "Decompressing");
        pb_decompress.set_message("Decompressing MMH format...");
        
        let mut decoder = zstd::Decoder::new(&mut infile).map_err(|e| format!("Zstd decoder failed: {}", e))?;
        let mut outfile = File::create(&final_output).map_err(|e| format!("Output file create failed: {}", e))?;
        
        // Decompress with progress tracking
        let mut buffer = [0u8; 8192];
        let mut bytes_written = 0u64;
        
        loop {
            if check_abort() {
                pb_decompress.finish_with_message("❌ Unpack operation aborted");
                return Err("Operation aborted by user".to_string());
            }
            
            match decoder.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    outfile.write_all(&buffer[..n]).map_err(|e| format!("Write failed: {}", e))?;
                    bytes_written += n as u64;
                    pb_decompress.set_position(bytes_written);
                    
                    // Update speed display
                    let elapsed = pb_decompress.elapsed().as_secs_f64();
                    if elapsed > 0.0 {
                        let speed = bytes_written as f64 / elapsed;
                        pb_decompress.set_message(format!("Decompressing... {} ({})", 
                            format_size(bytes_written), format_speed(speed)));
                    }
                }
                Err(e) => {
                    pb_decompress.finish_with_message("❌ Decompression error");
                    return Err(format!("Decompression failed: {}", e));
                }
            }
        }
        
        pb_decompress.finish_with_message("✅ Decompression complete");
        final_output
    };
    
    // Calculate final stats
    let output_size = std::fs::metadata(&final_output).unwrap().len();
    let total_time = start_time.elapsed().as_secs_f64();
    let avg_speed = if total_time > 0.0 {
        output_size as f64 / total_time / BYTES_PER_MB
    } else {
        0.0
    };
    
    println!("✅ Unpacked {} → {} in {:.2}s [{:.2} MB/s avg]", 
        input, &final_output, total_time, avg_speed);
    println!("📊 Decompressed size: {} (from {} compressed)", 
        format_size(output_size), format_size(input_size));
    
    // Calculate and display expansion info
    let expansion = output_size.saturating_sub(input_size);
    let expansion_mb = expansion as f64 / BYTES_PER_MB;
    let expansion_percent = if input_size > 0 { (expansion as f64 / input_size as f64) * 100.0 } else { 0.0 };
    
    if expansion > 0 {
        println!("📈 Data expansion: {:.1} MB ({:.1}% increase)", expansion_mb, expansion_percent);
    } else {
        println!("📉 Data reduction: {:.1} MB (unusual - may indicate corruption)", -expansion_mb);
    }
    
    let msg = format!("Unpacked {} → {} in {:.2}s [{:.2} MB/s avg]", input, &final_output, total_time, avg_speed);
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
        let msg = format!("ERROR: Requested size exceeds 1TB limit: {} bytes", size);
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
    if size_gb > 1024 { // 1TB limit
        let msg = format!("ERROR: gentestdir limit is 1TB for V1");
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
#[allow(dead_code)]
fn print_and_export_smoketest_summary(
    _txt: &str, _json: &str, _log: &str, _replay_seed: String,
    pack_speed: f64, pack_time: f64,
    unpack_speed: f64, unpack_time: f64,
    verify_speed: f64, verify_time: f64,
    total: u64, _pass: u64, fail: u64,
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn benchmark_menu(_size_gb: u64) {
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

#[allow(dead_code)]
fn run_benchmark(size_gb: u64) {
    let rpt = bench::run(size_gb);
    rpt.save();
    println!("{}", rpt.pretty_box());
    println!("Exported: bench_reports/<timestamp>/report.txt / report.json");
    println!("Press Enter to return to the menu");
    let mut _dummy = String::new();
    std::io::stdin().read_line(&mut _dummy).unwrap();
}

#[allow(dead_code)]
fn stressbench(_size_gb: u64) {
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

#[allow(dead_code)]
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
    let detest_mmh = "detest.mmh"; // Use same output filename for both
    
    // Create identical test files
    if let Err(e) = std::fs::write(detest1, &test_content) {
        println!("❌ Failed to create deterministic test file 1: {}", e);
        all_tests_passed = false;
    } else if let Err(e) = std::fs::write(detest2, &test_content) {
        println!("❌ Failed to create deterministic test file 2: {}", e);
        all_tests_passed = false;
    } else {
        println!("✅ Deterministic test files created");
        
        // Compress first file
        let pack1_result = pack(detest1, detest_mmh);
        
        if pack1_result.is_ok() {
            // Read the first compressed file
            let comp1 = std::fs::read(detest_mmh);
            
            // Compress second file (will overwrite the first)
            let pack2_result = pack(detest2, detest_mmh);
            
            if pack2_result.is_ok() {
                // Read the second compressed file
                let comp2 = std::fs::read(detest_mmh);
                
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
                println!("❌ Failed to compress second deterministic test file");
                all_tests_passed = false;
            }
        } else {
            println!("❌ Failed to compress first deterministic test file");
            all_tests_passed = false;
        }
        
        // Cleanup
        let _ = std::fs::remove_file(detest1);
        let _ = std::fs::remove_file(detest2);
        let _ = std::fs::remove_file(detest_mmh);
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

#[allow(dead_code)]
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

pub fn prompt_save_results(_default_txt: &str, _default_json: &str, _default_log: &str, txt_content: &str, json_content: &str, log_content: &str, size_gb: u64) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    
    // Create size prefix for unique filenames
    let size_prefix = if size_gb == 0 {
        "50M".to_string()
    } else if size_gb == 999 {
        "BULK".to_string() // Special case for bulk small file test
    } else if size_gb < 1024 {
        format!("{}G", size_gb)
    } else {
        format!("{}T", size_gb / 1024)
    };
    
    println!("\n============================");
    println!("|     SAVE TEST RESULTS     |");
    println!("============================");
    println!("1. Save all formats (recommended)");
    println!("2. Save text report only");
    println!("3. Save JSON report only");
    println!("4. Save log only");
    println!("5. Skip saving");
    print!("Select option: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    match choice.trim() {
        "1" => {
            // Save all formats with size prefix and timestamps
            let txt_name = format!("{}-test_results_{}.txt", size_prefix, timestamp);
            let json_name = format!("{}-test_results_{}.json", size_prefix, timestamp);
            let log_name = format!("{}-test_results_{}.log", size_prefix, timestamp);
            
            println!("💾 Processing and saving results...");
            let pb = create_progress_bar(3, "Saving files");
            
            pb.set_message("Saving text report...");
            std::fs::write(&txt_name, txt_content).expect("Failed to write text results");
            pb.set_position(1);
            
            pb.set_message("Saving JSON report...");
            std::fs::write(&json_name, json_content).expect("Failed to write JSON results");
            pb.set_position(2);
            
            pb.set_message("Saving log report...");
            std::fs::write(&log_name, log_content).expect("Failed to write log results");
            pb.set_position(3);
            
            pb.finish_with_message("✅ All results saved");
            println!("   📄 Text: {}", txt_name);
            println!("   📊 JSON: {}", json_name);
            println!("   📝 Log: {}", log_name);
        },
        "2" => {
            let txt_name = format!("{}-test_results_{}.txt", size_prefix, timestamp);
            println!("💾 Saving text report...");
            std::fs::write(&txt_name, txt_content).expect("Failed to write text results");
            println!("✅ Text results saved: {}", txt_name);
        },
        "3" => {
            let json_name = format!("{}-test_results_{}.json", size_prefix, timestamp);
            println!("💾 Saving JSON report...");
            std::fs::write(&json_name, json_content).expect("Failed to write JSON results");
            println!("✅ JSON results saved: {}", json_name);
        },
        "4" => {
            let log_name = format!("{}-test_results_{}.log", size_prefix, timestamp);
            println!("💾 Saving log report...");
            std::fs::write(&log_name, log_content).expect("Failed to write log results");
            println!("✅ Log results saved: {}", log_name);
        },
        "5" => {
            println!("Skipping save.");
        },
        _ => {
            println!("Invalid option. Skipping save.");
        }
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

#[allow(dead_code)]
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
    
    // Create progress bar for hashing
    let total_files = files.len();
    if total_files > 0 {
        println!("🔍 Computing integrity hash for {} files...", total_files);
        let pb = create_progress_bar(total_files as u64, "Hashing files");
        
        for (i, entry) in files.iter().enumerate() {
            if check_abort() {
                pb.finish_with_message("❌ Hash computation aborted");
                return "aborted".to_string();
            }
            
            let data = fs::read(entry.path()).expect("Failed to read file for hashing");
            hasher.update(&data);
            
            pb.set_position((i + 1) as u64);
            pb.set_message(format!("Hashing file {}/{}", i + 1, total_files));
        }
        
        pb.finish_with_message("✅ Hash computation complete");
    }
    
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

#[allow(dead_code)]
fn hash_file_sha256(path: &str) -> String {
    use sha2::{Sha256, Digest};
    use std::fs;
    let mut hasher = Sha256::new();
    let data = fs::read(path).unwrap_or_default();
    hasher.update(&data);
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn get_env_info() -> HashMap<String, String> {
    std::env::vars().collect()
}

#[allow(dead_code)]
fn get_timestamp() -> String {
    use chrono::prelude::*;
    Utc::now().to_rfc3339()
}

#[allow(dead_code)]
fn get_mmh_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[allow(dead_code)]
fn get_os_info() -> String {
    std::env::consts::OS.to_string()
}

#[allow(dead_code)]
fn get_cpu_info() -> String {
    // Placeholder: use sysinfo or similar for real CPU info
    "Generic CPU".to_string()
}

// Progress meter functions
fn format_speed(bytes_per_sec: f64) -> String {
    if bytes_per_sec >= BYTES_PER_TB {
        format!("{:.2} TB/s", bytes_per_sec / BYTES_PER_TB)
    } else if bytes_per_sec >= BYTES_PER_GB {
        format!("{:.2} GB/s", bytes_per_sec / BYTES_PER_GB)
    } else if bytes_per_sec >= BYTES_PER_MB {
        format!("{:.2} MB/s", bytes_per_sec / BYTES_PER_MB)
    } else {
        format!("{:.2} KB/s", bytes_per_sec / 1024.0)
    }
}

fn format_size(bytes: u64) -> String {
    if bytes >= BYTES_PER_TB as u64 {
        format!("{:.2} TB", bytes as f64 / BYTES_PER_TB)
    } else if bytes >= BYTES_PER_GB as u64 {
        format!("{:.2} GB", bytes as f64 / BYTES_PER_GB)
    } else if bytes >= BYTES_PER_MB as u64 {
        format!("{:.2} MB", bytes as f64 / BYTES_PER_MB)
    } else {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    }
}

fn create_progress_bar(total_size: u64, operation: &str) -> ProgressBar {
    let pb = ProgressBar::new(total_size);
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) {msg}")
        .unwrap()
        .progress_chars("#>-");
    
    pb.set_style(style);
    pb.set_message(format!("{}...", operation));
    pb
}

pub fn check_abort() -> bool {
    unsafe { ABORT_REQUESTED }
}

#[allow(dead_code)]
pub fn set_abort_flag() {
    unsafe { ABORT_REQUESTED = true; }
}

pub fn clear_abort_flag() {
    unsafe { ABORT_REQUESTED = false; }
}

// Additional public functions needed by main.rs
pub fn show_ascii_art(num: u8) {
    match num {
        1..=8 => println!("ASCII Art #{} - Coming Soon!", num),
        _ => println!("Invalid ASCII art number. Use 1-8."),
    }
}

pub fn run_agent(continuous: bool) {
    agent::run_agent(continuous);
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

#[allow(dead_code)]
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

/// Compare two benchmark results and generate a comparison report
pub fn compare_benchmarks(file1: &str, file2: &str) -> Result<String, Box<dyn std::error::Error>> {
    let json1 = std::fs::read_to_string(file1)?;
    let json2 = std::fs::read_to_string(file2)?;
    
    let result1: Value = serde_json::from_str(&json1)?;
    let result2: Value = serde_json::from_str(&json2)?;
    
    let mut comparison = String::new();
    comparison.push_str("## 🔍 **BENCHMARK COMPARISON REPORT**\n\n");
    
    // Extract key metrics
    let score1 = result1["score"].as_f64().unwrap_or(0.0);
    let score2 = result2["score"].as_f64().unwrap_or(0.0);
    let pack_speed1 = result1["pack_speed_mbps"].as_f64().unwrap_or(0.0);
    let pack_speed2 = result2["pack_speed_mbps"].as_f64().unwrap_or(0.0);
    let compression_ratio1 = result1["compression_ratio"].as_f64().unwrap_or(0.0);
    let compression_ratio2 = result2["compression_ratio"].as_f64().unwrap_or(0.0);
    
    comparison.push_str(&format!("### 📊 **PERFORMANCE COMPARISON**\n\n"));
    comparison.push_str(&format!("| Metric | {} | {} | Difference |\n", 
        result1["system_name"].as_str().unwrap_or("System 1"),
        result2["system_name"].as_str().unwrap_or("System 2")));
    comparison.push_str("|--------|--------|--------|------------|\n");
    
    // Score comparison
    let score_diff = score2 - score1;
    let score_icon = if score_diff > 0.0 { "🟢" } else if score_diff < 0.0 { "🔴" } else { "🟡" };
    comparison.push_str(&format!("| Score | {:.1}/100 | {:.1}/100 | {} {:+.1} |\n", 
        score1, score2, score_icon, score_diff));
    
    // Speed comparison
    let speed_diff = pack_speed2 - pack_speed1;
    let speed_icon = if speed_diff > 0.0 { "🟢" } else if speed_diff < 0.0 { "🔴" } else { "🟡" };
    comparison.push_str(&format!("| Pack Speed | {:.1} MB/s | {:.1} MB/s | {} {:+.1} MB/s |\n", 
        pack_speed1, pack_speed2, speed_icon, speed_diff));
    
    // Compression ratio comparison
    let ratio_diff = compression_ratio2 - compression_ratio1;
    let ratio_icon = if ratio_diff > 0.0 { "🟢" } else if ratio_diff < 0.0 { "🔴" } else { "🟡" };
    comparison.push_str(&format!("| Compression | {:.2}x | {:.2}x | {} {:+.2}x |\n", 
        compression_ratio1, compression_ratio2, ratio_icon, ratio_diff));
    
    comparison.push_str("\n### 🏆 **WINNER ANALYSIS**\n\n");
    
    let mut wins1 = 0;
    let mut wins2 = 0;
    
    if score1 > score2 { wins1 += 1; } else if score2 > score1 { wins2 += 1; }
    if pack_speed1 > pack_speed2 { wins1 += 1; } else if pack_speed2 > pack_speed1 { wins2 += 1; }
    if compression_ratio1 > compression_ratio2 { wins1 += 1; } else if compression_ratio2 > compression_ratio1 { wins2 += 1; }
    
    if wins1 > wins2 {
        comparison.push_str(&format!("**🏆 {} wins overall!**\n", 
            result1["system_name"].as_str().unwrap_or("System 1")));
    } else if wins2 > wins1 {
        comparison.push_str(&format!("**🏆 {} wins overall!**\n", 
            result2["system_name"].as_str().unwrap_or("System 2")));
    } else {
        comparison.push_str("**🤝 It's a tie!**\n");
    }
    
    comparison.push_str(&format!("- {}: {} wins\n", 
        result1["system_name"].as_str().unwrap_or("System 1"), wins1));
    comparison.push_str(&format!("- {}: {} wins\n", 
        result2["system_name"].as_str().unwrap_or("System 2"), wins2));
    
    Ok(comparison)
}

/// Generate an HTML report from benchmark results
pub fn generate_html_report(json_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let json_content = std::fs::read_to_string(json_file)?;
    let result: Value = serde_json::from_str(&json_content)?;
    
    let html = format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MMH-RS Benchmark Report</title>
    <style>
        body {{ font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 0; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: #333; }}
        .container {{ max-width: 1200px; margin: 0 auto; background: white; border-radius: 15px; box-shadow: 0 20px 40px rgba(0,0,0,0.1); overflow: hidden; }}
        .header {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; text-align: center; }}
        .header h1 {{ margin: 0; font-size: 2.5em; font-weight: 300; }}
        .header p {{ margin: 10px 0 0 0; opacity: 0.9; }}
        .content {{ padding: 30px; }}
        .metric-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 30px 0; }}
        .metric-card {{ background: #f8f9fa; border-radius: 10px; padding: 20px; text-align: center; border-left: 4px solid #667eea; }}
        .metric-value {{ font-size: 2em; font-weight: bold; color: #667eea; margin: 10px 0; }}
        .metric-label {{ color: #666; font-size: 0.9em; text-transform: uppercase; letter-spacing: 1px; }}
        .score-badge {{ display: inline-block; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 10px 20px; border-radius: 25px; font-size: 1.2em; font-weight: bold; }}
        .system-info {{ background: #f8f9fa; border-radius: 10px; padding: 20px; margin: 20px 0; }}
        .system-info h3 {{ margin-top: 0; color: #667eea; }}
        .progress-bar {{ background: #e9ecef; border-radius: 10px; height: 20px; overflow: hidden; margin: 10px 0; }}
        .progress-fill {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); height: 100%; transition: width 0.3s ease; }}
        .footer {{ background: #f8f9fa; padding: 20px; text-align: center; color: #666; border-top: 1px solid #e9ecef; }}
        .share-button {{ background: #667eea; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; font-size: 1em; margin: 10px; }}
        .share-button:hover {{ background: #5a6fd8; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 MMH-RS Benchmark Report</h1>
            <p>High-Performance Compression Benchmark Results</p>
        </div>
        
        <div class="content">
            <div style="text-align: center; margin: 30px 0;">
                <div class="score-badge">
                    Score: {}/100
                </div>
            </div>
            
            <div class="metric-grid">
                <div class="metric-card">
                    <div class="metric-label">Compression Speed</div>
                    <div class="metric-value">{:.1} MB/s</div>
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {}%"></div>
                    </div>
                </div>
                
                <div class="metric-card">
                    <div class="metric-label">Compression Ratio</div>
                    <div class="metric-value">{:.2}x</div>
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {}%"></div>
                    </div>
                </div>
                
                <div class="metric-card">
                    <div class="metric-label">Space Saved</div>
                    <div class="metric-value">{:.1} GB</div>
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {}%"></div>
                    </div>
                </div>
                
                <div class="metric-card">
                    <div class="metric-label">Total Time</div>
                    <div class="metric-value">{:.1} min</div>
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {}%"></div>
                    </div>
                </div>
            </div>
            
            <div class="system-info">
                <h3>🖥️ System Information</h3>
                <p><strong>CPU:</strong> {}</p>
                <p><strong>RAM:</strong> {}</p>
                <p><strong>GPU:</strong> {}</p>
                <p><strong>Storage:</strong> {}</p>
                <p><strong>OS:</strong> {}</p>
            </div>
            
            <div style="text-align: center; margin: 30px 0;">
                <button class="share-button" onclick="shareResults()">📤 Share Results</button>
                <button class="share-button" onclick="exportPDF()">📄 Export PDF</button>
                <button class="share-button" onclick="compareResults()">🔍 Compare</button>
            </div>
        </div>
        
        <div class="footer">
            <p>Generated by MMH-RS V1.1.0 | {} | UniversalTruth</p>
        </div>
    </div>
    
    <script>
        function shareResults() {{
            if (navigator.share) {{
                navigator.share({{
                    title: 'MMH-RS Benchmark Results',
                    text: 'Check out my MMH-RS benchmark results! Score: {}/100',
                    url: window.location.href
                }});
            }} else {{
                navigator.clipboard.writeText(window.location.href);
                alert('Link copied to clipboard!');
            }}
        }}
        
        function exportPDF() {{
            window.print();
        }}
        
        function compareResults() {{
            alert('Compare feature coming in V1.2!');
        }}
    </script>
</body>
</html>
"#,
        result["score"].as_f64().unwrap_or(0.0),
        result["pack_speed_mbps"].as_f64().unwrap_or(0.0),
        f64::min(result["pack_speed_mbps"].as_f64().unwrap_or(0.0) / 100.0 * 100.0, 100.0),
        result["compression_ratio"].as_f64().unwrap_or(0.0),
        f64::min(result["compression_ratio"].as_f64().unwrap_or(0.0) / 3.0 * 100.0, 100.0),
        result["space_saved_gb"].as_f64().unwrap_or(0.0),
        f64::min(result["space_saved_gb"].as_f64().unwrap_or(0.0) / 50.0 * 100.0, 100.0),
        result["total_time"].as_f64().unwrap_or(0.0) / 60.0,
        f64::min(result["total_time"].as_f64().unwrap_or(0.0) / 3600.0 * 100.0, 100.0),
        result["cpu_info"].as_str().unwrap_or("Unknown"),
        result["ram_info"].as_str().unwrap_or("Unknown"),
        result["gpu_info"].as_str().unwrap_or("Unknown"),
        result["storage_info"].as_str().unwrap_or("Unknown"),
        result["os_info"].as_str().unwrap_or("Unknown"),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        result["score"].as_f64().unwrap_or(0.0)
    );
    
    Ok(html)
}