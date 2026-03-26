// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Emitter;
use tauri::Manager;

#[derive(serde::Serialize, Clone)]
struct DependencyStatus {
    yt_dlp_installed: bool,
    yt_dlp_version: Option<String>,
    yt_dlp_outdated: bool,
    ffmpeg_source: String,
}

fn main() {
    tauri::Builder::default()
        // ✅ Store plugin
        .plugin(tauri_plugin_store::Builder::default().build())
        // ✅ Dialog plugin
        .plugin(tauri_plugin_dialog::init())
        // ✅ Your commands
        .invoke_handler(tauri::generate_handler![
            list_folders,
            download_from_link,
            check_yt_dlp_installed,
            install_yt_dlp,
            install_yt_dlp_elevated,
            check_ffmpeg_installed,
            install_ffmpeg,
            check_yt_dlp_version,
            get_dependency_status,
            update_yt_dlp,
            get_ffmpeg_path,
            open_folder,
            delete_download_folder
        ])
        // ✅ Setup logic
        .setup(|_app| {
            println!("Tauri app is starting...");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}!", name)
// }

// #[tauri::command]
// fn shout(name: &str) -> String {
//     format!("HELLO, {}!!!", name.to_uppercase())
// }

// import std::fs and std::path::Path
use std::path::Path;
#[tauri::command]
fn list_folders(path: &str) -> Vec<String> {
    println!("list_folders called with path: {}", path);
    let mut folders = Vec::new();
    match std::fs::read_dir(Path::new(path)) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        folders.push(name.to_string());
                    }
                }
            }
            println!("Found {} folders", folders.len());
        }
        Err(e) => {
            println!("Error reading directory: {}", e);
        }
    }
    folders
}

use std::process::{Command as StdCommand, Stdio};
use tauri::AppHandle;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(serde::Serialize)]
struct DownloadResult {
    success: bool,
    message: String,
    file_path: Option<String>,
}

#[derive(serde::Serialize, Clone)]
struct ConsoleOutputEvent {
    line: String,
    timestamp: String,
}

#[derive(serde::Serialize, Clone)]
struct DownloadProgressEvent {
    percentage: Option<f32>,
    speed: Option<String>,
    eta: Option<String>,
    size: Option<String>,
}

#[derive(serde::Serialize, Clone)]
struct DownloadCompleteEvent {
    success: bool,
    message: String,
    file_path: Option<String>,
}

