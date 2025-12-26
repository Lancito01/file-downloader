<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    import { FORMATS, SETTINGS_KEYS } from "$lib/assets/keys";
    import { Store } from "@tauri-apps/plugin-store";

    async function testDownload() {
        await invoke("download_from_link", {
            link: "ytsearch1:never gonna give you up",
            folder: "C:\\Users\\User\\Desktop",
            format: FORMATS.VIDEO,
            extension: "mp3"
        })
    }

    onMount(async () => {
        const settings = await Store.load("settings.json");
        const musicPath = await settings.get(SETTINGS_KEYS.MUSIC_FOLDER_PATH.id);
        console.log("Music path from settings:", musicPath);
    })
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
    <button on:click={testDownload}>Test Download</button>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";
</style>