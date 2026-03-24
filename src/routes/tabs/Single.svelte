<script lang="ts">
    import { onDestroy } from "svelte";
    import { FORMATS } from "$lib/assets/keys";
    import {
        downloadFromLink,
        selectedDownloadFolder,
        setStatus,
        defaultFormat,
        embedMetadata,
        type DownloadFormat,
    } from "$lib/utils";

    type HistoryItem = {
        link: string;
        status: "success" | "error";
        message: string;
        timestamp: Date;
    };

    let link = "";
    let format: DownloadFormat = $defaultFormat;
    let extension = "";
    let embedExtras = $embedMetadata;
    let downloading = false;
    let history: HistoryItem[] = [];

    // Subscribe to default format changes
    const unsubFormat = defaultFormat.subscribe((value) => {
        if (!downloading) format = value;
    });

    const unsubEmbed = embedMetadata.subscribe((value) => {
        if (!downloading) embedExtras = value;
    });

    onDestroy(() => {
        unsubFormat();
        unsubEmbed();
    });

    function validateURL(url: string): boolean {
        const trimmed = url.trim();
        if (!trimmed) return false;
        
        // Allow yt-dlp search syntax (e.g., "ytsearch:" or "scsearch:")
        if (/^(yt|sc|bc)search\d*:/.test(trimmed)) return true;
        
        // Basic URL validation
        try {
            new URL(trimmed);
            return true;
        } catch {
            // Not a valid URL, but might be a search term
            return trimmed.length > 0;
        }
    }

    function validateExtension(ext: string): boolean {
        if (!ext.trim()) return true; // Optional field
        return /^[a-z0-9]+$/i.test(ext.trim());
    }

    async function handleDownload() {
        if (downloading) return;

        // Validation
        if (!validateURL(link)) {
            setStatus("Please enter a valid URL or search term.", "error");
            return;
        }

        if (!$selectedDownloadFolder) {
            setStatus("Please set a download folder in Settings first.", "error");
            return;
        }

        if (extension && !validateExtension(extension)) {
            setStatus("Extension should only contain letters and numbers (e.g., mp3, mp4).", "error");
            return;
        }

        setStatus("Starting download...", "warning");
        downloading = true;

        try {
            const result = await downloadFromLink({
                link,
                format,
                extension: extension.trim() || undefined,
                embeds: embedExtras,
            });

            history = [
                {
                    link,
                    status: (result.success ? "success" : "error") as "success" | "error",
                    message: result.message,
                    timestamp: new Date(),
                },
                ...history,
            ].slice(0, 10); // Keep last 10

            setStatus(result.message, result.success ? "success" : "error");
            
            if (result.success) {
                link = ""; // Clear input on success
            }
        } catch (error) {
            const message = error instanceof Error ? error.message : String(error);
            history = [
                { link, status: "error" as const, message, timestamp: new Date() },
                ...history,
            ].slice(0, 10);
            setStatus(message, "error");
        } finally {
            downloading = false;
        }
    }

    function formatTime(date: Date): string {
        return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    }

    function clearHistory() {
        history = [];
        setStatus("History cleared.", "info");
    }
</script>

