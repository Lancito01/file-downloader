<script lang="ts">
    import { onMount } from "svelte";
    import { getVersion } from "@tauri-apps/api/app";
    import {
        activeTab,
        appVersion,
        browseFolder,
        checkDependencies,
        checkForUpdates,
        firstRun,
        initializeSettings,
        isReady,
        setStatus,
    } from "$lib/utils";

    import Single from "./tabs/Single.svelte";
    import Bulk from "./tabs/Bulk.svelte";
    import Settings from "./tabs/Settings.svelte";
    import Nav from "./Nav.svelte";
    import StatusBar from "./StatusBar.svelte";
    import Icon from "$lib/components/Icon.svelte";
    import DependencyBanner from "$lib/components/DependencyBanner.svelte";
    import UpdateBanner from "$lib/components/UpdateBanner.svelte";

    onMount(async () => {
        try {
            await initializeSettings();
        } catch (error) {
            setStatus(`Failed to load settings: ${error}`, "error");
        }
        isReady.set(true);

        // Not awaited: the window stays usable while this runs
        checkDependencies();
        getVersion().then((version) => appVersion.set(version));
        if (!import.meta.env.DEV) checkForUpdates(true);
    });
</script>

<div class="app">
    {#if $isReady && $firstRun}
        <main class="welcome">
            <div class="welcome-card panel">
                <h1>Welcome to Andy's Downloader</h1>
                <p>Choose the folder your downloads should go to. You can change it later in Settings.</p>
                <button type="button" class="btn btn-primary btn-large" on:click={browseFolder}>
                    <Icon name="folder" /> Choose download folder
                </button>
            </div>
        </main>
    {:else if $isReady}
        <div class="workspace">
            <Nav />
            <main class="content">
                <div class="banner-slot">
                    <DependencyBanner />
                    <UpdateBanner />
                </div>
                <!-- Tabs stay mounted so typed links and running queues survive switching -->
                <div hidden={$activeTab !== "single"}><Single /></div>
                <div hidden={$activeTab !== "bulk"}><Bulk /></div>
                <div hidden={$activeTab !== "settings"}><Settings /></div>
            </main>
        </div>
    {/if}
    <StatusBar />
</div>

<style lang="scss">
    .app {
        height: 100vh;
        width: 100vw;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .workspace {
        flex: 1;
        min-height: 0;
        display: flex;
    }

    .content {
        flex: 1;
        min-width: 0;
        overflow-y: auto;
    }

    .banner-slot {
        max-width: 860px;
        margin: 0 auto;
        padding: 0 32px;

        display: flex;
        flex-direction: column;
        gap: 8px;

        &:has(:global(.banner)) {
            padding-top: 16px;
            margin-bottom: -12px;
        }
    }

    .welcome {
        flex: 1;
        min-height: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 24px;
    }

    .welcome-card {
        max-width: 440px;
        padding: 32px;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 12px;

        h1 {
            font-size: 22px;
            font-weight: 650;
            letter-spacing: -0.01em;
        }

        p {
            color: var(--text-muted);
            margin-bottom: 10px;
        }
    }
</style>
