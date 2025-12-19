<script lang="ts">
    import { onMount } from "svelte";
    import { SETTINGS_KEYS } from "$lib/assets/keys";
    import { Store } from "@tauri-apps/plugin-store";

    const updateSettingsWithKey = async (
        key: string,
        value: any
    ): Promise<void> => {
        if (settings) {
            await settings.set(key, value);
            await settings.save();
        }
    };

    const readSettingsWithKey = async (key: string): Promise<any | null> => {
        if (settings) {
            return await settings.get(key);
        }
        return null;
    };

    let settings: Store | null = null;
    let musicPath: string | null | undefined;

    onMount(async () => {
        settings = await Store.load("settings.json");

        musicPath = await settings.get(SETTINGS_KEYS.MUSIC_FOLDER_PATH);
        if (!musicPath) {
            firstRun = true;
        }
    });

    import { open } from "@tauri-apps/plugin-dialog";
    import Single from "./tabs/Single.svelte";
    import Bulk from "./tabs/Bulk.svelte";
    import Settings from "./tabs/Settings.svelte";
    let selectedFolder = "";
    async function browseFolder() {
        const result = await open({
            directory: true,
            multiple: false,
            title: "Select Music folder",
        });

        if (typeof result === "string") {
            selectedFolder = result;
            status.type = "warning";
            status.message = `Selected folder ${selectedFolder}`;
        }
    }

    async function saveSettings() {
        if (selectedFolder) {
            await updateSettingsWithKey(
                SETTINGS_KEYS.MUSIC_FOLDER_PATH,
                selectedFolder
            );
            firstRun = false;
            status.type = "success";
            status.message = "Settings saved successfully.";
        } else {
            status.type = "error";
            status.message = "Please select a folder before saving.";
        }
    }

    function getStatusColor() {
        switch (status.type) {
            case "success":
                return "#4dff88";
            case "error":
                return "#ff4d4d";
            case "warning":
                return "#ffd24d";
            default:
                return "white";
        }
    }

    type Status = {
        type: "info" | "success" | "error" | "warning";
        message: string;
    };

    let firstRun = false;
    let status: Status = {
        type: "info",
        message: "Idle 🌙",
    };
    $: {
        if (status.type) {
            statusColor = getStatusColor();
        }
    }
    let statusColor = "white";

    let activeTab: string = "single";

    $: console.log(selectedFolder);
</script>

<div class="app-container">
    {#if firstRun}
        <div class="first-run">
            <h1>Welcome to Andy's Downloader!</h1>
            <p>Please configure your settings to get started.</p>
            <div class="settings">
                <div class="musicFolder">
                    <h4>1️⃣ First, choose your default Music folder:</h4>
                    <button on:click={browseFolder}>Select Music Folder</button>
                    {#if selectedFolder}
                        <p>{selectedFolder}</p>
                    {/if}
                </div>
            </div>
            <div class="confirm">
                <h2 class="confirm">Everything looks good? ⬇️</h2>
                <button class="confirm" on:click={saveSettings}>Save</button>
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
                    >{status.message}</span
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
            height: 100%;
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
            nav {
                height: 0%;
                min-height: 28px;
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
                height: calc(100% - 28px - 34px);
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
        z-index: 100;
        p {
            width: 100%;
            overflow: hidden;
            text-overflow: ellipsis;
            text-wrap: nowrap;
        }
    }
</style>
