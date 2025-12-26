<script lang="ts">
    import { onMount } from "svelte";

    import { SETTINGS_KEYS } from "$lib/assets/keys";
    import { Store } from "@tauri-apps/plugin-store";
    import {
        browseFolder,
        saveDownloadFolderSetting,
        setStatus,
        readSettingsWithKey,
    } from "$lib/utils";
    import { selectedDownloadFolder } from "$lib/utils";
    import { get } from "svelte/store";

    let settings: Store | null = null;
    let settingsEntries: [string, any][] = [];
    onMount(async () => {
        settings = await Store.load("settings.json");

        settingsEntries = await settings.entries();

        selectedDownloadFolder.set(
            await readSettingsWithKey(SETTINGS_KEYS.MUSIC_FOLDER_PATH.id)
        );
    });

    async function saveSettings() {
        // * Music
        saveDownloadFolderSetting();

        setStatus("Settings saved successfully!", "success");

        // ? Reload settings entries
        settingsEntries = await settings!.entries();
    }
</script>

<div class="w-full h-full flex flex-col">
    <h2 class="text-2xl max-w-max">⚙️ Settings</h2>

    <div class="flex flex-col gap-2 mt-2 h-full">
        {#each settingsEntries as [key, v]}
            <div class="setting-entry flex flex-col p-1">
                <p class="mb-2 text-sm text-gray-300">
                    {Object.values(SETTINGS_KEYS).find(
                        (item) => item.id === key
                    )!.text}:
                </p>

                <div class="value flex items-center gap-2 w-full">
                    <p
                        class="setting-p grow h-full p-2 rounded text-base overflow-ellipsis"
                    >
                        {$selectedDownloadFolder}
                        <!-- TODO: make this dynamic ? -->
                    </p>
                    <button class="change-setting-btn" on:click={browseFolder}>
                        Change
                    </button>
                </div>
            </div>
        {/each}
    </div>
    <button
        class="save-btn w-1/2 font-bold h-8.5 self-center max-w-2xl rounded"
    >
        Save
    </button>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";

    .setting-entry {
        .value {
            p {
                background-color: $bg;
                border-bottom: 1px solid $accent;
                font-family: $font-mono;
            }
        }
    }

    .change-setting-btn {
        height: 100%;
        width: 80px;
        align-self: center;
        padding: 0.5rem;
        border: none;
        background-color: $surface;
        border-radius: 0.375rem;
        border: 1px solid $border;
        color: $text;
        font-size: 0.9rem;
        cursor: pointer;
        transition: background-color 0.1s ease-in-out;
    }

    .change-setting-btn:hover {
        background-color: lighten($bg, 10%);
    }

    .save-btn {
        background-color: $accent;
        color: $bg;
        cursor: pointer;
        transition: background-color 0.1s ease-in-out;
        box-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
    }

    .save-btn:hover {
        background-color: darken($accent, 20%);
    }
</style>
