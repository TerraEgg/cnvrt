use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::Read;

pub fn get_ffmpeg_dir() -> PathBuf {
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("./cache"))
        .join("cnvrt");
    
    std::fs::create_dir_all(&cache_dir).ok();
    cache_dir
}

pub fn get_ffmpeg_path() -> PathBuf {
    let ffmpeg_dir = get_ffmpeg_dir();
    
    #[cfg(windows)]
    {
        ffmpeg_dir.join("ffmpeg.exe")
    }
    #[cfg(not(windows))]
    {
        ffmpeg_dir.join("ffmpeg")
    }
}

pub fn is_ffmpeg_available() -> bool {
    let bundled = get_ffmpeg_path();
    if bundled.exists() {
        return true;
    }
    
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn ensure_ffmpeg() -> Result<PathBuf, String> {
    let ffmpeg_path = get_ffmpeg_path();
    
    if ffmpeg_path.exists() {
        return Ok(ffmpeg_path);
    }
    
    if Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
    {
        return Ok(PathBuf::from("ffmpeg"));
    }
    
    download_ffmpeg(&ffmpeg_path)
}

fn download_ffmpeg(target_path: &Path) -> Result<PathBuf, String> {
    let url = get_ffmpeg_download_url()
        .ok_or_else(|| "Unsupported platform for FFmpeg download".to_string())?;
    
    eprintln!("[FFmpeg] Starting download from: {}", url);
    println!("[FFmpeg] Downloading... (this may take a few minutes)");
    
    #[cfg(windows)]
    {
        use std::process::Command;
        
        let temp_path = target_path.parent().unwrap().join("ffmpeg-download.tmp");
        eprintln!("[FFmpeg] Downloading to: {}", temp_path.display());
        
        let mut cmd = Command::new("powershell");
        cmd.arg("-NoProfile")
           .arg("-Command")
           .arg(format!(
               "$ProgressPreference = 'SilentlyContinue'; Write-Host '[FFmpeg] Downloading FFmpeg...'; Invoke-WebRequest -Uri '{}' -OutFile '{}'; Write-Host '[FFmpeg] Download complete'",
               url, 
               temp_path.display()
           ));
        
        match cmd.status() {
            Ok(status) if status.success() => {
                eprintln!("[FFmpeg] Download completed successfully");
                if url.ends_with(".zip") {
                    eprintln!("[FFmpeg] Extracting ZIP file...");
                    extract_zip_from_file(&temp_path, target_path.parent().unwrap())?;
                    std::fs::remove_file(&temp_path).ok();
                    eprintln!("[FFmpeg] Extraction complete");
                } else if url.ends_with(".tar.xz") {
                    eprintln!("[FFmpeg] Extracting TAR.XZ file...");
                    extract_tar_from_file(&temp_path, target_path.parent().unwrap())?;
                    std::fs::remove_file(&temp_path).ok();
                    eprintln!("[FFmpeg] Extraction complete");
                } else {
                    std::fs::rename(&temp_path, target_path)
                        .map_err(|e| format!("Failed to move FFmpeg: {}", e))?;
                    
                    #[cfg(not(windows))]
                    {
                        use std::fs;
                        use std::os::unix::fs::PermissionsExt;
                        let perms = fs::Permissions::from_mode(0o755);
                        fs::set_permissions(target_path, perms).ok();
                    }
                }
                eprintln!("[FFmpeg] Setup complete!");
                Ok(target_path.to_path_buf())
            }
            _ => {
                eprintln!("[FFmpeg] Download failed!");
                Err("Failed to download FFmpeg with PowerShell".to_string())
            }
        }
    }
    
    #[cfg(not(windows))]
    {
        use std::process::Command;
        
        let temp_path = target_path.parent().unwrap().join("ffmpeg-download.tmp");
        eprintln!("[FFmpeg] Downloading to: {}", temp_path.display());
        
        let mut cmd = Command::new("curl");
        cmd.arg("-L")
           .arg("-o")
           .arg(temp_path.to_str().unwrap())
           .arg(url);
        
        match cmd.status() {
            Ok(status) if status.success() => {
                eprintln!("[FFmpeg] Download completed successfully");
                if url.ends_with(".tar.xz") {
                    eprintln!("[FFmpeg] Extracting TAR.XZ file...");
                    extract_tar_from_file(&temp_path, target_path.parent().unwrap())?;
                    std::fs::remove_file(&temp_path).ok();
                    eprintln!("[FFmpeg] Extraction complete");
                } else {
                    std::fs::rename(&temp_path, target_path)
                        .map_err(|e| format!("Failed to move FFmpeg: {}", e))?;
                    
                    use std::fs;
                    use std::os::unix::fs::PermissionsExt;
                    let perms = fs::Permissions::from_mode(0o755);
                    fs::set_permissions(target_path, perms)
                        .map_err(|e| format!("Failed to set permissions: {}", e))?;
                }
                eprintln!("[FFmpeg] Setup complete!");
                Ok(target_path.to_path_buf())
            }
            _ => {
                eprintln!("[FFmpeg] Download failed!");
                Err("Failed to download FFmpeg with curl".to_string())
            }
        }
    }
}

fn get_ffmpeg_download_url() -> Option<&'static str> {
    #[cfg(target_os = "windows")]
    {
        // Using ffbinaries GitHub releases
        Some("https://github.com/ffbinaries/ffbinaries-prebuilt/releases/download/v6.1/ffmpeg-6.1-win-64.zip")
    }
    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "aarch64")]
        {
            // macOS ARM64
            Some("https://evermeet.cx/ffmpeg/ffmpeg-7.0.1.7z")
        }
        #[cfg(target_arch = "x86_64")]
        {
            // macOS Intel
            Some("https://evermeet.cx/ffmpeg/ffmpeg-7.0.1.7z")
        }
    }
    #[cfg(target_os = "linux")]
    {
        // Using static build from BtbN
        Some("https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz")
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        None
    }
}

