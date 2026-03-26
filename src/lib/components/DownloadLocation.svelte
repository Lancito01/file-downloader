<script lang="ts">
    import { open } from "@tauri-apps/plugin-dialog";
    import {
        selectedDownloadFolder,
        currentDestination,
        availableSubfolders,
        newSubfolderName,
        createSubfolder
    } from "$lib/utils";

    // Props
    export let disabled = false;

    // Local state for selected options
    let selectedSubfolder = "";
    let customDestination = "";

    // Reset selected subfolder when destination changes
    $: if ($currentDestination !== "subfolder") {
        selectedSubfolder = "";
    }

    async function handleCreateSubfolder() {
        if ($newSubfolderName.trim()) {
            const success = await createSubfolder($newSubfolderName);
            if (success) {
                currentDestination.set($newSubfolderName);
                newSubfolderName.set("");
            }
        }
    }

    async function handleBrowseDestination() {
        const result = await open({
            directory: true,
            multiple: false,
            title: "Select destination folder for this download",
        });

        if (typeof result === "string") {
            customDestination = result;
            currentDestination.set("browse");
        }
    }
</script>

<div class="destination-section">
    <span class="label-text">Download Location</span>
    
    <div class="destination-card">
        <label class="destination-option">
            <input 
                type="radio" 
                bind:group={$currentDestination} 
                value="root" 
                {disabled}
            />
            <div class="option-content">
                <span class="option-icon">📁</span>
                <div class="option-text">
                    <span class="option-title">Root Folder</span>
                    <span class="option-desc">Save directly to main download folder</span>
                </div>
            </div>
        </label>

        {#if $availableSubfolders.length > 0}
            <label class="destination-option">
                <input 
                    type="radio" 
                    bind:group={$currentDestination} 
                    value="subfolder" 
                    {disabled}
                />
                <div class="option-content">
                    <span class="option-icon">📂</span>
                    <div class="option-text">
                        <span class="option-title">Existing Subfolder</span>
                        <span class="option-desc">Choose from available subfolders</span>
                    </div>
                </div>
            </label>
            
            {#if $currentDestination === "subfolder"}
                <div class="subfolder-dropdown">
                    <select bind:value={selectedSubfolder} {disabled} class="field-select">
                        <option value="">Select a subfolder...</option>
                        {#each $availableSubfolders as folder}
                            <option value={folder}>{folder}</option>
                        {/each}
                    </select>
                </div>
            {/if}
        {/if}

        <label class="destination-option">
            <input 
                type="radio" 
                bind:group={$currentDestination} 
                value="new" 
                {disabled}
            />
            <div class="option-content">
                <span class="option-icon">➕</span>
                <div class="option-text">
                    <span class="option-title">New Subfolder</span>
                    <span class="option-desc">Create a new subfolder</span>
                </div>
            </div>
        </label>
        
        {#if $currentDestination === "new"}
            <div class="new-folder-section">
                <div class="input-group">
                    <input
                        type="text"
                        placeholder="Enter folder name"
                        bind:value={$newSubfolderName}
                        {disabled}
                        class="field-input"
                        on:keydown={(e) => e.key === 'Enter' && handleCreateSubfolder()}
                    />
                    <button 
                        class="create-folder-btn"
                        on:click={handleCreateSubfolder}
                        disabled={disabled || !$newSubfolderName.trim()}
                    >
                        Create
                    </button>
                </div>
            </div>
        {/if}

        <label class="destination-option">
            <input 
                type="radio" 
                bind:group={$currentDestination} 
                value="browse" 
                {disabled}
            />
            <div class="option-content">
                <span class="option-icon">🌍</span>
                <div class="option-text">
                    <span class="option-title">Browse Anywhere</span>
                    <span class="option-desc">Select any folder on your computer</span>
                </div>
            </div>
        </label>
        
        {#if $currentDestination === "browse"}
            <div class="browse-section">
                <button 
                    class="browse-folder-btn"
                    on:click={handleBrowseDestination}
                    {disabled}
                >
                    <span class="browse-icon">📂</span>
                    <span class="browse-text">
                        {customDestination ? customDestination.split(/[/\\]/).pop() : "Select folder..."}
                    </span>
                </button>
                {#if customDestination}
                    <div class="selected-path">{customDestination}</div>
                {/if}
            </div>
        {/if}
    </div>

    <!-- Destination Preview -->
    <div class="destination-preview">
        <span class="preview-label">Final destination:</span>
        <span class="preview-path">
            {#if $currentDestination === "root"}
                {$selectedDownloadFolder || "Not set"}
            {:else if $currentDestination === "browse"}
                {customDestination || "Not selected"}
            {:else if $currentDestination === "subfolder" && selectedSubfolder}
                {$selectedDownloadFolder}/{selectedSubfolder}
            {:else if $currentDestination === "new" && $newSubfolderName}
                {$selectedDownloadFolder}/{$newSubfolderName}
            {:else}
                {$selectedDownloadFolder || "Not set"}
            {/if}
        </span>
    </div>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";

    .destination-section {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        margin: 1rem 0;

        .label-text {
            font-size: 0.85rem;
            font-weight: 500;
            color: $text;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }
    }

    .destination-card {
        background: $surface;
        border: 1px solid $border;
        border-radius: 0.75rem;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .destination-option {
        display: flex;
        align-items: center; // Fixed: was flex-start, now centers radio button properly
        gap: 0.75rem;
        padding: 0.75rem;
        border-radius: 0.5rem;
        cursor: pointer;
        transition: background-color 0.2s ease;
        border: 1px solid transparent;

        &:hover {
            background-color: rgba(255, 255, 255, 0.05);
        }

        &:has(input:checked) {
            background-color: rgba(245, 177, 27, 0.1);
            border-color: rgba(245, 177, 27, 0.3);
        }

        input[type="radio"] {
            // Removed manual margin-top since we now use proper alignment
            cursor: pointer;
            flex-shrink: 0;
        }
    }

    .option-content {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        flex: 1;
    }

    .option-icon {
        font-size: 1.25rem;
        flex-shrink: 0;
    }

    .option-text {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;

        .option-title {
            font-size: 0.95rem;
            font-weight: 500;
            color: $text;
        }

        .option-desc {
            font-size: 0.8rem;
            color: $text-muted;
            line-height: 1.3;
        }
    }

    .subfolder-dropdown,
    .new-folder-section,
    .browse-section {
        margin-left: calc(1.5rem + 0.75rem); // More flexible: radio width + gap
        margin-top: 0.5rem;
    }

    .field-select,
    .field-input {
        width: 100%;
        padding: 0.7rem 0.9rem;
        background: $bg1;
        border: 1px solid $border;
        border-radius: 0.6rem;
        color: $text;
        font-family: $font-ui;
        font-size: 0.95rem;
        transition: border-color 0.2s ease;

        &:focus {
            outline: none;
            border-color: $accent;
        }

        &:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
    }

    .input-group {
        display: flex;
        gap: 0.75rem;
        align-items: center;

        .field-input {
            flex: 1;
        }
    }

    .create-folder-btn {
        padding: 0.7rem 1.25rem;
        background: rgba(245, 177, 27, 0.15);
        border: 1px solid $accent;
        border-radius: 0.6rem;
        color: $accent;
        font-size: 0.9rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;
        white-space: nowrap;

        &:hover {
            background: rgba(245, 177, 27, 0.25);
            transform: translateY(-1px);
        }

        &:disabled {
            opacity: 0.5;
            cursor: not-allowed;
            transform: none;
        }
    }

    .browse-folder-btn {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        width: 100%;
        padding: 0.75rem 1rem;
        background: $bg1;
        border: 1px solid $border;
        border-radius: 0.6rem;
        color: $text;
        font-size: 0.9rem;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
            background: rgba(255, 255, 255, 0.05);
            border-color: $accent;
            transform: translateY(-1px);
        }

        .browse-icon {
            font-size: 1.1rem;
        }

        .browse-text {
            flex: 1;
            text-align: left;
            font-family: $font-mono;
        }
    }

    .selected-path {
        margin-top: 0.5rem;
        padding: 0.5rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 0.5rem;
        font-size: 0.8rem;
        color: $text-muted;
        font-family: $font-mono;
        word-break: break-all;
        line-height: 1.4;
    }

    .destination-preview {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        padding: 0.75rem;
        background: rgba(245, 177, 27, 0.05);
        border-left: 3px solid $accent;
        border-radius: 0.5rem;

        .preview-label {
            font-size: 0.75rem;
            color: $accent;
            font-weight: 500;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }

        .preview-path {
            font-size: 0.85rem;
            color: $text;
            font-family: $font-mono;
            word-break: break-all;
            line-height: 1.4;
        }
    }
</style>