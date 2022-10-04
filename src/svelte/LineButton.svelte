<script lang="ts">
    export let clickCallback: (e: MouseEvent) => void;
    export let disabled: boolean = false;
</script>

<button on:click={clickCallback} {disabled}><slot /></button>

<style>
    button {
        color: #eee;
        padding: 10px;
        font-family: "Inter Tight";
        font-size: 14px;
        transition: box-shadow 0.2s, color 0.1s;
        box-shadow: 0 -1px 0 var(--primary-highlight) inset;
    }

    button:hover:not(:disabled) {
        color: #fff;
        box-shadow: 0 -4px 0 var(--primary-highlight) inset;
    }

    button:disabled {
        color: #777;
        box-shadow: 0 -1px 0 rgba(255, 255, 255, 0.1) inset;
    }
</style>
