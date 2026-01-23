<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    import { FORMATS, SETTINGS_KEYS } from "$lib/assets/keys";
    import { Store } from "@tauri-apps/plugin-store";
    import { setStatus } from "$lib/utils";

    async function testDownload() {
        console.log("Starting test download...");
        setStatus("Starting test download...", "warning");
        await invoke("download_from_link", {
            link: "scsearch1:knock2 aarena",
            folder: "C:\\Users\\User\\Desktop",
            format: FORMATS.AUDIO,
            embeds: true,
        });
        setStatus("Test download completed!", "success");
    }

    onMount(async () => {
        const settings = await Store.load("settings.json");
        const musicPath = await settings.get(
            SETTINGS_KEYS.MUSIC_FOLDER_PATH.id
        );
        console.log("Music path from settings:", musicPath);
    });
</script>

<div class="content">
    <h2>1️⃣ Single Download</h2>
    <!-- 
    1. Basic selections
        1a. Sound/video
        1b. Download link/search
            1b1. Where to search (website)
    2. Where to save the file?
    3. Format/extension
    4. Download button
    -->
    <button on:click={testDownload}>Teswasdt Download</button>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";
    .content {
        button {
            width: max-content;
            padding: 0.5rem 1rem;
            border-radius: 0.375rem;
            color: white;
            font-weight: 600;
            cursor: pointer;
        }
    }
</style>
