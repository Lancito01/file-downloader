import { SETTINGS_KEYS } from "$lib/assets/keys";
import { Store } from "@tauri-apps/plugin-store";
import { writable } from "svelte/store";

export let settings: Store | null = null;

export type Status = {
    type: "success" | "error" | "warning" | "info" | null;
    message: string;
};

export const initializeSettings = async (): Promise<void> => {
    settings = await Store.load("settings.json");
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
    } else {
        status.set({
            type: "error",
            message: "Please select a valid download folder.",
        });
    }
}

import { get } from "svelte/store";
export function getStatusColor(): string {
    switch (get(status).type) {
        case "success":
            return "#4dff88";
        case "error":
            return "#ff4d4d";
        case "warning":
            return "#ffd24d";
        default:
            return "white";
    }
}
