// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio::process::Command;

// Every command here is async on purpose: Tauri runs sync commands on the main
// thread, so anything that spawns a process or touches the disk would freeze the UI.

// Markers for the yt-dlp output we parse, so they can't collide with normal log lines.
const PROGRESS_MARKER: &str = "[fd-progress]";
const FILE_MARKER: &str = "[fd-file]";
const PROGRESS_EMIT_INTERVAL: Duration = Duration::from_millis(120);

static ACTIVE_DOWNLOAD_PID: Mutex<Option<u32>> = Mutex::new(None);
static CANCEL_REQUESTED: AtomicBool = AtomicBool::new(false);

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            list_folders,
            create_folder,
            download_from_link,
            cancel_download,
            install_yt_dlp,
            install_ffmpeg,
            check_yt_dlp_version,
            get_dependency_status,
            update_yt_dlp,
            open_folder_location
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn list_folders(path: String) -> Result<Vec<String>, String> {
    let mut entries = tokio::fs::read_dir(&path)
        .await
        .map_err(|e| format!("Could not read {}: {}", path, e))?;

    let mut folders = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        if entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false) {
            if let Some(name) = entry.file_name().to_str() {
                folders.push(name.to_string());
            }
        }
    }
    folders.sort_by_key(|name| name.to_lowercase());
    Ok(folders)
}

#[tauri::command]
async fn create_folder(path: String) -> Result<(), String> {
    tokio::fs::create_dir_all(&path)
        .await
        .map_err(|e| format!("Failed to create folder: {}", e))
}

// Helper function to hide console window on Windows
fn hidden_command(program: impl AsRef<std::ffi::OsStr>) -> Command {
    let mut cmd = Command::new(program);
    // CREATE_NO_WINDOW flag = 0x08000000
    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000);
    cmd.stdin(Stdio::null());
    cmd
}

// Directories to look for tools in. Installs made while the app is running don't
// reach this process's PATH, so the usual per-user install locations are added too.
fn tool_search_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).collect())
        .unwrap_or_default();

    if let Some(local) = std::env::var_os("LOCALAPPDATA") {
        dirs.push(Path::new(&local).join("Microsoft").join("WinGet").join("Links"));
    }
    if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        dirs.push(Path::new(&home).join(".local").join("bin"));
    }
    if cfg!(target_os = "macos") {
        dirs.push(PathBuf::from("/opt/homebrew/bin"));
        dirs.push(PathBuf::from("/usr/local/bin"));
    }
    dirs
}

// Finds a tool without running it. yt-dlp alone takes seconds to start, so
// launching each tool just to see if it exists is what made startup slow.
fn find_tool(name: &str) -> Option<PathBuf> {
    let file_names: Vec<String> = if cfg!(target_os = "windows") {
        ["exe", "cmd", "bat"].iter().map(|ext| format!("{}.{}", name, ext)).collect()
    } else {
        vec![name.to_string()]
    };

    tool_search_dirs().iter().find_map(|dir| {
        file_names.iter().map(|file| dir.join(file)).find(|path| path.is_file())
    })
}

fn find_bundled_ffmpeg_dir(app_handle: &AppHandle) -> Option<PathBuf> {
    let base = app_handle.path().resource_dir().ok()?;
    let (dir, exe) = if cfg!(target_os = "windows") {
        (base.join("bin").join("windows"), "ffmpeg.exe")
    } else if cfg!(target_os = "macos") {
        (base.join("bin").join("macos"), "ffmpeg")
    } else {
        return None;
    };
    dir.join(exe).is_file().then_some(dir)
}

fn find_system_ffmpeg_dir() -> Option<PathBuf> {
    find_tool("ffprobe")?;
    find_tool("ffmpeg")?.parent().map(Path::to_path_buf)
}

#[derive(serde::Serialize)]
struct DownloadResult {
    success: bool,
    cancelled: bool,
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
    percentage: Option<f64>,
    speed: Option<f64>,
    eta: Option<f64>,
    total_bytes: Option<f64>,
    title: Option<String>,
}

fn emit_console_line(app_handle: &AppHandle, line: String) {
    let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
    let _ = app_handle.emit("console-output", ConsoleOutputEvent { line, timestamp });
}

// Reads bytes rather than text: a name that isn't valid UTF-8 would otherwise end the
// read loop, and yt-dlp would then hang once the unread pipe filled up.
async fn for_each_line<R: AsyncRead + Unpin>(stream: R, mut on_line: impl FnMut(String)) {
    let mut reader = BufReader::new(stream);
    let mut buffer = Vec::new();
    loop {
        buffer.clear();
        match reader.read_until(b'\n', &mut buffer).await {
            Ok(0) | Err(_) => break,
            Ok(_) => on_line(String::from_utf8_lossy(&buffer).trim_end().to_string()),
        }
    }
}