#[tauri::command]
async fn download_from_link(
    link: &str,
    folder: &str,
    format: &str,
    extension: Option<&str>,
    embeds: bool,
    app_handle: AppHandle,
) -> Result<DownloadResult, String> {
    println!("Starting async download from: {}", link);
    println!("Using yt-dlp from PATH");

    if !command_available("yt-dlp", &["--version"]) {
        let error_msg = "yt-dlp is not installed or not on PATH. Please install yt-dlp and try again.".to_string();
        let _ = app_handle.emit("download-complete", DownloadCompleteEvent {
            success: false,
            message: error_msg.clone(),
            file_path: None,
        });
        return Err(error_msg);
    }

    // Get ffmpeg path (system or bundled)
    let ffmpeg_dir = get_ffmpeg_path(app_handle.clone());
    
    // Build command arguments based on format
    let mut args: Vec<String> = vec![
        "--newline".to_string(),
        "--no-colors".to_string(),
        "--progress-template".to_string(),
        "download:%(progress.percentage)s|%(progress.speed)s|%(progress.eta)s|%(progress.total_bytes)s".to_string(),
    ];
    
    // Only add ffmpeg location if we found it
    if !ffmpeg_dir.is_empty() {
        args.push("--ffmpeg-location".to_string());
        args.push(ffmpeg_dir);
    }

    // Add format-specific arguments
    if let Some(ext) = extension {
        if format == "audio" {
            args.push("-x".to_string());
            args.push("--audio-format".to_string());
        } else {
            args.push("--merge-output-format".to_string());
        }
        args.push(ext.to_string());
    }

    // Add link
    args.push(link.to_string());

    if embeds {
        args.push("--embed-thumbnail".to_string());
        args.push("--embed-metadata".to_string());
    }

    println!("Running yt-dlp {}", args.join(" "));

    let mut cmd = Command::new("yt-dlp");
    cmd.current_dir(folder)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(&args);

    // Spawn the process
    match cmd.spawn() {
        Ok(mut child) => {
            // Get handles to stdout and stderr
            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let stderr = child.stderr.take().expect("Failed to capture stderr");

            let app_handle_stdout = app_handle.clone();
            let app_handle_stderr = app_handle.clone();
            let folder_path = folder.to_string();
            
            // Use Arc<Mutex<>> to share the downloaded file path between tasks
            use std::sync::{Arc, Mutex};
            let downloaded_file = Arc::new(Mutex::new(None::<String>));
            let downloaded_file_clone = downloaded_file.clone();

            // Spawn tasks to read stdout and stderr concurrently
            let stdout_task = tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                
                while let Ok(Some(line)) = lines.next_line().await {
                    let timestamp = chrono::Utc::now().format("%H:%M:%S%.3f").to_string();
                    
                    println!("[yt-dlp] {}", line);

                    // Parse progress information
                    if let Some(progress) = parse_progress_line(&line) {
                        let _ = app_handle_stdout.emit("download-progress", progress);
                    }
                    
                    // Capture downloaded file path
                    // yt-dlp outputs: "[download] Destination: filename.ext"
                    // or "[ExtractAudio] Destination: filename.ext"
                    // or after merge: "[Merger] Merging formats into \"filename.ext\""
                    if line.contains("[download] Destination:") {
                        if let Some(filename) = line.split("Destination:").nth(1) {
                            let filename = filename.trim();
                            let full_path = format!("{}\\{}", folder_path, filename);
                            *downloaded_file_clone.lock().unwrap() = Some(full_path);
                            println!("Captured download destination: {}", filename);
                        }
                    } else if line.contains("[ExtractAudio] Destination:") {
                        if let Some(filename) = line.split("Destination:").nth(1) {
                            let filename = filename.trim();
                            let full_path = format!("{}\\{}", folder_path, filename);
                            *downloaded_file_clone.lock().unwrap() = Some(full_path);
                            println!("Captured audio destination: {}", filename);
                        }
                    } else if line.contains("[Merger] Merging formats into") {
                        if let Some(quoted) = line.split("into \"").nth(1) {
                            if let Some(filename) = quoted.split('"').next() {
                                let full_path = format!("{}\\{}", folder_path, filename);
                                *downloaded_file_clone.lock().unwrap() = Some(full_path);
                                println!("Captured merged file: {}", filename);
                            }
                        }
                    }
                    
                    // Emit console output
                    let _ = app_handle_stdout.emit("console-output", ConsoleOutputEvent {
                        line: line.clone(),
                        timestamp: timestamp.clone(),
                    });
                }
            });

            let stderr_task = tokio::spawn(async move {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                
                while let Ok(Some(line)) = lines.next_line().await {
                    let timestamp = chrono::Utc::now().format("%H:%M:%S%.3f").to_string();
                    
                    eprintln!("[yt-dlp] {}", line);

                    // Emit error output to console
                    let _ = app_handle_stderr.emit("console-output", ConsoleOutputEvent {
                        line: format!("ERROR: {}", line),
                        timestamp,
                    });
                }
            });

            // Wait for the child process to complete
            match child.wait().await {
                Ok(status) => {
                    // Wait for output reading tasks to complete
                    let _ = tokio::join!(stdout_task, stderr_task);
                    
                    // Get the downloaded file path
                    let file_path = downloaded_file.lock().unwrap().clone();
                    
                    let result = if status.success() {
                        println!("Download completed successfully for: {}", link);
                        if let Some(ref path) = file_path {
                            println!("Downloaded file: {}", path);
                        }
                        DownloadResult {
                            success: true,
                            message: format!("Download completed successfully: {}", link),
                            file_path: file_path.clone(),
                        }
                    } else {
                        println!("Download failed for: {}", link);
                        DownloadResult {
                            success: false,
                            message: format!("Download failed for: {}", link),
                            file_path: None,
                        }
                    };
                    
                    // Emit completion event
                    let _ = app_handle.emit("download-complete", DownloadCompleteEvent {
                        success: result.success,
                        message: result.message.clone(),
                        file_path: result.file_path.clone(),
                    });
                    
                    Ok(result)
                }
                Err(e) => {
                    let error_msg = format!("Process execution error: {}", e);
                    println!("{}", error_msg);
                    
                    let result = DownloadResult {
                        success: false,
                        message: error_msg,
                        file_path: None,
                    };
                    
                    let _ = app_handle.emit("download-complete", DownloadCompleteEvent {
                        success: false,
                        message: result.message.clone(),
                        file_path: None,
                    });
                    
                    Ok(result)
                }
            }
        }
        Err(e) => {
            let error_msg = format!("Failed to spawn yt-dlp process: {}. Ensure yt-dlp is installed and available on PATH.", e);
            println!("{}", error_msg);
            
            let result = DownloadResult {
                success: false,
                message: error_msg,
                file_path: None,
            };
            
            let _ = app_handle.emit("download-complete", DownloadCompleteEvent {
                success: false,
                message: result.message.clone(),
                file_path: None,
            });
            
            Ok(result)
        }
    }
}

