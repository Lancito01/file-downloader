<script lang="ts">
    import { activeTab, setActiveTab } from "$lib/utils";
</script>

<nav class="flex flex-row items-center h-8 gap-2">
    <button
        type="button"
        class:active={$activeTab == "single"}
        aria-pressed={$activeTab == "single"}
        on:click={() => setActiveTab("single")}
    >
        1️⃣ Single Download
    </button>

    <button
        type="button"
        class:active={$activeTab == "bulk"}
        aria-pressed={$activeTab == "bulk"}
        on:click={() => setActiveTab("bulk")}
    >
        🎒 Bulk Download
    </button>

    <button
        type="button"
        class:active={$activeTab == "settings"}
        aria-pressed={$activeTab == "settings"}
        on:click={() => setActiveTab("settings")}
    >
        ⚙️ Settings
    </button>
</nav>

<style lang="scss">
    @import "$lib/styles/global.scss";
    nav {
        background-color: $bg0;

        button {
            height: 100%;
            background-color: $bg1;
            border-bottom: none;
            border-radius: 4px 4px 0 0;
            color: $text;
            cursor: pointer;
            padding: 5px 10px;
            margin: 2px;

            transition: background-color 0.1s ease-in-out;

            &:hover {
                background-color: lighten($bg0, 10%);
            }

            &.active {
                background-color: $bg0;
            }
        }
    }
</style>
