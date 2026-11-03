<script lang="ts">
    import { bulkRunning, installUpdate, isDownloading, updateState } from "$lib/utils";
    import Icon from "$lib/components/Icon.svelte";

    let dismissed = false;

    $: busy = $isDownloading || $bulkRunning;
</script>

{#if $updateState.phase === "installing"}
    <div class="banner" role="status">
        <span class="spinner"></span>
        <p>
            Installing version {$updateState.version}{$updateState.percentage != null
                ? ` (${$updateState.percentage.toFixed(0)}%)`
                : ""}. The app will restart when it's done.
        </p>
    </div>
{:else if $updateState.phase === "available" && !dismissed}
    <div class="banner" role="status">
        <Icon name="download" />
        <p>
            Version {$updateState.version} is available.{busy ? " You can install it when your downloads finish." : ""}
        </p>
        <button type="button" class="btn btn-ghost btn-small" on:click={() => (dismissed = true)}>Later</button>
        <button type="button" class="btn btn-small" on:click={installUpdate} disabled={busy}>Update and restart</button>
    </div>
{/if}

<style lang="scss">
    .banner {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 9px 12px;
        border: 1px solid rgba(245, 177, 27, 0.35);
        border-radius: var(--radius);
        background: rgba(245, 177, 27, 0.08);
        color: var(--accent);
        font-size: 13px;

        p {
            flex: 1;
            color: var(--text);
        }
    }
</style>
