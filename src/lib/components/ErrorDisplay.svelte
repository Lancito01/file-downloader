<script lang="ts">
    import type { InstallationError } from "$lib/types";

    export let error: InstallationError | null = null;
    export let onDismiss: (() => void) | null = null;

    function copyToClipboard() {
        if (!error) return;
        const text = `Error: ${error.message}\nCode: ${error.error_code || "unknown"}\nSuggestion: ${error.suggested_action || "No suggestion available"}`;
        navigator.clipboard.writeText(text);
    }
</script>

<div class="error-display" class:visible={error !== null}>
    {#if error}
        <div class="error-content">
            <div class="error-header">
                <span class="error-icon">⚠️</span>
                <span class="error-title">Error</span>
                <button class="close-btn" on:click={onDismiss} title="Close">✕</button>
            </div>

            <div class="error-message">{error.message}</div>

            {#if error.error_code}
                <div class="error-code">Code: <code>{error.error_code}</code></div>
            {/if}

            {#if error.suggested_action}
                <div class="error-suggestion">
                    <strong>What to do:</strong>
                    <p>{error.suggested_action}</p>
                </div>
            {/if}

            <div class="error-actions">
                <button class="copy-btn" on:click={copyToClipboard} title="Copy error details">
                    📋 Copy Details
                </button>
                {#if onDismiss}
                    <button class="dismiss-btn" on:click={onDismiss}>Dismiss</button>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style lang="scss">
    .error-display {
        position: fixed;
        bottom: 80px;
        right: 20px;
        max-width: 400px;
        opacity: 0;
        pointer-events: none;
        transition: opacity 0.3s ease;
        z-index: 1000;

        &.visible {
            opacity: 1;
            pointer-events: all;
        }
    }

    .error-content {
        background: linear-gradient(135deg, rgba(255, 77, 77, 0.1), rgba(255, 77, 77, 0.05));
        border: 1px solid rgba(255, 77, 77, 0.3);
        border-radius: 8px;
        padding: 16px;
        backdrop-filter: blur(10px);
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    }

    .error-header {
        display: flex;
        align-items: center;
        gap: 8px;
        margin-bottom: 12px;

        .error-icon {
            font-size: 1.25rem;
        }

        .error-title {
            font-weight: 700;
            color: #ff4d4d;
            flex: 1;
        }

        .close-btn {
            background: rgba(255, 77, 77, 0.2);
            border: none;
            color: #ff4d4d;
            width: 24px;
            height: 24px;
            border-radius: 4px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 0.9rem;
            transition: background 0.2s;

            &:hover {
                background: rgba(255, 77, 77, 0.3);
            }
        }
    }

    .error-message {
        font-size: 0.9rem;
        color: rgba(255, 255, 255, 0.9);
        margin-bottom: 12px;
        line-height: 1.4;
    }

    .error-code {
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.6);
        margin-bottom: 8px;

        code {
            background: rgba(0, 0, 0, 0.2);
            padding: 2px 6px;
            border-radius: 3px;
            font-family: monospace;
        }
    }

    .error-suggestion {
        background: rgba(77, 255, 136, 0.05);
        border-left: 3px solid rgba(77, 255, 136, 0.3);
        padding: 8px 12px;
        margin-bottom: 12px;
        border-radius: 4px;
        font-size: 0.85rem;

        strong {
            color: rgba(77, 255, 136, 0.8);
        }

        p {
            margin: 6px 0 0 0;
            color: rgba(255, 255, 255, 0.7);
            line-height: 1.3;
        }
    }

    .error-actions {
        display: flex;
        gap: 8px;
        justify-content: flex-end;
    }

    .copy-btn,
    .dismiss-btn {
        padding: 6px 12px;
        border: none;
        border-radius: 4px;
        font-size: 0.85rem;
        cursor: pointer;
        transition: background 0.2s;
    }

    .copy-btn {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.7);

        &:hover {
            background: rgba(255, 255, 255, 0.15);
            color: rgba(255, 255, 255, 0.9);
        }
    }

    .dismiss-btn {
        background: rgba(255, 77, 77, 0.2);
        color: #ff4d4d;

        &:hover {
            background: rgba(255, 77, 77, 0.3);
        }
    }

    @media (max-width: 600px) {
        .error-display {
            bottom: 70px;
            right: 10px;
            left: 10px;
            max-width: none;
        }
    }
</style>
