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
};

// UI-specific types
export type HistoryItem = {
    link: string;
    status: "success" | "error";
    message: string;
    timestamp: Date;
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
};