<script lang="ts">
    import {
        bulkRunning,
        clearHistory,
        dependencyStatus,
        displayLink,
        downloadSingle,
        fileNameOf,
        history,
        isDownloading,
        isSearchTerm,
        openFolder,
        searchPlatform,
    } from "$lib/utils";
    import type { HistoryItem } from "$lib/types";
    import Icon from "$lib/components/Icon.svelte";
    import ActiveDownload from "$lib/components/ActiveDownload.svelte";
    import DownloadOptions from "$lib/components/DownloadOptions.svelte";

    let link = "";

    $: busy = $isDownloading || $bulkRunning;
    $: canDownload = !busy && link.trim().length > 0 && $dependencyStatus?.yt_dlp_installed !== false;
    $: showSearchPlatform = isSearchTerm(link);

    async function handleDownload() {
        if (!canDownload) return;
        const succeeded = await downloadSingle(link);
        if (succeeded) link = "";
    }

    function formatTime(timestamp: string): string {
        const date = new Date(timestamp);
        const isToday = date.toDateString() === new Date().toDateString();
        return date.toLocaleString([], isToday
            ? { hour: "2-digit", minute: "2-digit" }
            : { month: "short", day: "numeric" });
    }

    function itemTitle(item: HistoryItem): string {
        return item.file_path ? fileNameOf(item.file_path) : displayLink(item.link);
    }
</script>

<div class="page">
    <h1 class="page-title">Download</h1>

    <form class="composer panel" on:submit|preventDefault={handleDownload}>
        <div class="link-row">
            <input
                class="input link-input"
                type="text"
                placeholder="Paste a link, or type something to search for"
                aria-label="Link or search"
                spellcheck="false"
                bind:value={link}
                disabled={busy}
            />
            <button type="submit" class="btn btn-primary btn-large" disabled={!canDownload}>
                <Icon name="download" /> Download
            </button>
        </div>

        {#if showSearchPlatform}
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
                <span class="search-hint">The top result gets downloaded.</span>
            </div>
        {/if}

        <DownloadOptions disabled={busy} />
    </form>

    {#if !$bulkRunning}
        <ActiveDownload />
    {/if}

    <section>
        <div class="section-header">
            <h2 class="section-title">Recent</h2>
            {#if $history.length > 0}
                <button type="button" class="btn btn-ghost btn-small" on:click={clearHistory}>Clear</button>
            {/if}
        </div>

        <div class="panel">
            {#if $history.length === 0}
                <p class="empty-state">Finished downloads will show up here.</p>
            {:else}
                <div class="row-list">
                    {#each $history as item}
                        <div class="row">
                            <span class="result" class:is-error={item.status === "error"}>
                                <Icon name={item.status === "success" ? "check" : "alert"} />
                            </span>
                            <div class="row-body">
                                <p class="row-title selectable" title={itemTitle(item)}>{itemTitle(item)}</p>
                                {#if item.status === "error"}
                                    <p class="row-detail is-error selectable">{item.message}</p>
                                {:else}
                                    <p class="row-detail" title={item.folder}>{item.folder}</p>
                                {/if}
                            </div>
                            <span class="row-meta">{formatTime(item.timestamp)}</span>
                            <button
                                type="button"
                                class="btn btn-ghost btn-icon"
                                title="Open folder"
                                aria-label="Open folder"
                                on:click={() => openFolder(item.folder)}
                            >
                                <Icon name="folder" />
                            </button>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
    </section>
</div>

<style lang="scss">
    .composer {
        padding: 16px;
        display: flex;
        flex-direction: column;
        gap: 14px;
    }

    .link-row {
        display: flex;
        gap: 10px;
    }

    .link-input {
        height: 42px;
        padding: 0 12px;
        font-size: 14px;
    }

    .search-on {
        display: flex;
        align-items: center;
        gap: 10px;

        .segmented {
            width: 220px;
        }
    }

    .search-hint {
        font-size: 12px;
        color: var(--text-faint);
    }

    .result {
        color: var(--success);

        &.is-error {
            color: var(--error);
        }
    }
</style>
