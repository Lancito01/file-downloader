<script lang="ts">
    import { FORMATS } from "$lib/assets/keys";
    import {
        availableSubfolders,
        browseDestination,
        destination,
        downloadOptions,
        fileNameOf,
        selectedDownloadFolder,
    } from "$lib/utils";
    import Icon from "$lib/components/Icon.svelte";
    import ExtensionSelect from "$lib/components/ExtensionSelect.svelte";

    export let disabled = false;

    // The select holds one value, so subfolders are stored as "sub:<name>"
    $: destinationValue =
        $destination.kind === "subfolder" ? `sub:${$destination.subfolder}` : $destination.kind;

    function handleDestinationChange(event: Event) {
        const value = (event.currentTarget as HTMLSelectElement).value;

        if (value.startsWith("sub:")) {
            destination.update((current) => ({ ...current, kind: "subfolder", subfolder: value.slice(4) }));
        } else if (value === "browse") {
            destination.update((current) => ({ ...current, kind: "browse" }));
            browseDestination();
        } else {
            destination.update((current) => ({ ...current, kind: value as "root" | "new" }));
        }
    }
</script>

<div class="options">
    <div class="field">
        <span class="field-label">Type</span>
        <div class="segmented" role="group" aria-label="Download type">
            <button
                type="button"
                class:active={$downloadOptions.format === FORMATS.AUDIO}
                aria-pressed={$downloadOptions.format === FORMATS.AUDIO}
                {disabled}
                on:click={() => ($downloadOptions.format = FORMATS.AUDIO)}
            >
                <Icon name="music" size={14} /> Audio
            </button>
            <button
                type="button"
                class:active={$downloadOptions.format === FORMATS.VIDEO}
                aria-pressed={$downloadOptions.format === FORMATS.VIDEO}
                {disabled}
                on:click={() => ($downloadOptions.format = FORMATS.VIDEO)}
            >
                <Icon name="video" size={14} /> Video
            </button>
        </div>
    </div>

    <label class="field">
        <span class="field-label">File format</span>
        {#if $downloadOptions.format === FORMATS.AUDIO}
            <ExtensionSelect format="audio" bind:value={$downloadOptions.audioExtension} {disabled} />
        {:else}
            <ExtensionSelect format="video" bind:value={$downloadOptions.videoExtension} {disabled} />
        {/if}
    </label>

    <label class="field">
        <span class="field-label">Save to</span>
        <select class="input" value={destinationValue} on:change={handleDestinationChange} {disabled}>
            <option value="root">
                {$selectedDownloadFolder ? fileNameOf($selectedDownloadFolder) : "Download folder"}
            </option>
            {#if $availableSubfolders.length > 0}
                <optgroup label="Subfolders">
                    {#each $availableSubfolders as folder}
                        <option value={`sub:${folder}`}>{folder}</option>
                    {/each}
                </optgroup>
            {/if}
            <optgroup label="Other">
                <option value="new">New subfolder…</option>
                <option value="browse">Choose another folder…</option>
            </optgroup>
        </select>
    </label>

    {#if $destination.kind === "new"}
        <label class="field detail">
            <span class="field-label">New subfolder name</span>
            <input
                class="input"
                type="text"
                placeholder="Created when the download starts"
                bind:value={$destination.newName}
                {disabled}
            />
        </label>
    {:else if $destination.kind === "browse"}
        <div class="field detail">
            <span class="field-label">Folder for this download</span>
            <div class="browse-row">
                <span class="path selectable" title={$destination.custom}>
                    {$destination.custom || "No folder selected"}
                </span>
                <button type="button" class="btn" on:click={browseDestination} {disabled}>
                    <Icon name="folder" size={14} /> Browse
                </button>
            </div>
        </div>
    {/if}

    <label class="checkbox detail">
        <input type="checkbox" bind:checked={$downloadOptions.embed} {disabled} />
        Embed artwork and metadata
    </label>
</div>

<style lang="scss">
    .options {
        display: grid;
        grid-template-columns: minmax(170px, 1fr) minmax(140px, 1fr) minmax(170px, 1.3fr);
        gap: 14px;

        .detail {
            grid-column: 1 / -1;
        }
    }

    .browse-row {
        display: flex;
        align-items: center;
        gap: 10px;
    }

    .path {
        flex: 1;
        min-width: 0;
        color: var(--text-muted);
        font-family: var(--font-mono);
        font-size: 12px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
</style>
