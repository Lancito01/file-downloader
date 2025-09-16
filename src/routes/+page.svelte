<script lang="ts">
    import Input from "$lib/components/Input.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    const soundcloud = "sc";
    const youtube = "yt";

    const musicPath = "D:\\Andy\\Assets\\02 Sound\\Music";

    let folders: string[] = ["", "<New folder>"];
    async function loadFolders() {
        const result: string[] = await invoke("list_folders", {
            path: musicPath,
        });
        folders = [...folders, ...result];
        console.log(folders);
    }

    const audioExtensions = ["mp3", "m4a", "wav", "flac", "aac", "ogg"];
    const videoExtensions = ["mp4", "mov", "avi", "mkv"];

    let extensions: string[] = audioExtensions;

    let selectedFormat: string;
    $: if (selectedFormat === "audio") {
        extensions = audioExtensions;
    } else if (selectedFormat === "video") {
        extensions = videoExtensions;
    } else {
        extensions = [];
    }

    let selectedDownloadWebsite: string;
    let selectedInputValue: string;
    let selectedFolder: string;
    let selectedExtension: string;
    let selectedFolderType: string;

    $: {
        console.log({
            selectedDownloadWebsite,
            selectedInputValue,
            selectedFolder,
            selectedFormat,
            selectedExtension,
        });
    }

    onMount(() => {
        loadFolders();
    });

    let error: string = "";

    function onSingleDownloadPress() {
        if (
            !selectedDownloadWebsite ||
            !selectedInputValue ||
            !selectedFolder ||
            !selectedFormat ||
            !selectedExtension
        ) {
            // error = "Please fill in all fields.";
            alert("Please fill in all fields.");
            return;
        }
    }
</script>

{#if error}
    <div class="error">
        <p>{error}</p>
        <button on:click={() => (error = "")}>Continue</button>
    </div>
{/if}
<div class="app">
    <h1>Video/Song downloader</h1>
    <div class="column-containers">
        <div class="single-download">
            <h2>Single Download</h2>
            <small
                >Used to download a single video/sound from either YouTube or
                SoundCloud.</small
            >
            <div class="fields-container">
                <div class="inputs">
                    <div class="from-where">
                        <label for="website">Download from:</label>
                        <select
                            name="website"
                            id="website"
                            bind:value={selectedDownloadWebsite}
                        >
                            <option value="" selected>Choose option</option>
                            <option value={soundcloud}>SoundCloud</option>
                            <option value={youtube}>YouTube</option>
                        </select>
                    </div>
                    <div class="what-to-download">
                        <label for="input">Link or name:</label>
                        <input
                            type="text"
                            id="input"
                            bind:value={selectedInputValue}
                            placeholder="Enter URL or name..."
                        />
                    </div>
                    <div class="where-to-save-container">
                        <label for="folder-type">Save to:</label>
                        <select
                            name="folder-type"
                            id="folder-type"
                            bind:value={selectedFolderType}
                        >
                            <option value="" selected>Choose option</option>
                            <option value="desktop">Desktop</option>
                            <option value="music">Music folder</option>
                        </select>
                    </div>
                    {#if selectedFolderType === "music"}
                        <div class="sub-choice">
                            <label for="location">Choose subfolder:</label>
                            <select
                                name="location"
                                id="location"
                                bind:value={selectedFolder}
                                placeholder="Select folder"
                            >
                                {#each folders as folder}
                                    <option value={folder}>📂 ./{folder}</option
                                    >
                                {/each}
                            </select>
                        </div>
                    {/if}
                    <div class="format-type">
                        <label for="format-type">Type:</label>
                        <select
                            name="format"
                            id="format-type"
                            bind:value={selectedFormat}
                        >
                            <option value="" selected>Choose option</option>
                            <option value="audio">Audio</option>
                            <option value="video">Video</option>
                        </select>
                    </div>
                    {#if selectedFormat}
                        <div class="sub-choice">
                            <label for="format-extension">Extension:</label>
                            <select
                                name="extension"
                                id="format-extension"
                                bind:value={selectedExtension}
                            >
                                {#each extensions as ext}
                                    <option value={ext}>{ext}</option>
                                {/each}
                            </select>
                        </div>
                    {/if}
                </div>
                <button on:click={onSingleDownloadPress}>Download</button>
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

    .error {
        position: absolute;
        background-color: rgba(255, 0, 0, 0.2);
        backdrop-filter: blur(4px);
        color: white;
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        gap: 10px;
        align-items: center;
        justify-content: center;
        opacity: 0;
        animation: fadeInError 0.1s ease forwards;

        p {
            font-size: 3rem;
            font-family: $font;
        }
        button {
            padding: 10px 20px;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            background-color: white;
            transition: transform 0.1s;

            &:hover {
                background-color: lighten(red, 10%);
                transform: scale(1.05);
            }
        }
    }

    @keyframes fadeInError {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

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

                .fields-container {
                    display: flex;
                    flex-direction: column;
                    border: 1px solid white;
                    padding: 20px;
                    gap: 10px;
                    width: 100%;

                    .inputs {
                        display: inherit;
                        flex-direction: column;
                        align-items: center;
                        justify-content: space-between;
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

                            &.sub-choice {
                                margin: 0 0 0 auto;
                                width: 90%;
                            }
                        }
                    }

                    button {
                        width: 100px;
                        margin: 0 auto;
                        padding: 7px;
                        border: none;
                        border-radius: 5px;
                        cursor: pointer;
                        background-color: $button-color;
                        transition: transform 0.1s;

                        &:hover {
                            background-color: lighten($button-color, 10%);
                            transform: scale(1.05);
                        }
                    }

                    transition: all 0.2s;
                    &:hover {
                        box-shadow: 0 0 10px rgba(255, 255, 255, 0.5);
                    }
                }
            }
        }
    }
    @media (max-width: 500px) {
        .app {
            .column-containers {
                flex-direction: column;
                gap: 30px;

                & > * {
                    width: 100%;
                }
            }
        }
    }
</style>