fn build_download_args(
    link: &str,
    format: &str,
    extension: Option<&str>,
    embeds: bool,
    ffmpeg_dir: Option<&Path>,
) -> Vec<String> {
    let mut args: Vec<String> = [
        "--newline",
        "--no-colors",
        "--encoding",
        "utf-8",
        "--no-quiet",
        "--progress",
    ]
    .iter()
    .map(|arg| arg.to_string())
    .collect();

    args.push("--progress-template".to_string());
    // Title goes last because it's the only field that can contain the separator.
    args.push(format!(
        "download:{}%(progress.downloaded_bytes)s|%(progress.total_bytes)s|%(progress.total_bytes_estimate)s|%(progress.speed)s|%(progress.eta)s|%(info.title)s",
        PROGRESS_MARKER
    ));
    args.push("--print".to_string());
    args.push(format!("after_move:{}%(filepath)s", FILE_MARKER));

    if let Some(dir) = ffmpeg_dir {
        args.push("--ffmpeg-location".to_string());
        args.push(dir.display().to_string());
    }

    if format == "audio" {
        args.push("-x".to_string());
        if let Some(ext) = extension {
            args.push("--audio-format".to_string());
            args.push(ext.to_string());
        }
    } else if let Some(ext) = extension {
        args.push("--merge-output-format".to_string());
        args.push(ext.to_string());
    }

    if embeds {
        args.push("--embed-thumbnail".to_string());
        args.push("--embed-metadata".to_string());
    }

    // "--" keeps anything in the link from being read as an option
    args.push("--".to_string());
    args.push(link.to_string());
    args
}

#[tauri::command]
async fn download_from_link(
    link: String,
    folder: String,
    format: String,
    extension: Option<String>,
    embeds: bool,
    app_handle: AppHandle,
) -> Result<DownloadResult, String> {
    let yt_dlp = find_tool("yt-dlp")
        .ok_or("yt-dlp is not installed. Install it from Settings and try again.")?;

    tokio::fs::create_dir_all(&folder)
        .await
        .map_err(|e| format!("Failed to create download folder: {}", e))?;

    let ffmpeg_dir = find_system_ffmpeg_dir().or_else(|| find_bundled_ffmpeg_dir(&app_handle));
    let args = build_download_args(&link, &format, extension.as_deref(), embeds, ffmpeg_dir.as_deref());

    let mut cmd = hidden_command(&yt_dlp);
    cmd.current_dir(&folder)
        .env("PYTHONIOENCODING", "utf-8")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(&args);

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("Failed to start yt-dlp: {}", e))?;

    CANCEL_REQUESTED.store(false, Ordering::SeqCst);
    *ACTIVE_DOWNLOAD_PID.lock().unwrap() = child.id();

    let stdout = child.stdout.take().ok_or("Failed to capture yt-dlp output")?;
    let stderr = child.stderr.take().ok_or("Failed to capture yt-dlp output")?;

    let downloaded_file = Arc::new(Mutex::new(None::<String>));
    let last_error = Arc::new(Mutex::new(None::<String>));

    let stdout_task = {
        let app_handle = app_handle.clone();
        let downloaded_file = downloaded_file.clone();
        let folder = folder.clone();
        tokio::spawn(async move {
            let mut last_emit: Option<Instant> = None;
            for_each_line(stdout, |line| {
                if let Some(rest) = line.strip_prefix(PROGRESS_MARKER) {
                    if last_emit.map_or(true, |at| at.elapsed() >= PROGRESS_EMIT_INTERVAL) {
                        last_emit = Some(Instant::now());
                        let _ = app_handle.emit("download-progress", parse_progress(rest));
                    }
                } else if let Some(path) = line.strip_prefix(FILE_MARKER) {
                    let full_path = Path::new(&folder).join(path).display().to_string();
                    emit_console_line(&app_handle, format!("Saved {}", full_path));
                    *downloaded_file.lock().unwrap() = Some(full_path);
                } else {
                    emit_console_line(&app_handle, line);
                }
            })
            .await;
        })
    };

    let stderr_task = {
        let app_handle = app_handle.clone();
        let last_error = last_error.clone();
        tokio::spawn(async move {
            for_each_line(stderr, |line| {
                if let Some(message) = line.strip_prefix("ERROR:") {
                    *last_error.lock().unwrap() = Some(message.trim().to_string());
                }
                emit_console_line(&app_handle, line);
            })
            .await;
        })
    };

    let status = child.wait().await;
    let _ = tokio::join!(stdout_task, stderr_task);
    *ACTIVE_DOWNLOAD_PID.lock().unwrap() = None;

    let cancelled = CANCEL_REQUESTED.swap(false, Ordering::SeqCst);
    let file_path = downloaded_file.lock().unwrap().clone();
    let exited_cleanly = status.as_ref().map(|s| s.success()).unwrap_or(false);
    // A search with no results also exits cleanly, so a success has to have produced a file
    let succeeded = exited_cleanly && file_path.is_some();
    let error = last_error.lock().unwrap().clone();

    let message = if cancelled {
        "Download cancelled.".to_string()
    } else if succeeded {
        "Download complete.".to_string()
    } else if exited_cleanly {
        "Nothing was downloaded. A search may have found no results.".to_string()
    } else {
        error.unwrap_or_else(|| match status {
            Ok(status) => format!("yt-dlp exited with {}.", status),
            Err(e) => format!("yt-dlp stopped unexpectedly: {}", e),
        })
    };

    Ok(DownloadResult {
        success: succeeded && !cancelled,
        cancelled,
        message,
        file_path,
    })
}

