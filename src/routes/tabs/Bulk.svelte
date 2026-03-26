<script lang="ts">
    import { onDestroy } from "svelte";
    import { FORMATS } from "$lib/assets/keys";
    import {
        downloadFromLink,
        selectedDownloadFolder,
        setStatus,
        defaultFormat,
        embedMetadata,
        defaultAudioExtension,
        defaultVideoExtension,
        currentDestination,
        availableSubfolders,
        newSubfolderName,
        refreshSubfolders,
        createSubfolder,
        getDownloadPath,
        downloadProgress,
        isDownloading,
        consoleOutput,
        isDependenciesReady,
    } from "$lib/utils";
    import type { DownloadFormat, QueueItem } from "$lib/types";
    import DownloadLocation from "$lib/components/DownloadLocation.svelte";

    let linksInput = "";
    let format: DownloadFormat = $defaultFormat;
    let extension = getDefaultExtension(format);
    let embedExtras = $embedMetadata;
    let running = false;
    let queue: QueueItem[] = [];
    let completed = 0;
    let failed = 0;
    let selectedSubfolder = "";
    let customDestination = "";

    function getDefaultExtension(fmt: DownloadFormat): string {
        return fmt === FORMATS.AUDIO ? $defaultAudioExtension : $defaultVideoExtension;
    }

    // Initialize subfolder list when component loads
    $: if ($selectedDownloadFolder) {
        refreshSubfolders();
    }

    // Update extension when format changes (user selection)
    $: if (!running) {
        extension = getDefaultExtension(format);
    }

    const unsubFormat = defaultFormat.subscribe((value) => {
        if (!running) {
            format = value;
            extension = getDefaultExtension(value);
        }
    });

    const unsubEmbed = embedMetadata.subscribe((value) => {
        if (!running) embedExtras = value;
    });

    const unsubAudioExt = defaultAudioExtension.subscribe((value) => {
        if (!running && format === FORMATS.AUDIO) {
            extension = value;
        }
    });

    const unsubVideoExt = defaultVideoExtension.subscribe((value) => {
        if (!running && format === FORMATS.VIDEO) {
            extension = value;
        }
    });

    onDestroy(() => {
        unsubFormat();
        unsubEmbed();
        unsubAudioExt();
        unsubVideoExt();
    });

    function parseLinks(): string[] {
        return linksInput
            .split(/[\n,]+/)
            .map((link) => link.trim())
            .filter((link) => link.length > 0);
    }

    function validateExtension(ext: string): boolean {
        if (!ext.trim()) return true;
        return /^[a-z0-9]+$/i.test(ext.trim());
    }

    // Compute if bulk download is possible (reactive statement)
    $: canStartBulk = !running && 
        linksInput.trim() && 
        $selectedDownloadFolder &&
        (!extension || validateExtension(extension)) &&
        $isDependenciesReady;

    async function startBulkDownload() {
        const links = parseLinks();

        if (links.length === 0) {
            setStatus("Please add at least one link to download.", "error");
            return;
        }

        if (!$selectedDownloadFolder) {
            setStatus("Please set a download folder in Settings first.", "error");
            return;
        }

        if (extension && !validateExtension(extension)) {
            setStatus("Extension should only contain letters and numbers.", "error");
            return;
        }

        // Determine final destination folder
        let finalFolder = $selectedDownloadFolder;
        
        if ($currentDestination === "browse") {
            if (!customDestination) {
                setStatus("Please select a destination folder.", "error");
                return;
            }
            finalFolder = customDestination;
        } else if ($currentDestination === "new") {
            if (!$newSubfolderName.trim()) {
                setStatus("Please enter a folder name.", "error");
                return;
            }
            finalFolder = `${$selectedDownloadFolder}/${$newSubfolderName.trim()}`;
        } else if ($currentDestination === "subfolder") {
            if (!selectedSubfolder) {
                setStatus("Please select a subfolder.", "error");
                return;
            }
            finalFolder = `${$selectedDownloadFolder}/${selectedSubfolder}`;
        }
        // If "root", finalFolder stays as $selectedDownloadFolder

        running = true;
        completed = 0;
        failed = 0;

        // Initialize queue
        queue = links.map((link) => ({
            link,
            status: "pending",
            message: "Waiting...",
        }));

        setStatus(`Starting bulk download of ${links.length} items...`, "info");

        // Process queue sequentially
        for (let i = 0; i < queue.length; i++) {
            const item = queue[i];
            item.status = "downloading";
            item.message = "Downloading...";
            queue = [...queue]; // Trigger reactivity

            setStatus(`Downloading ${i + 1}/${queue.length}: ${item.link}`, "warning");

            try {
                const result = await downloadFromLink({
                    link: item.link,
                    folder: finalFolder,
                    format,
                    extension: extension.trim() || undefined,
                    embeds: embedExtras,
                });

                item.status = result.success ? "success" : "error";
                item.message = result.message;
                item.timestamp = new Date();
                
                if (result.success) {
                    completed++;
                } else {
                    failed++;
                }
            } catch (error) {
                const message = error instanceof Error ? error.message : String(error);
                item.status = "error";
                item.message = message;
                item.timestamp = new Date();
                failed++;
            }

            queue = [...queue]; // Trigger reactivity
        }

        running = false;
        const successMsg = `Bulk download complete! ✅ ${completed} successful, ❌ ${failed} failed.`;
        setStatus(successMsg, failed > 0 ? "warning" : "success");
    }

    function clearQueue() {
        if (running) {
            setStatus("Cannot clear queue while downloading.", "error");
            return;
        }
        queue = [];
        linksInput = "";
        completed = 0;
        failed = 0;
        setStatus("Queue cleared.", "info");
    }

    function formatTime(date?: Date): string {
        if (!date) return "";
        return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
    }

    $: progressPercent = queue.length > 0 ? ((completed + failed) / queue.length) * 100 : 0;
