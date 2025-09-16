<script lang="ts">
    import Input from "$lib/components/Input.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    const soundcloud = "sc";
    const youtube = "yt";

    const musicPath = "D:\\Andy\\Assets\\02 Sound\\Music";

    let folders: string[] = ["", "<New folder>"];
    let selectedFolder = "";
    async function loadFolders() {
        const result: string[] = await invoke("list_folders", {
            path: musicPath,
        });
        folders = [...folders, ...result];
        console.log(folders);
    }

    onMount(() => {
        loadFolders();
    });
</script>

<div class="app">
    <h1>Video/Song downloader</h1>
    <div class="column-containers">
        <div class="single-download">
            <h2>Single Download</h2>
            <small
                >Used to download a single video/sound from either YouTube or
                SoundCloud.</small
            >
            <div class="input-container">
                <div class="from-where">
                    <label for="website">Download from:</label>
                    <select name="website" id="website">
                        <option value={soundcloud}>SoundCloud</option>
                        <option value={youtube}>YouTube</option>
                    </select>
                </div>
                <div class="what-to-download">
                    <label for="input">Link or name:</label>
                    <input
                        type="text"
                        id="input"
                        placeholder="Enter URL or name..."
                    />
                </div>
                <div class="where-to-save">
                    <label for="location">Save to:</label>
                    <select
                        name="location"
                        id="location"
                        bind:value={selectedFolder}
                        placeholder="Select folder"
                    >
                        {#each folders as folder}
                            <option value={folder}>📂 ./{folder}</option>
                        {/each}
                    </select>
                </div>
            </div>
        </div>
        <div class="multi-download">
            <h2>Multi Download</h2>
            <small
                >Used to download songs in bulk, usually from SoundCloud.</small
            >
        </div>
    </div>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";

    .app {
        background-color: $background-color;
        font-family: $font;
        color: $font-color;
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        h1 {
            text-align: center;
            margin: 0 0 30px 0;
        }

        .column-containers {
            display: flex;
            flex-direction: row;

            & > * {
                // single-download, multi-download
                width: 50%;
                display: flex;
                flex-direction: column;
                text-align: center;
                align-items: center;
                justify-content: center;
                gap: 15px;

                p,
                small {
                    text-wrap: wrap;
                    width: 70%;
                }

                .input-container {
                    width: 85%;
                    display: inherit;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    border: 1px solid white;
                    padding: 20px;
                    gap: 4px;

                    & > * {
                        width: 100%;
                        display: flex;
                        flex-direction: row;
                        justify-content: space-between;

                        input,
                        select {
                            width: 150px;
                            background-color: $background-color;
                            color: $font-color;
                            border: 1px solid white;
                            padding: 2px;
                            border-radius: 5px;
                        }
                        @media (max-width: 775px) {
                            flex-direction: column;
                            align-items: center;
                            gap: 5px;

                            input,
                            select {
                                width: 150px;
                            }
                        }
                    }
                }
            }
        }
        @media (max-width: 500px) {
            &.column-containers {
                flex-direction: column;
                gap: 30px;

                & > * {
                    width: 100%;
                }
            }
        }
    }
</style>
