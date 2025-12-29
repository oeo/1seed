use std::fs;
use std::io::{Read, Write};
use std::path::Path;

const REPO: &str = "oeo/1seed";

pub fn update(check_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    let current_version = env!("CARGO_PKG_VERSION");
    let binary_path = std::env::current_exe()?;

    if !is_writable(&binary_path) {
        return Err(format!(
            "Cannot write to {}. Try: sudo 1seed update",
            binary_path.display()
        )
        .into());
    }

    eprintln!("Checking for updates...");
    let latest = fetch_latest_version()?;
    let latest_version = latest.trim_start_matches('v');

    if latest_version == current_version {
        eprintln!("Already up to date (v{})", current_version);
        return Ok(());
    }

    eprintln!("Current: v{}", current_version);
    eprintln!("Latest:  v{}", latest_version);

    if check_only {
        eprintln!("\nRun '1seed update' to install the latest version");
        return Ok(());
    }

    eprint!("\nUpdate to v{}? [y/N] ", latest_version);
    std::io::stderr().flush()?;

    let mut response = String::new();
    std::io::stdin().read_line(&mut response)?;

    if response.trim().to_lowercase() != "y" {
        eprintln!("Cancelled");
        return Ok(());
    }

    let platform = detect_platform();
    eprintln!("Downloading v{} for {}...", latest_version, platform);

    let binary_data = download_and_extract(&latest, &platform)?;

    eprintln!("Installing...");
    let temp_path = binary_path.with_extension("new");
    fs::write(&temp_path, &binary_data)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&temp_path, fs::Permissions::from_mode(0o755))?;
    }

    fs::rename(&temp_path, &binary_path)?;

    eprintln!("✓ Updated to v{}", latest_version);
    Ok(())
}

fn fetch_latest_version() -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);

    let response = ureq::get(&url).set("User-Agent", "1seed-updater").call()?;

    let json: serde_json::Value = response.into_json()?;

    Ok(json["tag_name"]
        .as_str()
        .ok_or("no tag_name in response")?
        .to_string())
}

fn detect_platform() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let os_name = match os {
        "linux" => "linux",
        "macos" => "darwin",
        "windows" => "windows",
        _ => os,
    };

    let arch_name = match arch {
        "x86_64" => "amd64",
        "aarch64" | "arm64" => "arm64",
        _ => arch,
    };

    format!("{}-{}", os_name, arch_name)
}

fn download_and_extract(
    version: &str,
    platform: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let ext = if platform.contains("windows") {
        "zip"
    } else {
        "tar.gz"
    };

    let asset_name = format!("1seed-{}.{}", platform, ext);
    let url = format!(
        "https://github.com/{}/releases/download/{}/{}",
        REPO, version, asset_name
    );

    let response = ureq::get(&url).set("User-Agent", "1seed-updater").call()?;

    let mut archive_bytes = Vec::new();
    response.into_reader().read_to_end(&mut archive_bytes)?;

    if ext == "tar.gz" {
        extract_from_targz(&archive_bytes, platform)
    } else {
        Err("Windows zip extraction not yet implemented".into())
    }
}

fn extract_from_targz(
    archive_bytes: &[u8],
    platform: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let decompressor = GzDecoder::new(archive_bytes);
    let mut archive = Archive::new(decompressor);

    let binary_name = format!("1seed-{}", platform);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;

        if path.file_name().and_then(|n| n.to_str()) == Some(&binary_name) {
            let mut binary_data = Vec::new();
            entry.read_to_end(&mut binary_data)?;
            return Ok(binary_data);
        }
    }

    Err(format!("Binary {} not found in archive", binary_name).into())
}

fn is_writable(path: &Path) -> bool {
    fs::OpenOptions::new().append(true).open(path).is_ok()
}
