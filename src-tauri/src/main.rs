// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // app_lib::run();
    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![greet, shout])
        .invoke_handler(tauri::generate_handler![list_folders])
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
