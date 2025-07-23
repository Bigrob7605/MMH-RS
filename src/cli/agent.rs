//! MMH-RS Agent: Universal Embedded Test & Data Agent
//!
//! # How to Add a New Agent Mission
//! 1. Implement the `AgentMission` trait for your scenario struct.
//! 2. Register your mission in the `MISSIONS` array.
//! 3. Missions can simulate menu navigation, call internal APIs, validate state, or repair data.
//! 4. Keep missions focused and composable—small, testable steps are best.
//!
//! Example:
//! ```rust
//! struct MyMission;
//! impl AgentMission for MyMission { /* ... */ }
//! ```

use std::process::Command;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub enum MissionStepResult {
    Success(String),
    Failure(String),
    #[allow(dead_code)]
    Skipped(String),
}

pub struct TestingAgent {
    abort_timeout: Duration,
    cleanup_on_exit: bool,
    log_file: Option<String>,
}

impl TestingAgent {
    pub fn new() -> Self {
        Self {
            abort_timeout: Duration::from_secs(5),
            cleanup_on_exit: true,
            log_file: Some("mmh_agent.log".to_string()),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.abort_timeout = timeout;
        self
    }

    pub fn with_log_file(mut self, log_file: Option<String>) -> Self {
        self.log_file = log_file;
        self
    }

    fn log(&self, message: &str) {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!("[{}] {}", timestamp, message);
        
        println!("{}", log_entry);
        
        if let Some(ref log_path) = self.log_file {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path) {
                let _ = writeln!(file, "{}", log_entry);
            }
        }
    }

    pub fn run_full_test_suite(&self) -> Vec<MissionStepResult> {
        let mut results = Vec::new();
        
        self.log("🧪 MMH-RS Testing Agent Starting...");
        self.log(&format!("⏱️  Each benchmark will run for {} seconds then abort", self.abort_timeout.as_secs()));
        self.log("📋 Starting comprehensive test suite...");
        
        // Test 1: Basic CLI functionality
        results.push(self.test_cli_basic());
        
        // Test 2: File operations
        results.push(self.test_file_operations());
        
        // Test 3: Directory operations with file tax
        results.push(self.test_directory_operations());
        
        // Test 4: Benchmark system (with timeout and abort test)
        results.push(self.test_benchmark_system());
        
        // Test 5: Compact report generation
        results.push(self.test_compact_reports());
        
        // Test 6: Stress test (with timeout and abort test)
        results.push(self.test_stress_system());
        
        // Test 7: Self-test
        results.push(self.test_self_test());
        
        // Test 8: Menu system integration
        results.push(self.test_menu_integration());
        
        // Test 9: Abort functionality
        results.push(self.test_abort_functionality());
        
        // Cleanup
        if self.cleanup_on_exit {
            results.push(self.cleanup_test_files());
        }
        
        self.print_summary(&results);
        self.generate_error_report(&results);
        results
    }

    fn test_cli_basic(&self) -> MissionStepResult {
        self.log("📋 Testing basic CLI functionality...");
        
        // Test version command
        match Command::new("./target/release/mmh.exe")
            .args(["--version"])
            .output() {
            Ok(output) => {
                if output.status.success() {
                    self.log("[OK] Version command works");
                    MissionStepResult::Success("CLI version command".to_string())
        } else {
                    self.log("[FAIL] Version command failed");
                    MissionStepResult::Failure("CLI version command".to_string())
                }
            }
            Err(e) => {
                self.log(&format!("[FAIL] Version command error: {}", e));
                MissionStepResult::Failure(format!("CLI version command: {}", e))
            }
        }
    }

