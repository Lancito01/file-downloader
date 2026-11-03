// Shared TypeScript types for the File Downloader application

export type Status = {
    type: "success" | "error" | "warning" | "info" | null;
    message: string;
};

export type ConsoleLogEntry = {
    line: string;
    timestamp: string;
};

export type DownloadFormat = "audio" | "video";

export type SearchPlatform = "youtube" | "soundcloud";

export type DownloadOptions = {
    format: DownloadFormat;
    audioExtension: string;
    videoExtension: string;
    embed: boolean;
};

export type DestinationKind = "root" | "subfolder" | "new" | "browse";

export type Destination = {
    kind: DestinationKind;
    subfolder: string;
    newName: string;
    custom: string;
};

export type DownloadPayload = {
    link: string;
    folder: string;
    options: DownloadOptions;
};

export type DownloadResult = {
    success: boolean;
    cancelled: boolean;
    message: string;
    file_path?: string | null;
};

// Live state of the running download, built from backend events
export type DownloadProgress = {
    phase: "starting" | "downloading" | "processing";
    title?: string | null;
    percentage?: number | null;
    speed?: number | null;
    eta?: number | null;
    total_bytes?: number | null;
};

// UI-specific types
export type HistoryItem = {
    link: string;
    status: "success" | "error";
    message: string;
    timestamp: string;
    folder: string;
    file_path?: string | null;
};

export type QueueItem = {
    link: string;
    status: "pending" | "downloading" | "success" | "error" | "cancelled";
    message: string;
    folder?: string;
    file_path?: string | null;
};

export type InstallResult = {
    success: boolean;
    message: string;
};

export type DependencyStatus = {
    yt_dlp_installed: boolean;
    ffmpeg_source: "System" | "Bundled" | "Missing";
};

export type UpdateState =
    | { phase: "idle" | "checking" | "current" }
    | { phase: "available"; version: string }
    | { phase: "installing"; version: string; percentage: number | null }
    | { phase: "error"; message: string };
