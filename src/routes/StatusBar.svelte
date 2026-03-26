<script lang="ts">
    import { status, consoleOutput, downloadProgress, isDownloading } from "$lib/utils.ts";
    import { getStatusColor } from "$lib/utils.ts";
    import { onMount } from "svelte";
    
    let consoleContainer: HTMLElement;
    let showConsole = false;
    
    $: statusColor = getStatusColor($status?.type ?? null);
    
    // Auto-scroll console output to bottom when new messages arrive
    $: if ($consoleOutput.length > 0 && consoleContainer) {
        consoleContainer.scrollTop = consoleContainer.scrollHeight;
    }
    
    // Get the latest console message for the main status line
    $: latestConsoleMessage = $consoleOutput.length > 0 
        ? $consoleOutput[$consoleOutput.length - 1].line 
        : ($status?.message || "Ready");
    
    // Toggle console display
    function toggleConsole() {
        showConsole = !showConsole;
    }
</script>

<div class="status-bar">
    <!-- Main status line -->
    <div class="status flex flex-row w-full h-9 items-center z-50 px-2.5 text-sm">
        <div class="flex items-center w-full h-full">
            <p class="status-text flex-1 overflow-hidden flex items-center">
                Status:&nbsp;<span style="color: {statusColor};">{latestConsoleMessage}</span>
            </p>
            
            <!-- Download progress indicator -->
            {#if $isDownloading && $downloadProgress.percentage !== undefined}
                <div class="progress-indicator ml-2 flex items-center">
                    <span class="text-xs mr-1">{$downloadProgress.percentage.toFixed(1)}%</span>
                    <div class="progress-bar">
                        <div 
                            class="progress-fill" 
                            style="width: {$downloadProgress.percentage}%"
                        ></div>
                    </div>
                </div>
            {/if}
            
            <!-- Console toggle button -->
            {#if $consoleOutput.length > 0}
                <button 
                    class="console-toggle ml-2 px-1 text-xs"
                    on:click={toggleConsole}
                    title={showConsole ? "Hide console" : "Show console"}
                >
                    {showConsole ? "▼" : "▲"}
                </button>
            {/if}
        </div>
    </div>
    
    <!-- Expandable console output -->
    {#if showConsole && $consoleOutput.length > 0}
        <div class="console-output">
            <div class="console-header">
                <span>Console Output</span>
                <button 
                    class="clear-console" 
                    on:click={() => consoleOutput.set([])}
                    title="Clear console"
                >
                    Clear
                </button>
            </div>
            <div 
                class="console-messages" 
                bind:this={consoleContainer}
            >
                {#each $consoleOutput as entry}
                    <div class="console-entry">
                        <span class="timestamp">[{entry.timestamp}]</span>
                        <span class="message">{entry.line}</span>
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";
    
    .status-bar {
        position: relative;
    }
    
    .status {
        font-family: $font-mono;
        background-color: $bg1;
        color: $text;
        
        .status-text {
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            height: 100%;
            max-height: 100%;
        }
        
        .progress-indicator .progress-bar {
            width: 60px;
            height: 4px;
            background-color: rgba(255, 255, 255, 0.2);
            border-radius: 2px;
            overflow: hidden;
            
            .progress-fill {
                height: 100%;
                background: linear-gradient(90deg, #4dff88, #00ff47);
                transition: width 0.3s ease;
            }
        }
        
        .console-toggle {
            background: rgba(255, 255, 255, 0.1);
            border: none;
            color: $text;
            border-radius: 3px;
            cursor: pointer;
            height: 20px;
            min-width: 20px;
            
            &:hover {
                background: rgba(255, 255, 255, 0.2);
            }
        }
    }
    
    .console-output {
        position: absolute;
        bottom: 100%;
        left: 0;
        right: 0;
        background-color: $bg1;
        border-top: 1px solid rgba(255, 255, 255, 0.1);
        max-height: 200px;
        font-family: $font-mono;
        font-size: 0.75rem;
        z-index: 100;
        
        .console-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 0.25rem 0.5rem;
            background-color: rgba(255, 255, 255, 0.05);
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            
            .clear-console {
                background: rgba(255, 255, 255, 0.1);
                border: none;
                color: $text;
                border-radius: 3px;
                cursor: pointer;
                padding: 2px 6px;
                font-size: 0.7rem;
                
                &:hover {
                    background: rgba(255, 255, 255, 0.2);
                }
            }
        }
        
        .console-messages {
            max-height: 160px;
            overflow-y: auto;
            padding: 0.25rem 0;
            
            .console-entry {
                padding: 1px 0.5rem;
                display: flex;
                font-size: 0.7rem;
                line-height: 1.2;
                
                &:hover {
                    background-color: rgba(255, 255, 255, 0.02);
                }
                
                .timestamp {
                    color: rgba(255, 255, 255, 0.5);
                    margin-right: 0.5rem;
                    flex-shrink: 0;
                    font-size: 0.65rem;
                }
                
                .message {
                    flex: 1;
                    word-break: break-all;
                }
            }
        }
    }
</style>
