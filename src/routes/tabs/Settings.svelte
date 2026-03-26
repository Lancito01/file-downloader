<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { FORMATS, SETTINGS_KEYS } from "$lib/assets/keys";
    import {
        browseFolder,
        saveDownloadFolderSetting,
        setStatus,
        selectedDownloadFolder,
        defaultFormat,
        embedMetadata,
        defaultAudioExtension,
        defaultVideoExtension,
        updateSettingsWithKey,
    } from "$lib/utils";

    let localFormat = $defaultFormat;
    let localEmbed = $embedMetadata;
    let localAudioExt = $defaultAudioExtension;
    let localVideoExt = $defaultVideoExtension;

    // Track changes
    $: hasChanges = localFormat !== $defaultFormat || 
        localEmbed !== $embedMetadata ||
        localAudioExt !== $defaultAudioExtension ||
        localVideoExt !== $defaultVideoExtension;
    
    // Update local values when store values change
    $: localFormat = $defaultFormat;
    $: localEmbed = $embedMetadata;
    $: localAudioExt = $defaultAudioExtension;
    $: localVideoExt = $defaultVideoExtension;

    const unsubFormat = defaultFormat.subscribe((value) => {
        if (!hasChanges) localFormat = value;
    });

    const unsubEmbed = embedMetadata.subscribe((value) => {
        if (!hasChanges) localEmbed = value;
    });

    const unsubAudioExt = defaultAudioExtension.subscribe((value) => {
        if (!hasChanges) localAudioExt = value;
    });

    const unsubVideoExt = defaultVideoExtension.subscribe((value) => {
        if (!hasChanges) localVideoExt = value;
    });

    onDestroy(() => {
        unsubFormat();
        unsubEmbed();
        unsubAudioExt();
        unsubVideoExt();
    });

    function validateExtension(ext: string): boolean {
        if (!ext.trim()) return true; // Optional field
        return /^[a-z0-9]+$/i.test(ext.trim());
    }

    async function saveAllSettings() {
        // Validate extensions
        if (!validateExtension(localAudioExt)) {
            setStatus("Audio extension should only contain letters and numbers (e.g., mp3, m4a).", "error");
            return;
        }

        if (!validateExtension(localVideoExt)) {
            setStatus("Video extension should only contain letters and numbers (e.g., mp4, mkv).", "error");
            return;
        }

        let folderSaved = false;
        
        // Save folder if set
        if ($selectedDownloadFolder) {
            folderSaved = await saveDownloadFolderSetting();
        }

        // Save format preference
        await updateSettingsWithKey(SETTINGS_KEYS.DEFAULT_FORMAT.id, localFormat);
        defaultFormat.set(localFormat);

        // Save embed preference
        await updateSettingsWithKey(SETTINGS_KEYS.EMBED_METADATA.id, localEmbed);
        embedMetadata.set(localEmbed);

        // Save audio extension preference
        await updateSettingsWithKey(SETTINGS_KEYS.DEFAULT_AUDIO_EXTENSION.id, localAudioExt);
        defaultAudioExtension.set(localAudioExt);

        // Save video extension preference
        await updateSettingsWithKey(SETTINGS_KEYS.DEFAULT_VIDEO_EXTENSION.id, localVideoExt);
        defaultVideoExtension.set(localVideoExt);

        hasChanges = false;

        setStatus(
            folderSaved
                ? "All settings saved successfully with new folder!"
                : "Settings saved successfully!",
            "success"
        );
    }

    async function handleBrowse() {
        await browseFolder();
        hasChanges = true;
    }
</script>

