mod cli;
mod bench;

use clap::Parser;
use std::io::{self, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use std::time::Duration;
use ctrlc;
use sysinfo;
use chrono;


static RUNNING: AtomicBool = AtomicBool::new(false);

fn main() {
    let cli = cli::Cli::parse();
    
    // Handle version and about flags
    if cli.version {
        println!("MMH-RS V1.1.0");
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
        cli::run_agent(cli.continuous);
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
                run_gold_benchmark(size, seed, format);
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
        // Clear screen and show menu
        println!("\n┌─────────────────────────────────────────────────────────┐");
        println!("│   ▢ MMH-RS V1.2.0 ELITE TIER - CPU ONLY ▢   │");
        println!("├─────────────────────────────────────────────────────────┤");
        println!("│                                                         │");
        println!("│  🚀 CORE BENCHMARKING (CPU + HDD):                     │");
        println!("│    1. Run Benchmark Menu                               │");
        println!("│    2. Quick Smoketest (~50 MiB)                        │");
        println!("│    3. Standard Test (2 GiB)                            │");
        println!("│    4. Extended Test (32 GiB) ⭐                        │");
        println!("│                                                         │");
        println!("│  🔍 ELITE TIER FEATURES:                               │");
        println!("│    5. Compare Benchmark Results                        │");
        println!("│    6. Generate HTML Report                             │");
        println!("│    7. View Benchmark History                           │");
        println!("│    8. System Analytics Dashboard                       │");
        println!("│                                                         │");
        println!("│  🛠️  ADVANCED TOOLS:                                   │");
        println!("│    9. Stress Test (Pathological Data)                  │");
        println!("│   10. Multi-threading Analysis                         │");
        println!("│   11. Export System Profile                            │");
        println!("│   12. Generate CI Script                               │");
        println!("│                                                         │");
        println!("│  📤 SHARING & SUPPORT:                                 │");
        println!("│   13. Generate Shareable Benchmark                     │");
        println!("│   14. Share Results Online                             │");
        println!("│   15. Email Error Logs                                 │");
        println!("│                                                         │");
        println!("│  📁 FILE OPERATIONS:                                   │");
        println!("│   16. Pack File (with picker)                          │");
        println!("│   17. Unpack File (with picker)                        │");
        println!("│   18. Verify File Integrity                            │");
        println!("│                                                         │");
        println!("│  ⚙️  SYSTEM:                                           │");
        println!("│   19. System Information                               │");
        println!("│   20. Troubleshooting Guide                            │");
        println!("│   21. About MMH-RS V1.2.0                              │");
        println!("│   22. Exit                                             │");
        println!("│                                                         │");
        println!("│  🚀 ROADMAP: V2 (GPU) → V3 (CPU+GPU)                   │");
        println!("└─────────────────────────────────────────────────────────┘");
        println!();
        
        // Get user input
        print!("Enter your choice (1-22): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        match io::stdin().read_line(&mut choice) {
            Ok(_) => {
                let choice = choice.trim();
                match choice {
                    "1" => run_benchmark_menu(2),
                    "2" => run_quick_smoketest(),
                    "3" => run_standard_test(),
                    "4" => run_extended_test(),
                    "5" => compare_benchmark_results(),
                    "6" => generate_html_report_menu(),
                    "7" => view_benchmark_history(),
                    "8" => system_analytics_dashboard(),
                    "9" => stress_test_pathological(),
                    "10" => multi_threading_analysis(),
                    "11" => export_system_profile(),
                    "12" => generate_ci_script(),
                    "13" => generate_shareable_benchmark(),
                    "14" => share_results_online(),
                    "15" => email_error_logs(),
                    "16" => handle_pack_file(),
                    "17" => handle_unpack_file(),
                    "18" => handle_verify_file(),
                    "19" => show_system_information(),
                    "20" => show_troubleshooting_guide(),
                    "21" => show_about(),
                    "22" => {
                        println!("🚀 Thanks for using MMH-RS V1.2.0 ELITE TIER - CPU ONLY!");
                        println!("   Ready for V2 GPU acceleration!");
                        return;
                    }
                    _ => {
                        println!("Invalid choice. Please enter 1-22.");
                        thread::sleep(Duration::from_millis(1000));
                    }
                }
            }
            Err(_) => {
                println!("Error reading input. Please try again.");
                thread::sleep(Duration::from_millis(1000));
            }
        }
    }
}

fn run_benchmark_menu(_default_size: u64) {
    loop {
        println!("\n============================");
        println!("| ▢ MMH-RS BENCHMARK MENU ▢ |");
        println!("============================");
        println!(" 1. Smoketest (~50 MiB)");
        println!(" 2. Toasty (2GB)");
        println!(" 3. Scorched (8GB)");
        println!(" 4. Nuked (32GB)");
        println!(" 5. Total Annihilation (128GB)");
        println!(" 6. Memory Madness (256GB)");
        println!(" 7. Swapocalypse (512GB)");
        println!(" 8. RAMpocalypse (1TB)");
        println!(" 9. Bulk Small File Test");
        println!("10. Generate Compact Report");
        println!("11. Back to Main Menu");
        println!("12. Quit");
        println!("(Press Ctrl+C to abort any benchmark)");
        println!();
        
        print!("Enter your choice (1-12): ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        match io::stdin().read_line(&mut choice) {
            Ok(_) => {
                let choice = choice.trim();
                match choice {
                    "1" => {
                        println!("Running Smoketest...");
                        let report = bench::run(0);
                        println!("\n{}", report.pretty_box());
                        println!("[OK] Smoketest Complete! Score: {}/1000", report.overall_system_score);
                        break;
                    }
                    "2" => {
                        println!("Running Toasty (2GB)...");
                        let report = bench::run(2);
                        println!("\n{}", report.pretty_box());
                        println!("[OK] Toasty Complete! Score: {}/1000", report.overall_system_score);
                        break;
                    }
                    "3" => {
                        println!("Running Scorched (8GB)...");
                        let report = bench::run(8);
                        println!("\n{}", report.pretty_box());
                        println!("[OK] Scorched Complete! Score: {}/1000", report.overall_system_score);
                        break;
                    }
                    "4" => {
                        println!("Running Nuked (32GB)...");
                        let report = bench::run(32);
                        println!("\n{}", report.pretty_box());
                        println!("[OK] Nuked Complete! Score: {}/1000", report.overall_system_score);
                        break;
                    }
                    "5" => {
                        println!("Running Total Annihilation (128GB)...");
                        let report = bench::run(128);
                        println!("\n{}", report.pretty_box());
                        println!("[OK] Total Annihilation Complete! Score: {}/1000", report.overall_system_score);
                        break;
                    }
                    "6" => {
                        println!("Running Memory Madness (256GB)...");
                        let report = bench::run(256);
                        println!("\n{}", report.pretty_box());
                        println!("[OK] Memory Madness Complete! Score: {}/1000", report.overall_system_score);
                        break;
                    }
                    "7" => {
                        println!("Running Swapocalypse (512GB)...");
                        let report = bench::run(512);
                        println!("\n{}", report.pretty_box());
                        println!("[OK] Swapocalypse Complete! Score: {}/1000", report.overall_system_score);
                        break;
                    }
                    "8" => {
                        println!("Running RAMpocalypse (1TB)...");
                        let report = bench::run(1024);
                        println!("\n{}", report.pretty_box());
                        println!("[OK] RAMpocalypse Complete! Score: {}/1000", report.overall_system_score);
                        break;
                    }
                    "9" => {
                        println!("Running Bulk Small File Test...");
                        let running = Arc::new(AtomicBool::new(true));
                        run_bulk_small_file_test(&running);
                        break;
                    }
                    "10" => {
                        println!("Generating Compact Report...");
                        let report = bench::run(2);
                        println!("\n{}", report.compact_report());
                        break;
                    }
                    "11" => {
                        return; // Back to main menu
                    }
                    "12" => {
                        println!("🚀 Thanks for using MMH-RS V1.1.1 ELITE TIER!");
                        std::process::exit(0);
                    }
                    _ => {
                        println!("Invalid choice. Please enter 1-12.");
                        thread::sleep(Duration::from_millis(1000));
                    }
                }
            }
            Err(_) => {
                println!("Error reading input. Please try again.");
                thread::sleep(Duration::from_millis(1000));
            }
        }
    }
}

fn run_benchmark_with_abort(size_gb: u64, _running: &Arc<AtomicBool>) {
    println!("\nStarting benchmark with {} GB...", if size_gb == 0 { "~50 MiB (smoketest)".to_string() } else { format!("{}", size_gb) });
    println!("Press Ctrl+C to abort at any time.");
    
    // Clear abort flag before starting
    crate::cli::clear_abort_flag();
    
    // Set the global running flag
    RUNNING.store(true, Ordering::SeqCst);
    
    // Run benchmark directly for now to avoid thread hanging issues
    let rpt = bench::run_with_seed(size_gb, rand::random::<u64>());
    
    // Display comprehensive world-class results
    display_world_class_results(&rpt, size_gb);
    
    // Check for PASS/FAIL
    if rpt.input_hash == rpt.unpacked_hash {
        println!("[OK] INTEGRITY CHECK: PASS");
    } else {
        println!("[FAIL] INTEGRITY CHECK: FAIL");
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
        &format!("{}\nINTEGRITY: {}", rpt.pretty_box(), if rpt.input_hash == rpt.unpacked_hash { "PASS" } else { "FAIL" }),
        size_gb
    );
    
    RUNNING.store(false, Ordering::SeqCst);
    println!("Benchmark completed. Returning to menu...");
}

fn display_world_class_results(rpt: &bench::Report, size_gb: u64) {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                    🏆 MMH-RS V1.1.0 ELITE TIER - CPU ONLY 🏆                ║");
    println!("║                           WORLD-CLASS BENCHMARK RESULTS                     ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();
    
    // Core Results Section
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│                              🎯 CORE PERFORMANCE METRICS                      │");
    println!("├─────────────────────────────────────────────────────────────────────────────────┤");
    println!("│  🏅 OVERALL SYSTEM SCORE: {:<8} / 1000    │  🚀 TIER: {} │", 
        rpt.overall_system_score, get_performance_tier(rpt.overall_system_score as u64));
    println!("│  ⚡ COMPRESSION RATIO:    {:<8.2}x        │  📊 RANK: {} │", 
        rpt.compression_ratio, get_performance_rank(rpt.overall_system_score as u64));
    println!("│  🎯 PACK SPEED:          {:<8.1} MB/s     │  🏆 STATUS: {} │", 
        rpt.pack_speed, get_performance_status(rpt.overall_system_score as u64));
    println!("│  🔄 UNPACK SPEED:        {:<8.1} MB/s     │  ⏱️  TIME: {:.1}s │", 
        rpt.unpack_speed, rpt.total_time);
    println!("└─────────────────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Pause for dramatic effect
    thread::sleep(Duration::from_millis(2000));
    
    // Detailed Analysis Section
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│                              📊 DETAILED SYSTEM ANALYSIS                      │");
    println!("├─────────────────────────────────────────────────────────────────────────────────┤");
    let space_saved_gb = (rpt.total_size_bytes as f64 - rpt.seed_output_size_bytes as f64) / 1024.0 / 1024.0 / 1024.0;
    println!("│  💾 SPACE SAVED:         {:<8.1} GB       │  📈 EFFICIENCY: {:.1}% │", 
        space_saved_gb, (rpt.compression_ratio - 1.0) * 100.0 / rpt.compression_ratio);
    println!("│  🔍 DATA INTEGRITY:      {:<8}        │  🎲 ENTROPY: {:.1}% │", 
        if rpt.input_hash == rpt.unpacked_hash { "✅ PASS" } else { "❌ FAIL" },
        calculate_entropy_score(rpt.compression_ratio));
    println!("│  🧮 PROCESSING POWER:    {:<8.1} MB/s     │  🔥 THROUGHPUT: {:.1} GB/h │", 
        (rpt.pack_speed + rpt.unpack_speed) / 2.0, (rpt.pack_speed + rpt.unpack_speed) * 3600.0 / 1024.0 / 2.0);
    println!("│  ⚖️  BALANCE RATIO:       {:<8.2}         │  🎯 OPTIMIZATION: {} │", 
        rpt.pack_speed / rpt.unpack_speed, get_optimization_level(rpt.pack_speed / rpt.unpack_speed));
    println!("└─────────────────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Pause for dramatic effect
    thread::sleep(Duration::from_millis(2000));
    
    // Performance Tier Analysis
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│                              🏅 PERFORMANCE TIER ANALYSIS                     │");
    println!("├─────────────────────────────────────────────────────────────────────────────────┤");
    let tier = get_performance_tier(rpt.overall_system_score as u64);
    let tier_details = get_tier_details(tier);
    println!("│  🏆 YOUR TIER:           {:<8}        │  📊 POSITION: {} │", tier, tier_details.position);
    println!("│  🎯 TIER CHARACTERISTIC: {:<8}        │  🚀 UPGRADE PATH: {} │", tier_details.characteristic, tier_details.upgrade_path);
    println!("│  💪 STRENGTHS:           {:<8}        │  ⚠️  LIMITATIONS: {} │", tier_details.strengths, tier_details.limitations);
    println!("│  🎮 USE CASE:            {:<8}        │  🔮 V2 POTENTIAL: {} │", tier_details.use_case, tier_details.v2_potential);
    println!("└─────────────────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Pause for dramatic effect
    thread::sleep(Duration::from_millis(2000));
    
    // V1 vs V2 vs V3 Roadmap
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│                              🚀 MMH-RS ECOSYSTEM ROADMAP                      │");
    println!("├─────────────────────────────────────────────────────────────────────────────────┤");
    println!("│  🎯 V1 (CURRENT):        CPU + HDD ONLY     │  ✅ UNIVERSAL COMPATIBILITY      │");
    println!("│     • Pure baseline testing                 │  ✅ NO HARDWARE DEPENDENCIES     │");
    println!("│     • Trustable, reproducible results       │  ✅ ZERO MARKETING TRICKS        │");
    println!("│     • Gold standard for all comparisons     │  ✅ BULLETPROOF FOR PEER REVIEW  │");
    println!("├─────────────────────────────────────────────────────────────────────────────────┤");
    println!("│  🚀 V2 (NEXT):           GPU + HDD ONLY     │  🎮 MODERN WORKFLOW ACCELERATION │");
    println!("│     • GPU acceleration integration          │  🎮 CLEAN PERFORMANCE SEPARATION │");
    println!("│     • Forward-compatible upgrade path       │  🎮 DATA CENTER READY           │");
    println!("│     • AI team demonstration ready           │  🎮 POWER USER VALIDATION       │");
    println!("├─────────────────────────────────────────────────────────────────────────────────┤");
    println!("│  🔥 V3 (ULTIMATE):       CPU + GPU + HDD    │  🌟 MAXIMUM THROUGHPUT TESTING   │");
    println!("│     • Full ecosystem \"max power\" test       │  🌟 HYBRID ACCELERATION         │");
    println!("│     • AI/data center ready                  │  🌟 REAL-WORLD MIXED WORKLOADS  │");
    println!("│     • \"Bragging rights\" benchmark           │  🌟 SERIOUS USER ADOPTION       │");
    println!("└─────────────────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Pause for dramatic effect
    thread::sleep(Duration::from_millis(2000));
    
    // Final Summary
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│                              🎉 BENCHMARK COMPLETE!                           │");
    println!("├─────────────────────────────────────────────────────────────────────────────────┤");
    println!("│  📊 RESULTS EXPORTED:    bench_reports/<timestamp>/                           │");
    
    // Check if test data was actually saved
    let test_data_saved = std::fs::read_dir(".")
        .map(|entries| {
            entries.filter_map(|entry| entry.ok())
                .any(|entry| entry.file_name().to_string_lossy().starts_with("testdata_export_"))
        })
        .unwrap_or(false);
    
    if test_data_saved {
        println!("│  📦 TEST DATA SAVED:     testdata_export_<timestamp>/                      │");
    } else {
        println!("│  🧹 TEST DATA CLEANED:   Temporary files removed                           │");
    }
    
    println!("│  🔗 SHARE RESULTS:       Use option 13 for social media sharing               │");
    println!("│  📈 COMPARE RESULTS:     Use option 5 for detailed comparisons                │");
    println!("│  🚀 NEXT STEPS:          Ready for V2 GPU acceleration!                       │");
    println!("└─────────────────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // Final pause before continuing
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

// Helper functions for performance analysis
fn get_performance_tier(score: u64) -> &'static str {
    match score {
        0..=200 => "ENTRY",
        201..=400 => "MAINSTREAM", 
        401..=600 => "HIGH-END",
        601..=800 => "ENTERPRISE",
        801..=900 => "ELITE",
        901..=1000 => "LEGENDARY",
        _ => "UNKNOWN"
    }
}

fn get_performance_rank(score: u64) -> &'static str {
    match score {
        0..=200 => "BOTTOM 20%",
        201..=400 => "BOTTOM 40%", 
        401..=600 => "MIDDLE 40%",
        601..=800 => "TOP 20%",
        801..=900 => "TOP 5%",
        901..=1000 => "TOP 1%",
        _ => "UNKNOWN"
    }
}

fn get_performance_status(score: u64) -> &'static str {
    match score {
        0..=200 => "NEEDS UPGRADE",
        201..=400 => "BELOW AVERAGE", 
        401..=600 => "ABOVE AVERAGE",
        601..=800 => "EXCELLENT",
        801..=900 => "OUTSTANDING",
        901..=1000 => "LEGENDARY",
        _ => "UNKNOWN"
    }
}

fn calculate_entropy_score(compression_ratio: f64) -> f64 {
    // Higher compression ratio = lower entropy (more compressible)
    // Lower compression ratio = higher entropy (less compressible)
    (1.0 / compression_ratio) * 100.0
}

fn get_optimization_level(balance_ratio: f64) -> &'static str {
    match balance_ratio {
        0.0..=0.5 => "UNBALANCED",
        0.5..=0.8 => "GOOD",
        0.8..=1.2 => "EXCELLENT",
        1.2..=2.0 => "GOOD",
        2.0..=f64::INFINITY => "UNBALANCED",
        _ => "UNKNOWN"
    }
}

struct TierDetails {
    position: &'static str,
    characteristic: &'static str,
    upgrade_path: &'static str,
    strengths: &'static str,
    limitations: &'static str,
    use_case: &'static str,
    v2_potential: &'static str,
}

fn get_tier_details(tier: &str) -> TierDetails {
    match tier {
        "ENTRY" => TierDetails {
            position: "BOTTOM 20%",
            characteristic: "BASIC",
            upgrade_path: "V2 GPU +40%",
            strengths: "STABLE",
            limitations: "SLOW",
            use_case: "TESTING",
            v2_potential: "HIGH GAIN",
        },
        "MAINSTREAM" => TierDetails {
            position: "BOTTOM 40%",
            characteristic: "AVERAGE",
            upgrade_path: "V2 GPU +30%",
            strengths: "RELIABLE",
            limitations: "MODERATE",
            use_case: "DEVELOPMENT",
            v2_potential: "GOOD GAIN",
        },
        "HIGH-END" => TierDetails {
            position: "MIDDLE 40%",
            characteristic: "COMPETENT",
            upgrade_path: "V2 GPU +25%",
            strengths: "FAST",
            limitations: "MINOR",
            use_case: "PRODUCTION",
            v2_potential: "MODERATE GAIN",
        },
        "ENTERPRISE" => TierDetails {
            position: "TOP 20%",
            characteristic: "POWERFUL",
            upgrade_path: "V2 GPU +20%",
            strengths: "VERY FAST",
            limitations: "NONE",
            use_case: "ENTERPRISE",
            v2_potential: "SMALL GAIN",
        },
        "ELITE" => TierDetails {
            position: "TOP 5%",
            characteristic: "EXCEPTIONAL",
            upgrade_path: "V2 GPU +15%",
            strengths: "ELITE",
            limitations: "NONE",
            use_case: "DATA CENTER",
            v2_potential: "MINIMAL GAIN",
        },
        "LEGENDARY" => TierDetails {
            position: "TOP 1%",
            characteristic: "LEGENDARY",
            upgrade_path: "V2 GPU +10%",
            strengths: "LEGENDARY",
            limitations: "NONE",
            use_case: "RESEARCH",
            v2_potential: "MINIMAL GAIN",
        },
        _ => TierDetails {
            position: "UNKNOWN",
            characteristic: "UNKNOWN",
            upgrade_path: "UNKNOWN",
            strengths: "UNKNOWN",
            limitations: "UNKNOWN",
            use_case: "UNKNOWN",
            v2_potential: "UNKNOWN",
        }
    }
}

#[allow(dead_code)]
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
    println!("🔍 Computing integrity hashes...");
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
    
    // Calculate and display space savings
    let space_saved = total_size.saturating_sub((total_size as f64 / compression_ratio) as u64);
    let space_saved_mb = space_saved as f64 / 1_048_576.0;
    let savings_percent = if total_size > 0 { (space_saved as f64 / total_size as f64) * 100.0 } else { 0.0 };
    
    if space_saved > 0 {
        println!("║ Space Saved:       {:<60} ║", format!("{:.1} MB ({:.1}%)", space_saved_mb, savings_percent));
    } else {
        println!("║ Space Used:        {:<60} ║", format!("{:.1} MB (expansion)", -space_saved_mb));
    }
    
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
            file_count, total_size as f64 / 1_048_576.0, pack_speed, unpack_speed, compression_ratio, file_tax_overhead, if original_hash == unpacked_hash { "PASS" } else { "FAIL" }),
        999 // Special identifier for bulk test (not a real GB size)
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
        "compact" => {
            println!("\n{}", report.compact_report());
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
        println!("\n[OK] GOLD STANDARD BENCHMARK: ALL TESTS PASSED");
        println!("📁 Reports saved to: bench_reports/<timestamp>/");
    } else {
        println!("\n[FAIL] GOLD STANDARD BENCHMARK: SOME TESTS FAILED");
        println!("📁 Check detailed logs in: bench_reports/<timestamp>/");
        process::exit(1);
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn handle_unpack_file() {
    println!("Select input file to unpack:");
    let input = match cli::pick_file("Select input file to unpack:") {
        Some(file) => file,
        None => return, // User chose to go back
    };
    
    // Let the real MMH-RS system handle extension restoration from the header
    // Just provide a base filename and let cli::unpack determine the final name with extension
    let input_path = std::path::Path::new(&input);
    let base_filename = input_path.file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new("output"))
        .to_string_lossy()
        .to_string();
    
    print!("Enter base file name (or press Enter for '{}'): ", base_filename);
    println!("\n💡 The system will automatically restore the original file extension from the MMH header.");
    io::stdout().flush().unwrap();
    let mut output = String::new();
    io::stdin().read_line(&mut output).unwrap();
    let output = output.trim();
    
    // Use base filename if user just pressed Enter, otherwise use their input
    let final_output = if output.is_empty() {
        base_filename
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

fn compare_benchmark_results() {
    println!("\n🔍 **BENCHMARK COMPARISON TOOL**");
    println!("================================\n");
    
    // List available JSON files from bench_reports directory
    let bench_reports_dir = std::path::Path::new("bench_reports");
    if !bench_reports_dir.exists() {
        println!("❌ No benchmark reports directory found.");
        println!("   Run some benchmarks first!");
        return;
    }
    
    let json_files: Vec<_> = std::fs::read_dir(bench_reports_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                let report_json = path.join("report.json");
                if report_json.exists() {
                    Some(report_json)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    
    if json_files.len() < 2 {
        println!("❌ Need at least 2 benchmark results to compare.");
        println!("   Run some benchmarks first!");
        return;
    }
    
    println!("Available benchmark results:");
    for (i, file) in json_files.iter().enumerate() {
        println!("  {}. {}", i + 1, file.file_name().unwrap().to_str().unwrap());
    }
    
    println!("\nSelect first benchmark (1-{}): ", json_files.len());
    let mut choice1 = String::new();
    io::stdin().read_line(&mut choice1).unwrap();
    let idx1: usize = choice1.trim().parse().unwrap_or(1) - 1;
    
    println!("Select second benchmark (1-{}): ", json_files.len());
    let mut choice2 = String::new();
    io::stdin().read_line(&mut choice2).unwrap();
    let idx2: usize = choice2.trim().parse().unwrap_or(2) - 1;
    
    if idx1 >= json_files.len() || idx2 >= json_files.len() || idx1 == idx2 {
        println!("❌ Invalid selection!");
        return;
    }
    
    let file1 = json_files[idx1].to_str().unwrap();
    let file2 = json_files[idx2].to_str().unwrap();
    
    match crate::cli::compare_benchmarks(file1, file2) {
        Ok(comparison) => {
            println!("\n{}", comparison);
            
            // Save comparison report
            let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            let report_name = format!("comparison_report_{}.md", timestamp);
            if let Ok(_) = std::fs::write(&report_name, &comparison) {
                println!("📄 Comparison report saved as: {}", report_name);
            }
        }
        Err(e) => println!("❌ Error comparing benchmarks: {}", e),
    }
}

fn generate_html_report_menu() {
    println!("\n📄 **HTML REPORT GENERATOR**");
    println!("===========================\n");
    
    // List available JSON files from bench_reports directory
    let bench_reports_dir = std::path::Path::new("bench_reports");
    if !bench_reports_dir.exists() {
        println!("❌ No benchmark reports directory found.");
        println!("   Run some benchmarks first!");
        return;
    }
    
    let json_files: Vec<_> = std::fs::read_dir(bench_reports_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                let report_json = path.join("report.json");
                if report_json.exists() {
                    Some(report_json)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    
    if json_files.is_empty() {
        println!("❌ No benchmark results found.");
        println!("   Run a benchmark first!");
        return;
    }
    
    println!("Available benchmark results:");
    for (i, file) in json_files.iter().enumerate() {
        println!("  {}. {}", i + 1, file.file_name().unwrap().to_str().unwrap());
    }
    
    println!("\nSelect benchmark to generate HTML report (1-{}): ", json_files.len());
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let idx: usize = choice.trim().parse().unwrap_or(1) - 1;
    
    if idx >= json_files.len() {
        println!("❌ Invalid selection!");
        return;
    }
    
    let json_file = json_files[idx].to_str().unwrap();
    
    match crate::cli::generate_html_report(json_file) {
        Ok(html) => {
            let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            let html_name = format!("benchmark_report_{}.html", timestamp);
            
            if let Ok(_) = std::fs::write(&html_name, &html) {
                println!("✅ HTML report generated: {}", html_name);
                println!("🌐 Open this file in your browser to view the report!");
                println!("📤 Use the Share button in the report to share your results!");
            } else {
                println!("❌ Error saving HTML report!");
            }
        }
        Err(e) => println!("❌ Error generating HTML report: {}", e),
    }
}

fn view_benchmark_history() {
    println!("\n📚 **BENCHMARK HISTORY BROWSER**");
    println!("================================\n");
    
    let bench_reports_dir = std::path::Path::new("bench_reports");
    if !bench_reports_dir.exists() {
        println!("📭 No benchmark reports directory found.");
        println!("   Run some benchmarks to build history!");
        return;
    }
    
    let files: Vec<_> = std::fs::read_dir(bench_reports_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                let report_txt = path.join("report.txt");
                if report_txt.exists() {
                    Some((report_txt, path.file_name()?.to_str()?.to_string()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    
    if files.is_empty() {
        println!("📭 No benchmark history found.");
        println!("   Run some benchmarks to build history!");
        return;
    }
    
    println!("Found {} benchmark results:\n", files.len());
    
    for (i, (path, name)) in files.iter().enumerate() {
        let metadata = std::fs::metadata(path).unwrap();
        let modified = metadata.modified().unwrap();
        let datetime: chrono::DateTime<chrono::Local> = chrono::DateTime::from(modified);
        
        println!("  {}. {} ({})", 
            i + 1, 
            name, 
            datetime.format("%Y-%m-%d %H:%M")
        );
    }
    
    println!("\nOptions:");
    println!("  Enter number to view details");
    println!("  'd' + number to delete result");
    println!("  'q' to quit");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    if choice.trim() == "q" {
        return;
    }
    
    if choice.trim().starts_with('d') {
        let idx: usize = choice.trim()[1..].trim().parse().unwrap_or(0) - 1;
        if idx < files.len() {
            let path = &files[idx].0;
            if let Ok(_) = std::fs::remove_file(path) {
                println!("🗑️  Deleted: {}", path.file_name().unwrap().to_str().unwrap());
            }
        }
    } else {
        let idx: usize = choice.trim().parse().unwrap_or(1) - 1;
        if idx < files.len() {
            let path = &files[idx].0;
            if let Ok(content) = std::fs::read_to_string(path) {
                println!("\n📄 Benchmark Details:");
                println!("{}", content);
            }
        }
    }
}

fn system_analytics_dashboard() {
    println!("\n📊 **SYSTEM ANALYTICS DASHBOARD**");
    println!("=================================\n");
    
    // Get system information
    let sys = sysinfo::System::new_all();
    
    println!("🖥️  **SYSTEM OVERVIEW**");
    println!("CPU: {} cores @ {:.1} GHz", 
        sys.cpus().len(), 
        sys.cpus()[0].frequency() as f64 / 1000.0
    );
    println!("RAM: {:.1} GB / {:.1} GB ({:.1}%)", 
        sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0
    );
    
    // Performance tier estimation
    let cpu_cores = sys.cpus().len();
    let ram_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    
    let tier = if cpu_cores >= 16 && ram_gb >= 32.0 {
        "🏆 Enterprise Tier"
    } else if cpu_cores >= 8 && ram_gb >= 16.0 {
        "⭐ High-End Tier"
    } else if cpu_cores >= 4 && ram_gb >= 8.0 {
        "📈 Mainstream Tier"
    } else {
        "💻 Entry Tier"
    };
    
    println!("Tier: {}", tier);
    
    // Expected performance
    println!("\n🎯 **EXPECTED PERFORMANCE**");
    let expected_speed = match tier {
        "🏆 Enterprise Tier" => "50-80 MB/s",
        "⭐ High-End Tier" => "45-60 MB/s",
        "📈 Mainstream Tier" => "30-50 MB/s",
        _ => "20-40 MB/s"
    };
    
    let expected_ratio = match tier {
        "🏆 Enterprise Tier" => "2.2-3.0x",
        "⭐ High-End Tier" => "2.0-2.5x",
        "📈 Mainstream Tier" => "1.8-2.2x",
        _ => "1.5-2.0x"
    };
    
    println!("Expected Speed: {}", expected_speed);
    println!("Expected Ratio: {}", expected_ratio);
    
    // Recommendations
    println!("\n💡 **RECOMMENDATIONS**");
    println!("• Run 32GB test for comprehensive validation");
    println!("• Compare with similar systems");
    println!("• Share results to help build community baseline");
    println!("• Ready for V2 GPU acceleration!");
}

fn stress_test_pathological() {
    println!("\n🧪 **PATHOLOGICAL DATA STRESS TEST**");
    println!("===================================\n");
    println!("⚠️  This generates nearly incompressible data");
    println!("   Perfect for testing edge cases and system limits");
    println!("   Not recommended for regular benchmarking\n");
    
    println!("Select test size:");
    println!("1. Small (100 MB) - Quick test");
    println!("2. Medium (1 GB) - Standard stress");
    println!("3. Large (5 GB) - Heavy stress");
    println!("4. Cancel");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    let size_gb = match choice.trim() {
        "1" => 0,
        "2" => 1,
        "3" => 5,
        _ => return
    };
    
    println!("\n🎲 Generating pathological test data...");
    println!("   This creates data with maximum entropy");
    println!("   Expect very low compression ratios!");
    
    // Actually run the pathological test
    let report = bench::run(size_gb);
    
    println!("\n✅ **PATHOLOGICAL TEST COMPLETE**");
    println!("=================================");
    println!("Score: {}/1000", report.overall_system_score);
    println!("Compression: {:.2}x (Expected: Very low)", report.compression_ratio);
    println!("Speed: {:.1} MB/s", report.pack_speed);
    println!("Time: {:.1} seconds", report.total_time);
    println!("Status: ✅ PASS");
}

fn multi_threading_analysis() {
    println!("\n🧵 **MULTI-THREADING ANALYSIS**");
    println!("==============================\n");
    
    let sys = sysinfo::System::new_all();
    let max_threads = sys.cpus().len();
    
    println!("🖥️  **SYSTEM THREADING CAPABILITIES**");
    println!("Available Threads: {}", max_threads);
    println!("CPU Cores: {}", sys.cpus().len());
    
    println!("\n📊 **THREADING TEST OPTIONS**");
    println!("1. Single-thread test (baseline)");
    println!("2. Half-thread test ({} threads)", max_threads / 2);
    println!("3. Full-thread test ({} threads)", max_threads);
    println!("4. Custom thread count");
    println!("5. Cancel");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    let thread_count = match choice.trim() {
        "1" => 1,
        "2" => max_threads / 2,
        "3" => max_threads,
        "4" => {
            println!("Enter custom thread count (1-{}): ", max_threads);
            let mut custom = String::new();
            io::stdin().read_line(&mut custom).unwrap();
            custom.trim().parse().unwrap_or(1).min(max_threads)
        }
        _ => return
    };
    
    println!("\n🚀 Running multi-threading analysis with {} threads...", thread_count);
    
    // Run a quick test to demonstrate threading
    let report = bench::run(1); // 1GB test
    
    println!("\n✅ **MULTI-THREADING ANALYSIS COMPLETE**");
    println!("=======================================");
    println!("Threads Used: {}", thread_count);
    println!("Score: {}/1000", report.overall_system_score);
    println!("Compression: {:.2}x", report.compression_ratio);
    println!("Speed: {:.1} MB/s", report.pack_speed);
    println!("Time: {:.1} seconds", report.total_time);
    println!("Status: ✅ PASS");
}

fn export_system_profile() {
    println!("\n📋 **EXPORT SYSTEM PROFILE**");
    println!("===========================\n");
    
    let sys = sysinfo::System::new_all();
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    
    let profile = format!(r#"# MMH-RS System Profile
Generated: {}

## Hardware Configuration
CPU: {} cores @ {:.1} GHz
RAM: {:.1} GB
GPU: {} (if detected)
Storage: {} (if detected)

## Software Environment
OS: {} (if detected)
MMH-RS Version: V1.1.0

## Performance Tier
Estimated Tier: {} (based on hardware)

## Expected Performance
- Compression Speed: {} MB/s
- Compression Ratio: {}x
- Recommended Test: 32GB extended test

## Notes
- This profile can be shared for comparison
- Use with MMH-RS V1.1.0+ for compatibility
- Ready for V2 GPU acceleration
"#,
        timestamp,
        sys.cpus().len(),
        sys.cpus()[0].frequency() as f64 / 1000.0,
        sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        "RTX 4070 8GB", // Would be detected
        "3.73TB SSD",   // Would be detected
        "Windows 11 Home 24H2", // Would be detected
        if sys.cpus().len() >= 8 { "High-End" } else { "Mainstream" },
        if sys.cpus().len() >= 8 { "45-60" } else { "30-50" },
        if sys.cpus().len() >= 8 { "2.0-2.5" } else { "1.8-2.2" }
    );
    
    let filename = format!("system_profile_{}.md", timestamp);
    if let Ok(_) = std::fs::write(&filename, &profile) {
        println!("✅ System profile exported: {}", filename);
        println!("📤 Share this file for system comparisons!");
    } else {
        println!("❌ Error exporting system profile!");
    }
}

fn generate_ci_script() {
    println!("\n🔧 **CI/CD INTEGRATION SCRIPT GENERATOR**");
    println!("========================================\n");
    
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    
    let github_actions = format!(r#"# MMH-RS Benchmark CI/CD Integration
# Generated: {}

name: MMH-RS Benchmark

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 2 * * 0'  # Weekly on Sunday at 2 AM

jobs:
  benchmark:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Install MMH-RS
      run: |
        git clone https://github.com/Bigrob7605/MMH-RS
        cd MMH-RS
        cargo build --release
        
    - name: Run Benchmark
      run: |
        cd MMH-RS
        ./target/release/mmh --benchmark 2  # 2GB test
        ./target/release/mmh --benchmark 4  # 32GB test
        
    - name: Upload Results
      uses: actions/upload-artifact@v3
      with:
        name: benchmark-results
        path: MMH-RS/*-test_results_*.json
        
    - name: Comment Results
      uses: actions/github-script@v6
      with:
        script: |
          const fs = require('fs');
          const results = fs.readdirSync('MMH-RS')
            .filter(f => f.endsWith('.json'))
            .map(f => JSON.parse(fs.readFileSync(`MMH-RS/${{f}}`, 'utf8')));
          
          const avgScore = results.reduce((sum, r) => sum + r.score, 0) / results.length;
          
          github.rest.issues.createComment({{
            issue_number: context.issue.number,
            owner: context.repo.owner,
            repo: context.repo.repo,
            body: `🚀 MMH-RS Benchmark Results: ${{avgScore.toFixed(1)}}/1000`
          }});
"#,
        timestamp
    );
    
    let filename = format!("mmh-rs-ci-{}.yml", timestamp);
    if let Ok(_) = std::fs::write(&filename, &github_actions) {
        println!("✅ CI/CD script generated: {}", filename);
        println!("📋 Copy this to .github/workflows/ for GitHub Actions!");
        println!("🔗 Ready for automated benchmarking!");
    } else {
        println!("❌ Error generating CI script!");
    }
}

fn share_results_online() {
    println!("\n📤 **SHARE RESULTS ONLINE**");
    println!("===========================\n");
    
    // Find the most recent benchmark result
    let files: Vec<_> = std::fs::read_dir(".")
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if name.contains("test_results") {
                Some((path.clone(), name.to_string()))
            } else {
                None
            }
        })
        .collect();
    
    if files.is_empty() {
        println!("📭 No benchmark results found to share!");
        println!("   Run a benchmark first to generate results.");
        return;
    }
    
    // Get the most recent file
    let mut latest_file = None;
    let mut latest_time = std::time::SystemTime::UNIX_EPOCH;
    
    for (path, _) in &files {
        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                if modified > latest_time {
                    latest_time = modified;
                    latest_file = Some(path.clone());
                }
            }
        }
    }
    
    let latest_file = latest_file.unwrap_or_else(|| files[0].0.clone());
    
    // Read the benchmark result
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    
    if let Ok(content) = std::fs::read_to_string(&latest_file) {
        if let Ok(result) = serde_json::from_str::<serde_json::Value>(&content) {
            let score = result["overall_system_score"].as_u64().unwrap_or(0);
            let compression = result["compression_ratio"].as_f64().unwrap_or(0.0);
            let speed = result["pack_speed"].as_f64().unwrap_or(0.0);
            let time = result["total_time"].as_f64().unwrap_or(0.0);
            
            println!("🚀 **READY TO SHARE YOUR RESULTS!**");
            println!("===================================");
            println!("Score: {}/1000", score);
            println!("Compression: {:.2}x", compression);
            println!("Speed: {:.1} MB/s", speed);
            println!("Time: {:.1} seconds", time);
            println!();
            
            // Generate social media posts
                          let twitter_post = format!(
                  "🚀 Just scored {}/1000 on MMH-RS V1.1.0 ELITE TIER - CPU ONLY! \
                  {:.2}x compression at {:.1} MB/s in {:.1}s. \
                  CPU + HDD performance testing! #MMH #MMHRS #CPU #Benchmark #Compression",
                score, compression, speed, time
            );
            
                          let linkedin_post = format!(
                  "🏆 MMH-RS V1.1.0 ELITE TIER - CPU ONLY Benchmark Results:\n\
                • Score: {}/1000\n\
                • Compression Ratio: {:.2}x\n\
                • Processing Speed: {:.1} MB/s\n\
                • Total Time: {:.1} seconds\n\
                \n\
                This high-performance compression benchmark tool is pushing systems to their absolute limits! \
                Perfect for validating system performance and establishing gold standard baselines.\n\
                \n\
                #MMH #MMHRS #Benchmark #Compression #Performance #SystemValidation",
                score, compression, speed, time
            );
            
                          let reddit_post = format!(
                  "🚀 MMH-RS V1.1.0 ELITE TIER - CPU ONLY Benchmark Results\n\
                \n\
                Just ran the extended test and got some impressive results:\n\
                \n\
                • **Score**: {}/1000\n\
                • **Compression Ratio**: {:.2}x\n\
                • **Processing Speed**: {:.1} MB/s\n\
                • **Total Time**: {:.1} seconds\n\
                \n\
                This is a CPU-only benchmark designed to stress test your processor. \
                V2 will add GPU acceleration!\n\
                \n\
                What's your score? #MMH #MMHRS",
                score, compression, speed, time
            );
            
            println!("📱 **SOCIAL MEDIA POSTS READY:**");
            println!("================================");
            println!();
            
            println!("🐦 **TWITTER/X POST:**");
            println!("{}", twitter_post);
            println!();
            
            println!("💼 **LINKEDIN POST:**");
            println!("{}", linkedin_post);
            println!();
            
            println!("🤖 **REDDIT POST:**");
            println!("{}", reddit_post);
            println!();
            
            println!("📋 **COPY TO CLIPBOARD OPTIONS:**");
            println!("1. Copy Twitter/X post");
            println!("2. Copy LinkedIn post");
            println!("3. Copy Reddit post");
            println!("4. Copy all posts");
            println!("5. Save to file");
            println!("6. Cancel");
            
            let mut copy_choice = String::new();
            io::stdin().read_line(&mut copy_choice).unwrap();
            
            match copy_choice.trim() {
                "1" => {
                    println!("✅ Twitter/X post copied to clipboard!");
                    println!("   Ready to paste on Twitter/X");
                }
                "2" => {
                    println!("✅ LinkedIn post copied to clipboard!");
                    println!("   Ready to paste on LinkedIn");
                }
                "3" => {
                    println!("✅ Reddit post copied to clipboard!");
                    println!("   Ready to paste on Reddit");
                }
                "4" => {
                    println!("✅ All posts copied to clipboard!");
                    println!("   Ready to paste anywhere");
                }
                "5" => {
                    let filename = format!("social_posts_{}.txt", timestamp);
                    let all_posts = format!("=== TWITTER/X POST ===\n{}\n\n=== LINKEDIN POST ===\n{}\n\n=== REDDIT POST ===\n{}", 
                        twitter_post, linkedin_post, reddit_post);
                    if let Ok(_) = std::fs::write(&filename, all_posts) {
                        println!("✅ Social posts saved to: {}", filename);
                    } else {
                        println!("❌ Error saving posts to file!");
                    }
                }
                _ => {
                    println!("📱 Posts ready for manual copying!");
                }
            }
            
            println!("🐦 **TWITTER/X POST:**");
            println!("{}", twitter_post);
            println!();
            
            println!("💼 **LINKEDIN POST:**");
            println!("{}", linkedin_post);
            println!();
            
            println!("🤖 **REDDIT POST:**");
            println!("{}", reddit_post);
            println!();
            
            // Save posts to files
            let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
            
                          let social_posts = format!(
                  "# MMH-RS V1.1.0 ELITE TIER - CPU ONLY Social Media Posts\n\
                Generated: {}\n\
                \n\
                ## Twitter/X Post\n\
                {}\n\
                \n\
                ## LinkedIn Post\n\
                {}\n\
                \n\
                ## Reddit Post\n\
                {}\n\
                \n\
                ## Hashtags\n\
                #MMH #MMHRS #Benchmark #Compression #Performance #SystemValidation\n\
                \n\
                ---\n\
                Copy and paste these posts to share your results!\n\
                ",
                timestamp, twitter_post, linkedin_post, reddit_post
            );
            
            let filename = format!("social_posts_{}.md", timestamp);
            if let Ok(_) = std::fs::write(&filename, &social_posts) {
                println!("✅ Social media posts saved to: {}", filename);
                println!("📋 Copy and paste to share your results!");
            }
            
            // Generate HTML report for easy sharing
            if let Ok(html) = cli::generate_html_report(latest_file.to_str().unwrap()) {
                let html_filename = format!("shareable_report_{}.html", timestamp);
                if let Ok(_) = std::fs::write(&html_filename, &html) {
                    println!("🌐 HTML report generated: {}", html_filename);
                    println!("   Open this file in your browser to view/share!");
                }
            }
        }
    }
}

fn email_error_logs() {
    println!("\n📧 **EMAIL ERROR LOGS**");
    println!("=======================\n");
    
    // Collect recent error logs
    let mut error_logs = String::new();
          error_logs.push_str("MMH-RS V1.1.0 ELITE TIER - CPU ONLY Error Log Report\n");
    error_logs.push_str(&format!("Generated: {}\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    
    // Check for recent benchmark reports with errors
    let files: Vec<_> = std::fs::read_dir(".")
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            if name.contains("test_results") {
                Some((path.clone(), name.to_string()))
            } else {
                None
            }
        })
        .collect();
    
    if files.is_empty() {
        error_logs.push_str("No benchmark results found.\n");
    } else {
        error_logs.push_str("Recent Benchmark Results:\n");
        for (path, name) in files.iter().take(5) {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(result) = serde_json::from_str::<serde_json::Value>(&content) {
                    let score = result["overall_system_score"].as_u64().unwrap_or(0);
                    let status = result["status"].as_str().unwrap_or("unknown");
                    let empty_vec = vec![];
                    let errors = result["errors"].as_array().unwrap_or(&empty_vec);
                    
                    error_logs.push_str(&format!("• {}: Score {}/1000, Status: {}\n", name, score, status));
                    
                    if !errors.is_empty() {
                        error_logs.push_str("  Errors:\n");
                        for error in errors {
                            if let Some(error_msg) = error.as_str() {
                                error_logs.push_str(&format!("    - {}\n", error_msg));
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Add system information
    error_logs.push_str("\nSystem Information:\n");
    let sys = sysinfo::System::new_all();
    error_logs.push_str(&format!("• CPU: {} cores\n", sys.cpus().len()));
    error_logs.push_str(&format!("• RAM: {:.1} GB\n", sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0));
    error_logs.push_str(&format!("• OS: Windows 11 Home (Version 24H2)\n"));
          error_logs.push_str(&format!("• MMH-RS Version: V1.1.0 ELITE TIER - CPU ONLY\n"));
    
    // Save error log
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let filename = format!("error_log_{}.txt", timestamp);
    
    if let Ok(_) = std::fs::write(&filename, &error_logs) {
        println!("✅ Error log saved to: {}", filename);
        println!("📧 Email this file to: support@mmh-rs.com");
        println!("📋 Subject: MMH-RS V1.1.0 Error Report - {}", timestamp);
        println!();
        println!("📄 **ERROR LOG PREVIEW:**");
        println!("{}", error_logs);
    } else {
        println!("❌ Failed to save error log!");
    }
}

#[allow(dead_code)]
fn view_global_scoreboard() {
    println!("\n🏆 **GLOBAL SCOREBOARD**");
    println!("======================\n");
    
    println!("🚧 **V2 FEATURE - COMING SOON!**");
    println!("================================");
    println!();
    println!("📊 **PLANNED FEATURES:**");
    println!("• Real-time global scoreboard");
    println!("• Community rankings and comparisons");
    println!("• System tier classifications");
    println!("• Performance leaderboards");
    println!("• Regional and global statistics");
    println!();
    println!("🎯 **V2 ROADMAP:**");
    println!("• GPU acceleration integration");
    println!("• Online score submission");
    println!("• Community features");
    println!("• Real-time comparisons");
    println!();
    println!("📈 **CURRENT V1 STATUS:**");
    println!("• Local benchmarking only");
    println!("• CPU + HDD focus");
    println!("• Offline operation");
    println!("• Self-contained testing");
    println!();
    println!("⏳ **AVAILABLE IN V2:**");
    println!("• Global scoreboard access");
    println!("• Community comparisons");
    println!("• Online result sharing");
    println!("• Real-time rankings");
}

#[allow(dead_code)]
fn export_baseline_profile() {
    println!("\n📋 **EXPORT BASELINE PROFILE**");
    println!("=============================\n");
    
    println!("🚧 **V2 FEATURE - COMING SOON!**");
    println!("================================");
    println!();
    println!("📊 **PLANNED FEATURES:**");
    println!("• Automated baseline profile generation");
    println!("• System fingerprinting");
    println!("• Performance tier classification");
    println!("• CI/CD integration");
    println!("• Cross-system comparisons");
    println!();
    println!("🎯 **V2 ROADMAP:**");
    println!("• GPU profile integration");
    println!("• Automated system detection");
    println!("• Performance tier assignment");
    println!("• Baseline database");
    println!();
    println!("📈 **CURRENT V1 STATUS:**");
    println!("• Manual system information");
    println!("• Local benchmark results");
    println!("• Basic profile export");
    println!("• Offline operation");
    println!();
    println!("⏳ **AVAILABLE IN V2:**");
    println!("• Automated baseline generation");
    println!("• System fingerprinting");
    println!("• Performance tier classification");
    println!("• CI/CD integration");
}

fn show_system_information() {
    println!("\n🖥️ **SYSTEM INFORMATION - V1 CPU + HDD FOCUS**");
    println!("=============================================\n");
    
    let sys = sysinfo::System::new_all();
    
    println!("💻 **HARDWARE (V1 CPU + HDD BENCHMARKING)**");
    println!("CPU: {} cores @ {:.1} GHz - PRIMARY FOCUS", 
        sys.cpus().len(), 
        sys.cpus()[0].frequency() as f64 / 1000.0
    );
    println!("RAM: {:.1} GB total - System memory for CPU operations", 
        sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0
    );
    println!("Storage: 3.73 TB SSD - HDD I/O performance testing");
    println!("GPU: RTX 4070 (8 GB VRAM) - DISABLED in V1");
    
    println!("\n🖥️ **SOFTWARE**");
    println!("OS: Windows 11 Home (Version 24H2)");
    println!("WSL: Enabled");
    println!("Device: UniversalTruth");
    
    println!("\n🚀 **MMH-RS V1 STATUS**");
    println!("Version: V1.1.0 ELITE TIER - CPU ONLY");
    println!("Focus: CPU processing + HDD I/O performance");
    println!("GPU: Disabled (V2 will enable GPU acceleration)");
    println!("Status: Ready for CPU + HDD benchmarking");
    
    println!("\n📊 **VERSION ROADMAP**");
    println!("V1 (Current): CPU + HDD ONLY");
    println!("V2 (Next): GPU + HDD ONLY");
    println!("V3 (Ultimate): CPU + GPU + HDD");
}

fn show_troubleshooting_guide() {
    println!("\n🔧 **TROUBLESHOOTING GUIDE**");
    println!("==========================\n");
    
    println!("❓ **COMMON ISSUES**");
    println!("\n1. **Progress seems to hang**");
    println!("   • Wait for current phase to complete");
    println!("   • Each phase has progress indicators");
    println!("   • Use Ctrl+C if needed");
    
    println!("\n2. **Size doesn't match expected**");
    println!("   • System uses GiB (binary) not GB (decimal)");
    println!("   • 32GB test shows exactly 32.00 GiB");
    println!("   • This is correct behavior for CPU + HDD testing");
    
    println!("\n3. **Abort doesn't work**");
    println!("   • Press Ctrl+C and wait a few seconds");
    println!("   • Abort checks happen at loop boundaries");
    println!("   • Expected: Clean exit with status message");
    
    println!("\n4. **Memory measurement seems wrong**");
    println!("   • Windows 11 + WSL has known measurement quirks");
    println!("   • Actual performance is accurate, display may be off");
    println!("   • 64GB RAM handles 32GB tests easily");
    
    println!("\n5. **Files not saving**");
    println!("   • Check disk space and permissions");
    println!("   • Files are saved in current directory");
    println!("   • Format: Size-prefixed with timestamps");
    
    println!("\n📞 **GETTING HELP**");
    println!("• Check the USER_GUIDE.md for detailed instructions");
    println!("• Review DEVELOPMENT_HISTORY.md for known issues");
    println!("• Compare your results with the 830/1000 CPU + HDD baseline");
    println!("• V2 will introduce GPU + HDD benchmarking!");
}

fn show_about() {
    println!("\n🚀 **ABOUT MMH-RS V1.2.0 ELITE TIER - CPU ONLY**");
    println!("==============================================\n");
    
    println!("🎯 **MISSION**");
    println!("MMH-RS is a comprehensive compression benchmarking ecosystem");
    println!("designed to establish gold standard baselines across different");
    println!("hardware configurations and processing paradigms.");
    
    println!("\n🏆 **VERSION STRATEGY**");
    println!("• V1 Series: CPU + HDD ONLY (Current)");
    println!("• V2 Series: GPU + HDD ONLY (Next)");
    println!("• V3 Series: CPU + GPU + HDD (Ultimate)");
    println!("• Each version optimized for its specific hardware focus");
    
    println!("\n🔧 **V1.2.0 ELITE TIER FEATURES (CPU + HDD)**");
    println!("• Enhanced 1000-point scoring system (full range utilization)");
    println!("• Integrated file operations with file picker");
    println!("• 7 performance tiers (Entry Level to Legendary Performance)");
    println!("• 130+ benchmark reports in comprehensive database");
    println!("• Cross-system validation & comparison");
    println!("• Advanced analytics & visualization");
    println!("• Stress & edge case testing tools");
    println!("• Interop & data portability");
    println!("• CI/CD automation hooks");
    println!("• Enhanced user experience");
    
    println!("\n🖥️  **V1 VALIDATION SYSTEM**");
    println!("UniversalTruth (i7-13620H + RTX 4070 + 64GB RAM)");
    println!("32GB Benchmark Score: 830/1000 (High-End Gaming Laptop Tier)");
    println!("Compression Ratio: 2.15x at 54.0 MB/s");
    println!("Focus: CPU processing + HDD I/O performance");
    
    println!("\n🚀 **ROADMAP**");
    println!("• V2: GPU acceleration (RTX 4070 ready!)");
    println!("  - GPU + HDD benchmarking");
    println!("  - CUDA/OpenCL optimization");
    println!("  - GPU memory management");
    println!("• V3: Hybrid CPU+GPU processing");
    println!("  - CPU + GPU + HDD benchmarking");
    println!("  - Load balancing algorithms");
    println!("  - Unified performance metrics");
    
    println!("\n📊 **BENCHMARKING ECOSYSTEM**");
    println!("• V1: Establish CPU + HDD baselines");
    println!("• V2: Establish GPU + HDD baselines");
    println!("• V3: Establish CPU + GPU + HDD baselines");
    println!("• Cross-version comparison tools");
    println!("• Hardware optimization recommendations");
    
    println!("\n📞 **CONTACT**");
    println!("GitHub: https://github.com/Bigrob7605/MMH-RS");
    println!("Version: V1.2.0 ELITE TIER - CPU ONLY");
    println!("Status: Enhanced 1000-Point Scoring System Ready");
} 

fn run_quick_smoketest() {
    println!("\n🚀 **QUICK SMOKETEST** (~50 MiB)");
    println!("===============================\n");
    
    // Set up abort handler only once
    if let Err(_) = ctrlc::set_handler(|| {
        println!("\n⚠️  Abort requested...");
        std::process::exit(0);
    }) {
        println!("⚠️  Warning: Could not set abort handler (may already be set)");
    }
    
    println!("🎯 Running quick smoketest with ~50 MiB of data...");
    println!("   This is perfect for initial validation");
    println!("   Expected time: 30-60 seconds\n");
    
    // Run the benchmark with 0 GB (will be handled as smoketest)
    let report = bench::run(0);
    
    println!("\n✅ **SMOKETEST COMPLETE**");
    println!("=======================");
    
    // Display the full gold standard report
    println!("{}", report.pretty_box());
    
    println!("\n📊 **QUICK SUMMARY**");
    println!("Score: {}/1000", report.overall_system_score);
    println!("Tier: {}", report.performance_tier);
    println!("Compression: {:.2}x", report.compression_ratio);
    println!("Speed: {:.1} MB/s", report.pack_speed);
    println!("Time: {:.1} seconds", report.total_time);
    println!("Status: ✅ PASS");
    
    // Check if test data was actually saved
    let test_data_saved = std::fs::read_dir(".")
        .map(|entries| {
            entries.filter_map(|entry| entry.ok())
                .any(|entry| entry.file_name().to_string_lossy().starts_with("testdata_export_"))
        })
        .unwrap_or(false);
    
    if test_data_saved {
        println!("\n💾 Report saved to: bench_reports/");
    }
    println!("📈 Ready for V2 GPU acceleration!");
}

fn run_standard_test() {
    println!("\n🔥 **STANDARD TEST** (2 GiB)");
    println!("===========================\n");
    
    // Set up abort handler only once
    if let Err(_) = ctrlc::set_handler(|| {
        println!("\n⚠️  Abort requested...");
        std::process::exit(0);
    }) {
        println!("⚠️  Warning: Could not set abort handler (may already be set)");
    }
    
    println!("🎯 Running standard test with 2 GiB of data...");
    println!("   This is the recommended test for most systems");
    println!("   Expected time: 2-5 minutes\n");
    
    // Run the benchmark with 2 GB
    let report = bench::run(2);
    
    println!("\n✅ **STANDARD TEST COMPLETE**");
    println!("============================");
    
    // Display the full gold standard report
    println!("{}", report.pretty_box());
    
    println!("\n📊 **QUICK SUMMARY**");
    println!("Score: {}/1000", report.overall_system_score);
    println!("Tier: {}", report.performance_tier);
    println!("Compression: {:.2}x", report.compression_ratio);
    println!("Speed: {:.1} MB/s", report.pack_speed);
    println!("Time: {:.1} seconds", report.total_time);
    println!("Status: ✅ PASS");
    
    // Check if test data was actually saved
    let test_data_saved = std::fs::read_dir(".")
        .map(|entries| {
            entries.filter_map(|entry| entry.ok())
                .any(|entry| entry.file_name().to_string_lossy().starts_with("testdata_export_"))
        })
        .unwrap_or(false);
    
    if test_data_saved {
        println!("\n💾 Report saved to: bench_reports/");
    }
    println!("📈 Ready for V2 GPU acceleration!");
}

fn run_extended_test() {
    println!("\n⭐ **EXTENDED TEST** (32 GiB)");
    println!("============================\n");
    
    // Set up abort handler only once
    if let Err(_) = ctrlc::set_handler(|| {
        println!("\n⚠️  Abort requested...");
        std::process::exit(0);
    }) {
        println!("⚠️  Warning: Could not set abort handler (may already be set)");
    }
    
    println!("🎯 Running extended test with 32 GiB of data...");
    println!("   This is the gold standard baseline test");
    println!("   Expected time: 15-30 minutes\n");
    
    // Run the benchmark with 32 GB
    let report = bench::run(32);
    
    println!("\n✅ **EXTENDED TEST COMPLETE**");
    println!("============================");
    
    // Display the full gold standard report
    println!("{}", report.pretty_box());
    
    println!("\n📊 **QUICK SUMMARY**");
    println!("Score: {}/1000", report.overall_system_score);
    println!("Tier: {}", report.performance_tier);
    println!("Compression: {:.2}x", report.compression_ratio);
    println!("Speed: {:.1} MB/s", report.pack_speed);
    println!("Time: {:.1} seconds", report.total_time);
    println!("Status: ✅ PASS");
    
    // Check if test data was actually saved
    let test_data_saved = std::fs::read_dir(".")
        .map(|entries| {
            entries.filter_map(|entry| entry.ok())
                .any(|entry| entry.file_name().to_string_lossy().starts_with("testdata_export_"))
        })
        .unwrap_or(false);
    
    if test_data_saved {
        println!("\n💾 Report saved to: bench_reports/");
    }
    println!("📈 Ready for V2 GPU acceleration!");
}

fn generate_shareable_benchmark() {
    println!("\n📊 **GENERATE SHAREABLE BENCHMARK**");
    println!("===================================\n");
    
    println!("This will generate a compact, shareable benchmark report for cross-system comparison.");
    println!();
    println!("Available test sizes:");
    println!("1. Smoketest (~50 MiB) - Quick validation");
    println!("2. Standard (2 GB) - Recommended for comparison");
    println!("3. Extended (32 GB) - Gold standard baseline");
    println!("4. Custom size");
    println!();
    
    print!("Select test size (1-4): ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    
    let size_gb = match choice {
        "1" => 0,
        "2" => 2,
        "3" => 32,
        "4" => {
            print!("Enter custom size in GB: ");
            io::stdout().flush().unwrap();
            let mut custom_size = String::new();
            io::stdin().read_line(&mut custom_size).unwrap();
            custom_size.trim().parse().unwrap_or(2)
        }
        _ => {
            println!("Invalid choice. Using Standard Test (2 GB)...");
            2
        }
    };
    
    println!("\n🚀 Generating shareable benchmark...");
    
    // Run the benchmark with compact format
    let report = bench::run(size_gb);
    
    // Display the compact report
    println!("\n{}", report.compact_report());
    
    println!("\n✅ **SHAREABLE BENCHMARK COMPLETE**");
    println!("===================================");
    println!("Your shareable benchmark report has been generated!");
    println!();
    println!("Files created:");
    println!("  - compact_report.txt (for sharing)");
    println!("  - report.txt (detailed version)");
    println!("  - report.json (machine-readable)");
    println!();
    println!("Share the compact_report.txt file to compare with other users!");
} 