fn extract_zip_from_file(file_path: &Path, target_dir: &Path) -> Result<(), String> {
    let file = std::fs::File::open(file_path)
        .map_err(|e| format!("Failed to open ZIP file: {}", e))?;
    
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Failed to read ZIP: {}", e))?;
    
    for i in 0..archive.len() {
        let file = archive.by_index(i)
            .map_err(|e| format!("Failed to extract: {}", e))?;
        
        let is_ffmpeg = file.name().contains("ffmpeg") && !file.name().ends_with('/');
        
        if is_ffmpeg {
            let target_path = target_dir.join(
                if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" }
            );
            
            let mut outfile = std::fs::File::create(&target_path)
                .map_err(|e| format!("Failed to create file: {}", e))?;
            
            let bytes: Vec<u8> = file.bytes()
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("Failed to read file bytes: {}", e))?;
            
            std::io::Write::write_all(&mut outfile, &bytes)
                .map_err(|e| format!("Failed to write file: {}", e))?;
            
            #[cfg(not(windows))]
            {
                use std::fs;                use std::os::unix::fs::PermissionsExt;                let perms = fs::Permissions::from_mode(0o755);
                fs::set_permissions(&target_path, perms)
                    .map_err(|e| format!("Failed to set permissions: {}", e))?;
            }
            
            return Ok(());
        }
    }
    
    Err("FFmpeg binary not found in ZIP".to_string())
}

fn extract_tar_from_file(file_path: &Path, target_dir: &Path) -> Result<(), String> {
    let file = std::fs::File::open(file_path)
        .map_err(|e| format!("Failed to open TAR file: {}", e))?;
    
    let decoder = xz2::read::XzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    
    for entry_result in archive.entries().map_err(|e| format!("Failed to read TAR: {}", e))? {
        let mut entry = entry_result
            .map_err(|e| format!("Failed to extract entry: {}", e))?;
        
        let path = entry.path()
            .map_err(|e| format!("Invalid path: {}", e))?;
        
        if path.file_name().map(|n| n == "ffmpeg").unwrap_or(false) {
            let target_path = target_dir.join("ffmpeg");
            entry.unpack(&target_path)
                .map_err(|e| format!("Failed to extract: {}", e))?;
            
            #[cfg(not(windows))]
            {
                use std::fs;
                let perms = fs::Permissions::from_mode(0o755);
                fs::set_permissions(&target_path, perms)
                    .map_err(|e| format!("Failed to set permissions: {}", e))?;
            }
            
            return Ok(());
        }
    }
    
    Err("FFmpeg binary not found in TAR".to_string())
}
