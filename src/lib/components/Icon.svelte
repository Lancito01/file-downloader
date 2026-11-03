<script lang="ts">
    export let name: keyof typeof paths;
    export let size = 16;

    const paths = {
        download: "M12 3v12m0 0 5-5m-5 5-5-5M4 21h16",
        layers: "m12 3 9 5-9 5-9-5 9-5Zm-9 9.5 9 5 9-5m-18 4 9 5 9-5",
        settings: "M4 7h9m4 0h3M4 17h3m4 0h9M15 5v4m-6 6v4",
        folder: "M3 7a2 2 0 0 1 2-2h4l2 2.5h8a2 2 0 0 1 2 2V18a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V7Z",
        music: "M9 18V6l11-2v12M9 18a3 3 0 1 1-6 0 3 3 0 0 1 6 0Zm11-2a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z",
        video: "M4 6h11a1 1 0 0 1 1 1v10a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1Zm12 4.5L21 7v10l-5-3.5",
        check: "m5 12.5 4.5 4.5L19 7.5",
        alert: "M12 8v5m0 3.5v.01M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18Z",
        close: "M6 6l12 12M18 6 6 18",
        clock: "M12 7v5l3 2m6-2a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
        terminal: "m5 8 4 4-4 4m6 0h8",
    };
</script>

<svg
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.9"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
    style="flex-shrink: 0"
>
    <path d={paths[name]} />
</svg>
