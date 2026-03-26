<script lang="ts">
    import type { DependencyStatus } from "$lib/types";

    export let status: DependencyStatus | null = null;
    export let loading: boolean = false;

    function getYtDlpStatus(): string {
        if (!status) return "Checking...";
        if (!status.yt_dlp_installed) return "Not installed";
        if (status.yt_dlp_version) {
            return `Installed - ${status.yt_dlp_version}`;
        }
        return "Installed";
    }

    function getFfmpegStatus(): string {
        if (!status) return "Checking...";
        return status.ffmpeg_source === "Missing" ? "Not available" : `${status.ffmpeg_source}`;
    }

    function getYtDlpIcon(): string {
        if (!status) return "⏳";
        return status.yt_dlp_installed ? "✓" : "✗";
    }

    function getFfmpegIcon(): string {
        if (!status) return "⏳";
        return status.ffmpeg_source !== "Missing" ? "✓" : "✗";
    }

    function getYtDlpColor(): string {
        if (!status) return "rgba(255, 255, 255, 0.5)";
        return status.yt_dlp_installed ? "#4dff88" : "#ff4d4d";
    }

    function getFfmpegColor(): string {
        if (!status) return "rgba(255, 255, 255, 0.5)";
        return status.ffmpeg_source !== "Missing" ? "#4dff88" : "#ff4d4d";
    }
</script>

<div class="dependency-check">
    <div class="dependency-item">
        <div class="status-icon" style="color: {getYtDlpColor()};">
            {getYtDlpIcon()}
        </div>
        <div class="dependency-info">
            <div class="dependency-name">yt-dlp</div>
            <div class="dependency-status">{getYtDlpStatus()}</div>
        </div>
    </div>

    <div class="dependency-item">
        <div class="status-icon" style="color: {getFfmpegColor()};">
            {getFfmpegIcon()}
        </div>
        <div class="dependency-info">
            <div class="dependency-name">ffmpeg</div>
            <div class="dependency-status">{getFfmpegStatus()}</div>
        </div>
    </div>
</div>

<style lang="scss">
    .dependency-check {
        display: flex;
        flex-direction: column;
        gap: 12px;
        padding: 16px;
        background-color: rgba(255, 255, 255, 0.02);
        border-radius: 8px;
        border: 1px solid rgba(255, 255, 255, 0.1);
    }

    .dependency-item {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 8px;
        border-radius: 6px;
        background-color: rgba(255, 255, 255, 0.01);

        &:hover {
            background-color: rgba(255, 255, 255, 0.02);
        }
    }

    .status-icon {
        font-size: 1.5rem;
        font-weight: bold;
        min-width: 24px;
        text-align: center;
    }

    .dependency-info {
        flex: 1;
        min-width: 0;
    }

    .dependency-name {
        font-weight: 600;
        font-size: 0.95rem;
        color: white;
        margin-bottom: 4px;
    }

    .dependency-status {
        font-size: 0.85rem;
        color: rgba(255, 255, 255, 0.6);
        font-family: monospace;
    }
</style>
