<script lang="ts">
    import { dependencyStatus, installFfmpeg, installYtDlp } from "$lib/utils";
    import Icon from "$lib/components/Icon.svelte";

    let installing = false;

    $: missingYtDlp = $dependencyStatus?.yt_dlp_installed === false;
    $: missingFfmpeg = $dependencyStatus?.ffmpeg_source === "Missing";

    async function install() {
        installing = true;
        if (missingYtDlp) await installYtDlp();
        if (missingFfmpeg) await installFfmpeg();
        installing = false;
    }
</script>

{#if missingYtDlp || missingFfmpeg}
    <div class="banner" class:blocking={missingYtDlp} role="alert">
        <Icon name="alert" />
        <p>
            {#if missingYtDlp && missingFfmpeg}
                yt-dlp and FFmpeg aren't installed. They're needed to download and convert files.
            {:else if missingYtDlp}
                yt-dlp isn't installed. It's needed to download anything.
            {:else}
                FFmpeg isn't installed. Audio conversion and high quality video won't work without it.
            {/if}
        </p>
        <button type="button" class="btn btn-small" on:click={install} disabled={installing}>
            {#if installing}
                <span class="spinner"></span> Installing…
            {:else}
                Install
            {/if}
        </button>
    </div>
{/if}

<style lang="scss">
    .banner {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 9px 12px;
        border: 1px solid rgba(234, 181, 77, 0.35);
        border-radius: var(--radius);
        background: rgba(234, 181, 77, 0.08);
        color: var(--warning);
        font-size: 13px;

        &.blocking {
            border-color: rgba(242, 109, 111, 0.4);
            background: rgba(242, 109, 111, 0.08);
            color: var(--error);
        }

        p {
            flex: 1;
            color: var(--text);
        }
    }
</style>
