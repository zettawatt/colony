use std::fs;
use std::path::Path;

fn main() {
    // Fetch dweb binaries before building
    fetch_binaries();

    tauri_build::build()
}

fn fetch_binaries() {
    // Use a directory outside src-tauri to avoid triggering file watcher
    let binaries_dir = Path::new("../target/binaries");

    println!("cargo:info=Checking for dweb binaries...");

    // Create binaries directory if it doesn't exist
    if !binaries_dir.exists() {
        if let Err(e) = fs::create_dir_all(binaries_dir) {
            println!("cargo:warning=Failed to create binaries directory: {e}");
            return;
        }
    }

    // Define the binaries we need
    let binaries = [
        ("dweb-x86_64-unknown-linux-gnu", "dweb-linux-amd64"),
        ("dweb-x86_64-pc-windows-msvc.exe", "dweb-amd64.exe"),
        ("dweb-aarch64-apple-darwin", "dweb-darwin-arm64"),
        ("dweb-x86_64-apple-darwin", "dweb-darwin-amd64"),
    ];

    // Check if all binaries already exist
    let all_exist = binaries
        .iter()
        .all(|(local_name, _)| binaries_dir.join(local_name).exists());

    if all_exist {
        println!("cargo:info=Binaries already exist, skipping download");
        return;
    }

    println!("cargo:info=Fetching dweb binaries...");

    // Get the latest release tag
    let latest_tag = match get_latest_release_tag() {
        Ok(tag) => tag,
        Err(e) => {
            println!("cargo:warning=Failed to get latest release tag: {e}");
            return;
        }
    };

    println!("cargo:info=Latest dweb release: {latest_tag}");

    // Remove existing dweb binaries
    if let Ok(entries) = fs::read_dir(binaries_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("dweb-") {
                    let _ = fs::remove_file(entry.path());
                }
            }
        }
    }

    // Download each binary
    for (local_name, remote_name) in &binaries {
        let url = format!(
            "https://github.com/happybeing/dweb/releases/download/{latest_tag}/{remote_name}"
        );
        let local_path = binaries_dir.join(local_name);

        match download_file(&url, &local_path) {
            Ok(_) => {
                println!("cargo:info=Downloaded {local_name}");

                // Make executable on Unix systems
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = fs::metadata(&local_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755);
                        let _ = fs::set_permissions(&local_path, perms);
                    }
                }
            }
            Err(e) => {
                println!("cargo:warning=Failed to download {local_name}: {e}");
            }
        }
    }

    println!("cargo:info=Binary fetch completed");

    // Copy the appropriate binary for the current platform to the expected location
    copy_platform_binary(binaries_dir);
}

fn copy_platform_binary(binaries_dir: &Path) {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let source_binary = match (target_os.as_str(), target_arch.as_str()) {
        ("linux", "x86_64") => "dweb-x86_64-unknown-linux-gnu",
        ("windows", "x86_64") => "dweb-x86_64-pc-windows-msvc.exe",
        ("macos", "aarch64") => "dweb-aarch64-apple-darwin",
        ("macos", "x86_64") => "dweb-x86_64-apple-darwin",
        _ => {
            println!("cargo:warning=Unsupported platform: {target_os}-{target_arch}");
            return;
        }
    };

    let source_path = binaries_dir.join(source_binary);
    let dest_path = binaries_dir.join("dweb");

    if source_path.exists() {
        if let Err(e) = fs::copy(&source_path, &dest_path) {
            println!(
                "cargo:warning=Failed to copy binary from {} to {}: {e}",
                source_path.display(),
                dest_path.display()
            );
        } else {
            println!("cargo:info=Copied {source_binary} to dweb");
        }
    } else {
        println!(
            "cargo:warning=Source binary not found: {}",
            source_path.display()
        );
    }
}

fn get_latest_release_tag() -> Result<String, Box<dyn std::error::Error>> {
    const FALLBACK_VERSION: &str = "v0.10.8"; // Latest known working version as of 2025-07-15
    const MAX_RETRIES: u32 = 4;
    const RETRY_DELAY_SECONDS: u64 = 30;

    let url = "https://api.github.com/repos/happybeing/dweb/releases/latest";

    for attempt in 1..=MAX_RETRIES {
        match try_get_release_tag(url) {
            Ok(tag) => return Ok(tag),
            Err(e) => {
                // Check if it's a 403 (rate limit) error
                if let Some(ureq::Error::Status(403, _)) = e.downcast_ref::<ureq::Error>() {
                    println!("cargo:warning=GitHub API rate limit hit (attempt {attempt}/{MAX_RETRIES}), waiting {RETRY_DELAY_SECONDS} seconds...");

                    if attempt < MAX_RETRIES {
                        std::thread::sleep(std::time::Duration::from_secs(RETRY_DELAY_SECONDS));
                        continue;
                    } else {
                        println!("cargo:warning=Max retries reached, falling back to hardcoded version: {FALLBACK_VERSION}");
                        return Ok(FALLBACK_VERSION.to_string());
                    }
                }

                // For non-403 errors, fail immediately
                println!("cargo:warning=Failed to get release tag (attempt {attempt}): {e}");
                if attempt == 1 {
                    // If it's not a rate limit error, don't retry
                    return Err(e);
                }
            }
        }
    }

    // This should never be reached, but just in case
    Ok(FALLBACK_VERSION.to_string())
}

#[allow(dead_code)]
fn try_get_release_tag(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = ureq::get(url).call()?;
    let json: serde_json::Value = response.into_json()?;

    json["tag_name"]
        .as_str()
        .ok_or("tag_name not found in response".into())
        .map(|s| s.to_string())
}

fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    const MAX_RETRIES: u32 = 3;
    const RETRY_DELAY_SECONDS: u64 = 10;

    for attempt in 1..=MAX_RETRIES {
        match try_download_file(url, path) {
            Ok(_) => return Ok(()),
            Err(e) => {
                // Check if it's a 403 (rate limit) error
                if let Some(ureq::Error::Status(403, _)) = e.downcast_ref::<ureq::Error>() {
                    println!("cargo:warning=Rate limit hit downloading {url} (attempt {attempt}/{MAX_RETRIES}), waiting {RETRY_DELAY_SECONDS} seconds...");

                    if attempt < MAX_RETRIES {
                        std::thread::sleep(std::time::Duration::from_secs(RETRY_DELAY_SECONDS));
                        continue;
                    }
                }

                // For non-403 errors or final attempt, return the error
                if attempt == MAX_RETRIES {
                    return Err(e);
                }
            }
        }
    }

    // This should never be reached
    Err("Max retries exceeded".into())
}

fn try_download_file(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = ureq::get(url).call()?;
    let mut file = fs::File::create(path)?;
    std::io::copy(&mut response.into_reader(), &mut file)?;
    Ok(())
}
