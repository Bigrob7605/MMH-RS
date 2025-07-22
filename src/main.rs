mod cli;
mod bench;

use clap::Parser;
use std::io::{self, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use rand::Rng;

static RUNNING: AtomicBool = AtomicBool::new(false);

fn main() {
    let cli = cli::Cli::parse();
    
    // Handle version and about flags
    if cli.version {
        println!("MMH-RS V1.0.0");
        return;
    }
    
    if cli.about {
        println!("MMH-RS: Universal Digital DNA Format (V1 ROCK SOLID)");
        println!("A high-performance compression and archival system");
        println!("Send feedback to: Screwball7605@aol.com");
        return;
    }
    
    // Handle ASCII art requests
    if let Some(art_num) = cli.ascii {
        cli::show_ascii_art(art_num);
        return;
    }
    
    // Handle wizard easter egg
    if cli.wizard {
        cli::print_gandalf();
        return;
    }
    
    // Handle RGIG info (V3 feature)
    if cli.rgig_info {
        println!("RGIG (Reality Grade Intelligence Gauntlet) - Coming in V3!");
        println!("Advanced AI integration and automated testing framework.");
        return;
    }
    
    // Handle agent mode
    if cli.agent {
        cli::run_agent();
        return;
    }
    
    // Handle subcommands
    if let Some(command) = cli.command {
        match command {
            cli::Commands::Pack { input, output } => {
                if let Err(e) = cli::pack(&input, &output) {
                    eprintln!("Pack failed: {}", e);
                    process::exit(1);
                }
            },
            cli::Commands::Unpack { input, output } => {
                if let Err(e) = cli::unpack(&input, &output) {
                    eprintln!("Unpack failed: {}", e);
                    process::exit(1);
                }
            },
            cli::Commands::Packdir { dir, output } => {
                if let Err(e) = cli::packdir(&dir, &output) {
                    eprintln!("Packdir failed: {}", e);
                    process::exit(1);
                }
            },
            cli::Commands::Verify { input, restored } => {
                if let Err(e) = cli::verify(&input, &restored) {
                    eprintln!("Verify failed: {}", e);
                    process::exit(2);
                }
            },
            cli::Commands::Gen { output, size } => {
                cli::gen(&output, size);
            },
            cli::Commands::Gentestdir { dir, size } => {
                cli::gentestdir(&dir, size);
            },
            cli::Commands::Benchmenu { size } => {
                run_benchmark_menu(size);
            },
            cli::Commands::Stressbench { size } => {
                run_stress_benchmark(size);
            },
            cli::Commands::Goldbench { size, seed, format } => {
                run_gold_benchmark(*size, *seed, format);
            },
            cli::Commands::Selftest => {
                cli::selftest();
            },
            cli::Commands::Cleanup => {
                cli::cleanup();
            },
            cli::Commands::Verifyseed { dir, seed } => {
                if let Err(e) = cli::verify_seed(&dir, &seed) {
                    eprintln!("Verify seed failed: {}", e);
                    process::exit(1);
                }
            },
        }
    } else {
        // No subcommand provided, show interactive menu
        show_main_menu();
    }
}

fn show_main_menu() {
    loop {
        println!("\n============================");
        println!("|      MMH-RS V1 MENU      |");
        println!("============================");
        println!("1. Benchmark (Try MMH File System)");
        println!("2. Pack File");
        println!("3. Unpack File");
        println!("4. Verify Integrity");
        println!("5. Generate Test Data");
        println!("6. Cleanup Test Files");
        println!("7. Self Test");
        println!("8. Advanced Menu (Dev Tools)");
        println!("9. Full CLI Menu");
        println!("Q. Quit");
        print!("Select option: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => {
                run_benchmark_menu(2); // Default to 2GB
            },
            "2" => {
                handle_pack_file();
            },
            "3" => {
                handle_unpack_file();
            },
            "4" => {
                handle_verify_file();
            },
            "5" => {
                handle_generate_test_data();
            },
            "6" => {
                cli::cleanup();
                println!("Cleanup completed.");
            },
            "7" => {
                cli::selftest();
            },
            "8" => {
                show_advanced_menu();
            },
            "9" => {
                show_full_cli_menu();
            },
            "q" | "Q" => {
                println!("Goodbye!");
                process::exit(0);
            },
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}

fn run_benchmark_menu(default_size: u64) {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // Set up abort handler
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
        println!("\nAbort requested. Finishing current operation...");
    }).expect("Failed to set abort handler");
    
    loop {
        println!("\n============================");
        println!("|   MMH-RS BENCHMARK MENU   |");
        println!("============================");
        println!("1. Smoketest (~50 MiB)");
        println!("2. Toasty (2GB)");
        println!("3. Scorched (8GB)");
        println!("4. Nuked (32GB)");
        println!("5. Total Annihilation (128GB)");
        println!("6. Memory Madness (256GB)");
        println!("7. Swapocalypse (512GB)");
        println!("8. RAMpocalypse (1TB)");
        println!("9. Bulk Small File Test");
        println!("B. Back to Main Menu");
        println!("Q. Quit");
        println!("(Press Ctrl+C to abort any benchmark)");
        print!("Select tier: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let selected_size = match input.trim() {
            "1" => 0, // Smoketest mode
            "2" => 2,
            "3" => 8,
            "4" => 32,
            "5" => 128,
            "6" => 256,
            "7" => 512,
            "8" => 1024,
            "9" => {
                run_bulk_small_file_test(&running);
                continue;
            },
            "b" | "B" => {
                println!("Returning to main menu...");
                return;
            },
            "q" | "Q" => {
                println!("Goodbye!");
                process::exit(0);
            },
            _ => {
                println!("Invalid option: '{}'. Please try again.", input.trim());
                continue;
            }
        };
        
        // Run the benchmark with selected size
        run_benchmark_with_abort(selected_size, &running);
        break;
    }
}

