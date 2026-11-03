<script lang="ts">
    import { activeTab, bulkRunning, bulkQueue, isDownloading, setActiveTab } from "$lib/utils";
    import Icon from "$lib/components/Icon.svelte";

    const tabs = [
        { id: "single", label: "Download", icon: "download" },
        { id: "bulk", label: "Bulk", icon: "layers" },
        { id: "settings", label: "Settings", icon: "settings" },
    ] as const;

    $: finishedCount = $bulkQueue.filter((item) => item.status !== "pending" && item.status !== "downloading").length;
</script>

<nav class="sidebar" aria-label="Main">
    <div class="brand">Andy's Downloader</div>

    {#each tabs as tab}
        <button
            type="button"
            class="nav-item"
            class:active={$activeTab === tab.id}
            aria-current={$activeTab === tab.id ? "page" : undefined}
            on:click={() => setActiveTab(tab.id)}
        >
            <Icon name={tab.icon} />
            <span class="label">{tab.label}</span>
            {#if tab.id === "bulk" && $bulkRunning}
                <span class="count">{finishedCount}/{$bulkQueue.length}</span>
            {:else if tab.id === "single" && $isDownloading && !$bulkRunning}
                <span class="spinner"></span>
            {/if}
        </button>
    {/each}
</nav>

<style lang="scss">
    .sidebar {
        width: 184px;
        flex-shrink: 0;
        padding: 16px 10px;
        display: flex;
        flex-direction: column;
        gap: 2px;
        background: var(--panel);
        border-right: 1px solid var(--line);
    }

    .brand {
        padding: 4px 10px 16px;
        font-size: 13px;
        font-weight: 650;
        letter-spacing: -0.005em;
    }

    .nav-item {
        display: flex;
        align-items: center;
        gap: 10px;
        height: 36px;
        padding: 0 10px;
        border-radius: var(--radius);
        color: var(--text-muted);
        font-size: 13px;
        font-weight: 550;
        text-align: left;
        cursor: pointer;
        transition: background-color 0.12s ease, color 0.12s ease;

        &:hover {
            background: var(--panel-raised);
            color: var(--text);
        }

        &.active {
            background: var(--accent-soft);
            color: var(--accent);
        }
    }

    .label {
        flex: 1;
    }

    .count {
        font-size: 11px;
        font-variant-numeric: tabular-nums;
    }
</style>
