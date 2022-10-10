<script lang="ts">
    export let checked = false;
    export let disabled = false;
</script>

<div class="container {checked ? 'checked' : ''} {disabled ? 'disabled' : ''}">
    <button on:click={() => (checked = !checked)} {disabled} /><span
        ><slot /></span
    >
    <div class="checkmark" />
</div>

<style>
    .container {
        display: inline-block;
        position: relative;
        height: 20px;
    }

    button {
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

    .container:not(.checked):not(.disabled) button:hover {
        background-color: rgba(255, 255, 255, 0.05);
    }

    .container.checked button {
        background-color: var(--primary-highlight);
    }

    .container.disabled button {
        background-color: #333;
    }

    span {
        display: inline-block;
        height: 20px;
        font-size: 14px;
        vertical-align: middle;
        line-height: 20px;
    }

    .container.disabled span {
        color: #777;
    }

    .checkmark {
        position: absolute;
        opacity: 0;
        left: 6px;
        top: 6px;
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

    .container.checked.disabled .checkmark {
        opacity: 0.4;
    }
</style>
