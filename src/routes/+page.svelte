<script lang="ts">
    import { onMount } from "svelte";
    import { SETTINGS_KEYS } from "$lib/assets/keys";
    import {
        readSettingsWithKey,
        status,
        updateStatus,
        getStatusColor,
        initializeSettings,
    } from "$lib/utils.ts";
    import {
        selectedDownloadFolder,
        browseFolder,
        saveDownloadFolderSetting,
    } from "$lib/utils.ts";
    import { firstRun } from "$lib/utils.ts";

    import Single from "./tabs/Single.svelte";
    import Bulk from "./tabs/Bulk.svelte";
    import Settings from "./tabs/Settings.svelte";

    // * Automatic changing of status color based on detection of status.type
    $: {
        if ($status.type) {
            statusColor = getStatusColor();
        }
    }
    let statusColor = "white";

    let activeTab: string = "single";

    onMount(async () => {
        // ! Initialization of settings store
        await initializeSettings();

        const downloadFolder: string | null = await readSettingsWithKey(
            SETTINGS_KEYS.MUSIC_FOLDER_PATH.id
        );
        if (!downloadFolder) {
            firstRun.set(true);
            updateStatus({
                type: "error",
                message: "Please set your default Music folder to continue.",
            });
        } else {
            firstRun.set(false);
        }

        // * Set initial status
        updateStatus({
            type: "info",
            message: "Idle 🌙",
        });
    });
</script>

<div class="app-container">
    {#if $firstRun}
        <div
            class="first-run flex flex-col items-center justify-center w-full text-center z-10 absolute"
        >
            <h1>Welcome to Andy's Downloader!</h1>
            <p>Please configure your settings to get started.</p>
            <div class="settings flex flex-col mt-1 gap-2.5">
                <div class="musicFolder">
                    <h4>1️⃣ First, choose your default Music folder:</h4>
                    <button on:click={browseFolder}>Select Music Folder</button>
                    {#if $selectedDownloadFolder}
                        <p>{$selectedDownloadFolder}</p>
                    {/if}
                </div>
            </div>
            <div class="flex flex-col mt-4">
                <h2 class="text-lg">Everything looks good? ⬇️</h2>
                <button
                    class="w-1/2 max-w-20 self-center text-center"
                    on:click={saveDownloadFolderSetting}>Save</button
                >
            </div>
        </div>
    {/if}
    <div class="app flex flex-col h-full justify-between">
        <nav class="flex flex-row items-center h-8 gap-2">
            <button
                type="button"
                class:active={activeTab == "single"}
                aria-pressed={activeTab == "single"}
                on:click={() => (activeTab = "single")}
            >
                1️⃣ Single Download
            </button>

            <button
                type="button"
                class:active={activeTab == "bulk"}
                aria-pressed={activeTab == "bulk"}
                on:click={() => (activeTab = "bulk")}
            >
                🎒 Bulk Download
            </button>

            <button
                type="button"
                class:active={activeTab == "settings"}
                aria-pressed={activeTab == "settings"}
                on:click={() => (activeTab = "settings")}
            >
                ⚙️ Settings
            </button>
        </nav>
        <div
            class="render-tab flex justify-center items-center p-1.5 z-40 w-full h-full grow"
        >
            <div
                class="container flex flex-col w-full h-full rounded-lg items-center max-w-full p-5"
            >
                {#if activeTab == "single"}
                    <Single />
                {:else if activeTab == "bulk"}
                    <Bulk />
                {:else if activeTab == "settings"}
                    <Settings />
                {/if}
            </div>
        </div>
        <div
            class="status flex flex-row w-full h-9 items-center z-50 px-2.5 text-sm"
        >
            <p class="w-full h-full overflow-hidden flex items-center">
                Status: <span style="color: {statusColor};"
                    >{$status.message}</span
                >
            </p>
        </div>
    </div>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";
    .app-container {
        background-color: $bg;
        color: $text;
        height: 100vh;
        width: 100vw;

        font-family: $font-ui;
        h1 {
            width: auto;
            height: auto;
        }

        .first-run {
            height: calc(100% - 34px);
            background-color: $surface-alt;

            .settings {
                > * {
                    margin: 5px 0;
                    display: flex;
                    flex-direction: column;
                    width: 100%;

                    padding: 5px;
                    background: lighten($bg, $amount: 5%);
                    border-radius: 5px;

                    align-items: center;
                }
            }
        }

        .app {
            nav {
                background-color: lighten($bg, 5%);

                button {
                    height: 100%;
                    background-color: lighten($bg, 5%);
                    border: $accent 1px solid;
                    border-bottom: none;
                    border-radius: 4px 4px 0 0;
                    color: $text;
                    cursor: pointer;
                    padding: 5px 5px;
                    margin: 2px 2px 0 2px;

                    transition: background-color 0.1s ease-in-out;

                    &:hover {
                        background-color: lighten($bg, 10%);
                    }

                    &.active {
                        background-color: $bg;
                    }
                }
            }
            .render-tab {
                background-color: $bg;

                .container {
                    background-color: lighten($bg, $amount: 5%);
                }
            }
            .status {
                font-family: $font-mono;
                background-color: lighten($bg, 5%);
                color: $text;
            }
        }
    }
</style>
