<script lang="ts">
    import { activeTab, setActiveTab } from "$lib/utils";

    let menuOpen = false;

    async function handleTabClick(tab: string) {
        await setActiveTab(tab);
        menuOpen = false;
    }

    function toggleMenu() {
        menuOpen = !menuOpen;
    }

    function closeMenu() {
        menuOpen = false;
    }
</script>

<nav class="nav-header">
    <button
        type="button"
        class="hamburger-btn"
        aria-label="Toggle navigation menu"
        aria-expanded={menuOpen}
        on:click={toggleMenu}
    >
        <span class="hamburger-icon">☰</span>
    </button>
</nav>

{#if menuOpen}
    <div class="menu-overlay" on:click={closeMenu}></div>
{/if}

<div class="sidebar-menu" class:open={menuOpen}>
    <div class="sidebar-header">
        <button
            type="button"
            class="close-btn"
            aria-label="Close navigation menu"
            on:click={closeMenu}
        >
            ✕
        </button>
    </div>

    <div class="menu-items">
        <button
            type="button"
            class="menu-item"
            class:active={$activeTab == "single"}
            on:click={() => handleTabClick("single")}
        >
            <span class="menu-icon">1️⃣</span>
            <span class="menu-text">Single Download</span>
        </button>

        <button
            type="button"
            class="menu-item"
            class:active={$activeTab == "bulk"}
            on:click={() => handleTabClick("bulk")}
        >
            <span class="menu-icon">🎒</span>
            <span class="menu-text">Bulk Download</span>
        </button>

        <button
            type="button"
            class="menu-item"
            class:active={$activeTab == "settings"}
            on:click={() => handleTabClick("settings")}
        >
            <span class="menu-icon">⚙️</span>
            <span class="menu-text">Settings</span>
        </button>
    </div>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";

    .nav-header {
        position: relative;
        background-color: $bg0;
        height: 3rem;
        display: flex;
        align-items: center;
        padding: 0 0.75rem;
        border-bottom: 1px solid $border;
        z-index: 45;
    }

    .hamburger-btn {
        background: none;
        border: none;
        color: $text;
        cursor: pointer;
        padding: 0.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 0.5rem;
        transition: background-color 0.2s ease;

        &:hover {
            background-color: rgba(255, 255, 255, 0.1);
        }

        &:active {
            background-color: rgba(255, 255, 255, 0.15);
        }

        .hamburger-icon {
            font-size: 1.5rem;
            line-height: 1;
        }
    }

    .menu-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(0, 0, 0, 0.5);
        z-index: 40;
        animation: fadeIn 0.2s ease;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    .sidebar-menu {
        position: fixed;
        top: 0;
        left: 0;
        width: 280px;
        height: 100vh;
        background-color: $bg1;
        border-right: 1px solid $border;
        z-index: 50;
        display: flex;
        flex-direction: column;
        transform: translateX(-100%);
        transition: transform 0.3s ease;

        &.open {
            transform: translateX(0);
        }
    }

    .sidebar-header {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        padding: 1rem;
        border-bottom: 1px solid $border;
    }

    .close-btn {
        background: none;
        border: none;
        color: $text;
        cursor: pointer;
        font-size: 1.5rem;
        padding: 0.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 0.5rem;
        transition: background-color 0.2s ease;

        &:hover {
            background-color: rgba(255, 255, 255, 0.1);
        }

        &:active {
            background-color: rgba(255, 255, 255, 0.15);
        }
    }

    .menu-items {
        display: flex;
        flex-direction: column;
        padding: 1rem 0;
        flex: 1;
        overflow-y: auto;
    }

    .menu-item {
        background: none;
        border: none;
        color: $text;
        padding: 0.875rem 1.25rem;
        text-align: left;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 0.875rem;
        transition: background-color 0.15s ease;
        font-size: 0.9rem;
        font-weight: 500;
        margin: 0.25rem 0.5rem;
        border-radius: 0.5rem;

        &:hover {
            background-color: rgba(255, 255, 255, 0.08);
        }

        &:active {
            background-color: rgba(255, 255, 255, 0.12);
        }

        &.active {
            background-color: rgba(245, 177, 27, 0.15);
            color: $accent;
            border-left: 3px solid $accent;
            padding-left: calc(1.25rem - 3px);
            font-weight: 600;
        }

        .menu-icon {
            font-size: 1.25rem;
            flex-shrink: 0;
        }

        .menu-text {
            flex: 1;
        }
    }

    @media (max-width: 768px) {
        .sidebar-menu {
            width: 240px;
        }

        .menu-item {
            padding: 0.75rem 1rem;
            font-size: 0.85rem;

            .menu-icon {
                font-size: 1.1rem;
            }
        }
    }

    @media (max-width: 480px) {
        .sidebar-menu {
            width: 260px; // Reduced from 280px for better mobile UX
        }

        .nav-header {
            height: 2.5rem;
            padding: 0 0.5rem;
        }

        .hamburger-btn {
            padding: 0.4rem;

            .hamburger-icon {
                font-size: 1.3rem;
            }
        }

        .menu-item {
            padding: 0.875rem 1rem;
            font-size: 0.9rem;

            .menu-icon {
                font-size: 1.2rem;
            }
        }

        .sidebar-header {
            padding: 0.75rem;
        }

        .close-btn {
            padding: 0.4rem;
            font-size: 1.3rem;
        }
    }
</style>
