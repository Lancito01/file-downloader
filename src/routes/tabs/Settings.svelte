<script lang="ts">
    import { FORMATS } from "$lib/assets/keys";
    import {
        activeTab,
        appVersion,
        browseFolder,
        bulkRunning,
        checkDependencies,
        checkForUpdates,
        defaultOptions,
        dependencyStatus,
        installFfmpeg,
        installUpdate,
        installYtDlp,
        isDownloading,
        openFolder,
        refreshYtDlpVersion,
        selectedDownloadFolder,
        setDefaultOption,
        updateState,
        updateYtDlp,
        ytDlpVersion,
    } from "$lib/utils";
    import Icon from "$lib/components/Icon.svelte";
    import ExtensionSelect from "$lib/components/ExtensionSelect.svelte";

    let busyTool: "yt-dlp" | "ffmpeg" | null = null;
    let versionRequested = false;

    // Getting the version means starting yt-dlp, which is slow. Only do it once it's needed.
    $: if ($activeTab === "settings" && $dependencyStatus?.yt_dlp_installed && !versionRequested) {
        versionRequested = true;
        refreshYtDlpVersion();
    }

    $: ffmpegLabel =
        $dependencyStatus?.ffmpeg_source === "Bundled" ? "Using the copy bundled with this app" : "Installed";

    async function runTool(tool: "yt-dlp" | "ffmpeg", action: () => Promise<void>) {
        busyTool = tool;
        await action();
        busyTool = null;
    }
</script>

