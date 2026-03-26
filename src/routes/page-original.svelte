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
    import { activeTab } from "$lib/utils.ts";

    import Single from "./tabs/Single.svelte";
    import Bulk from "./tabs/Bulk.svelte";
    import Settings from "./tabs/Settings.svelte";
    import Nav from "./Nav.svelte";
    import StatusBar from "./StatusBar.svelte";

    // Automatic changing of status color based on detection of status.type
    let statusColor = $state("white");
    
    $effect(() => {
        if ($status.type) {
            statusColor = getStatusColor();
        }
    });

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
    <div class="app flex flex-col h-full">
        <Nav />
        <div
            class="render-tab flex justify-center items-center p-1.5 z-40 w-full h-full grow"
        >
            {#if $firstRun}
                <div
                    class="first-run-overlay flex flex-col items-center justify-center w-full text-center"
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
            {:else}
                <div
                    class="container flex flex-col w-full h-full rounded-lg items-center max-w-full p-5"
                >
                    {#if $activeTab == "single"}
                        <Single />
                    {:else if $activeTab == "bulk"}
                        <Bulk />
                    {:else if $activeTab == "settings"}
                        <Settings />
                    {/if}
                </div>
            {/if}
        </div>
        <StatusBar />
    </div>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";
    .app-container {
        background-color: $bg0;
        color: $text;
        height: 100vh;
        width: 100vw;
        min-height: 0;
        overflow: hidden;
        display: flex;
        flex-direction: column;

        font-family: $font-ui;
        h1 {
            width: auto;
            height: auto;
        }

        .app {
            flex: 1;
            min-height: 0;
            display: flex;
            flex-direction: column;
            
            .render-tab {
                background-color: $bg0;
                position: relative;
                flex: 1;
                min-height: 0;
                overflow: hidden;

                .first-run-overlay {
                    height: 100%;
                    background-color: $surface-alt;
                    overflow-y: auto;
                    padding: 1rem;
                    box-sizing: border-box;

                    @media (max-width: 768px) {
                        padding: 0.75rem;
                    }

                    @media (max-width: 480px) {
                        padding: 0.5rem;
                    }

                    .settings {
                        > * {
                            margin: 5px 0;
                            display: flex;
                            flex-direction: column;
                            width: 100%;

                            padding: 5px;
                            background: $bg1;
                            border-radius: 5px;

                            align-items: center;
                        }
                    }
                }

                .container {
                    background-color: $bg1;
                    overflow-y: auto;
                    height: 100%;
                    min-height: 0;
                }
            }
        }
    }
</style>