fn command_available(command: &str, args: &[&str]) -> bool {
    StdCommand::new(command)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[derive(serde::Serialize)]
struct InstallResult {
    success: bool,
    message: String,
    error_code: Option<String>,
    suggested_action: Option<String>,
}

#[tauri::command]
fn check_yt_dlp_installed() -> bool {
    command_available("yt-dlp", &["--version"])
}

#[tauri::command]
fn install_yt_dlp() -> Result<InstallResult, String> {
    match run_install_command() {
        Ok(message) => Ok(InstallResult {
            success: true,
            message,
            error_code: None,
            suggested_action: None,
        }),
        Err(e) => Ok(InstallResult {
            success: false,
            message: e.clone(),
            error_code: Some("install_failed".to_string()),
            suggested_action: Some("Install Python from python.org or use a supported package manager (winget, brew, pipx).".to_string()),
        }),
    }
}

#[tauri::command]
fn check_ffmpeg_installed() -> bool {
    command_available("ffmpeg", &["-version"]) && command_available("ffprobe", &["-version"])
}

#[tauri::command]
fn install_ffmpeg() -> Result<InstallResult, String> {
    match run_install_ffmpeg() {
        Ok(message) => Ok(InstallResult {
            success: true,
            message,
            error_code: None,
            suggested_action: None,
        }),
        Err(e) => Ok(InstallResult {
            success: false,
            message: e.clone(),
            error_code: Some("install_failed".to_string()),
            suggested_action: Some("Install ffmpeg manually from https://ffmpeg.org or use your package manager (brew, winget).".to_string()),
        }),
    }
}

fn run_install_command() -> Result<String, String> {
    let mut attempts: Vec<(&str, Vec<&str>)> = Vec::new();

    if cfg!(target_os = "macos") {
        // Try user-scope installers first (no admin needed)
        if command_available("pipx", &["--version"]) {
            attempts.push(("pipx", vec!["install", "yt-dlp"]));
        }
        if command_available("python3", &["--version"]) {
            attempts.push(("python3", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
        // System installers (may require elevation)
        if command_available("brew", &["--version"]) {
            attempts.push(("brew", vec!["install", "yt-dlp"]));
        }
    } else if cfg!(target_os = "windows") {
        // Try user-scope installers first (no admin needed)
        if command_available("pipx", &["--version"]) {
            attempts.push(("pipx", vec!["install", "yt-dlp"]));
        }
        if command_available("python", &["--version"]) {
            attempts.push(("python", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
        // System installers (may require elevation)
        if command_available("winget", &["--version"]) {
            attempts.push((
                "winget",
                vec![
                    "install",
                    "-e",
                    "--id",
                    "yt-dlp.yt-dlp",
                    "--accept-source-agreements",
                    "--accept-package-agreements",
                ],
            ));
        }
    } else {
        // Linux: Try user-scope installers first
        if command_available("pipx", &["--version"]) {
            attempts.push(("pipx", vec!["install", "yt-dlp"]));
        }
        if command_available("python3", &["--version"]) {
            attempts.push(("python3", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
    }

    if attempts.is_empty() {
        return Err("No supported installer found. Please install Python (python.org) or pipx, then try again.".to_string());
    }

    let mut last_error = String::from("Unknown error");
    for (command, args) in attempts {
        let output = StdCommand::new(command).args(&args).output();
        match output {
            Ok(output) if output.status.success() => {
                if command_available("yt-dlp", &["--version"]) {
                    return Ok(format!("yt-dlp installed successfully using {}.", command));
                }
                return Err(format!(
                    "{} completed, but yt-dlp is still not on PATH. Please restart your terminal and try again.",
                    command
                ));
            }
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                last_error = format!("{} {}",
                    stdout.trim(),
                    stderr.trim()
                ).trim().to_string();
            }
            Err(error) => {
                last_error = error.to_string();
            }
        }
    }

    Err(format!("yt-dlp install failed. {}", last_error))
}

fn run_install_ffmpeg() -> Result<String, String> {
    let mut attempts: Vec<(&str, Vec<&str>)> = Vec::new();

    if cfg!(target_os = "macos") {
        if command_available("brew", &["--version"]) {
            attempts.push(("brew", vec!["install", "ffmpeg"]));
        }
    } else if cfg!(target_os = "windows") {
        if command_available("winget", &["--version"]) {
            attempts.push((
                "winget",
                vec![
                    "install",
                    "-e",
                    "--id",
                    "Gyan.FFmpeg",
                    "--accept-source-agreements",
                    "--accept-package-agreements",
                ],
            ));
            attempts.push((
                "winget",
                vec![
                    "install",
                    "-e",
                    "--id",
                    "FFmpeg.FFmpeg",
                    "--accept-source-agreements",
                    "--accept-package-agreements",
                ],
            ));
        }
    } else {
        if command_available("sudo", &["-n", "true"]) && command_available("apt-get", &["--version"]) {
            attempts.push(("sudo", vec!["apt-get", "install", "-y", "ffmpeg"]));
        } else if command_available("apt-get", &["--version"]) {
            attempts.push(("apt-get", vec!["install", "-y", "ffmpeg"]));
        }
    }

    if attempts.is_empty() {
        return Err("No supported installer found. Please install ffmpeg manually from https://ffmpeg.org/".to_string());
    }

    let mut last_error = String::from("Unknown error");
    for (command, args) in attempts {
        let output = StdCommand::new(command).args(&args).output();
        match output {
            Ok(output) if output.status.success() => {
                if check_ffmpeg_installed() {
                    return Ok(format!("ffmpeg installed using {}.", command));
                }
                return Err(format!(
                    "{} completed, but ffmpeg/ffprobe are still not on PATH. Please restart your terminal and try again.",
                    command
                ));
            }
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                last_error = format!("{} {}", stdout.trim(), stderr.trim()).trim().to_string();
            }
            Err(error) => {
                last_error = error.to_string();
            }
        }
    }

    Err(format!("ffmpeg install failed. {}", last_error))
}

fn parse_progress_line(line: &str) -> Option<DownloadProgressEvent> {
    // Parse yt-dlp progress output format
    if line.starts_with("download:") {
        let parts: Vec<&str> = line.strip_prefix("download:")?.split('|').collect();
        if parts.len() >= 4 {
            return Some(DownloadProgressEvent {
                percentage: parts[0].parse::<f32>().ok(),
                speed: if parts[1] != "N/A" { Some(parts[1].to_string()) } else { None },
                eta: if parts[2] != "N/A" { Some(parts[2].to_string()) } else { None },
                size: if parts[3] != "N/A" { Some(parts[3].to_string()) } else { None },
            });
        }
    }
    
    // Fallback: try to parse standard yt-dlp progress lines
    if line.contains("%") && (line.contains("MiB/s") || line.contains("KiB/s") || line.contains("B/s")) {
        // Extract percentage
        if let Some(pct_pos) = line.find('%') {
            if let Some(start) = line[..pct_pos].rfind(' ') {
                if let Ok(percentage) = line[start + 1..pct_pos].trim().parse::<f32>() {
                    return Some(DownloadProgressEvent {
                        percentage: Some(percentage),
                        speed: None,
                        eta: None,
                        size: None,
                    });
                }
            }
        }
    }
    
    None
}

#[tauri::command]
fn check_yt_dlp_version() -> Result<(Option<String>, Option<String>, bool), String> {
    let output = StdCommand::new("yt-dlp")
        .arg("--version")
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok((Some(version), None, false))
            } else {
                Err("yt-dlp version check failed".to_string())
            }
        }
        Err(e) => Err(format!("Failed to run yt-dlp version: {}", e)),
    }
}

#[tauri::command]
fn get_ffmpeg_path(app_handle: AppHandle) -> String {
    // Try to find ffmpeg on PATH and get full path
    if cfg!(target_os = "windows") {
        if let Ok(output) = StdCommand::new("cmd")
            .args(&["/C", "where ffmpeg"])
            .output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    // Extract directory from full path
                    if let Some(dir) = std::path::Path::new(&path).parent() {
                        let dir_str = dir.display().to_string();
                        println!("✓ Found ffmpeg on PATH: {}", path);
                        println!("  Returning directory: {}", dir_str);
                        return dir_str;
                    }
                }
            } else {
                println!("✗ FFmpeg not found on PATH (where ffmpeg failed)");
            }
        }
    } else {
        // macOS and Linux: use `which` command
        if let Ok(output) = StdCommand::new("which")
            .arg("ffmpeg")
            .output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    if let Some(dir) = std::path::Path::new(&path).parent() {
                        let dir_str = dir.display().to_string();
                        println!("✓ Found ffmpeg on PATH: {}", path);
                        println!("  Returning directory: {}", dir_str);
                        return dir_str;
                    }
                }
            } else {
                println!("✗ FFmpeg not found on PATH (which ffmpeg failed)");
            }
        }
    }
    
    // Fallback: check bundled ffmpeg
    let resource_path: Option<String> = app_handle
        .path()
        .resource_dir()
        .ok()
        .and_then(|p| Some(p.display().to_string()));
    
    if let Some(base) = resource_path {
        let ffmpeg_dir = if cfg!(target_os = "windows") {
            format!("{}\\bin\\windows", base)
        } else if cfg!(target_os = "macos") {
            format!("{}/bin/macos", base)
        } else {
            // Linux: return common system directories
            if std::path::Path::new("/usr/bin/ffmpeg").exists() {
                return "/usr/bin".to_string();
            }
            if std::path::Path::new("/usr/local/bin/ffmpeg").exists() {
                return "/usr/local/bin".to_string();
            }
            return String::new();
        };
        
        let ffmpeg_exe_path = if cfg!(target_os = "windows") {
            format!("{}\\ffmpeg.exe", ffmpeg_dir)
        } else {
            format!("{}/ffmpeg", ffmpeg_dir)
        };
        
        if std::path::Path::new(&ffmpeg_exe_path).exists() {
            println!("✓ Found bundled ffmpeg: {}", ffmpeg_exe_path);
            println!("  Returning directory: {}", ffmpeg_dir);
            return ffmpeg_dir;
        } else {
            println!("✗ Bundled ffmpeg not found at: {}", ffmpeg_exe_path);
        }
    } else {
        println!("✗ Could not get resource directory from app_handle");
    }
    
    println!("✗ FFmpeg not found anywhere (PATH or bundled). Downloads requiring video merging will fail.");
    String::new()
}

