use std::fs;
use std::path::Path;

fn main() {
    // Fetch dweb binaries before building
    fetch_binaries();

    tauri_build::build()
}

fn fetch_binaries() {
    let binaries_dir = Path::new("binaries");

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
    ];

    // Check if all binaries already exist
    let all_exist = binaries
        .iter()
        .all(|(local_name, _)| binaries_dir.join(local_name).exists());

    if all_exist {
        println!("cargo:warning=Binaries already exist, skipping download");
        return;
    }

    println!("cargo:warning=Fetching dweb binaries...");

    // Get the latest release tag
    let latest_tag = match get_latest_release_tag() {
        Ok(tag) => tag,
        Err(e) => {
            println!("cargo:warning=Failed to get latest release tag: {e}");
            return;
        }
    };

    println!("cargo:warning=Latest dweb release: {latest_tag}");

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
                println!("cargo:warning=Downloaded {local_name}");

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

    println!("cargo:warning=Binary fetch completed");
}

fn get_latest_release_tag() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/happybeing/dweb/releases/latest";
    let response = ureq::get(url).call()?;
    let json: serde_json::Value = response.into_json()?;

    json["tag_name"]
        .as_str()
        .ok_or("tag_name not found in response".into())
        .map(|s| s.to_string())
}

fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = ureq::get(url).call()?;
    let mut file = fs::File::create(path)?;
    std::io::copy(&mut response.into_reader(), &mut file)?;
    Ok(())
}
