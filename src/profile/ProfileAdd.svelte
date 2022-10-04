<script lang="ts">
    export let clickCallback: (e: MouseEvent) => void;
</script>

<div on:click={clickCallback}>
    <svg xmlns="http://www.w3.org/2000/svg"
        ><path d="M11 19v-6H5v-2h6V5h2v6h6v2h-6v6Z" /></svg
    >
</div>

<style>
    div {
        padding: 8px;
        border: 1px solid;
        border-image-source: linear-gradient(
            90deg,
            rgba(255, 255, 255, 0.1),
            rgba(255, 255, 255, 0.1)
        );
        cursor: pointer;
        border-image-slice: 1;
        transition: border-image-source 0.1s;
        margin-top: 12px;
        text-align: center;
    }

    div:hover {
        border-image-source: linear-gradient(
            45deg,
            var(--secondary-highlight-light),
            var(--primary-highlight-light)
        );
    }

    svg {
        width: 24px;
        height: 24px;
        vertical-align: middle;
        fill: #aaa;
        transition: fill 0.1s;
    }

    div:hover svg {
        fill: #fff;
    }
</style>