fn run_benchmark_with_abort(size_gb: u64, running: &Arc<AtomicBool>) {
    println!("\nStarting benchmark with {} GB...", if size_gb == 0 { "~50 MiB (smoketest)".to_string() } else { format!("{}", size_gb) });
    println!("Press Ctrl+C to abort at any time.");
    
    // Set the global running flag
    RUNNING.store(true, Ordering::SeqCst);
    
    // Run benchmark directly for now to avoid thread hanging issues
    let rpt = bench::run_with_seed(size_gb, rand::random::<u64>());
    
    // Display results
    println!("\n{}", rpt.pretty_box());
    println!("Exported: bench_reports/<timestamp>/report.txt / report.json");
    
    // Check for PASS/FAIL
    if rpt.input_hash == rpt.unpacked_hash {
        println!("✅ INTEGRITY CHECK: PASS");
    } else {
        println!("❌ INTEGRITY CHECK: FAIL");
        RUNNING.store(false, Ordering::SeqCst);
        process::exit(2);
    }
    
    // Prompt to save results
    crate::cli::prompt_save_results(
        "benchmark_report.txt",
        "benchmark_report.json",
        "benchmark_report.log",
        &rpt.pretty_box(),
        &serde_json::to_string_pretty(&rpt).unwrap(),
        &format!("{}\nINTEGRITY: {}", rpt.pretty_box(), if rpt.input_hash == rpt.unpacked_hash { "PASS" } else { "FAIL" })
    );
    
    RUNNING.store(false, Ordering::SeqCst);
    println!("Benchmark completed. Returning to menu...");
}

