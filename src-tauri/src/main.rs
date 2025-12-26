// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        // ✅ Store plugin
        .plugin(tauri_plugin_store::Builder::default().build())

        // ✅ Dialog plugin
        .plugin(tauri_plugin_dialog::init())

        // ✅ Your commands
        .invoke_handler(tauri::generate_handler![
            list_folders,
            download_from_link
        ])

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
use std::process::Command;
use tauri::State;

#[tauri::command]
fn download_from_link(
    link: &str,
    folder: &str,
    format: &str,
    extension: &str,
    embeds: bool,
    resource_path: State<'_, PathBuf>,
) -> bool {
    // Build the output template
    // Constructs a file output path template using the specified folder and file extension.
    //
    // The template uses `%(title)s` as a placeholder for the downloaded file's title,
    // which will be replaced at runtime with the actual filename.
    //
    // Arguments
    // * `link` - The URL link to download from (e.g., YouTube video/playlist, SoundCloud song, etc.)
    // * `folder` - The destination directory path where the file will be saved
    // * `format` - String format of download for code logic purposes only (e.g., "audio" or "video")
    // * `extension` - The file extension (without the leading dot) for the output file
    //
    // Example
    // If `folder` is `/downloads` and `extension` is `mp4`, the resulting template
    // will be `/downloads/%(title)s.mp4`
    //
    // * Function will use yt-dlp to download the media from the provided link.
    // * yt-dlp is in ../bin/yt-dlp.exe, relative to this very file.
    // * From project root, the file tree is as follows:
    // * project/
    // *    └──src-tauri/
    // *       ├── src/
    // *       │   └── main.rs
    // *       └── bin/
    // *           └── yt-dlp.exe

    // Get the path to yt-dlp.exe from the bundled resources
    let yt_dlp_path = resource_path.inner().join("bin").join("yt-dlp.exe");

    println!("Starting download from: {}", link);
    println!("Using yt-dlp from: {}", yt_dlp_path.display());

    // Build command arguments based on format
    let mut cmd = Command::new(&yt_dlp_path);
    cmd.current_dir(folder);  // Files will be saved to this directory

    // Add format-specific arguments
    if format == "audio" {
        cmd.arg("-x");
        cmd.arg("--audio-format");
    } else {
        cmd.arg("--merge-output-format");
    }
    cmd.arg(extension);
    
    // Add link
    cmd.arg(link);

    if embeds {
        cmd.arg("--embed-thumbnail");
        cmd.arg("--embed-metadata");
    }

    // Execute the command
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                println!("Download completed successfully for: {}", link);
                true
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                println!("Download failed: {}", error_msg);
                false
            }
        }
        Err(e) => {
            println!("Error executing yt-dlp: {}", e);
            false
        }
    }
}
