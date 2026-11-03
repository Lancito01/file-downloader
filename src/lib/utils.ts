import { SETTINGS_KEYS, FORMATS } from "$lib/assets/keys";
import { Store } from "@tauri-apps/plugin-store";
import { open } from "@tauri-apps/plugin-dialog";
import { writable, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import type {
    Status,
    ConsoleLogEntry,
    Destination,
    DownloadFormat,
    DownloadOptions,
    DownloadPayload,
    DownloadProgress,
    DownloadResult,
    HistoryItem,
    InstallResult,
    DependencyStatus,
    QueueItem,
    SearchPlatform,
    UpdateState,
} from "$lib/types";

const MAX_CONSOLE_LINES = 300;
const MAX_HISTORY_ITEMS = 30;
const POST_PROCESSING_LINE = /^\[(Merger|ExtractAudio|Fixup\w*|Metadata|EmbedThumbnail|ThumbnailsConvertor|VideoRemuxer|VideoConvertor|MoveFiles)\]/;

let settings: Store | null = null;

//? Status
export const status = writable<Status>({ type: null, message: "" });

export const setStatus = (message: string, type: Status["type"]): void => {
    status.set({ message, type });
};

const errorMessage = (error: unknown): string =>
    error instanceof Error ? error.message : String(error);

//? App state
export const isReady = writable<boolean>(false);
export const firstRun = writable<boolean>(false);
export const activeTab = writable<string>("single");
export const selectedDownloadFolder = writable<string | null>(null);
export const availableSubfolders = writable<string[]>([]);
export const history = writable<HistoryItem[]>([]);

// Defaults are what Settings edits and what gets saved. The options are the controls
// on the download pages: they start from the defaults and can be changed per download.
const FALLBACK_OPTIONS: DownloadOptions = {
    format: FORMATS.AUDIO,
    audioExtension: "",
    videoExtension: "",
    embed: true,
};
export const defaultOptions = writable<DownloadOptions>({ ...FALLBACK_OPTIONS });
export const downloadOptions = writable<DownloadOptions>({ ...FALLBACK_OPTIONS });
export const searchPlatform = writable<SearchPlatform>("youtube");
export const destination = writable<Destination>({
    kind: "root",
    subfolder: "",
    newName: "",
    custom: "",
});

//? Download state
export const consoleOutput = writable<ConsoleLogEntry[]>([]);
export const isDownloading = writable<boolean>(false);
export const downloadProgress = writable<DownloadProgress | null>(null);
export const bulkRunning = writable<boolean>(false);
export const bulkQueue = writable<QueueItem[]>([]);

//? Dependencies - null until the first check comes back
export const dependencyStatus = writable<DependencyStatus | null>(null);
export const ytDlpVersion = writable<string | null>(null);

//? Settings persistence
const saveSetting = async (key: string, value: unknown): Promise<void> => {
    if (!settings) {
        console.error("Settings store is not initialized.");
        return;
    }
    await settings.set(key, value);
    await settings.save();
};

export const initializeSettings = async (): Promise<void> => {
    settings = await Store.load("settings.json");

    const savedTab = await settings.get<string>(SETTINGS_KEYS.ACTIVE_TAB.id);
    if (savedTab) {
        activeTab.set(savedTab);
    }

    const savedFolder = await settings.get<string>(SETTINGS_KEYS.MUSIC_FOLDER_PATH.id);
    if (savedFolder) {
        selectedDownloadFolder.set(savedFolder);
        refreshSubfolders();
    } else {
        firstRun.set(true);
    }

    const savedFormat = await settings.get<DownloadFormat>(SETTINGS_KEYS.DEFAULT_FORMAT.id);
    const savedEmbed = await settings.get<boolean>(SETTINGS_KEYS.EMBED_METADATA.id);
    const savedAudioExt = await settings.get<string>(SETTINGS_KEYS.DEFAULT_AUDIO_EXTENSION.id);
    const savedVideoExt = await settings.get<string>(SETTINGS_KEYS.DEFAULT_VIDEO_EXTENSION.id);

    const options: DownloadOptions = {
        format: savedFormat === FORMATS.VIDEO ? FORMATS.VIDEO : FORMATS.AUDIO,
        embed: typeof savedEmbed === "boolean" ? savedEmbed : FALLBACK_OPTIONS.embed,
        audioExtension: savedAudioExt ?? FALLBACK_OPTIONS.audioExtension,
        videoExtension: savedVideoExt ?? FALLBACK_OPTIONS.videoExtension,
    };
    defaultOptions.set(options);
    downloadOptions.set({ ...options });

    history.set((await settings.get<HistoryItem[]>(SETTINGS_KEYS.HISTORY.id)) ?? []);

    await initializeEventListeners();
};

const DEFAULT_OPTION_KEYS: Record<keyof DownloadOptions, string> = {
    format: SETTINGS_KEYS.DEFAULT_FORMAT.id,
    audioExtension: SETTINGS_KEYS.DEFAULT_AUDIO_EXTENSION.id,
    videoExtension: SETTINGS_KEYS.DEFAULT_VIDEO_EXTENSION.id,
    embed: SETTINGS_KEYS.EMBED_METADATA.id,
};

export async function setDefaultOption<K extends keyof DownloadOptions>(
    key: K,
    value: DownloadOptions[K]
): Promise<void> {
    defaultOptions.update((options) => ({ ...options, [key]: value }));
    downloadOptions.update((options) => ({ ...options, [key]: value }));
    await saveSetting(DEFAULT_OPTION_KEYS[key], value);
}

export const setActiveTab = async (tab: string): Promise<void> => {
    activeTab.set(tab);
    await saveSetting(SETTINGS_KEYS.ACTIVE_TAB.id, tab);
};

//? Backend events
const initializeEventListeners = async (): Promise<void> => {
    await listen<ConsoleLogEntry>("console-output", (event) => {
        consoleOutput.update((logs) => [...logs, event.payload].slice(-MAX_CONSOLE_LINES));

        if (POST_PROCESSING_LINE.test(event.payload.line)) {
            downloadProgress.update((progress) =>
                progress ? { ...progress, phase: "processing", percentage: null, speed: null, eta: null } : progress
            );
        }
    });

    await listen<Omit<DownloadProgress, "phase">>("download-progress", (event) => {
        downloadProgress.set({ ...event.payload, phase: "downloading" });
    });
};

//? Dependencies
export const checkDependencies = async (): Promise<void> => {
    try {
        const result = await invoke<DependencyStatus>("get_dependency_status");
        dependencyStatus.set(result);
    } catch (error) {
        setStatus(`Failed to check dependencies: ${errorMessage(error)}`, "warning");
    }
};

export const refreshYtDlpVersion = async (): Promise<void> => {
    ytDlpVersion.set(await invoke<string | null>("check_yt_dlp_version"));
};

// Runs an install or update command, reporting through the status bar
async function runInstall(
    command: string,
    progressMessage: string
): Promise<void> {
    setStatus(progressMessage, "info");
    try {
        const result = await invoke<InstallResult>(command);
        setStatus(result.message, result.success ? "success" : "error");
    } catch (error) {
        setStatus(errorMessage(error), "error");
    }
    await checkDependencies();
}

export const installYtDlp = async (): Promise<void> => {
    await runInstall("install_yt_dlp", "Installing yt-dlp. This can take a minute...");
    await refreshYtDlpVersion();
};

export const installFfmpeg = (): Promise<void> =>
    runInstall("install_ffmpeg", "Installing FFmpeg. This can take a few minutes...");

export const updateYtDlp = async (): Promise<void> => {
    await runInstall("update_yt_dlp", "Updating yt-dlp...");
    await refreshYtDlpVersion();
};

//? Folders
export async function browseFolder(): Promise<void> {
    const result = await open({
        directory: true,
        multiple: false,
        title: "Select download folder",
        defaultPath: get(selectedDownloadFolder) ?? undefined,
    });

    if (typeof result !== "string") return;

    try {
        // Saved right away: picking a folder is the whole action, there's no second step.
        selectedDownloadFolder.set(result);
        await saveSetting(SETTINGS_KEYS.MUSIC_FOLDER_PATH.id, result);
        firstRun.set(false);
        destination.update((current) => ({ ...current, kind: "root", subfolder: "" }));
        setStatus(`Downloads will be saved to ${result}`, "success");
        await refreshSubfolders();
    } catch (error) {
        setStatus(`Failed to save download folder: ${errorMessage(error)}`, "error");
    }
}

export async function browseDestination(): Promise<void> {
    const result = await open({
        directory: true,
        multiple: false,
        title: "Select destination folder",
    });

    if (typeof result === "string") {
        destination.update((current) => ({ ...current, kind: "browse", custom: result }));
    }
}

export async function refreshSubfolders(): Promise<void> {
    const rootFolder = get(selectedDownloadFolder);
    if (!rootFolder) {
        availableSubfolders.set([]);
        return;
    }

    try {
        availableSubfolders.set(await invoke<string[]>("list_folders", { path: rootFolder }));
    } catch (error) {
        availableSubfolders.set([]);
        setStatus(errorMessage(error), "warning");
    }
}

export async function openFolder(path: string): Promise<void> {
    try {
        await invoke("open_folder_location", { path });
    } catch (error) {
        setStatus(errorMessage(error), "error");
    }
}

// Works out the folder for the current destination; throws a message for the user if incomplete
export function resolveDownloadFolder(): string {
    const rootFolder = get(selectedDownloadFolder);
    if (!rootFolder) {
        throw new Error("Please set a download folder in Settings first.");
    }

    const { kind, subfolder, newName, custom } = get(destination);
    if (kind === "browse") {
        if (!custom) throw new Error("Please select a destination folder.");
        return custom;
    }
    if (kind === "subfolder") {
        if (!subfolder) throw new Error("Please select a subfolder.");
        return `${rootFolder}/${subfolder}`;
    }
    if (kind === "new") {
        const name = newName.trim();
        if (!name) throw new Error("Please enter a name for the new folder.");
        if (/[\\/:*?"<>|]|^\.+$/.test(name)) {
            throw new Error('Folder names can\'t contain \\ / : * ? " < > |');
        }
        return `${rootFolder}/${name}`;
    }
    return rootFolder;
}

//? Links
const SEARCH_PREFIX = /^(yt|sc|bc)search\d*:/;

// Only http(s) counts as a link, so a search like "artist:song" isn't mistaken for a URL
export function isUrl(input: string): boolean {
    return /^https?:\/\/\S+$/i.test(input.trim());
}

export function isSearchTerm(input: string): boolean {
    const trimmed = input.trim();
    return trimmed.length > 0 && !isUrl(trimmed) && !SEARCH_PREFIX.test(trimmed);
}

// Apply search platform prefix to non-URL queries
export function applySearchPrefix(input: string): string {
    const trimmed = input.trim();
    if (!isSearchTerm(trimmed)) return trimmed;
    return (get(searchPlatform) === "youtube" ? "ytsearch:" : "scsearch:") + trimmed;
}

// Readable label for a link: the search words without the prefix
export const displayLink = (link: string): string => link.replace(SEARCH_PREFIX, "");

export const fileNameOf = (path: string): string => path.split(/[/\\]/).pop() ?? path;

//? App updates
export const appVersion = writable<string | null>(null);
export const updateState = writable<UpdateState>({ phase: "idle" });

let pendingUpdate: Update | null = null;

// Quiet checks (the one at launch) stay silent when offline or when GitHub can't be reached
export async function checkForUpdates(quiet = false): Promise<void> {
    updateState.set({ phase: "checking" });

    try {
        pendingUpdate = await check();
        updateState.set(
            pendingUpdate ? { phase: "available", version: pendingUpdate.version } : { phase: quiet ? "idle" : "current" }
        );
    } catch (error) {
        updateState.set(quiet ? { phase: "idle" } : { phase: "error", message: errorMessage(error) });
    }
}

// Installing closes the app, so this must never run during a download
export async function installUpdate(): Promise<void> {
    if (!pendingUpdate || get(isDownloading) || get(bulkRunning)) return;

    const version = pendingUpdate.version;
    let total = 0;
    let received = 0;
    updateState.set({ phase: "installing", version, percentage: null });

    try {
        await pendingUpdate.downloadAndInstall((event) => {
            if (event.event === "Started") {
                total = event.data.contentLength ?? 0;
            } else if (event.event === "Progress") {
                received += event.data.chunkLength;
                if (total > 0) updateState.set({ phase: "installing", version, percentage: (received / total) * 100 });
            }
        });
        await relaunch();
    } catch (error) {
        updateState.set({ phase: "error", message: errorMessage(error) });
    }
}

//? Downloading
export async function downloadFromLink({ link, folder, options }: DownloadPayload): Promise<DownloadResult> {
    const extension = options.format === FORMATS.AUDIO ? options.audioExtension : options.videoExtension;

    isDownloading.set(true);
    downloadProgress.set({ phase: "starting" });

    try {
        return await invoke<DownloadResult>("download_from_link", {
            link,
            folder,
            format: options.format,
            extension: extension.trim() || null,
            embeds: options.embed,
        });
    } finally {
        isDownloading.set(false);
        downloadProgress.set(null);
    }
}

export async function cancelDownload(): Promise<void> {
    try {
        await invoke("cancel_download");
    } catch (error) {
        setStatus(errorMessage(error), "error");
    }
}

export async function downloadSingle(input: string): Promise<boolean> {
    let folder: string;
    try {
        folder = resolveDownloadFolder();
    } catch (error) {
        setStatus(errorMessage(error), "error");
        return false;
    }

    const link = applySearchPrefix(input);
    setStatus(`Downloading ${displayLink(link)}`, "info");

    let result: DownloadResult;
    try {
        result = await downloadFromLink({ link, folder, options: get(downloadOptions) });
    } catch (error) {
        result = { success: false, cancelled: false, message: errorMessage(error) };
    }

    if (result.cancelled) {
        setStatus(result.message, "warning");
        return false;
    }

    setStatus(result.message, result.success ? "success" : "error");
    addToHistory({
        link,
        status: result.success ? "success" : "error",
        message: result.message,
        timestamp: new Date().toISOString(),
        folder,
        file_path: result.file_path,
    });
    await refreshSubfolders();
    return result.success;
}

export async function startBulk(links: string[]): Promise<void> {
    let folder: string;
    try {
        folder = resolveDownloadFolder();
    } catch (error) {
        setStatus(errorMessage(error), "error");
        return;
    }

    const options = get(downloadOptions);
    const updateItem = (index: number, changes: Partial<QueueItem>) =>
        bulkQueue.update((queue) => queue.map((item, i) => (i === index ? { ...item, ...changes } : item)));

    bulkRunning.set(true);
    bulkQueue.set(
        links.map((link) => ({ link: applySearchPrefix(link), status: "pending", message: "", folder }))
    );

    let succeeded = 0;
    let failed = 0;
    let cancelled = false;

    for (let i = 0; i < links.length && !cancelled; i++) {
        const link = get(bulkQueue)[i].link;
        updateItem(i, { status: "downloading" });
        setStatus(`Downloading ${i + 1} of ${links.length}: ${displayLink(link)}`, "info");

        try {
            const result = await downloadFromLink({ link, folder, options });
            cancelled = result.cancelled;
            if (result.success) succeeded++;
            else if (!result.cancelled) failed++;
            updateItem(i, {
                status: result.cancelled ? "cancelled" : result.success ? "success" : "error",
                message: result.message,
                file_path: result.file_path,
            });
        } catch (error) {
            failed++;
            updateItem(i, { status: "error", message: errorMessage(error) });
        }
    }

    if (cancelled) {
        bulkQueue.update((queue) =>
            queue.map((item) => (item.status === "pending" ? { ...item, status: "cancelled" } : item))
        );
    }

    bulkRunning.set(false);
    await refreshSubfolders();

    const summary = `${succeeded} downloaded, ${failed} failed.`;
    if (cancelled) setStatus(`Bulk download cancelled. ${summary}`, "warning");
    else setStatus(`Bulk download finished. ${summary}`, failed > 0 ? "warning" : "success");
}

//? History
function addToHistory(item: HistoryItem): void {
    history.update((items) => [item, ...items].slice(0, MAX_HISTORY_ITEMS));
    saveSetting(SETTINGS_KEYS.HISTORY.id, get(history));
}

export function clearHistory(): void {
    history.set([]);
    saveSetting(SETTINGS_KEYS.HISTORY.id, []);
}

//? Formatting
export function formatBytes(bytes?: number | null): string {
    if (bytes == null) return "";
    const units = ["B", "KB", "MB", "GB", "TB"];
    let value = bytes;
    let unit = 0;
    while (value >= 1024 && unit < units.length - 1) {
        value /= 1024;
        unit++;
    }
    return `${value.toFixed(unit < 2 ? 0 : 1)} ${units[unit]}`;
}

export function formatDuration(seconds?: number | null): string {
    if (seconds == null) return "";
    const total = Math.round(seconds);
    const minutes = Math.floor(total / 60);
    const hours = Math.floor(minutes / 60);
    const pad = (value: number) => String(value).padStart(2, "0");
    return hours > 0
        ? `${hours}:${pad(minutes % 60)}:${pad(total % 60)}`
        : `${minutes}:${pad(total % 60)}`;
}
