<script lang="ts">
    import { onMount } from "svelte";

    import { SETTINGS_KEYS } from "$lib/assets/keys";
    import { Store } from "@tauri-apps/plugin-store";
    import {
        browseFolder,
        saveDownloadFolderSetting,
        setStatus,
        status,
    } from "$lib/utils";
    import { selectedDownloadFolder } from "$lib/utils";
    import { get } from "svelte/store";

    let settings: Store | null = null;
    let settingsEntries: [string, any][] = [];
    onMount(async () => {
        settings = await Store.load("settings.json");

        settingsEntries = await settings.entries();
    });

    async function saveSettings() {
        // * Music
        saveDownloadFolderSetting();

        setStatus("Settings saved successfully!", "success");

        // ? Reload settings entries
        settingsEntries = await settings!.entries();
    }
</script>

<div class="settings-container">
    <div class="settings">
        <h2>⚙️ Settings</h2>
        <div class="settings-entries">
            {#each settingsEntries as [key, value]}
                <div class="setting-entry">
                    <div class="setting-entry-values">
                        <p>
                            {Object.values(SETTINGS_KEYS).find(
                                (item) => item.id === key
                            )!.text}:
                        </p>
                        <p>
                            {value}
                        </p>
                    </div>
                    <button class="change-setting" on:click={browseFolder}>
                        Change
                    </button>
                </div>
            {/each}
        </div>
        <button id="save-settings" on:click={saveSettings}>Save</button>
    </div>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";

    .settings-container {
        padding: 5px;
        border-radius: 5px;
        height: 100%;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;

        .settings {
            background-color: lighten($background-color, $amount: 5%);
            height: 100%;
            width: 100%;
            border-radius: 5px;
            padding: 5px;
            display: flex;
            flex-direction: column;
            justify-content: space-between;

            .settings-entries {
                margin: 10px 0 0 0;
                display: flex;
                flex-direction: column;
                gap: 7px;
                height: 100%;

                .setting-entry {
                    display: flex;
                    flex-direction: row;
                    padding: 6px 3px;
                    justify-content: space-between;
                    align-content: center;
                    background-color: $background-color;
                    border-radius: 5px;
                    border-bottom: 1px solid $font-color;

                    .setting-entry-values {
                        display: flex;
                        flex-direction: row;
                        flex-grow: 1;
                        align-items: center;
                        height: 100%;
                        gap: 4px;

                        p {
                            margin: 0;
                        }
                    }

                    .change-setting {
                        padding: 7px 8px;
                        border: none;
                        border-radius: 5px;
                        background-color: darken($accent-color, 10%);
                        color: $background-color;
                        font-size: 0.9rem;
                        cursor: pointer;
                        transition: background-color 0.1s ease-in-out;

                        &:hover {
                            background-color: darken(
                                $accent-color,
                                $amount: 10%
                            );
                        }
                    }
                }
            }

            #save-settings {
                width: 100px;
                align-self: center;
                padding: 8px 16px;
                border: none;
                border-radius: 5px;
                background-color: $background-color;
                color: $accent-color;
                font-size: 1rem;
                cursor: pointer;
                transition: background-color 0.1s ease-in-out;
                box-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);

                &:hover {
                    background-color: darken($background-color, $amount: 10%);
                }
            }
        }
    }
</style>
