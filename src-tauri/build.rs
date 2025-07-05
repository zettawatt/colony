use std::process::Command;
use std::path::Path;

fn main() {
    // Run fetch_binaries.sh before building
    fetch_binaries();

    tauri_build::build()
}

fn fetch_binaries() {
    let binaries_dir = Path::new("binaries");
    let script_path = binaries_dir.join("fetch_binaries.sh");

    if !script_path.exists() {
        println!("cargo:warning=fetch_binaries.sh not found at {:?}", script_path);
        return;
    }

    println!("cargo:rerun-if-changed=binaries/fetch_binaries.sh");

    // Change to the binaries directory and run the script
    let output = Command::new("bash")
        .arg("fetch_binaries.sh")
        .current_dir(binaries_dir)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("cargo:warning=Successfully fetched binaries");
                // Print stdout for visibility
                if !output.stdout.is_empty() {
                    println!("cargo:warning={}", String::from_utf8_lossy(&output.stdout));
                }
            } else {
                println!("cargo:warning=Failed to fetch binaries");
                if !output.stderr.is_empty() {
                    println!("cargo:warning=Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
        Err(e) => {
            println!("cargo:warning=Failed to execute fetch_binaries.sh: {}", e);
        }
    }
}