<div class="settings-container">
    <div class="settings-header">
        <h3>⚙️ Settings</h3>
        <p class="subtitle">Configure your download preferences</p>
    </div>

    <div class="settings-grid">
        <!-- Download Folder Card -->
        <div class="settings-card">
            <div class="card-section">
                <h4>📁 Download Folder</h4>
                <p class="section-description">Choose where downloaded files will be saved</p>
                
                <div class="folder-display">
                    {#if $selectedDownloadFolder}
                        <span class="folder-path">{$selectedDownloadFolder}</span>
                    {:else}
                        <span class="folder-placeholder">No folder selected</span>
                    {/if}
                </div>

                <button class="browse-btn" on:click={handleBrowse}>
                    📂 Browse Folders
                </button>
            </div>
        </div>

        <!-- Default Settings Card -->
        <div class="settings-card">
            <div class="card-section">
                <h4>🎛️ Default Options</h4>
                <p class="section-description">Set your preferred download options</p>

                <label class="field">
                    <span class="label-text">Default Format</span>
                    <select bind:value={localFormat}>
                        <option value={FORMATS.AUDIO}>🎵 Audio (mp3)</option>
                        <option value={FORMATS.VIDEO}>🎬 Video (mp4)</option>
                    </select>
                </label>

                <label class="field">
                    <span class="label-text">Default Audio Extension</span>
                    <input
                        type="text"
                        placeholder="mp3, m4a, wav, etc."
                        bind:value={localAudioExt}
                        maxlength="10"
                    />
                    <span class="help-text">Leave blank for auto-detection</span>
                </label>

                <label class="field">
                    <span class="label-text">Default Video Extension</span>
                    <input
                        type="text"
                        placeholder="mp4, mkv, webm, etc."
                        bind:value={localVideoExt}
                        maxlength="10"
                    />
                    <span class="help-text">Leave blank for auto-detection</span>
                </label>

                <label class="checkbox-field">
                    <input type="checkbox" bind:checked={localEmbed} />
                    <span>Embed artwork & metadata by default</span>
                </label>

                <div class="info-box">
                    <span class="info-icon">ℹ️</span>
                    <p>These settings will be used as defaults for new downloads. You can override them per download.</p>
                </div>
            </div>
        </div>
    </div>

    <!-- Save Button -->
    <div class="save-section">
        {#if hasChanges}
            <div class="changes-indicator">
                <span class="indicator-dot"></span>
                <span>Unsaved changes</span>
            </div>
        {/if}
        <button class="save-btn" class:has-changes={hasChanges} on:click={saveAllSettings}>
            💾 Save All Settings
        </button>
    </div>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";

    .settings-container {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        padding: 1rem;
        overflow-y: auto;
        box-sizing: border-box;

        @media (max-width: 768px) {
            padding: 0.75rem;
            gap: 1rem;
        }

        @media (max-width: 480px) {
            padding: 0.5rem;
            gap: 0.75rem;
        }
    }

    .settings-header {
        h3 {
            margin: 0 0 0.25rem 0;
            font-size: 1.6rem;
            color: $text;

            @media (max-width: 768px) {
                font-size: 1.4rem;
            }

            @media (max-width: 480px) {
                font-size: 1.25rem;
            }
        }

        .subtitle {
            margin: 0;
            color: $text-muted;
            font-size: 0.9rem;

            @media (max-width: 480px) {
                font-size: 0.8rem;
            }
        }
    }

    .settings-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
        gap: 1.5rem;
        flex: 1;

        @media (max-width: 768px) {
            grid-template-columns: 1fr;
            gap: 1rem;
        }

        @media (max-width: 480px) {
            gap: 0.75rem;
        }
    }

    .settings-card {
        background: $bg1;
        border: 1px solid $border;
        border-radius: 1rem;
        padding: 1.5rem;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
        display: flex;
        flex-direction: column;
        min-width: 0; // Prevent overflow

        @media (max-width: 768px) {
            padding: 1.25rem;
            border-radius: 0.75rem;
        }

        @media (max-width: 480px) {
            padding: 1rem;
            border-radius: 0.5rem;
        }
    }

    .card-section {
        display: flex;
        flex-direction: column;
        gap: 1.25rem;

        h4 {
            margin: 0;
            font-size: 1.1rem;
            color: $text;
        }

        .section-description {
            margin: -0.75rem 0 0 0;
            font-size: 0.8rem;
            color: $text-muted;
            line-height: 1.4;
        }
    }

    .folder-display {
        padding: 1rem;
        background: $surface;
        border: 1px solid $border;
        border-radius: 0.7rem;
        min-height: 60px;
        display: flex;
        align-items: center;
        justify-content: center;

        .folder-path {
            color: $text;
            font-family: $font-mono;
            font-size: 0.85rem;
            text-align: center;
            word-break: break-all;
            line-height: 1.4;
        }

        .folder-placeholder {
            color: $text-muted;
            font-style: italic;
            font-size: 0.9rem;
        }
    }

    .browse-btn {
        padding: 0.8rem 1.2rem;
        background: rgba(255, 255, 255, 0.08);
        border: 1px solid $border;
        border-radius: 0.7rem;
        color: $text;
        font-size: 0.95rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
            background: rgba(255, 255, 255, 0.12);
            border-color: $accent;
            transform: translateY(-1px);
        }

        &:active {
            transform: translateY(0);
        }

        @media (max-width: 480px) {
            padding: 0.7rem 1rem;
            font-size: 0.9rem;
        }
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

            @media (max-width: 480px) {
                font-size: 0.8rem;
            }
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
            cursor: pointer;
            transition: border-color 0.2s ease;
            width: 100%;
            box-sizing: border-box;

            &:focus {
                outline: none;
                border-color: $accent;
            }

            @media (max-width: 480px) {
                padding: 0.6rem 0.8rem;
                font-size: 0.9rem;
            }
        }

        .help-text {
            font-size: 0.75rem;
            color: $text-muted;
            font-style: italic;
            margin-top: -0.2rem;

            @media (max-width: 480px) {
                font-size: 0.7rem;
            }
        }
    }

    .checkbox-field {
        display: flex;
        align-items: center;
        gap: 0.6rem;
        color: $text;
        cursor: pointer;
        padding: 0.5rem;
        border-radius: 0.5rem;
        transition: background-color 0.2s ease;

        &:hover {
            background: rgba(255, 255, 255, 0.03);
        }

        input[type="checkbox"] {
            width: 1.1rem;
            height: 1.1rem;
            cursor: pointer;
        }

        span {
            font-size: 0.9rem;
        }
    }

    .info-box {
        display: flex;
        gap: 0.7rem;
        padding: 0.9rem;
        background: rgba(77, 136, 255, 0.1);
        border-left: 3px solid #4d88ff;
        border-radius: 0.5rem;
        align-items: flex-start;

        .info-icon {
            font-size: 1.1rem;
            flex-shrink: 0;
        }

        p {
            margin: 0;
            font-size: 0.8rem;
            color: $text-muted;
            line-height: 1.5;
        }
    }

    .save-section {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        gap: 1rem;
        padding-top: 0.5rem;
        border-top: 1px solid $border;
    }

    .changes-indicator {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: $accent;
        font-size: 0.85rem;
        font-weight: 500;

        .indicator-dot {
            width: 8px;
            height: 8px;
            background: $accent;
            border-radius: 50%;
            animation: pulse 1.5s ease-in-out infinite;
        }
    }

    @keyframes pulse {
        0%, 100% {
            opacity: 1;
        }
        50% {
            opacity: 0.5;
        }
    }

    .save-btn {
        padding: 0.9rem 2rem;
        background: transparent;
        border: 1px solid $border;
        border-radius: 0.7rem;
        color: $text;
        font-size: 1rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
            background: rgba(255, 255, 255, 0.05);
            border-color: $accent;
        }

        &.has-changes {
            background: linear-gradient(135deg, $accent, lighten($accent, 10%));
            border-color: $accent;
            color: $bg0;
            box-shadow: 0 4px 15px rgba(254, 198, 0, 0.3);

            &:hover {
                transform: translateY(-2px);
                box-shadow: 0 6px 20px rgba(254, 198, 0, 0.4);
            }

            &:active {
                transform: translateY(0);
            }
        }
    }

    @media (max-width: 768px) {
        .settings-grid {
            grid-template-columns: 1fr;
        }

        .save-section {
            flex-direction: column;
            align-items: stretch;

            .save-btn {
                width: 100%;
            }
        }
    }
</style>
