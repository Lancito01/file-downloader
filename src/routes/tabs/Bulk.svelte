<script lang="ts">
    import {
        bulkQueue,
        bulkRunning,
        cancelDownload,
        dependencyStatus,
        displayLink,
        fileNameOf,
        isDownloading,
        isSearchTerm,
        openFolder,
        searchPlatform,
        startBulk,
    } from "$lib/utils";
    import type { QueueItem } from "$lib/types";
    import Icon from "$lib/components/Icon.svelte";
    import ActiveDownload from "$lib/components/ActiveDownload.svelte";
    import DownloadOptions from "$lib/components/DownloadOptions.svelte";

    let linksInput = "";

    $: links = linksInput
        .split(/\n+/)
        .map((link) => link.trim())
        .filter((link) => link.length > 0);
    $: hasSearchTerms = links.some(isSearchTerm);
    $: busy = $isDownloading || $bulkRunning;
    $: canStart = !busy && links.length > 0 && $dependencyStatus?.yt_dlp_installed !== false;

    $: finished = $bulkQueue.filter((item) => item.status === "success" || item.status === "error").length;

    function clearQueue() {
        bulkQueue.set([]);
        linksInput = "";
    }

    function itemTitle(item: QueueItem): string {
        return item.file_path ? fileNameOf(item.file_path) : displayLink(item.link);
    }
</script>

<div class="page">
    <h1 class="page-title">Bulk download</h1>

    <div class="composer panel">
        <label class="field">
            <span class="field-label">Links or searches, one per line</span>
            <textarea
                class="input links"
                rows="6"
                spellcheck="false"
                placeholder={"https://youtube.com/watch?v=...\nhttps://soundcloud.com/...\nartist - song name"}
                bind:value={linksInput}
                disabled={busy}
            ></textarea>
        </label>

        {#if hasSearchTerms}
            <div class="search-on">
                <span class="field-label">Search on</span>
                <div class="segmented" role="group" aria-label="Search on">
                    <button
                        type="button"
                        class:active={$searchPlatform === "youtube"}
                        aria-pressed={$searchPlatform === "youtube"}
                        disabled={busy}
                        on:click={() => ($searchPlatform = "youtube")}
                    >
                        YouTube
                    </button>
                    <button
                        type="button"
                        class:active={$searchPlatform === "soundcloud"}
                        aria-pressed={$searchPlatform === "soundcloud"}
                        disabled={busy}
                        on:click={() => ($searchPlatform = "soundcloud")}
                    >
                        SoundCloud
                    </button>
                </div>
            </div>
        {/if}

        <DownloadOptions disabled={busy} />

        <div class="actions">
            {#if $bulkRunning}
                <button type="button" class="btn btn-large" on:click={cancelDownload}>
                    Stop queue
                </button>
            {:else}
                <button type="button" class="btn btn-primary btn-large" disabled={!canStart} on:click={() => startBulk(links)}>
                    <Icon name="download" />
                    Download {links.length > 0 ? links.length : ""} {links.length === 1 ? "item" : "items"}
                </button>
            {/if}
            <button type="button" class="btn btn-ghost" on:click={clearQueue} disabled={busy}>Clear</button>
        </div>
    </div>

    {#if $bulkRunning}
        <ActiveDownload />
    {/if}

    {#if $bulkQueue.length > 0}
        <section>
            <div class="section-header">
                <h2 class="section-title">Queue</h2>
                <span class="row-meta">{finished} of {$bulkQueue.length} done</span>
            </div>

            <div class="panel row-list">
                {#each $bulkQueue as item}
                    <div class="row">
                        <span class="state {item.status}">
                            {#if item.status === "downloading"}
                                <span class="spinner"></span>
                            {:else if item.status === "success"}
                                <Icon name="check" />
                            {:else if item.status === "error"}
                                <Icon name="alert" />
                            {:else if item.status === "cancelled"}
                                <Icon name="close" />
                            {:else}
                                <Icon name="clock" />
                            {/if}
                        </span>
                        <div class="row-body">
                            <p class="row-title selectable" title={itemTitle(item)}>{itemTitle(item)}</p>
                            {#if item.status === "error"}
                                <p class="row-detail is-error selectable">{item.message}</p>
                            {/if}
                        </div>
                        {#if item.status === "success" && item.folder}
                            <button
                                type="button"
                                class="btn btn-ghost btn-icon"
                                title="Open folder"
                                aria-label="Open folder"
                                on:click={() => item.folder && openFolder(item.folder)}
                            >
                                <Icon name="folder" />
                            </button>
                        {/if}
                    </div>
                {/each}
            </div>
        </section>
    {/if}
</div>

<style lang="scss">
    .composer {
        padding: 16px;
        display: flex;
        flex-direction: column;
        gap: 14px;
    }

    .links {
        height: auto;
        padding: 9px 10px;
        min-height: 90px;
        font-family: var(--font-mono);
        font-size: 12px;
        line-height: 1.6;
        resize: vertical;
    }

    .search-on {
        display: flex;
        align-items: center;
        gap: 10px;

        .segmented {
            width: 220px;
        }
    }

    .actions {
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .state {
        display: flex;
        width: 16px;
        justify-content: center;
        color: var(--text-faint);

        &.success {
            color: var(--success);
        }
        &.error {
            color: var(--error);
        }
    }
</style>