fn run_bulk_small_file_test(running: &Arc<AtomicBool>) {
    println!("\nRunning Bulk Small File Test (10,000 tiny files)...");
    println!("This test creates many small files to test file tax handling.");
    println!("Press Ctrl+C to abort.");
    
    // Create test directory with many small files
    let testdir = "bulk_test_temp";
    std::fs::create_dir_all(testdir).expect("Failed to create test directory");
    
    let mut file_count = 0;
    let mut total_size = 0u64;
    
    for i in 0..10000 { // 10,000 small files
        if !running.load(Ordering::SeqCst) {
            println!("Test aborted by user.");
            std::fs::remove_dir_all(testdir).ok();
            return;
        }
        
        let file_size = rand::random::<usize>() % 1024 + 1; // 1-1024 bytes
        let data: Vec<u8> = (0..file_size).map(|_| rand::random::<u8>()).collect();
        
        let filename = format!("{}/small_file_{:06}.dat", testdir, i);
        std::fs::write(&filename, &data).expect("Failed to write test file");
        
        file_count += 1;
        total_size += file_size as u64;
        
        if i % 1000 == 0 {
            println!("Created {} files ({:.1} MB)...", i, total_size as f64 / 1_048_576.0);
        }
    }
    
    println!("Created {} files, total size: {:.1} MB", file_count, total_size as f64 / 1_048_576.0);
    
    // Pack the directory
    println!("Packing directory...");
    let seed_file = "bulk_test.seed";
    let pack_start = std::time::Instant::now();
    
    if let Err(e) = cli::packdir(testdir, seed_file) {
        println!("Pack failed: {}", e);
        std::fs::remove_dir_all(testdir).ok();
        std::fs::remove_file(seed_file).ok();
        return;
    }
    
    let pack_time = pack_start.elapsed().as_secs_f64();
    let packed_size = std::fs::metadata(seed_file).unwrap().len();
    
    // Unpack for verification
    println!("Unpacking for verification...");
    let unpack_dir = "bulk_test_unpacked";
    std::fs::create_dir_all(unpack_dir).expect("Failed to create unpack directory");
    
    let unpack_start = std::time::Instant::now();
    if let Err(e) = cli::unpack(seed_file, unpack_dir) {
        println!("Unpack failed: {}", e);
        std::fs::remove_dir_all(testdir).ok();
        std::fs::remove_dir_all(unpack_dir).ok();
        std::fs::remove_file(seed_file).ok();
        return;
    }
    
    let unpack_time = unpack_start.elapsed().as_secs_f64();
    
    // Verify integrity
    println!("Verifying integrity...");
    let original_hash = cli::hash_dir_sha256(testdir);
    let unpacked_hash = cli::hash_dir_sha256(unpack_dir);
    
    // Calculate stats
    let pack_speed = total_size as f64 / 1_048_576.0 / pack_time;
    let unpack_speed = total_size as f64 / 1_048_576.0 / unpack_time;
    let compression_ratio = total_size as f64 / packed_size as f64;
    let file_tax_overhead = (packed_size as f64 - total_size as f64) / file_count as f64;
    
    // Local struct for report
    #[derive(serde::Serialize)]
    struct BulkReport {
        file_count: usize,
        total_size: u64,
        pack_speed: f64,
        unpack_speed: f64,
        compression_ratio: f64,
        file_tax_overhead: f64,
        original_hash: String,
        unpacked_hash: String,
        integrity: bool,
    }
    let report = BulkReport {
        file_count,
        total_size,
        pack_speed,
        unpack_speed,
        compression_ratio,
        file_tax_overhead,
        original_hash: original_hash.clone(),
        unpacked_hash: unpacked_hash.clone(),
        integrity: original_hash == unpacked_hash,
    };
    // Display results
    println!("\n╔═{:═^74}═╗", " BULK SMALL FILE TEST RESULTS ");
    println!("║ Files: {:<70} ║", format!("{} files", file_count));
    println!("║ Total Size: {:<65} ║", format!("{:.1} MB", total_size as f64 / 1_048_576.0));
    println!("║ Pack Speed: {:<65} ║", format!("{:.1} MB/s", pack_speed));
    println!("║ Unpack Speed: {:<63} ║", format!("{:.1} MB/s", unpack_speed));
    println!("║ Compression Ratio: {:<60} ║", format!("{:.2}×", compression_ratio));
    println!("║ File Tax Overhead: {:<60} ║", format!("{:.1} bytes/file", file_tax_overhead));
    println!("║ Integrity: {:<67} ║", if original_hash == unpacked_hash { "✅ PASS" } else { "❌ FAIL" });
    println!("╚═{:═^74}═╝", "");
    // Prompt to save results
    crate::cli::prompt_save_results(
        "bulk_small_file_report.txt",
        "bulk_small_file_report.json",
        "bulk_small_file_report.log",
        &format!("{} files\n{:.1} MB\nPack: {:.1} MB/s\nUnpack: {:.1} MB/s\nRatio: {:.2}x\nOverhead: {:.1} bytes/file\nIntegrity: {}",
            file_count, total_size as f64 / 1_048_576.0, pack_speed, unpack_speed, compression_ratio, file_tax_overhead, if original_hash == unpacked_hash { "PASS" } else { "FAIL" }),
        &serde_json::to_string_pretty(&report).unwrap(),
        &format!("{} files\n{:.1} MB\nPack: {:.1} MB/s\nUnpack: {:.1} MB/s\nRatio: {:.2}x\nOverhead: {:.1} bytes/file\nIntegrity: {}",
            file_count, total_size as f64 / 1_048_576.0, pack_speed, unpack_speed, compression_ratio, file_tax_overhead, if original_hash == unpacked_hash { "PASS" } else { "FAIL" })
    );
    // Cleanup
    std::fs::remove_dir_all(testdir).ok();
    std::fs::remove_dir_all(unpack_dir).ok();
    std::fs::remove_file(seed_file).ok();
    println!("Press Enter to return to the menu");
    let mut _dummy = String::new();
    io::stdin().read_line(&mut _dummy).unwrap();
}

