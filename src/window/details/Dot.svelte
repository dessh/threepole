<script lang="ts">
    export let completed: boolean;
</script>

<span class={completed ? "completed" : ""} />

<style>
    span {
        display: inline-block;
        vertical-align: middle;
        width: 8px;
        height: 8px;
        border-radius: 50%;
        margin-right: 8px;
        background-color: #e33;
    }

    span.completed {
        background-color: #3e3;
    }
</style>