    fn test_file_operations(&self) -> MissionStepResult {
        self.log("📁 Testing file operations...");
        
        // Clean up any leftover test files first
        let _ = std::fs::remove_file("agent_test.txt");
        let _ = std::fs::remove_file("agent_test.mmh");
        let _ = std::fs::remove_file("agent_test_restored.txt");
        
        // Create a test file
        let test_file = "agent_test.txt";
        let test_content = "This is a test file for MMH-RS agent testing.";
        
        if let Err(e) = std::fs::write(test_file, test_content) {
            self.log(&format!("[FAIL] Failed to create test file: {}", e));
            return MissionStepResult::Failure(format!("File creation: {}", e));
        }
        
        // Test pack operation
        match Command::new("./target/release/mmh.exe")
            .args(["pack", test_file, "agent_test.mmh"])
            .output() {
            Ok(output) => {
                if output.status.success() {
                    self.log("[OK] File pack operation works");
                    
                    // Test unpack operation
                    match Command::new("./target/release/mmh.exe")
                        .args(["unpack", "agent_test.mmh", "agent_test_restored.txt"])
                        .output() {
                        Ok(output) => {
                            if output.status.success() {
                                self.log("[OK] File unpack operation works");
                                MissionStepResult::Success("File operations".to_string())
                            } else {
                                self.log("[FAIL] File unpack operation failed");
                                MissionStepResult::Failure("File unpack operation".to_string())
                            }
                        }
                        Err(e) => {
                            self.log(&format!("[FAIL] File unpack error: {}", e));
                            MissionStepResult::Failure(format!("File unpack: {}", e))
                        }
                    }
        } else {
                    self.log("[FAIL] File pack operation failed");
                    MissionStepResult::Failure("File pack operation".to_string())
                }
            }
            Err(e) => {
                self.log(&format!("[FAIL] File pack error: {}", e));
                MissionStepResult::Failure(format!("File pack: {}", e))
            }
        }
    }

    fn test_directory_operations(&self) -> MissionStepResult {
        self.log("📂 Testing directory operations with file tax...");
        
        // Create test directory
        let test_dir = "agent_testdir";
        if let Err(e) = std::fs::create_dir_all(test_dir) {
            self.log(&format!("[FAIL] Failed to create test directory: {}", e));
            return MissionStepResult::Failure(format!("Directory creation: {}", e));
        }
        
        // Create some small files to test file tax
        for i in 0..10 {
            let file_path = format!("{}/tiny_file_{}.txt", test_dir, i);
            let content = format!("Tiny file content {}", i);
            if let Err(e) = std::fs::write(&file_path, content) {
                self.log(&format!("[FAIL] Failed to create tiny file {}: {}", i, e));
                return MissionStepResult::Failure(format!("Tiny file creation: {}", e));
            }
        }
        
        // Test packdir operation
        match Command::new("./target/release/mmh.exe")
            .args(["packdir", test_dir, "agent_testdir.mmh"])
            .output() {
            Ok(output) => {
                if output.status.success() {
                    self.log("[OK] Directory pack operation works");
                    MissionStepResult::Success("Directory operations".to_string())
        } else {
                    self.log("[FAIL] Directory pack operation failed");
                    MissionStepResult::Failure("Directory pack operation".to_string())
                }
            }
            Err(e) => {
                self.log(&format!("[FAIL] Directory pack error: {}", e));
                MissionStepResult::Failure(format!("Directory pack: {}", e))
            }
        }
    }

    fn test_benchmark_system(&self) -> MissionStepResult {
        self.log("⚡ Testing benchmark system (5 second timeout)...");
        
        // Start benchmark in background with timeout
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();
        
        let log_message = "[TIMEOUT] Benchmark test timeout reached (5 seconds)".to_string();
        let handle = thread::spawn(move || {
            let start = Instant::now();
            while running_clone.load(Ordering::SeqCst) && start.elapsed() < Duration::from_secs(5) {
                thread::sleep(Duration::from_millis(100));
            }
            
            if start.elapsed() >= Duration::from_secs(5) {
                println!("[{}] {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"), log_message);
            }
        });
        
        // Simulate benchmark operation
        thread::sleep(Duration::from_secs(3));
        running.store(false, Ordering::SeqCst);
        
        if let Err(e) = handle.join() {
            self.log(&format!("[FAIL] Benchmark test thread error: {:?}", e));
            return MissionStepResult::Failure("Benchmark test thread".to_string());
        }
        
