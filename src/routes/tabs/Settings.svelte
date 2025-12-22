<script lang="ts">
    import { onMount } from "svelte";

    import { SETTINGS_KEYS } from "$lib/assets/keys";
    import { Store } from "@tauri-apps/plugin-store";

    let settings: Store | null = null;
    let musicPath: string | null | undefined;
    let settingsEntries: any[] = [];
    onMount(async () => {
        settings = await Store.load("settings.json");

        musicPath = await settings.get(SETTINGS_KEYS.MUSIC_FOLDER_PATH.id);
        settingsEntries = await settings.entries();
    });
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
                    <button> Change </button>
                </div>
            {/each}
        </div>
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

            .settings-entries {
                margin: 10px 0 0 0;
                display: flex;
                flex-direction: column;
                gap: 10px;

                .setting-entry {
                    display: flex;
                    flex-direction: row;
                    padding: 6px 3px;
                    justify-content: space-between;
                    background-color: $background-color;
                    border-radius: 5px;
                    border-bottom: 1px solid $font-color;

                    .setting-entry-values {
                        flex-grow: 1;
                        display: flex;
                        flex-direction: row;
                        gap: 4px;

                        p {
                            margin: 0;
                        }
                    }
                }
            }
        }
    }
</style>
