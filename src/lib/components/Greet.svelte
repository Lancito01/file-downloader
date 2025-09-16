<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let name = "";
    let greetMsg = "";

    async function greet(): Promise<void> {
        try {
            greetMsg = await invoke("greet", { name });
        } catch (e) {
            console.error("Greet failed:", e);
            greetMsg = "Error: " + e;
        }
    }
</script>

<div>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>Greet</button>
    <p>{greetMsg}</p>
</div>
