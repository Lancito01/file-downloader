<script lang="ts">
    import { onMount } from "svelte";

    import { SETTINGS_KEYS } from "$lib/assets/keys";
    import { Store } from "@tauri-apps/plugin-store";

    let settings: Store | null = null;
    let musicPath: string | null | undefined;
    let settingsEntries: any[] = [];
    onMount(async () => {
        settings = await Store.load("settings.json");

        musicPath = await settings.get(SETTINGS_KEYS.MUSIC_FOLDER_PATH);
        settingsEntries = await settings.entries();
    });

</script>

<div class="settings-container">
    <div class="settings">
        <h2>⚙️ Settings</h2>
        {#each settingsEntries as [key, value]}
            <div class="setting-entry">
                <p>{key}:</p> {value}
            </div>
        {/each}
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

            .setting-entry {
                display: flex;
                gap: 10px;
                padding: 6px;
                background-color: $background-color;
                border-radius: 5px;
                border-bottom: 1px solid $font-color;
            }
        }
    }
</style>