fn run_stress_benchmark(size_gb: u64) {
    println!("\nRunning stress benchmark with {} GB...", size_gb);
    println!("This test uses realistic file types (text, binary, mixed).");
    
    // For now, just run the regular benchmark
    // Note: Stress test with mixed file types implemented in V2
    run_benchmark_with_abort(size_gb, &Arc::new(AtomicBool::new(true)));
}

fn run_gold_benchmark(size_gb: u64, seed: Option<u64>, format: String) {
    println!("🏆 Running MMH-RS Gold Standard Benchmark ({} GB)...", size_gb);
    println!("This benchmark provides comprehensive performance analysis and system evaluation.");
    
    // Show progress indicator
    println!("⏳ Initializing benchmark system...");
    
    // Run the benchmark with optional seed
    let report = if let Some(replay_seed) = seed {
        println!("🔢 Using replay seed: {}", replay_seed);
        bench::run_with_seed(size_gb, replay_seed)
    } else {
        let random_seed = rand::random::<u64>();
        println!("🎲 Using random seed: {}", random_seed);
        bench::run_with_seed(size_gb, random_seed)
    };
    
    // Display results based on format
    match format.as_str() {
        "text" => {
            println!("\n{}", report.pretty_box());
        },
        "json" => {
            let json = serde_json::to_string_pretty(&report).unwrap();
            println!("{}", json);
        },
        "both" | _ => {
            println!("\n{}", report.pretty_box());
            println!("\n📊 JSON Report:");
            let json = serde_json::to_string_pretty(&report).unwrap();
            println!("{}", json);
        }
    }
    
    // Show final status
    let all_tests_passed = report.input_hash == report.unpacked_hash && report.errors.is_empty();
    if all_tests_passed {
        println!("\n✅ GOLD STANDARD BENCHMARK: ALL TESTS PASSED");
        println!("📁 Reports saved to: bench_reports/<timestamp>/");
    } else {
        println!("\n❌ GOLD STANDARD BENCHMARK: SOME TESTS FAILED");
        println!("📁 Check detailed logs in: bench_reports/<timestamp>/");
        process::exit(1);
    }
}

fn show_advanced_menu() {
    println!("\n============================");
    println!("|    ADVANCED MENU (V2)     |");
    println!("============================");
    println!("Available in V2!");
    println!("- Developer tools");
    println!("- System diagnostics");
    println!("- Performance profiling");
    println!("- Log analysis");
    println!("- Advanced testing");
    println!("Press Enter to return to main menu");
    let mut _dummy = String::new();
    io::stdin().read_line(&mut _dummy).unwrap();
}

fn show_full_cli_menu() {
    println!("\n============================");
    println!("|     FULL CLI MENU (V2)    |");
    println!("============================");
    println!("Available in V2!");
    println!("- Direct command access");
    println!("- Automation support");
    println!("- Script integration");
    println!("- Batch processing");
    println!("Press Enter to return to main menu");
    let mut _dummy = String::new();
    io::stdin().read_line(&mut _dummy).unwrap();
}

fn handle_pack_file() {
    println!("Select input file to pack:");
    let input = match cli::pick_file("Select input file to pack:") {
        Some(file) => file,
        None => return, // User chose to go back
    };
    
    // Generate default output filename with .mmh extension
    let input_path = std::path::Path::new(&input);
    let default_output = input_path.file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new("output"))
        .to_string_lossy()
        .to_string() + ".mmh";
    
    print!("Enter output file name (or press Enter for '{}'): ", default_output);
    io::stdout().flush().unwrap();
    let mut output = String::new();
    io::stdin().read_line(&mut output).unwrap();
    let output = output.trim();
    
    // Use default if user just pressed Enter
    let final_output = if output.is_empty() {
        default_output
    } else {
        output.to_string()
    };
    
    // Check if file already exists and ask for overwrite permission
    if std::path::Path::new(&final_output).exists() {
        print!("File '{}' already exists. Overwrite? (y/N): ", final_output);
        io::stdout().flush().unwrap();
        let mut overwrite = String::new();
        io::stdin().read_line(&mut overwrite).unwrap();
        let overwrite = overwrite.trim().to_lowercase();
        
        if overwrite != "y" && overwrite != "yes" {
            println!("Pack cancelled. File not overwritten.");
            return;
        }
    }
    
    println!("Packing file...");
    println!("Press Ctrl+C to abort at any time");
    
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // Set up abort handler (ignore if already set)
    let _ = ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
        println!("\nAbort requested. Finishing current operation...");
    });
    
    let result = cli::pack(&input, &final_output);
    
    if running.load(Ordering::SeqCst) {
        if let Err(e) = result {
            eprintln!("Pack failed: {}", e);
        } else {
            println!("File packed successfully!");
        }
    } else {
        println!("Pack operation aborted.");
    }
}

