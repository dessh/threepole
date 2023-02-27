<script lang="ts">
    import { determineActivityType } from "../../core/util";
    import type { ActivityInfo, CompletedActivity } from "../../core/types";
    import Dot from "./Dot.svelte";

    export let activity: CompletedActivity;
    export let activityInfo: ActivityInfo;

    $: activityType = determineActivityType(activity.modes);

    function timeElapsed(): string {
        let millis =
            Number(new Date()) -
            Number(new Date(activity.period)) -
            activity.activityDurationSeconds * 1000;

        let minutes = Math.floor(millis / 60000);

        if (minutes == 0) {
            return "1m ago";
        } else if (minutes < 60) {
            return `${minutes}m ago`;
        } else {
            return `${Math.floor(minutes / 60)}h ago`;
        }
    }
</script>

<div class="raid">
    <div class="details">
        <p class="title">
            <Dot completed={activity.completed} />
            <span>{activityInfo.name}</span>
        </p>
        <p>
            {activity.activityDuration}<span
                class="center-dot"
            />{timeElapsed()}
        </p>
    </div>
    {#if activityType}
        <a
            href="https://{activityType}.report/pgcr/{activity.instanceId}"
            target="_blank"
            rel="noreferrer"
            ><svg xmlns="http://www.w3.org/2000/svg" height="20" width="20"
                ><path
                    d="M4.5 17q-.625 0-1.062-.438Q3 16.125 3 15.5v-11q0-.625.438-1.062Q3.875 3 4.5 3H10v1.5H4.5v11h11V10H17v5.5q0 .625-.438 1.062Q16.125 17 15.5 17Zm3.562-4L7 11.938 14.438 4.5H12V3h5v5h-1.5V5.562Z"
                /></svg
            ></a
        >
    {/if}
</div>

<style>
    .raid {
        background-position: center;
        z-index: 1;
        padding: 8px 0;
        display: block;
        color: #fff;
        overflow: visible;
        font-size: 0;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    }

    .raid > div {
        display: inline-block;
    }

    .details {
        padding: 8px 16px;
        font-size: 14px;
        font-weight: 300;
        color: #ccc;
    }

    .title {
        font-size: 16px;
        margin-bottom: 12px;
        font-weight: 400;
        color: #fff;
    }

    .title span {
        vertical-align: middle;
    }

    .center-dot {
        display: inline-block;
        vertical-align: middle;
        width: 3px;
        height: 3px;
        background-color: #ccc;
        border-radius: 50%;
        margin: 0 8px;
    }

    a {
        float: right;
        padding: 4px;
        font-size: 0;
        fill: #aaa;
        transition: background-color 0.1s, fill 0.1s;
    }

    a:hover {
        background-color: rgba(255, 255, 255, 0.05);
        fill: #fff;
    }
</style>