<div class="page">
    <h1 class="page-title">Settings</h1>

    <section>
        <div class="section-header"><h2 class="section-title">Download folder</h2></div>
        <div class="panel card">
            <div class="setting">
                <p class="folder-path selectable" title={$selectedDownloadFolder ?? ""}>
                    {$selectedDownloadFolder ?? "No folder selected"}
                </p>
                {#if $selectedDownloadFolder}
                    <button
                        type="button"
                        class="btn btn-ghost"
                        on:click={() => $selectedDownloadFolder && openFolder($selectedDownloadFolder)}
                    >
                        Open
                    </button>
                {/if}
                <button type="button" class="btn" on:click={browseFolder}>
                    <Icon name="folder" size={14} /> Change
                </button>
            </div>
        </div>
    </section>

    <section>
        <div class="section-header"><h2 class="section-title">Download defaults</h2></div>
        <div class="panel card">
            <div class="setting">
                <div class="setting-text">
                    <p>Type</p>
                    <p class="hint">What the download pages start with</p>
                </div>
                <div class="segmented control" role="group" aria-label="Default type">
                    <button
                        type="button"
                        class:active={$defaultOptions.format === FORMATS.AUDIO}
                        aria-pressed={$defaultOptions.format === FORMATS.AUDIO}
                        on:click={() => setDefaultOption("format", FORMATS.AUDIO)}
                    >
                        <Icon name="music" size={14} /> Audio
                    </button>
                    <button
                        type="button"
                        class:active={$defaultOptions.format === FORMATS.VIDEO}
                        aria-pressed={$defaultOptions.format === FORMATS.VIDEO}
                        on:click={() => setDefaultOption("format", FORMATS.VIDEO)}
                    >
                        <Icon name="video" size={14} /> Video
                    </button>
                </div>
            </div>

            <label class="setting">
                <span class="setting-text">Audio file format</span>
                <span class="control">
                    <ExtensionSelect
                        format="audio"
                        value={$defaultOptions.audioExtension}
                        on:change={(event) => setDefaultOption("audioExtension", event.detail)}
                    />
                </span>
            </label>

            <label class="setting">
                <span class="setting-text">Video file format</span>
                <span class="control">
                    <ExtensionSelect
                        format="video"
                        value={$defaultOptions.videoExtension}
                        on:change={(event) => setDefaultOption("videoExtension", event.detail)}
                    />
                </span>
            </label>

            <label class="setting">
                <span class="setting-text">Embed artwork and metadata</span>
                <input
                    class="toggle"
                    type="checkbox"
                    checked={$defaultOptions.embed}
                    on:change={(event) => setDefaultOption("embed", event.currentTarget.checked)}
                />
            </label>
        </div>
    </section>

    <section>
        <div class="section-header">
            <h2 class="section-title">Tools</h2>
            <button type="button" class="btn btn-ghost btn-small" on:click={checkDependencies}>Check again</button>
        </div>
        <div class="panel card">
            <div class="setting">
                <div class="setting-text">
                    <p>yt-dlp</p>
                    <p class="hint">
                        {#if !$dependencyStatus}
                            Checking…
                        {:else if $dependencyStatus.yt_dlp_installed}
                            {$ytDlpVersion ? `Version ${$ytDlpVersion}` : "Installed"}
                        {:else}
                            Not installed
                        {/if}
                    </p>
                </div>
                {#if $dependencyStatus?.yt_dlp_installed}
                    <button
                        type="button"
                        class="btn"
                        disabled={busyTool !== null}
                        on:click={() => runTool("yt-dlp", updateYtDlp)}
                    >
                        {#if busyTool === "yt-dlp"}<span class="spinner"></span> Updating…{:else}Update{/if}
                    </button>
                {:else if $dependencyStatus}
                    <button
                        type="button"
                        class="btn btn-primary"
                        disabled={busyTool !== null}
                        on:click={() => runTool("yt-dlp", installYtDlp)}
                    >
                        {#if busyTool === "yt-dlp"}<span class="spinner"></span> Installing…{:else}Install{/if}
                    </button>
                {/if}
            </div>

            <div class="setting">
                <div class="setting-text">
                    <p>FFmpeg</p>
                    <p class="hint">
                        {#if !$dependencyStatus}
                            Checking…
                        {:else if $dependencyStatus.ffmpeg_source === "Missing"}
                            Not installed
                        {:else}
                            {ffmpegLabel}
                        {/if}
                    </p>
                </div>
                {#if $dependencyStatus?.ffmpeg_source === "Missing"}
                    <button
                        type="button"
                        class="btn btn-primary"
                        disabled={busyTool !== null}
                        on:click={() => runTool("ffmpeg", installFfmpeg)}
                    >
                        {#if busyTool === "ffmpeg"}<span class="spinner"></span> Installing…{:else}Install{/if}
                    </button>
                {/if}
            </div>
        </div>
    </section>

    <section>
        <div class="section-header"><h2 class="section-title">About</h2></div>
        <div class="panel card">
            <div class="setting">
                <div class="setting-text">
                    <p>Andy's Downloader {$appVersion ? `v${$appVersion}` : ""}</p>
                    <p class="hint selectable">
                        {#if $updateState.phase === "checking"}
                            Checking for updates…
                        {:else if $updateState.phase === "current"}
                            You're on the latest version
                        {:else if $updateState.phase === "available"}
                            Version {$updateState.version} is available
                        {:else if $updateState.phase === "installing"}
                            Installing version {$updateState.version}…
                        {:else if $updateState.phase === "error"}
                            Update failed: {$updateState.message}
                        {:else}
                            Updates are checked each time the app starts
                        {/if}
                    </p>
                </div>
                {#if $updateState.phase === "available"}
                    <button
                        type="button"
                        class="btn btn-primary"
                        disabled={$isDownloading || $bulkRunning}
                        on:click={installUpdate}
                    >
                        Update and restart
                    </button>
                {:else}
                    <button
                        type="button"
                        class="btn"
                        disabled={$updateState.phase === "checking" || $updateState.phase === "installing"}
                        on:click={() => checkForUpdates()}
                    >
                        Check for updates
                    </button>
                {/if}
            </div>
        </div>
    </section>
</div>

<style lang="scss">
    .card {
        display: flex;
        flex-direction: column;
    }

    .setting {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 16px;
        min-height: 56px;
        padding: 10px 16px;

        & + .setting {
            border-top: 1px solid var(--line);
        }
    }

    .setting-text {
        flex: 1;
        min-width: 0;
        font-size: 13px;
        font-weight: 500;
    }

    .hint {
        font-size: 12px;
        font-weight: 400;
        color: var(--text-muted);
    }

    .control {
        width: 220px;
        flex-shrink: 0;
    }

    .folder-path {
        flex: 1;
        min-width: 0;
        font-family: var(--font-mono);
        font-size: 12px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .toggle {
        width: 16px;
        height: 16px;
        accent-color: var(--accent);
        cursor: pointer;
    }
</style>
