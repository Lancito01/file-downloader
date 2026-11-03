<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { DownloadFormat } from "$lib/types";

    export let format: DownloadFormat;
    export let value = "";
    export let disabled = false;

    const dispatch = createEventDispatcher<{ change: string }>();

    const PRESETS: Record<DownloadFormat, string[]> = {
        audio: ["mp3", "m4a", "opus", "flac", "wav"],
        video: ["mp4", "mkv", "webm"],
    };

    // Keep a previously saved format selectable even if it isn't one of the presets
    $: options = value && !PRESETS[format].includes(value) ? [...PRESETS[format], value] : PRESETS[format];
</script>

<select class="input" bind:value {disabled} on:change={() => dispatch("change", value)}>
    <option value="">Best available</option>
    {#each options as option}
        <option value={option}>{option}</option>
    {/each}
</select>
