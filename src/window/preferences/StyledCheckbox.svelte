<script lang="ts">
    export let checked = false;
    export let disabled = false;
</script>

<button
    class="container {checked ? 'checked' : ''}"
    on:click={() => (checked = !checked)}
    {disabled}
>
    <div class="box">
        <div class="checkmark" />
    </div>
    <span><slot /></span>
</button>

<style>
    .container {
        display: block;
        height: 20px;
    }

    .box {
        position: relative;
        display: inline-block;
        width: 16px;
        height: 16px;
        margin-right: 12px;
        border: 1px solid rgba(255, 255, 255, 0.1);
        background-color: rgba(0, 0, 0, 0.1);
        vertical-align: middle;
        border-radius: 2px;
        transition: background-color 0.1s;
    }

    .container:not(.checked):not(:disabled) .box:hover {
        background-color: rgba(255, 255, 255, 0.05);
    }

    .container.checked .box {
        background-color: var(--primary-highlight);
    }

    .container:disabled .box {
        background-color: rgba(40, 40, 40, 0.4);
    }

    .container.checked:disabled .box {
        background-color: rgba(100, 100, 100, 0.4);
    }

    .checkmark {
        position: absolute;
        opacity: 0;
        left: 6px;
        top: 2px;
        width: 3px;
        height: 8px;
        border: solid #fff;
        border-width: 0 2px 2px 0;
        transform: rotate(45deg);
        pointer-events: none;
        transition: opacity 0.1s;
    }

    .container.checked .checkmark {
        opacity: 1;
    }

    .container.checked:disabled .checkmark {
        opacity: 0.4;
    }

    span {
        display: inline-block;
        height: 20px;
        font-size: 14px;
        vertical-align: middle;
        line-height: 20px;
        color: #fff;
    }

    .container:disabled span {
        color: #777;
    }
</style>