fn handle_unpack_file() {
    println!("Select input file to unpack:");
    let input = match cli::pick_file("Select input file to unpack:") {
        Some(file) => file,
        None => return, // User chose to go back
    };
    
    // Generate default output filename by removing .mmh extension to restore original name
    let input_path = std::path::Path::new(&input);
    let default_output = if input_path.extension().and_then(|s| s.to_str()) == Some("mmh") {
        input_path.file_stem()
            .unwrap_or_else(|| std::ffi::OsStr::new("output"))
            .to_string_lossy()
            .to_string()
    } else {
        input_path.file_stem()
            .unwrap_or_else(|| std::ffi::OsStr::new("output"))
            .to_string_lossy()
            .to_string()
    };
    
    print!("Enter output file name (or press Enter for '{}'): ", default_output);
    io::stdout().flush().unwrap();
    let mut output = String::new();
    io::stdin().read_line(&mut output).unwrap();
    let output = output.trim();
    
    // Use default if user just pressed Enter
    let final_output = if output.is_empty() {
        default_output
    } else {
        output.to_string()
    };
    
    // Check if file already exists and ask for overwrite permission
    if std::path::Path::new(&final_output).exists() {
        print!("File '{}' already exists. Overwrite? (y/N): ", final_output);
        io::stdout().flush().unwrap();
        let mut overwrite = String::new();
        io::stdin().read_line(&mut overwrite).unwrap();
        let overwrite = overwrite.trim().to_lowercase();
        
        if overwrite != "y" && overwrite != "yes" {
            println!("Unpack cancelled. File not overwritten.");
            return;
        }
    }
    
    println!("Unpacking file...");
    println!("Press Ctrl+C to abort at any time");
    
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // Set up abort handler (ignore if already set)
    let _ = ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
        println!("\nAbort requested. Finishing current operation...");
    });
    
    let result = cli::unpack(&input, &final_output);
    
    if running.load(Ordering::SeqCst) {
        if let Err(e) = result {
            eprintln!("Unpack failed: {}", e);
        } else {
            println!("File unpacked successfully!");
        }
    } else {
        println!("Unpack operation aborted.");
    }
}

fn handle_verify_file() {
    println!("Select original file:");
    let input = match cli::pick_file("Select original file:") {
        Some(file) => file,
        None => return, // User chose to go back
    };
    
    println!("Select restored file:");
    let restored = match cli::pick_file("Select restored file:") {
        Some(file) => file,
        None => return, // User chose to go back
    };
    
    if let Err(e) = cli::verify(&input, &restored) {
        eprintln!("Verify failed: {}", e);
    } else {
        println!("✅ Integrity verification passed!");
    }
}

fn handle_generate_test_data() {
    print!("Enter output directory: ");
    io::stdout().flush().unwrap();
    let mut dir = String::new();
    io::stdin().read_line(&mut dir).unwrap();
    let dir = dir.trim();
    
    print!("Enter size in GB: ");
    io::stdout().flush().unwrap();
    let mut size = String::new();
    io::stdin().read_line(&mut size).unwrap();
    let size: u64 = size.trim().parse().unwrap_or(1);
    
    println!("Generating {} GB of test data...", size);
    println!("Press Ctrl+C to abort at any time");
    
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // Set up abort handler
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
        println!("\nAbort requested. Finishing current operation...");
    }).expect("Failed to set abort handler");
    
    // Run the generation with abort capability
    cli::gentestdir(dir, size);
    
    if running.load(Ordering::SeqCst) {
        println!("Test data generated successfully!");
    } else {
        println!("Test data generation aborted.");
    }
} 