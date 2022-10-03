<script lang="ts">
    export let checked = false;
</script>

<div class="container {checked ? 'checked' : ''}">
    <button on:click={() => (checked = !checked)} /><span class="label"
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
        width: 20px;
        height: 20px;
        margin-right: 12px;
        border: 1px solid rgba(255, 255, 255, 0.1);
        background-color: rgba(0, 0, 0, 0.1);
        vertical-align: top;
        border-radius: 2px;
        transition: background-color 0.1s;
    }

    .container:not(.container.checked) button:hover {
        background-color: rgba(255, 255, 255, 0.05);
    }

    .container.checked button {
        background-color: var(--primary-highlight);
    }

    span {
        display: inline-block;
        height: 20px;
        font-size: 14px;
        vertical-align: top;
        line-height: 20px;
    }

    .checkmark {
        position: absolute;
        opacity: 0;
        left: 7px;
        top: 3px;
        width: 4px;
        height: 10px;
        border: solid #fff;
        border-width: 0 2px 2px 0;
        transform: rotate(45deg);
        pointer-events: none;
        transition: opacity 0.1s;
    }

    .container.checked .checkmark {
        opacity: 1;
    }
</style>
