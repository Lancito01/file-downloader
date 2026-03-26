// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Emitter};

fn main() {
    tauri::Builder::default()
        // ✅ Store plugin
        .plugin(tauri_plugin_store::Builder::default().build())
        // ✅ Dialog plugin
        .plugin(tauri_plugin_dialog::init())
        // ✅ Your commands
        .invoke_handler(tauri::generate_handler![list_folders, download_from_link])
        // ✅ Setup logic
        .setup(|app| {
            println!("Tauri app is starting...");

            let resource_path = app
                .path()
                .resource_dir()
                .expect("failed to get resource dir");

            app.manage(resource_path);
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

use std::path::PathBuf;
use std::process::Stdio;
use tauri::{AppHandle, State};
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
    resource_path: State<'_, PathBuf>,
) -> Result<DownloadResult, String> {
    // Get the path to yt-dlp.exe from the bundled resources
    let yt_dlp_path = resource_path.inner().join("bin").join("yt-dlp.exe");

    println!("Starting async download from: {}", link);
    println!("Using yt-dlp from: {}", yt_dlp_path.display());

    // Build command arguments based on format
    let mut cmd = Command::new(&yt_dlp_path);
    cmd.current_dir(folder)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--newline") // Force newlines for better parsing
        .arg("--no-colors"); // Disable color codes for cleaner output

    // Add progress hook for better progress parsing
    cmd.arg("--progress-template")
        .arg("download:%(progress.percentage)s|%(progress.speed)s|%(progress.eta)s|%(progress.total_bytes)s");

    // Add format-specific arguments
    if let Some(ext) = extension {
        if format == "audio" {
            cmd.arg("-x");
            cmd.arg("--audio-format");
        } else {
            cmd.arg("--merge-output-format");
        }
        cmd.arg(ext);
    }

    // Add link
    cmd.arg(link);

    if embeds {
        cmd.arg("--embed-thumbnail");
        cmd.arg("--embed-metadata");
    }

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
            let error_msg = format!("Failed to spawn yt-dlp process: {}. Make sure yt-dlp.exe is bundled with the app.", e);
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
