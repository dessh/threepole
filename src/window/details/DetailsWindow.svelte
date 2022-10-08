<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow } from "@tauri-apps/api/window";
    import Window from "../Window.svelte";
</script>

<Window>
    <h1>Details</h1>
</Window>

<style>
    h1 {
        margin: 24px 48px;
    }
</style>