</script>

<div class="bulk-container">
    <div class="input-card">
        <div class="card-header">
            <h3>📦 Bulk Download</h3>
            <p class="subtitle">Download multiple items at once</p>
        </div>

        <div class="form-section">
            <label class="field">
                <span class="label-text">Links (one per line)</span>
                <textarea
                    placeholder="https://youtube.com/watch?v=...&#10;https://soundcloud.com/...&#10;ytsearch:song name"
                    bind:value={linksInput}
                    disabled={running}
                    rows="8"
                ></textarea>
                <span class="help-text">Enter URLs or search terms, one per line. You can also separate with commas.</span>
            </label>

            <div class="options-grid">
                <label class="field">
                    <span class="label-text">Format</span>
                    <select bind:value={format} disabled={running}>
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
                        disabled={running}
                        maxlength="10"
                    />
                </label>
            </div>

            <label class="checkbox-field">
                <input
                    type="checkbox"
                    bind:checked={embedExtras}
                    disabled={running}
                />
                <span>Embed artwork & metadata</span>
            </label>

            <!-- Download Location -->
            <DownloadLocation
                disabled={running}
                bind:selectedSubfolder
                bind:customDestination
            />

            <div class="button-group">
                <button
                    class="start-btn"
                    class:running
                    on:click={startBulkDownload}
                    disabled={!canStartBulk}
                    title={!$isDependenciesReady ? "Install yt-dlp in Settings before downloading" : ""}
                >
                    {#if running}
                        <span class="spinner"></span>
                        Processing...
                    {:else if !$isDependenciesReady}
                        🔒 Install Dependencies First
                    {:else}
                        ▶️ Start Bulk Download
                    {/if}
                </button>
                <button
                    class="clear-btn"
                    on:click={clearQueue}
                    disabled={running}
                >
                    🗑️ Clear
                </button>
            </div>
        </div>
    </div>

    <div class="queue-card">
        <div class="queue-header">
            <div>
                <h3>📋 Download Queue</h3>
                <p class="queue-stats">
                    {#if queue.length > 0}
                        {completed + failed}/{queue.length} completed
                    {:else}
                        Empty queue
                    {/if}
                </p>
            </div>
            <div class="status-badge" class:running class:idle={!running}>
                {running ? "⚡ Running" : "💤 Idle"}
            </div>
        </div>

        {#if queue.length > 0 && running}
            <div class="progress-bar-container">
                <div class="progress-bar" style="width: {progressPercent}%"></div>
                <span class="progress-text">{Math.round(progressPercent)}%</span>
            </div>
        {/if}

        <div class="queue-list">
            {#if queue.length === 0}
                <div class="empty-state">
                    <p>No items in queue</p>
                    <span class="empty-icon">📭</span>
                    <p class="empty-hint">Add links above to get started</p>
                </div>
            {:else}
                {#each queue as item, index}
                    <div class="queue-item" class:pending={item.status === 'pending'} class:downloading={item.status === 'downloading'} class:success={item.status === 'success'} class:error={item.status === 'error'}>
                        <div class="item-number">{index + 1}</div>
                        <div class="status-indicator">
                            {#if item.status === 'pending'}
                                ⏳
                            {:else if item.status === 'downloading'}
                                <span class="mini-spinner"></span>
                            {:else if item.status === 'success'}
                                ✅
                            {:else if item.status === 'error'}
                                ❌
                            {/if}
                        </div>
                        <div class="item-content">
                            <p class="item-link">{item.link}</p>
                            <p class="item-message">{item.message}</p>
                            {#if item.timestamp}
                                <span class="item-time">{formatTime(item.timestamp)}</span>
                            {/if}
                        </div>
                    </div>
                {/each}
            {/if}
        </div>
    </div>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";

    .bulk-container {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1.5rem;
        min-height: 100%;
        width: 100%;

        @media (max-width: 900px) {
            grid-template-columns: 1fr;
        }
    }

    .input-card,
    .queue-card {
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
        select,
        textarea {
            padding: 0.7rem 0.9rem;
            background: $surface;
            border: 1px solid $border;
            border-radius: 0.6rem;
            color: $text;
            font-family: $font-ui;
            font-size: 0.95rem;
            transition: border-color 0.2s ease, background-color 0.2s ease;
            resize: vertical;

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

        textarea {
            font-family: $font-mono;
            min-height: 120px;
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

    // Destination Selection Styles
    .destination-section {
        margin-top: 1rem;

        .label-text {
            display: block;
            margin-bottom: 0.5rem;
            font-weight: 500;
            color: $text;
        }
    }

    .destination-card {
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 0.5rem;
        padding: 0.5rem;
        background: $surface-alt;
    }

    .destination-option {
        display: block;
        margin: 0.25rem 0;
        padding: 0.5rem;
        border-radius: 0.4rem;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
            background: rgba(255, 255, 255, 0.03);
        }

        input[type="radio"] {
            margin-right: 0.5rem;
        }

        .option-content {
            display: flex;
            align-items: center;
            gap: 0.5rem;

            .option-icon {
                font-size: 1rem;
            }

            .option-text {
                display: flex;
                flex-direction: column;
                gap: 0.1rem;

                .option-title {
                    font-weight: 500;
                    color: $text;
                }

                .option-desc {
                    font-size: 0.8rem;
                    color: $text-muted;
                }
            }
        }
    }

    .subfolder-dropdown {
        margin-left: 2rem;
        margin-top: 0.5rem;

        .field-select {
            width: 100%;
            padding: 0.4rem 0.6rem;
            background: $bg1;
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 0.4rem;
            color: $text;
        }
    }

    .new-folder-section {
        margin-left: 2rem;
        margin-top: 0.5rem;

        .input-group {
            display: flex;
            gap: 0.5rem;

            .field-input {
                flex: 1;
                padding: 0.4rem 0.6rem;
                background: $bg1;
                border: 1px solid rgba(255, 255, 255, 0.1);
                border-radius: 0.4rem;
                color: $text;
            }

            .create-folder-btn {
                padding: 0.4rem 0.8rem;
                background: $accent;
                color: $bg0;
                border: none;
                border-radius: 0.4rem;
                cursor: pointer;
                font-size: 0.8rem;

                &:hover:not(:disabled) {
                    background: lighten($accent, 10%);
                }

                &:disabled {
                    opacity: 0.5;
                    cursor: not-allowed;
                }
            }
        }
    }

    .browse-section {
        margin-left: 2rem;
        margin-top: 0.5rem;

        .browse-folder-btn {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.4rem 0.8rem;
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 0.4rem;
            color: $text;
            cursor: pointer;
            transition: background 0.2s ease;

            &:hover:not(:disabled) {
                background: rgba(255, 255, 255, 0.15);
            }

            &:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }

            .browse-icon {
                font-size: 0.9rem;
            }

            .browse-text {
                font-size: 0.8rem;
            }
        }

        .selected-path {
            margin-top: 0.3rem;
            font-size: 0.7rem;
            color: $text-muted;
            font-family: $font-mono;
            padding: 0.2rem 0.4rem;
            background: rgba(255, 255, 255, 0.05);
            border-radius: 0.3rem;
            word-break: break-all;
        }
    }

    .destination-preview {
        margin-top: 0.8rem;
        padding: 0.6rem;
        background: rgba(254, 198, 0, 0.1);
        border-left: 3px solid $accent;
        border-radius: 0.4rem;
        font-size: 0.8rem;

        .preview-label {
            color: $text-muted;
            font-weight: 500;
        }

        .preview-path {
            color: $text;
            font-family: $font-mono;
            display: block;
            margin-top: 0.2rem;
            word-break: break-all;
        }
    }

    .button-group {
        display: flex;
        gap: 0.75rem;
    }

    .start-btn {
        flex: 1;
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

        &:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }

        &.running {
            background: linear-gradient(135deg, darken($accent, 10%), $accent);
        }
    }

    .clear-btn {
        padding: 0.9rem 1.2rem;
        background: transparent;
        border: 1px solid $border;
        border-radius: 0.7rem;
        color: $text;
        font-size: 0.95rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover:not(:disabled) {
            background: rgba(255, 255, 255, 0.05);
            border-color: $text-muted;
        }

        &:disabled {
            opacity: 0.4;
            cursor: not-allowed;
        }
    }

    .spinner,
    .mini-spinner {
        display: inline-block;
        width: 1rem;
        height: 1rem;
        border: 2px solid rgba(0, 0, 0, 0.3);
        border-top-color: $bg0;
        border-radius: 50%;
        animation: spin 0.6s linear infinite;
    }

    .mini-spinner {
        width: 0.9rem;
        height: 0.9rem;
        border-width: 2px;
        border-top-color: $accent;
        border-right-color: transparent;
        border-bottom-color: transparent;
        border-left-color: transparent;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .queue-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 1rem;

        h3 {
            margin: 0 0 0.25rem 0;
            font-size: 1.2rem;
            color: $text;
        }

        .queue-stats {
            margin: 0;
            font-size: 0.8rem;
            color: $text-muted;
        }

        .status-badge {
            padding: 0.4rem 0.9rem;
            border-radius: 999px;
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            border: 1px solid;

            &.running {
                background: rgba(254, 198, 0, 0.2);
                border-color: $accent;
                color: $accent;
            }

            &.idle {
                background: rgba(255, 255, 255, 0.05);
                border-color: $border;
                color: $text-muted;
            }
        }
    }

    .progress-bar-container {
        position: relative;
        height: 8px;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 999px;
        overflow: hidden;
        margin-bottom: 1rem;

        .progress-bar {
            height: 100%;
            background: linear-gradient(90deg, $accent, lighten($accent, 10%));
            transition: width 0.3s ease;
            border-radius: 999px;
        }

        .progress-text {
            position: absolute;
            top: -22px;
            right: 0;
            font-size: 0.7rem;
            color: $text-muted;
            font-weight: 600;
        }
    }

    .queue-list {
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

        .empty-hint {
            font-size: 0.8rem;
            opacity: 0.7;
        }
    }

    .queue-item {
        display: flex;
        gap: 0.8rem;
        padding: 0.9rem;
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid rgba(255, 255, 255, 0.05);
        border-radius: 0.7rem;
        transition: all 0.2s ease;
        align-items: flex-start;

        &.pending {
            opacity: 0.7;
        }

        &.downloading {
            border-left: 3px solid $accent;
            background: rgba(254, 198, 0, 0.05);
        }

        &.success {
            border-left: 3px solid #4dff88;
        }

        &.error {
            border-left: 3px solid #ff4d4d;
        }
    }

    .item-number {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 28px;
        height: 28px;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 50%;
        font-size: 0.75rem;
        font-weight: 600;
        color: $text-muted;
        flex-shrink: 0;
    }

    .status-indicator {
        font-size: 1.2rem;
        flex-shrink: 0;
        margin-top: 2px;
    }

    .item-content {
        flex: 1;
        min-width: 0;

        .item-link {
            margin: 0 0 0.3rem 0;
            font-size: 0.85rem;
            color: $text;
            font-family: $font-mono;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        .item-message {
            margin: 0 0 0.4rem 0;
            font-size: 0.8rem;
            color: $text-muted;
            line-height: 1.4;
        }

        .item-time {
            font-size: 0.7rem;
            color: rgba(255, 255, 255, 0.4);
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }
    }
</style>