#[tauri::command]
fn get_dependency_status(app_handle: AppHandle) -> DependencyStatus {
    let yt_dlp_installed = command_available("yt-dlp", &["--version"]);
    
    let yt_dlp_version = if yt_dlp_installed {
        StdCommand::new("yt-dlp")
            .arg("--version")
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                } else {
                    None
                }
            })
    } else {
        None
    };
    
    let ffmpeg_source = if command_available("ffmpeg", &["-version"]) {
        "System".to_string()
    } else if let Some(base) = app_handle.path().resource_dir().ok() {
        let ffmpeg_path = if cfg!(target_os = "windows") {
            base.join("bin\\windows\\ffmpeg.exe")
        } else if cfg!(target_os = "macos") {
            base.join("bin/macos/ffmpeg")
        } else {
            std::path::PathBuf::from("")
        };
        
        if ffmpeg_path.exists() {
            "Bundled".to_string()
        } else {
            "Missing".to_string()
        }
    } else {
        "Missing".to_string()
    };
    
    DependencyStatus {
        yt_dlp_installed,
        yt_dlp_version,
        yt_dlp_outdated: false,
        ffmpeg_source,
    }
}

#[tauri::command]
fn update_yt_dlp() -> Result<InstallResult, String> {
    let mut attempts: Vec<(&str, Vec<&str>)> = Vec::new();
    
    if cfg!(target_os = "macos") {
        if command_available("pipx", &["--version"]) {
            attempts.push(("pipx", vec!["upgrade", "yt-dlp"]));
        }
        if command_available("brew", &["--version"]) {
            attempts.push(("brew", vec!["upgrade", "yt-dlp"]));
        }
    } else if cfg!(target_os = "windows") {
        if command_available("pipx", &["--version"]) {
            attempts.push(("pipx", vec!["upgrade", "yt-dlp"]));
        }
        if command_available("python", &["--version"]) {
            attempts.push(("python", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
    } else {
        if command_available("pipx", &["--version"]) {
            attempts.push(("pipx", vec!["upgrade", "yt-dlp"]));
        }
        if command_available("python3", &["--version"]) {
            attempts.push(("python3", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
    }
    
    for (command, args) in attempts {
        let output = StdCommand::new(command).args(&args).output();
        match output {
            Ok(output) if output.status.success() => {
                return Ok(InstallResult {
                    success: true,
                    message: format!("yt-dlp updated successfully using {}.", command),
                    error_code: None,
                    suggested_action: None,
                });
            }
            _ => continue,
        }
    }
    
    Err("Failed to update yt-dlp. Please update manually.".to_string())
}

#[tauri::command]
fn install_yt_dlp_elevated() -> Result<InstallResult, String> {
    if cfg!(target_os = "windows") {
        if command_available("winget", &["--version"]) {
            let output = StdCommand::new("powershell")
                .args(&["-Command", "Start-Process powershell -Verb RunAs -ArgumentList 'winget install -e --id yt-dlp.yt-dlp --accept-source-agreements --accept-package-agreements'"])
                .output();
            
            match output {
                Ok(output) if output.status.success() => {
                    return Ok(InstallResult {
                        success: true,
                        message: "Elevated installation started. Please complete the installation in the new window.".to_string(),
                        error_code: None,
                        suggested_action: None,
                    });
                }
                _ => {}
            }
        }
    } else if cfg!(target_os = "macos") {
        return Ok(InstallResult {
            success: false,
            message: "macOS does not require elevated permissions. Try the standard install.".to_string(),
            error_code: Some("not_needed".to_string()),
            suggested_action: Some("Run: brew install yt-dlp".to_string()),
        });
    }
    
    Err("Elevated installation not supported on this system.".to_string())
}

#[tauri::command]
fn open_folder(folder_path: &str) -> Result<String, String> {
    use std::path::Path;
    
    let path = Path::new(folder_path);
    
    // Check if path exists
    if !path.exists() {
        return Err("The file or folder no longer exists.".to_string());
    }
    
    // If it's a file, open the parent folder
    let folder_to_open = if path.is_file() {
        path.parent()
            .ok_or_else(|| "Could not determine parent folder.".to_string())?
    } else {
        path
    };
    
    let folder_str = folder_to_open.to_str()
        .ok_or_else(|| "Invalid path encoding.".to_string())?;
    
    // Platform-specific folder opening
    if cfg!(target_os = "windows") {
        match StdCommand::new("explorer")
            .args(&[folder_str])
            .output() {
            Ok(_) => {
                println!("✓ Opened folder: {}", folder_str);
                Ok("Folder opened in Explorer.".to_string())
            }
            Err(e) => Err(format!("Failed to open folder: {}", e)),
        }
    } else if cfg!(target_os = "macos") {
        match StdCommand::new("open")
            .args(&[folder_str])
            .output() {
            Ok(_) => {
                println!("✓ Opened folder: {}", folder_str);
                Ok("Folder opened in Finder.".to_string())
            }
            Err(e) => Err(format!("Failed to open folder: {}", e)),
        }
    } else {
        // Linux - try xdg-open
        match StdCommand::new("xdg-open")
            .args(&[folder_str])
            .output() {
            Ok(_) => {
                println!("✓ Opened folder: {}", folder_str);
                Ok("Folder opened.".to_string())
            }
            Err(e) => Err(format!("Failed to open folder: {}", e)),
        }
    }
}

#[tauri::command]
fn delete_download_folder(folder_path: &str) -> Result<String, String> {
    use std::path::Path;
    use std::fs;
    
    let path = Path::new(folder_path);
    
    // Check if path exists
    if !path.exists() {
        return Err("The folder or file no longer exists.".to_string());
    }
    
    // Delete file or folder recursively
    if path.is_dir() {
        match fs::remove_dir_all(path) {
            Ok(_) => {
                println!("✓ Deleted folder: {}", folder_path);
                Ok(format!("Folder deleted successfully."))
            }
            Err(e) => {
                Err(format!("Failed to delete folder: {}", e))
            }
        }
    } else {
        match fs::remove_file(path) {
            Ok(_) => {
                println!("✓ Deleted file: {}", folder_path);
                Ok(format!("File deleted successfully."))
            }
            Err(e) => {
                Err(format!("Failed to delete file: {}", e))
            }
        }
    }
}