// Fields are downloaded|total|total_estimate|speed|eta|title; yt-dlp writes NA for unknowns.
fn parse_progress(fields: &str) -> DownloadProgressEvent {
    let parts: Vec<&str> = fields.splitn(6, '|').collect();
    let number = |index: usize| parts.get(index).and_then(|value| value.parse::<f64>().ok());

    let total_bytes = number(1).or_else(|| number(2));
    let percentage = match (number(0), total_bytes) {
        (Some(done), Some(total)) if total > 0.0 => Some((done / total * 100.0).min(100.0)),
        _ => None,
    };

    DownloadProgressEvent {
        percentage,
        speed: number(3),
        eta: number(4),
        total_bytes,
        title: parts.get(5).filter(|title| **title != "NA").map(|title| title.to_string()),
    }
}

#[tauri::command]
async fn cancel_download() -> Result<(), String> {
    let pid = *ACTIVE_DOWNLOAD_PID.lock().unwrap();
    let Some(pid) = pid else {
        return Ok(());
    };
    CANCEL_REQUESTED.store(true, Ordering::SeqCst);

    // The whole tree has to go: yt-dlp.exe is a launcher with the real process and
    // ffmpeg running underneath it.
    let mut cmd = if cfg!(target_os = "windows") {
        let mut cmd = hidden_command("taskkill");
        cmd.args(["/PID", &pid.to_string(), "/T", "/F"]);
        cmd
    } else {
        let mut cmd = hidden_command("kill");
        cmd.arg(pid.to_string());
        cmd
    };

    cmd.output()
        .await
        .map(|_| ())
        .map_err(|e| format!("Failed to cancel download: {}", e))
}

#[derive(serde::Serialize)]
struct DependencyStatus {
    yt_dlp_installed: bool,
    ffmpeg_source: String,
}

#[tauri::command]
async fn get_dependency_status(app_handle: AppHandle) -> DependencyStatus {
    let ffmpeg_source = if find_system_ffmpeg_dir().is_some() {
        "System"
    } else if find_bundled_ffmpeg_dir(&app_handle).is_some() {
        "Bundled"
    } else {
        "Missing"
    };

    DependencyStatus {
        yt_dlp_installed: find_tool("yt-dlp").is_some(),
        ffmpeg_source: ffmpeg_source.to_string(),
    }
}

#[tauri::command]
async fn check_yt_dlp_version() -> Option<String> {
    let output = hidden_command(find_tool("yt-dlp")?)
        .arg("--version")
        .output()
        .await
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[derive(serde::Serialize)]
struct InstallResult {
    success: bool,
    message: String,
}

async fn command_available(command: &str, args: &[&str]) -> bool {
    hidden_command(command)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map(|status| status.success())
        .unwrap_or(false)
}

// Runs each installer in turn until one succeeds; returns its name or the last error.
async fn run_attempts(attempts: Vec<(&str, Vec<&str>)>) -> Result<String, String> {
    if attempts.is_empty() {
        return Err("No supported installer was found on this computer.".to_string());
    }

    let mut last_error = String::from("Unknown error");
    for (command, args) in attempts {
        match hidden_command(command).args(&args).output().await {
            Ok(output) if output.status.success() => return Ok(command.to_string()),
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                last_error = format!("{} {}", stdout.trim(), stderr.trim()).trim().to_string();
            }
            Err(error) => last_error = error.to_string(),
        }
    }
    Err(last_error)
}

