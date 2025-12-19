<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    import { SETTINGS_KEYS } from "$lib/assets/keys";
    import { Store } from "@tauri-apps/plugin-store";

    async function testDownload() {
        await invoke("download_from_link", {
            link: "",
            folder: "",
            format: "",
            extension: ""
        })
    }

    onMount(async () => {
        const settings = await Store.load("settings.json");
        const musicPath = await settings.get(SETTINGS_KEYS.MUSIC_FOLDER_PATH);
        console.log("Music path from settings:", musicPath);
    })
</script>

<div class="single">
    single download tab
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";
</style>