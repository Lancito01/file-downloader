// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Emitter;

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
            check_ffmpeg_installed,
            install_ffmpeg
        ])
        // ✅ Setup logic
        .setup(|app| {
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
        });
        return Err(error_msg);
    }

    // Build command arguments based on format
    let mut args: Vec<String> = vec![
        "--newline".to_string(),
        "--no-colors".to_string(),
        "--progress-template".to_string(),
        "download:%(progress.percentage)s|%(progress.speed)s|%(progress.eta)s|%(progress.total_bytes)s".to_string(),
    ];

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
                    
                    let result = if status.success() {
                        println!("Download completed successfully for: {}", link);
                        DownloadResult {
                            success: true,
                            message: format!("Download completed successfully: {}", link),
                        }
                    } else {
                        println!("Download failed for: {}", link);
                        DownloadResult {
                            success: false,
                            message: format!("Download failed for: {}", link),
                        }
                    };
                    
                    // Emit completion event
                    let _ = app_handle.emit("download-complete", DownloadCompleteEvent {
                        success: result.success,
                        message: result.message.clone(),
                    });
                    
                    Ok(result)
                }
                Err(e) => {
                    let error_msg = format!("Process execution error: {}", e);
                    println!("{}", error_msg);
                    
                    let result = DownloadResult {
                        success: false,
                        message: error_msg,
                    };
                    
                    let _ = app_handle.emit("download-complete", DownloadCompleteEvent {
                        success: false,
                        message: result.message.clone(),
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
            };
            
            let _ = app_handle.emit("download-complete", DownloadCompleteEvent {
                success: false,
                message: result.message.clone(),
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
}

#[tauri::command]
fn check_yt_dlp_installed() -> bool {
    command_available("yt-dlp", &["--version"])
}

#[tauri::command]
fn install_yt_dlp() -> Result<InstallResult, String> {
    let message = run_install_command()?;
    Ok(InstallResult {
        success: true,
        message,
    })
}

#[tauri::command]
fn check_ffmpeg_installed() -> bool {
    command_available("ffmpeg", &["-version"]) && command_available("ffprobe", &["-version"])
}

#[tauri::command]
fn install_ffmpeg() -> Result<InstallResult, String> {
    let message = run_install_ffmpeg()?;
    Ok(InstallResult {
        success: true,
        message,
    })
}

fn run_install_command() -> Result<String, String> {
    let mut attempts: Vec<(&str, Vec<&str>)> = Vec::new();

    if cfg!(target_os = "macos") {
        if command_available("brew", &["--version"]) {
            attempts.push(("brew", vec!["install", "yt-dlp"]));
        }
        if command_available("pipx", &["--version"]) {
            attempts.push(("pipx", vec!["install", "yt-dlp"]));
        }
        if command_available("python3", &["--version"]) {
            attempts.push(("python3", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
    } else if cfg!(target_os = "windows") {
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
        if command_available("pipx", &["--version"]) {
            attempts.push(("pipx", vec!["install", "yt-dlp"]));
        }
        if command_available("python", &["--version"]) {
            attempts.push(("python", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
    } else {
        if command_available("pipx", &["--version"]) {
            attempts.push(("pipx", vec!["install", "yt-dlp"]));
        }
        if command_available("python3", &["--version"]) {
            attempts.push(("python3", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
    }

    if attempts.is_empty() {
        return Err("No supported installer found. Please install yt-dlp manually from https://github.com/yt-dlp/yt-dlp".to_string());
    }

    let mut last_error = String::from("Unknown error");
    for (command, args) in attempts {
        let output = StdCommand::new(command).args(&args).output();
        match output {
            Ok(output) if output.status.success() => {
                if command_available("yt-dlp", &["--version"]) {
                    return Ok(format!("yt-dlp installed using {}.", command));
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