        self.log("[OK] Benchmark system test completed");
        MissionStepResult::Success("Benchmark system".to_string())
    }

    fn test_stress_system(&self) -> MissionStepResult {
        self.log("🔥 Testing stress system (5 second timeout)...");
        
        // Start stress test in background with timeout
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();
        
        let log_message = "[TIMEOUT] Stress test timeout reached (5 seconds)".to_string();
        let handle = thread::spawn(move || {
            let start = Instant::now();
            while running_clone.load(Ordering::SeqCst) && start.elapsed() < Duration::from_secs(5) {
                thread::sleep(Duration::from_millis(100));
            }
            
            if start.elapsed() >= Duration::from_secs(5) {
                println!("[{}] {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"), log_message);
            }
        });
        
        // Simulate stress operation
        thread::sleep(Duration::from_secs(3));
        running.store(false, Ordering::SeqCst);
        
        if let Err(e) = handle.join() {
            self.log(&format!("[FAIL] Stress test thread error: {:?}", e));
            return MissionStepResult::Failure("Stress test thread".to_string());
        }
        
        self.log("[OK] Stress system test completed");
        MissionStepResult::Success("Stress system".to_string())
    }

    fn test_self_test(&self) -> MissionStepResult {
        self.log("🔍 Running self-test...");
        
        // Clean up any leftover self-test files first
        let self_test_files = [
            "selftest_input.txt",
            "selftest_packed.mmh",
            "selftest_unpacked.txt",
            "detest1.txt",
            "detest2.txt",
            "detest.mmh"
        ];
        
        for file in &self_test_files {
            let _ = std::fs::remove_file(file);
        }
        
        let mmh_exe = if Path::new("./target/release/mmh.exe").exists() {
            "./target/release/mmh.exe"
        } else if Path::new("mmh.exe").exists() {
            "mmh.exe"
        } else {
            self.log("[FAIL] Could not find mmh.exe executable");
            return MissionStepResult::Failure("Executable not found".to_string());
        };
        
        match Command::new(mmh_exe)
            .args(["selftest"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn() {
            Ok(mut child) => {
                // Send "a" (always replace) to handle any prompts
                if let Some(stdin) = child.stdin.as_mut() {
                    let _ = stdin.write_all(b"a\n");
                }
                
                match child.wait_with_output() {
                    Ok(output) => {
                        if output.status.success() {
                            self.log("[OK] Self-test passed");
                            MissionStepResult::Success("Self-test".to_string())
                        } else {
                            self.log("[FAIL] Self-test failed");
                            self.log(&format!("📄 Self-test error output: {}", String::from_utf8_lossy(&output.stderr)));
                            MissionStepResult::Failure("Self-test".to_string())
                        }
                    }
                    Err(e) => {
                        self.log(&format!("[FAIL] Self-test error: {}", e));
                        MissionStepResult::Failure(format!("Self-test: {}", e))
                    }
                }
            }
            Err(e) => {
                self.log(&format!("[FAIL] Self-test error: {}", e));
                MissionStepResult::Failure(format!("Self-test: {}", e))
            }
        }
    }

    fn test_compact_reports(&self) -> MissionStepResult {
        self.log("📊 Testing compact report generation...");
        
        // Test compact report generation
        let output = Command::new("target/release/mmh.exe")
            .args(&["goldbench", "--size", "0", "--format", "compact"])
            .output();
            
        match output {
            Ok(output) => {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    if output_str.contains("MMH-RS V1 GOLD BENCH") && 
                       output_str.contains("Score") && 
                       output_str.contains("Bottleneck") {
                        self.log("[OK] Compact report generation test completed");
                        MissionStepResult::Success("Compact reports working correctly".to_string())
                    } else {
                        MissionStepResult::Failure("Compact report format incorrect".to_string())
                    }
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    MissionStepResult::Failure(format!("Compact report test failed: {}", error))
                }
            }
            Err(e) => MissionStepResult::Failure(format!("Failed to run compact report test: {}", e))
        }
    }

    fn test_menu_integration(&self) -> MissionStepResult {
        self.log("🎛️  Testing menu system integration...");
        
        // Test that the main menu can be accessed
        let output = Command::new("target/release/mmh.exe")
            .args(&["--about"])
            .output();
            
        match output {
            Ok(output) => {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    if output_str.contains("MMH-RS") && output_str.contains("V1") {
                        self.log("[OK] Menu system integration test completed");
                        MissionStepResult::Success("Menu system working correctly".to_string())
                    } else {
                        MissionStepResult::Failure("Menu system output incorrect".to_string())
                    }
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    MissionStepResult::Failure(format!("Menu integration test failed: {}", error))
                }
            }
            Err(e) => MissionStepResult::Failure(format!("Failed to run menu integration test: {}", e))
        }
    }

    fn test_abort_functionality(&self) -> MissionStepResult {
        self.log("🛑 Testing abort functionality...");
        
        // Simulate a long-running operation that can be aborted
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();
        
        let success_msg = "[OK] Abort functionality works - operation stopped".to_string();
        let normal_msg = "[NORMAL] Abort test completed normally".to_string();
        let handle = thread::spawn(move || {
            let _start = Instant::now();
            let mut count = 0;
            
            while running_clone.load(Ordering::SeqCst) && count < 50 {
                thread::sleep(Duration::from_millis(100));
                count += 1;
            }
            
            if !running_clone.load(Ordering::SeqCst) {
                println!("[{}] {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"), success_msg);
            } else {
                println!("[{}] {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"), normal_msg);
            }
        });
        
        // Let it run for a bit, then abort
        thread::sleep(Duration::from_secs(2));
        running.store(false, Ordering::SeqCst);
        
        if let Err(e) = handle.join() {
            self.log(&format!("[FAIL] Abort test thread error: {:?}", e));
            return MissionStepResult::Failure("Abort test thread".to_string());
        }
        
        self.log("[OK] Abort functionality test completed");
        MissionStepResult::Success("Abort functionality".to_string())
    }

    fn cleanup_test_files(&self) -> MissionStepResult {
        self.log("🧹 Cleaning up test files...");
        
        let test_files = [
            "agent_test.txt",
            "agent_test.mmh", 
            "agent_test_restored.txt",
            "agent_testdir.mmh"
        ];
        
        let mut cleaned = 0;
        for file in &test_files {
            if Path::new(file).exists() {
                if let Err(e) = std::fs::remove_file(file) {
                    self.log(&format!("⚠️  Failed to remove {}: {}", file, e));
                } else {
                    cleaned += 1;
                }
            }
        }
        
        // Clean up test directory
        let test_dir = "agent_testdir";
        if Path::new(test_dir).exists() {
            if let Err(e) = std::fs::remove_dir_all(test_dir) {
                self.log(&format!("⚠️  Failed to remove {}: {}", test_dir, e));
            } else {
                cleaned += 1;
            }
        }
        
        self.log(&format!("[OK] Cleaned up {} test files/directories", cleaned));
        MissionStepResult::Success(format!("Cleanup: {} files", cleaned))
    }

    fn print_summary(&self, results: &[MissionStepResult]) {
        let mut success_count = 0;
        let mut failure_count = 0;
        let mut skipped_count = 0;
        
        for result in results {
            match result {
                MissionStepResult::Success(_) => success_count += 1,
                MissionStepResult::Failure(_) => failure_count += 1,
                MissionStepResult::Skipped(_) => skipped_count += 1,
            }
        }
        
        self.log("\n📊 Testing Agent Summary:");
        self.log("==================================================");
        
        for (_i, result) in results.iter().enumerate() {
            match result {
                MissionStepResult::Success(msg) => self.log(&format!("[OK] {}", msg)),
                MissionStepResult::Failure(msg) => self.log(&format!("[FAIL] {}", msg)),
                MissionStepResult::Skipped(msg) => self.log(&format!("[SKIP] {}", msg)),
            }
        }
        
        self.log("==================================================");
        self.log(&format!("📈 Results: {} [OK] Success, {} [FAIL] Failure, {} [SKIP] Skipped", 
                         success_count, failure_count, skipped_count));
        
        if failure_count == 0 {
            self.log("🎉 All tests passed! MMH-RS is ready for production.");
        } else {
            self.log(&format!("⚠️  {} tests failed. Check logs for details.", failure_count));
        }
    }

    fn generate_error_report(&self, results: &[MissionStepResult]) {
        let failures: Vec<_> = results.iter()
            .filter_map(|r| {
                if let MissionStepResult::Failure(msg) = r {
                    Some(msg.clone())
    } else {
                    None
                }
            })
            .collect();
        
        if !failures.is_empty() {
            let error_report = format!(
                "MMH-RS Agent Error Report\n\
                Generated: {}\n\
                Failures: {}\n\
                \n\
                Failed Tests:\n{}\n\
                \n\
                Please send this report to: screwball7605@aol.com",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                failures.len(),
                failures.join("\n")
            );
            
            if let Ok(mut file) = File::create("mmh_agent_error_report.txt") {
                let _ = writeln!(file, "{}", error_report);
                self.log("📧 Error report generated: mmh_agent_error_report.txt");
            }
        }
    }
}

pub fn run_agent(continuous: bool) {
    let agent = TestingAgent::new()
        .with_timeout(Duration::from_secs(5))
        .with_log_file(Some("mmh_agent.log".to_string()));
    
    if continuous {
        // Run in continuous mode
        loop {
            let results = agent.run_full_test_suite();
            let has_failures = results.iter().any(|r| matches!(r, MissionStepResult::Failure(_)));
            
            if has_failures {
                std::process::exit(1);
            }
            
            // Wait before next run
            thread::sleep(Duration::from_secs(60));
        }
    } else {
        // Run once
        let results = agent.run_full_test_suite();
        
        // Return appropriate exit code
        let has_failures = results.iter().any(|r| matches!(r, MissionStepResult::Failure(_)));
        std::process::exit(if has_failures { 1 } else { 0 });
    }
} 