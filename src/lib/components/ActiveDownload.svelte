<script lang="ts">
    import { cancelDownload, downloadProgress, formatBytes, formatDuration, isDownloading } from "$lib/utils";

    let cancelling = false;

    $: if (!$isDownloading) cancelling = false;

    $: percentage = $downloadProgress?.percentage ?? null;
    $: phaseLabel =
        $downloadProgress?.phase === "processing"
            ? "Finishing up (converting and tagging)"
            : $downloadProgress?.phase === "starting"
              ? "Looking up the link"
              : "Downloading";

    async function handleCancel() {
        cancelling = true;
        await cancelDownload();
    }
</script>

{#if $isDownloading && $downloadProgress}
    <div class="active panel" aria-live="polite">
        <div class="top">
            <div class="row-body">
                <p class="row-title" title={$downloadProgress.title ?? ""}>
                    {$downloadProgress.title ?? phaseLabel}
                </p>
                <p class="row-detail">
                    {#if $downloadProgress.phase === "downloading"}
                        {[
                            percentage != null ? `${percentage.toFixed(0)}%` : "",
                            formatBytes($downloadProgress.total_bytes),
                            $downloadProgress.speed != null ? `${formatBytes($downloadProgress.speed)}/s` : "",
                            $downloadProgress.eta != null ? `${formatDuration($downloadProgress.eta)} left` : "",
                        ]
                            .filter(Boolean)
                            .join("  ·  ")}
                    {:else}
                        {phaseLabel}
                    {/if}
                </p>
            </div>
            <button type="button" class="btn btn-small" on:click={handleCancel} disabled={cancelling}>
                {cancelling ? "Cancelling…" : "Cancel"}
            </button>
        </div>
        <div
            class="track"
            role="progressbar"
            aria-label="Download progress"
            aria-valuenow={percentage ?? undefined}
            aria-valuemin="0"
            aria-valuemax="100"
        >
            {#if percentage != null}
                <div class="fill" style="width: {percentage}%"></div>
            {:else}
                <div class="fill indeterminate"></div>
            {/if}
        </div>
    </div>
{/if}

<style lang="scss">
    .active {
        padding: 12px 14px 14px;
        display: flex;
        flex-direction: column;
        gap: 11px;
        border-color: rgba(245, 177, 27, 0.35);
    }

    .top {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    .track {
        height: 6px;
        background: var(--panel-raised);
        border-radius: 6px;
        overflow: hidden;
    }

    .fill {
        height: 100%;
        background: var(--accent);
        border-radius: 6px;
        transition: width 0.15s linear;

        &.indeterminate {
            width: 30%;
            animation: slide 1.3s ease-in-out infinite;
        }
    }

    @keyframes slide {
        from {
            transform: translateX(-100%);
        }
        to {
            transform: translateX(340%);
        }
    }

    @media (prefers-reduced-motion: reduce) {
        .fill.indeterminate {
            width: 100%;
            opacity: 0.5;
            animation: none;
        }
    }
</style>
