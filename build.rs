use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get git commit hash
    let git_sha = get_git_sha();
    let build_time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    let rust_version = env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string());

    // Create version info
    let version_info = format!(
        r#"// Auto-generated build information
pub const BUILD_TIME: &str = "{}";
pub const GIT_SHA: &str = "{}";
pub const TARGET: &str = "{}";
pub const RUST_VERSION: &str = "{}";
"#,
        build_time, git_sha, target, rust_version
    );

    // Write to out directory
    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(Path::new(&out_dir).join("version.rs"), version_info).unwrap();

    // Set environment variables for the build
    println!("cargo:rustc-env=VERGEN_BUILD_TIMESTAMP={}", build_time);
    println!("cargo:rustc-env=VERGEN_GIT_SHA_SHORT={}", git_sha);
    println!("cargo:rustc-env=TARGET={}", target);
    println!("cargo:rustc-env=VERGEN_RUSTC_SEMVER={}", rust_version);

    // Re-run if git changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
}

fn get_git_sha() -> String {
    use std::process::Command;
    
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "unknown".to_string()
            }
        }
        Err(_) => "unknown".to_string()
    }
} 