<div class="single-container">
    <div class="download-card">
        <div class="card-header">
            <h3>🎵 Single Download</h3>
            <p class="subtitle">Download individual tracks or videos</p>
        </div>

        <div class="form-section">
            <label class="field">
                <span class="label-text">Link or Search</span>
                <input
                    type="text"
                    placeholder="https://youtube.com/... or search term"
                    bind:value={link}
                    disabled={downloading}
                    on:keydown={(e) => e.key === 'Enter' && handleDownload()}
                />
                <span class="help-text">Enter a URL or use "ytsearch:song name" for YouTube search</span>
            </label>

            <div class="options-grid">
                <label class="field">
                    <span class="label-text">Format</span>
                    <select bind:value={format} disabled={downloading}>
                        <option value={FORMATS.AUDIO}>🎵 Audio</option>
                        <option value={FORMATS.VIDEO}>🎬 Video</option>
                    </select>
                </label>

                <label class="field">
                    <span class="label-text">Extension (optional)</span>
                    <input
                        type="text"
                        placeholder="mp3, mp4, etc."
                        bind:value={extension}
                        disabled={downloading}
                        maxlength="10"
                    />
                </label>
            </div>

            <label class="checkbox-field">
                <input
                    type="checkbox"
                    bind:checked={embedExtras}
                    disabled={downloading}
                />
                <span>Embed artwork & metadata</span>
            </label>

            <div class="destination-info">
                <span class="destination-label">📁 Destination:</span>
                <span class="destination-path">{$selectedDownloadFolder || "Not set"}</span>
            </div>

            <button
                class="download-btn"
                class:downloading
                on:click={handleDownload}
                disabled={downloading || !link.trim()}
            >
                {#if downloading}
                    <span class="spinner"></span>
                    Downloading...
                {:else}
                    ⬇️ Download Now
                {/if}
            </button>
        </div>
    </div>

    <div class="history-card">
        <div class="history-header">
            <div>
                <h3>📋 Recent Downloads</h3>
                <p class="history-count">{history.length} {history.length === 1 ? 'item' : 'items'}</p>
            </div>
            {#if history.length > 0}
                <button class="clear-btn" on:click={clearHistory}>Clear</button>
            {/if}
        </div>

        <div class="history-list">
            {#if history.length === 0}
                <div class="empty-state">
                    <p>No downloads yet</p>
                    <span class="empty-icon">📭</span>
                </div>
            {:else}
                {#each history as item}
                    <div class="history-item" class:success={item.status === 'success'} class:error={item.status === 'error'}>
                        <div class="status-indicator"></div>
                        <div class="history-content">
                            <p class="history-link">{item.link}</p>
                            <p class="history-message">{item.message}</p>
                            <span class="history-time">{formatTime(item.timestamp)}</span>
                        </div>
                    </div>
                {/each}
            {/if}
        </div>
    </div>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";

    .single-container {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1.5rem;
        height: 100%;
        width: 100%;

        @media (max-width: 900px) {
            grid-template-columns: 1fr;
        }
    }

    .download-card,
    .history-card {
        background: $bg1;
        border: 1px solid $border;
        border-radius: 1rem;
        padding: 1.5rem;
        display: flex;
        flex-direction: column;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    }

    .card-header {
        margin-bottom: 1.5rem;

        h3 {
            margin: 0 0 0.25rem 0;
            font-size: 1.4rem;
            color: $text;
        }

        .subtitle {
            margin: 0;
            color: $text-muted;
            font-size: 0.9rem;
        }
    }

    .form-section {
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
    }

    .field {
        display: flex;
        flex-direction: column;
        gap: 0.4rem;

        .label-text {
            font-size: 0.85rem;
            font-weight: 500;
            color: $text;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }

        input,
        select {
            padding: 0.7rem 0.9rem;
            background: $surface;
            border: 1px solid $border;
            border-radius: 0.6rem;
            color: $text;
            font-family: $font-ui;
            font-size: 0.95rem;
            transition: border-color 0.2s ease, background-color 0.2s ease;

            &:focus {
                outline: none;
                border-color: $accent;
                background: lighten($surface, 3%);
            }

            &:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
        }

        .help-text {
            font-size: 0.75rem;
            color: $text-muted;
            font-style: italic;
        }
    }

    .options-grid {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1rem;

        @media (max-width: 600px) {
            grid-template-columns: 1fr;
        }
    }

    .checkbox-field {
        display: flex;
        align-items: center;
        gap: 0.6rem;
        color: $text;
        cursor: pointer;

        input[type="checkbox"] {
            width: 1.1rem;
            height: 1.1rem;
            cursor: pointer;

            &:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
        }

        span {
            font-size: 0.9rem;
        }
    }

    .destination-info {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.8rem;
        background: rgba(254, 198, 0, 0.1);
        border-left: 3px solid $accent;
        border-radius: 0.5rem;
        font-size: 0.85rem;

        .destination-label {
            color: $text-muted;
            font-weight: 500;
        }

        .destination-path {
            color: $text;
            font-family: $font-mono;
            flex: 1;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
    }

    .download-btn {
        padding: 0.9rem 1.5rem;
        background: linear-gradient(135deg, $accent, lighten($accent, 10%));
        color: $bg0;
        border: none;
        border-radius: 0.7rem;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        transition: transform 0.2s ease, box-shadow 0.2s ease;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;

        &:hover:not(:disabled) {
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(254, 198, 0, 0.4);
        }

        &:active:not(:disabled) {
            transform: translateY(0);
        }

        &:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }

        &.downloading {
            background: linear-gradient(135deg, darken($accent, 10%), $accent);
        }
    }

    .spinner {
        width: 1rem;
        height: 1rem;
        border: 2px solid rgba(0, 0, 0, 0.3);
        border-top-color: $bg0;
        border-radius: 50%;
        animation: spin 0.6s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .history-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 1rem;

        h3 {
            margin: 0 0 0.25rem 0;
            font-size: 1.2rem;
            color: $text;
        }

        .history-count {
            margin: 0;
            font-size: 0.8rem;
            color: $text-muted;
        }

        .clear-btn {
            padding: 0.4rem 0.8rem;
            background: transparent;
            border: 1px solid $border;
            border-radius: 0.5rem;
            color: $text-muted;
            font-size: 0.8rem;
            cursor: pointer;
            transition: all 0.2s ease;

            &:hover {
                background: rgba(255, 255, 255, 0.05);
                border-color: $text-muted;
            }
        }
    }

    .history-list {
        flex: 1;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;

        &::-webkit-scrollbar {
            width: 6px;
        }

        &::-webkit-scrollbar-thumb {
            background: $border;
            border-radius: 3px;
        }
    }

    .empty-state {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: $text-muted;
        gap: 0.5rem;

        p {
            margin: 0;
            font-size: 0.9rem;
        }

        .empty-icon {
            font-size: 3rem;
            opacity: 0.3;
        }
    }

    .history-item {
        display: flex;
        gap: 0.8rem;
        padding: 0.9rem;
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid rgba(255, 255, 255, 0.05);
        border-radius: 0.7rem;
        transition: all 0.2s ease;

        &:hover {
            background: rgba(255, 255, 255, 0.04);
            border-color: rgba(255, 255, 255, 0.1);
        }

        &.success {
            border-left: 3px solid #4dff88;

            .status-indicator {
                background: #4dff88;
            }
        }

        &.error {
            border-left: 3px solid #ff4d4d;

            .status-indicator {
                background: #ff4d4d;
            }
        }
    }

    .status-indicator {
        width: 8px;
        height: 8px;
        border-radius: 50%;
        margin-top: 6px;
        flex-shrink: 0;
    }

    .history-content {
        flex: 1;
        min-width: 0;

        .history-link {
            margin: 0 0 0.3rem 0;
            font-size: 0.85rem;
            color: $text;
            font-family: $font-mono;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        .history-message {
            margin: 0 0 0.4rem 0;
            font-size: 0.8rem;
            color: $text-muted;
            line-height: 1.4;
        }

        .history-time {
            font-size: 0.7rem;
            color: rgba(255, 255, 255, 0.4);
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }
    }
</style>
