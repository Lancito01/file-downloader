// Shared TypeScript types for the File Downloader application

export type Status = {
    type: "success" | "error" | "warning" | "info" | null;
    message: string;
};

export type ConsoleLogEntry = {
    line: string;
    timestamp: string;
};

export type DownloadProgressState = {
    percentage?: number;
    speed?: string;
    eta?: string;
    size?: string;
};

export type DownloadFormat = "audio" | "video";

export type DownloadPayload = {
    link: string;
    folder?: string;
    format: DownloadFormat;
    extension?: string;
    embeds?: boolean;
};

export type DownloadResult = {
    success: boolean;
    message: string;
    file_path?: string;
};

// UI-specific types
export type HistoryItem = {
    link: string;
    status: "success" | "error";
    message: string;
    timestamp: Date;
    folder?: string;
    file_path?: string;
};

export type QueueItem = {
    link: string;
    status: "pending" | "downloading" | "success" | "error";
    message: string;
    timestamp?: Date;
};

// Backend event types (matching Rust structs)
export type ConsoleOutputEvent = {
    line: string;
    timestamp: string;
};

export type DownloadProgressEvent = {
    percentage?: number;
    speed?: string;
    eta?: string;
    size?: string;
};

export type DownloadCompleteEvent = {
    success: boolean;
    message: string;
    file_path?: string;
};

export type YtDlpInstallResult = {
    success: boolean;
    message: string;
};

export type FfmpegInstallResult = {
    success: boolean;
    message: string;
};

// Generic installation result returned by the Tauri backend for dependency installs/updates.
export type InstallResult = {
    success: boolean;
    message: string;
    error_code?: string | null;
    suggested_action?: string | null;
};

export type DependencyStatus = {
    yt_dlp_installed: boolean;
    yt_dlp_version: string | null;
    yt_dlp_outdated: boolean;
    ffmpeg_source: "System" | "Bundled" | "Missing";
};

export type DependencySource = "System" | "Bundled" | "Missing";

export type InstallationError = {
    message: string;
    error_code?: string;
    suggested_action?: string;
};
