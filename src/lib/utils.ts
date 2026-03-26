import { SETTINGS_KEYS, FORMATS } from "$lib/assets/keys";
import { Store } from "@tauri-apps/plugin-store";
import { writable, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type {
    Status,
    ConsoleLogEntry,
    DownloadProgressState,
    DownloadFormat,
    DownloadPayload,
    DownloadResult,
    YtDlpInstallResult,
    FfmpegInstallResult,
    InstallResult,
    DependencyStatus,
    InstallationError
} from "$lib/types";

export let settings: Store | null = null;

export const initializeSettings = async (): Promise<void> => {
    settings = await Store.load("settings.json");
    
    // Load saved download folder
    const savedFolder = await settings.get(SETTINGS_KEYS.MUSIC_FOLDER_PATH.id);
    if (savedFolder && typeof savedFolder === 'string') {
        selectedDownloadFolder.set(savedFolder);
    }
    
    // Load saved format preference
    const savedFormat = await settings.get(SETTINGS_KEYS.DEFAULT_FORMAT.id);
    if (savedFormat && (savedFormat === FORMATS.AUDIO || savedFormat === FORMATS.VIDEO)) {
        defaultFormat.set(savedFormat);
    }
    
    // Load embed metadata preference
    const savedEmbed = await settings.get(SETTINGS_KEYS.EMBED_METADATA.id);
    if (typeof savedEmbed === 'boolean') {
        embedMetadata.set(savedEmbed);
    }
    
    // Load saved extension preferences
    const savedAudioExt = await settings.get(SETTINGS_KEYS.DEFAULT_AUDIO_EXTENSION.id);
    if (savedAudioExt && typeof savedAudioExt === 'string') {
        defaultAudioExtension.set(savedAudioExt);
    }
    
    const savedVideoExt = await settings.get(SETTINGS_KEYS.DEFAULT_VIDEO_EXTENSION.id);
    if (savedVideoExt && typeof savedVideoExt === 'string') {
        defaultVideoExtension.set(savedVideoExt);
    }
    
    // Initialize event listeners for download progress
    await initializeEventListeners();
};

// Initialize Tauri event listeners for download progress
export const initializeEventListeners = async (): Promise<void> => {
    try {
        // Listen for console output
        await listen('console-output', (event: any) => {
            const logEntry: ConsoleLogEntry = event.payload;
            consoleOutput.update((logs) => {
                const newLogs = [...logs, logEntry];
                // Keep only the last 100 entries to prevent memory issues
                return newLogs.slice(-100);
            });
            
            // Update status with the latest console line
            setStatus(logEntry.line, "info");
        });
    
        // Listen for download completion
        await listen('download-complete', (event: any) => {
            const result = event.payload;
            isDownloading.set(false);
            
            if (result.success) {
                setStatus(result.message, "success");
            } else {
                setStatus(result.message, "error");
            }
        });
        
        console.log("Event listeners initialized successfully");
    } catch (error) {
        console.error("Failed to initialize event listeners:", error);
    }
};

export const installYtDlp = async (): Promise<YtDlpInstallResult> => {
    return await invoke<YtDlpInstallResult>("install_yt_dlp");
};

export const checkFfmpegInstalled = async (): Promise<boolean> => {
    return await invoke<boolean>("check_ffmpeg_installed");
};

export const installFfmpeg = async (): Promise<FfmpegInstallResult> => {
    return await invoke<FfmpegInstallResult>("install_ffmpeg");
};

export const checkYtDlpVersion = async (): Promise<[string | null, string | null, boolean]> => {
    return await invoke<[string | null, string | null, boolean]>("check_yt_dlp_version");
};

export const getDependencyStatus = async (): Promise<any> => {
    return await invoke<any>("get_dependency_status");
};

export const updateYtDlp = async (): Promise<InstallResult> => {
    return await invoke<InstallResult>("update_yt_dlp");
};

export const getYtDlpVersion = async (): Promise<[string | null, string | null, boolean]> => {
    return await checkYtDlpVersion();
};

export const updateStatus = (newStatus: Status): void => {
    status.set(newStatus);
};

export const getStatus = (): Status => {
    return get(status);
};

export const readSettingsWithKey = async (key: string): Promise<any | null> => {
    if (settings) {
        return await settings.get(key);
    }
    return null;
};

export const updateSettingsWithKey = async (
    key: string,
    value: any
): Promise<void> => {
    if (settings) {
        await settings.set(key, value);
        await settings.save();
    } else {
        console.error("Settings store is not initialized.");
    }
};

import { open } from "@tauri-apps/plugin-dialog";
export const selectedDownloadFolder = writable<string | null>(null);

export async function browseFolder() {
    const result = await open({
        directory: true,
        multiple: false,
        title: "Select Download folder",
        defaultPath: await getDefaultDownloadPath(),
    });

    if (typeof result === "string") {
        selectedDownloadFolder.set(result);
        // Refresh subfolders when root folder changes
        await refreshSubfolders();
        updateStatus({
            type: "success",
            message: `Selected folder: ${result}`,
        });
    }
}

// New stores for destination picking
export const currentDestination = writable<string>("root");
export const availableSubfolders = writable<string[]>([]);
export const newSubfolderName = writable<string>("");

// Scan subfolders in the selected download folder
export async function refreshSubfolders() {
    const rootFolder = get(selectedDownloadFolder);
    if (!rootFolder) {
        availableSubfolders.set([]);
        return;
    }

    try {
        console.log(`Scanning subfolders in: ${rootFolder}`);
        
        // Call the backend list_folders command
        const folders = await invoke<string[]>("list_folders", {
            path: rootFolder
        });
        
        console.log(`Found ${folders.length} subfolders:`, folders);
        availableSubfolders.set(folders);
    } catch (error) {
        console.error("Failed to read subfolders:", error);
        availableSubfolders.set([]);
        
        // Update status to show the error
        setStatus(`Failed to scan subfolders: ${error}`, "warning");
    }
}

// Create new subfolder in the root directory
export async function createSubfolder(folderName: string): Promise<boolean> {
    const rootFolder = get(selectedDownloadFolder);
    if (!rootFolder || !folderName.trim()) {
        return false;
    }

    try {
        // For now, we'll use the download process to create folders
        // The actual folder creation will be handled by the backend
        updateStatus({
            type: "info",
            message: `Will create folder "${folderName}" during download`,
        });
        return true;
    } catch (error) {
        updateStatus({
            type: "error",
            message: `Failed to create folder: ${error}`,
        });
        return false;
    }
}

// Get the final download path based on current destination selection
export function getDownloadPath(): string | null {
    const rootFolder = get(selectedDownloadFolder);
    const destination = get(currentDestination);
    
    if (!rootFolder) return null;
    
    if (destination === "root") {
        return rootFolder;
    } else if (destination === "browse") {
        // This will be handled by browse dialog in download components
        return null;
    } else {
        // It's a subfolder name
        return `${rootFolder}/${destination}`;
    }
}

// Helper function to get default download path
async function getDefaultDownloadPath(): Promise<string | undefined> {
    try {
        // For desktop apps, we'll just default to a common location
        // The user can browse from there
        return undefined; // Let the dialog open in default location
    } catch {
        return undefined;
    }
}

// Dependency and environment status stores
export const dependencyStatus = writable<DependencyStatus | null>(null);
export const isDependenciesReady = writable<boolean>(false);

export const checkDependenciesOnLaunch = async (): Promise<void> => {
    try {
        const status = await getDependencyStatus();
        dependencyStatus.set(status);
        
        // Downloads can proceed only if yt-dlp is installed
        const ready = status.yt_dlp_installed;
        isDependenciesReady.set(ready);
        
        if (!ready) {
            setStatus("yt-dlp is required to download. Please install it in Settings.", "error");
        } else if (status.yt_dlp_version) {
            // Show version info in console for debugging
            console.log(`✓ yt-dlp is ready (version: ${status.yt_dlp_version})`);
            
            // Check ffmpeg status
            if (status.ffmpeg_source === "System") {
                console.log("✓ FFmpeg found on system PATH");
            } else if (status.ffmpeg_source === "Bundled") {
                console.log("✓ Using bundled FFmpeg");
            } else {
                console.warn("⚠ FFmpeg not found - downloads requiring video merging will fail");
                setStatus("⚠ FFmpeg not found. Audio-only downloads should work, but video downloads may fail.", "warning");
            }
        }
    } catch (error) {
        console.error("Failed to check dependencies:", error);
        setStatus("Failed to check dependencies. Some features may not work.", "warning");
    }
};

export const installYtDlpElevated = async (): Promise<InstallResult> => {
    return await invoke<InstallResult>("install_yt_dlp_elevated");
};

export const checkYtDlpInstalled = async (): Promise<boolean> => {
    return await invoke<boolean>("check_yt_dlp_installed");
};

export const debugGetFfmpegPath = async (): Promise<string> => {
    return await invoke<string>("get_ffmpeg_path");
};

//* Always synchronizes `lastActiveTab` with the settings store.
export const activeTab = writable<string>("single");
export const setActiveTab = async (tab: string): Promise<void> => {
    activeTab.set(tab);
    await updateSettingsWithKey(SETTINGS_KEYS.ACTIVE_TAB.id, tab);
};

export const firstRun = writable<boolean>(false);
export const status = writable<Status>({
    type: null,
    message: "",
});

// New stores for download progress and console output
export const consoleOutput = writable<ConsoleLogEntry[]>([]);
export const isDownloading = writable<boolean>(false);

export const setStatus = (message: string, type: Status["type"]): void => {
    status.set({ message: message, type: type });
};

export async function saveDownloadFolderSetting() {
    if (get(selectedDownloadFolder)) {
        await updateSettingsWithKey(
            SETTINGS_KEYS.MUSIC_FOLDER_PATH.id,
            get(selectedDownloadFolder)
        );
        firstRun.set(false);
        status.set({
            type: "success",
            message: "Download folder saved successfully.",
        });
        return true;
    } else {
        status.set({
            type: "error",
            message: "Please select a valid download folder.",
        });
        return false;
    }
}

export function getStatusColor(type: Status["type"] | null = null): string {
    const resolvedType = type ?? get(status).type;
    switch (resolvedType) {
        case "success":
            return "#4dff88";
        case "error":
            return "#ff4d4d";
        case "warning":
            return "#fec600";
        default:
            return "white";
    }
}

// New stores for download settings
export const defaultFormat = writable<DownloadFormat>(FORMATS.AUDIO);
export const embedMetadata = writable<boolean>(true);
export const defaultAudioExtension = writable<string>("");
export const defaultVideoExtension = writable<string>("");

// Download function
export async function downloadFromLink(payload: DownloadPayload): Promise<DownloadResult> {
    const trimmedLink = payload.link?.trim();
    if (!trimmedLink) {
        throw new Error("Please provide a link to download.");
    }

    const folder = payload.folder?.trim() ?? get(selectedDownloadFolder);
    if (!folder) {
        throw new Error("Please set a download folder in Settings first.");
    }

    try {
        // Set downloading state
        isDownloading.set(true);
        setStatus("Starting download...", "info");
        
        // Clear previous progress and console output
        consoleOutput.set([]);
        
        const result = await invoke<DownloadResult>("download_from_link", {
            link: trimmedLink,
            folder,
            format: payload.format,
            extension: payload.extension?.trim() || null,
            embeds: payload.embeds ?? get(embedMetadata),
        });
        
        // The download completion will be handled by the event listener
        return result;
    } catch (error) {
        console.error("Download error:", error);
        isDownloading.set(false);
        
        const errorMessage = error instanceof Error ? error.message : String(error);
        setStatus(`Download failed: ${errorMessage}`, "error");
        throw new Error(errorMessage);
    }
}
