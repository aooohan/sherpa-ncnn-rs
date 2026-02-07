//! Download pre-built sherpa-ncnn binaries

use std::fs;
use std::io::Read;
use std::path::PathBuf;

/// Get the cache directory for downloaded binaries
pub fn get_cache_dir() -> Option<PathBuf> {
    dirs::cache_dir().map(|d| d.join("sherpa-ncnn-rs"))
}

/// Download a file from URL
pub fn fetch_file(url: &str) -> Vec<u8> {
    println!("cargo:warning=Downloading {}", url);

    let response = ureq::get(url).call().expect("Failed to download file");

    let mut data = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut data)
        .expect("Failed to read response");

    data
}

/// Extract a .tar.gz archive
pub fn extract_tgz(data: &[u8], dest: &PathBuf) {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let decoder = GzDecoder::new(data);
    let mut archive = Archive::new(decoder);

    fs::create_dir_all(dest).expect("Failed to create destination directory");
    archive.unpack(dest).expect("Failed to extract archive");
}

/// Map Rust target to sherpa-ncnn platform name and archive extension
fn get_platform_info(target: &str) -> Option<(&'static str, &'static str)> {
    match target {
        // Android
        t if t.contains("aarch64") && t.contains("android") => {
            Some(("android-arm64-v8a", "tar.gz"))
        }
        t if t.contains("armv7") && t.contains("android") => {
            Some(("android-armeabi-v7a", "tar.gz"))
        }
        t if t.contains("x86_64") && t.contains("android") => Some(("android-x86_64", "tar.gz")),
        t if t.contains("i686") && t.contains("android") => Some(("android-x86", "tar.gz")),

        // iOS
        t if t.contains("aarch64") && t.contains("ios") && !t.contains("sim") => {
            Some(("ios-arm64", "tar.gz"))
        }
        t if t.contains("aarch64") && t.contains("ios") && t.contains("sim") => {
            Some(("ios-simulator-arm64", "tar.gz"))
        }
        t if t.contains("x86_64") && t.contains("ios") => Some(("ios-simulator-x86_64", "tar.gz")),

        // macOS
        t if t.contains("aarch64") && t.contains("apple") && t.contains("darwin") => {
            Some(("macos-arm64", "tar.gz"))
        }
        t if t.contains("x86_64") && t.contains("apple") && t.contains("darwin") => {
            Some(("macos-x86_64", "tar.gz"))
        }

        // Linux
        t if t.contains("x86_64") && t.contains("linux") && !t.contains("android") => {
            Some(("linux-x86_64", "tar.gz"))
        }
        t if t.contains("aarch64") && t.contains("linux") && !t.contains("android") => {
            Some(("linux-aarch64", "tar.gz"))
        }
        t if t.contains("arm") && t.contains("linux") && t.contains("gnueabihf") => {
            Some(("linux-armv7", "tar.gz"))
        }

        // Windows
        t if t.contains("x86_64") && t.contains("windows") => Some(("windows-x64", "zip")),

        _ => None,
    }
}

/// Download and extract pre-built binaries
pub fn download_and_extract(target: &str, out_dir: &PathBuf) -> Option<PathBuf> {
    let (platform, _ext) = get_platform_info(target)?;

    // Get the release URL from environment or use default
    let base_url = std::env::var("SHERPA_NCNN_RELEASE_URL").unwrap_or_else(|_| {
        "https://github.com/aooohan/sherpa-ncnn-rs/releases/download/lib-2.1.15".to_string()
    });

    let archive_name = format!("sherpa-ncnn-{}.tar.gz", platform);
    let url = format!("{}/{}", base_url, archive_name);

    // Check cache first
    let cache_dir = get_cache_dir().unwrap_or_else(|| out_dir.clone());
    let cache_path = cache_dir.join(platform);

    if cache_path.exists() && cache_path.join("lib").exists() {
        println!(
            "cargo:warning=Using cached libraries from {}",
            cache_path.display()
        );
        return Some(cache_path);
    }

    // Download and extract
    let data = fetch_file(&url);

    fs::create_dir_all(&cache_path).ok()?;
    extract_tgz(&data, &cache_path);

    Some(cache_path)
}
