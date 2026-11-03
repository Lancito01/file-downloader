<script lang="ts">
    import { consoleOutput, status } from "$lib/utils";
    import Icon from "$lib/components/Icon.svelte";

    let showConsole = false;
    let copied = false;

    function toggleConsole() {
        showConsole = !showConsole;
    }

    async function copyLog() {
        const text = $consoleOutput.map((entry) => `[${entry.timestamp}] ${entry.line}`).join("\n");
        await navigator.clipboard.writeText(text);
        copied = true;
        setTimeout(() => (copied = false), 1500);
    }
</script>

<div class="status-bar">
    {#if showConsole}
        <div class="console">
            <div class="console-header">
                <span>Log</span>
                <div class="console-actions">
                    <button type="button" class="btn btn-ghost btn-small" on:click={copyLog} disabled={$consoleOutput.length === 0}>
                        {copied ? "Copied" : "Copy"}
                    </button>
                    <button type="button" class="btn btn-ghost btn-small" on:click={() => consoleOutput.set([])} disabled={$consoleOutput.length === 0}>
                        Clear
                    </button>
                </div>
            </div>
            <!-- column-reverse keeps this pinned to the newest line without any scripting -->
            <div class="console-scroll">
                <div class="console-messages selectable">
                    {#each $consoleOutput as entry}
                        <div
                            class="console-entry"
                            class:is-error={entry.line.startsWith("ERROR")}
                            class:is-warning={entry.line.startsWith("WARNING")}
                        >
                            <span class="timestamp">{entry.timestamp}</span>
                            <span>{entry.line}</span>
                        </div>
                    {:else}
                        <p class="console-empty">Output from yt-dlp will show up here.</p>
                    {/each}
                </div>
            </div>
        </div>
    {/if}

    <div class="status-line">
        <span class="dot {$status.type ?? 'idle'}"></span>
        <p class="message" title={$status.message}>{$status.message || "Ready"}</p>
        <button
            type="button"
            class="btn btn-ghost btn-small"
            aria-expanded={showConsole}
            on:click={toggleConsole}
        >
            <Icon name="terminal" size={14} /> Log
        </button>
    </div>
</div>

<style lang="scss">
    .status-bar {
        flex-shrink: 0;
        background: var(--panel);
        border-top: 1px solid var(--line);
    }

    .status-line {
        height: 36px;
        padding: 0 6px 0 14px;
        display: flex;
        align-items: center;
        gap: 9px;
        font-size: 12px;
    }

    .dot {
        width: 7px;
        height: 7px;
        flex-shrink: 0;
        border-radius: 50%;
        background: var(--text-faint);

        &.success {
            background: var(--success);
        }
        &.error {
            background: var(--error);
        }
        &.warning {
            background: var(--warning);
        }
        &.info {
            background: var(--accent);
        }
    }

    .message {
        flex: 1;
        min-width: 0;
        color: var(--text-muted);
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .console {
        border-bottom: 1px solid var(--line);
    }

    .console-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 4px 6px 4px 14px;
        font-size: 12px;
        font-weight: 600;
        color: var(--text-muted);
    }

    .console-actions {
        display: flex;
        gap: 2px;
    }

    .console-scroll {
        height: 200px;
        display: flex;
        flex-direction: column-reverse;
        overflow-y: auto;
    }

    .console-messages {
        padding: 2px 14px 10px;
        font-family: var(--font-mono);
        font-size: 12px;
        line-height: 1.55;
    }

    .console-entry {
        display: flex;
        gap: 10px;
        overflow-wrap: anywhere;

        &.is-error {
            color: var(--error);
        }
        &.is-warning {
            color: var(--warning);
        }
    }

    .timestamp {
        flex-shrink: 0;
        color: var(--text-faint);
    }

    .console-empty {
        color: var(--text-faint);
    }
</style>