#[tauri::command]
async fn install_yt_dlp() -> InstallResult {
    let mut attempts: Vec<(&str, Vec<&str>)> = Vec::new();

    // User-scope installers first (no admin needed)
    if command_available("pipx", &["--version"]).await {
        attempts.push(("pipx", vec!["install", "yt-dlp"]));
    }
    if cfg!(target_os = "windows") {
        if command_available("winget", &["--version"]).await {
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
        if command_available("python", &["--version"]).await {
            attempts.push(("python", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
    } else {
        if command_available("brew", &["--version"]).await {
            attempts.push(("brew", vec!["install", "yt-dlp"]));
        }
        if command_available("python3", &["--version"]).await {
            attempts.push(("python3", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
    }

    match run_attempts(attempts).await {
        Ok(_) if find_tool("yt-dlp").is_some() => InstallResult {
            success: true,
            message: "yt-dlp installed.".to_string(),
        },
        Ok(command) => InstallResult {
            success: false,
            message: format!("{} finished, but yt-dlp wasn't found. Restart the app and try again.", command),
        },
        Err(error) => InstallResult {
            success: false,
            message: format!("yt-dlp install failed: {}", error),
        },
    }
}

#[tauri::command]
async fn install_ffmpeg() -> InstallResult {
    let mut attempts: Vec<(&str, Vec<&str>)> = Vec::new();

    if cfg!(target_os = "macos") {
        if command_available("brew", &["--version"]).await {
            attempts.push(("brew", vec!["install", "ffmpeg"]));
        }
    } else if cfg!(target_os = "windows") {
        if command_available("winget", &["--version"]).await {
            for id in ["Gyan.FFmpeg", "FFmpeg.FFmpeg"] {
                attempts.push((
                    "winget",
                    vec![
                        "install",
                        "-e",
                        "--id",
                        id,
                        "--accept-source-agreements",
                        "--accept-package-agreements",
                    ],
                ));
            }
        }
    } else if command_available("sudo", &["-n", "true"]).await {
        attempts.push(("sudo", vec!["apt-get", "install", "-y", "ffmpeg"]));
    } else {
        attempts.push(("apt-get", vec!["install", "-y", "ffmpeg"]));
    }

    match run_attempts(attempts).await {
        Ok(_) if find_system_ffmpeg_dir().is_some() => InstallResult {
            success: true,
            message: "FFmpeg installed.".to_string(),
        },
        Ok(command) => InstallResult {
            success: false,
            message: format!("{} finished, but FFmpeg wasn't found. Restart the app and try again.", command),
        },
        Err(error) => InstallResult {
            success: false,
            message: format!("FFmpeg install failed: {}. You can install it from https://ffmpeg.org", error),
        },
    }
}

#[tauri::command]
async fn update_yt_dlp() -> InstallResult {
    let mut attempts: Vec<(&str, Vec<&str>)> = Vec::new();

    // Standalone builds update themselves; package manager installs refuse and fall through.
    let yt_dlp = find_tool("yt-dlp").map(|path| path.display().to_string());
    if let Some(path) = yt_dlp.as_deref() {
        attempts.push((path, vec!["-U"]));
    }
    if command_available("pipx", &["--version"]).await {
        attempts.push(("pipx", vec!["upgrade", "yt-dlp"]));
    }
    if cfg!(target_os = "windows") {
        if command_available("winget", &["--version"]).await {
            attempts.push(("winget", vec!["upgrade", "-e", "--id", "yt-dlp.yt-dlp"]));
        }
        if command_available("python", &["--version"]).await {
            attempts.push(("python", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
    } else {
        if command_available("brew", &["--version"]).await {
            attempts.push(("brew", vec!["upgrade", "yt-dlp"]));
        }
        if command_available("python3", &["--version"]).await {
            attempts.push(("python3", vec!["-m", "pip", "install", "-U", "--user", "yt-dlp"]));
        }
    }

    match run_attempts(attempts).await {
        Ok(_) => InstallResult {
            success: true,
            message: "yt-dlp is up to date.".to_string(),
        },
        Err(error) => InstallResult {
            success: false,
            message: format!("yt-dlp update failed: {}", error),
        },
    }
}

#[tauri::command]
async fn open_folder_location(path: String) -> Result<(), String> {
    // Normalize the path - convert forward slashes to backslashes on Windows
    let path = if cfg!(target_os = "windows") {
        path.replace('/', "\\")
    } else {
        path
    };

    if !Path::new(&path).exists() {
        return Err(format!("That folder no longer exists: {}", path));
    }

    let opener = if cfg!(target_os = "windows") {
        "explorer.exe"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        "xdg-open"
    };

    hidden_command(opener)
        .arg(&path)
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Failed to open folder: {}", e))
}
