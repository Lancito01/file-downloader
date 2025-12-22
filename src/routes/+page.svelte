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
        
        const downloadFolder = await readSettingsWithKey(
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
        <div class="first-run">
            <h1>Welcome to Andy's Downloader!</h1>
            <p>Please configure your settings to get started.</p>
            <div class="settings">
                <div class="musicFolder">
                    <h4>1️⃣ First, choose your default Music folder:</h4>
                    <button on:click={browseFolder}>Select Music Folder</button>
                    {#if $selectedDownloadFolder}
                        <p>{$selectedDownloadFolder}</p>
                    {/if}
                </div>
            </div>
            <div class="confirm">
                <h2 class="confirm">Everything looks good? ⬇️</h2>
                <button class="confirm" on:click={saveDownloadFolderSetting}
                    >Save</button
                >
            </div>
        </div>
    {/if}
    <div class="app">
        <nav>
            <button
                class:active={activeTab == "single"}
                on:click={() => (activeTab = "single")}
            >
                Single Download
            </button>

            <button
                class:active={activeTab == "bulk"}
                on:click={() => (activeTab = "bulk")}
            >
                Bulk Download
            </button>

            <button
                class:active={activeTab == "settings"}
                on:click={() => (activeTab = "settings")}
            >
                ⚙️ Settings
            </button>
        </nav>
        <div class="render-tab">
            {#if activeTab == "single"}
                <Single />
            {:else if activeTab == "bulk"}
                <Bulk />
            {:else if activeTab == "settings"}
                <Settings />
            {/if}
        </div>
        <div class="status">
            <p>
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
        background-color: $background-color;
        color: $font-color;
        height: 100vh;
        width: 100vw;

        font-family: $font;

        h1 {
            width: auto;
            height: auto;
        }

        .first-run {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: calc(100% - 34px);
            width: 100%;
            text-align: center;

            z-index: 10;
            position: absolute;

            background-color: $secondary-background-color;

            .settings {
                display: flex;
                flex-direction: column;
                margin: 5px 0 0 0;
                gap: 10px;

                > * {
                    margin: 5px 0;
                    display: flex;
                    flex-direction: column;
                    width: 100%;

                    padding: 5px;
                    background: lighten($background-color, $amount: 5%);
                    border-radius: 5px;

                    align-items: center;
                }
            }

            .confirm {
                display: flex;
                flex-direction: column;
                margin-top: 15px;

                h2.confirm {
                    font-size: 15px;
                    justify-content: center;
                }
                button.confirm {
                    width: 50%;
                    max-width: 75px;
                    align-self: center;
                    text-align: center;
                }
            }
        }

        .app {
            height: 100%;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            nav {
                height: 28px;
                background-color: lighten($background-color, 5%);

                button {
                    background-color: $background-color;
                    color: white;
                    &.active {
                        background-color: darken($accent-color, 35%);
                    }
                }
            }
            .render-tab {
                z-index: 40;
                width: 100%;
                flex-grow: 1;
            }
            .status {
                font-family: $font;
                width: 100%;
                height: 34px;
                background-color: lighten($background-color, 5%);
                color: $font-color;
                font-size: 14px;
                display: flex;
                align-items: center;
                padding: 0 10px;
                z-index: 100 !important;
                p {
                    width: 100%;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    text-wrap: nowrap;
                }
            }
        }

        // elements
        button {
            background-color: $accent-color;
            border: none;
            border-radius: 2px;
            color: $background-color;
            cursor: pointer;
            padding: 5px 5px;
            margin: 3px 3px;

            &:hover {
                background-color: darken($accent-color, 10%);
            }
        }
    }
</style>
