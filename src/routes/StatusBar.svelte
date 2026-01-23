<script lang="ts">
    import { status } from "$lib/utils.ts";
    import { getStatusColor } from "$lib/utils.ts";
    let statusColor = "white";
    $: if ($status.type) {
        statusColor = getStatusColor();
    }
</script>

<div class="status flex flex-row w-full h-9 items-center z-50 px-2.5 text-sm">
    <p class="w-full h-full overflow-hidden flex items-center">
        Status:&nbsp;<span style="color: {statusColor};">{$status.message}</span
        >
    </p>
</div>

<style lang="scss">
    @import "$lib/styles/global.scss";
    .status {
        font-family: $font-mono;
        background-color: $bg1;
        color: $text;

        &p {
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
            height: 100%;
            max-height: 100%;
        }
    }
</style>
