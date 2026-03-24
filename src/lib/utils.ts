import { SETTINGS_KEYS, FORMATS } from "$lib/assets/keys";
import { Store } from "@tauri-apps/plugin-store";
import { writable, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

export let settings: Store | null = null;

export type Status = {
    type: "success" | "error" | "warning" | "info" | null;
    message: string;
};

export type DownloadFormat = (typeof FORMATS)[keyof typeof FORMATS];

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

export const initializeSettings = async (): Promise<void> => {
    settings = await Store.load("settings.json");
    
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
        title: "Select Music folder",
    });

    if (typeof result === "string") {
        selectedDownloadFolder.set(result);
        updateStatus({
            type: "warning",
            message: `Selected folder ${get(selectedDownloadFolder)}`,
        });
    }
}

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

export function getStatusColor(): string {
    switch (get(status).type) {
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
        const result = await invoke<DownloadResult>("download_from_link", {
            link: trimmedLink,
            folder,
            format: payload.format,
            extension: payload.extension?.trim() || null,
            embeds: payload.embeds ?? get(embedMetadata),
        });
        
        return result;
    } catch (error) {
        console.error("Download error:", error);
        throw new Error(error instanceof Error ? error.message : String(error));
    }
}
