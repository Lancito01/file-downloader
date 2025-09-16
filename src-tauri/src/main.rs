// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // app_lib::run();
    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![greet, shout])
        .invoke_handler(tauri::generate_handler![list_folders, download_from_link])
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
    let mut folders = Vec::new();
    if let Ok(entries) = std::fs::read_dir(Path::new(path)) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    folders.push(name.to_string());
                }
            }
        }
    }
    folders
}

use std::path::PathBuf;
use std::process::Command;
#[tauri::command]
fn download_from_link(link: &str, folder: &str, format: &str, extension: &str) -> String {
    // Build the output template
    let output_template = format!("{}/%(title)s.{}", folder, extension);

    // Path to yt-dlp.exe (relative to the executable)
    let mut exe_path = std::env::current_exe().unwrap_or_default();
    exe_path.pop(); // remove the exe name
    exe_path.push("bin");
    exe_path.push("yt-dlp.exe");

    // Build yt-dlp arguments
    let mut args = vec![link, "-o", &output_template];
    if format == "audio" {
        args.push("-x");
        args.push("--audio-format");
        args.push(extension);
    }

    // Run yt-dlp
    let output = Command::new(exe_path).args(&args).output();

    match output {
        Ok(output) => {
            if output.status.success() {
                "Download successful".to_string()
            } else {
                format!("yt-dlp failed: {}", String::from_utf8_lossy(&output.stderr))
            }
        }
        Err(e) => format!("Failed to run yt-dlp: {}", e),
    }
}
