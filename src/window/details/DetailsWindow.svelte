<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";
    import type {
        ActivityInfo,
        PlayerData,
        PlayerDataStatus,
        TauriEvent,
    } from "../../core/types";
    import { countClears, formatMillis, formatTime } from "../../core/util";
    import PreviousRaid from "./PreviousRaid.svelte";
    import {
        RAID_ACTIVITY_MODE,
        DISCORD_INVITE,
        REPOSITORY_LINK,
    } from "../../core/consts";
    import Dot from "./Dot.svelte";
    import Loader from "../widgets/Loader.svelte";
    import * as ipc from "../../core/ipc";

    let timeText = "";
    let msText = "";

    let playerData: PlayerData;
    let error: string;
    $: countedClears = playerData ? countClears(playerData.activityHistory) : 0;
    let showBanner = false;

    let activityInfoMap: { [hash: number]: ActivityInfo } = {};

    setInterval(() => requestAnimationFrame(timerTick), 1000 / 30);

    function timerTick() {
        if (
            playerData?.currentActivity?.activityInfo?.activityModes.includes(
                RAID_ACTIVITY_MODE
            )
        ) {
            return;
        }

        let millis =
            Number(new Date()) -
            Number(new Date(playerData.currentActivity.startDate));

        timeText = formatTime(millis);
        msText = formatMillis(millis);
    }

    async function getActivityInfo(hash: number): Promise<ActivityInfo> {
        if (activityInfoMap[hash]) {
            return activityInfoMap[hash];
        }

        let activityInfo = await ipc.getActivityInfo(hash);

        activityInfoMap[hash] = activityInfo;

        return activityInfo;
    }

    function handleUpdate(status: PlayerDataStatus | null) {
        playerData = status?.lastUpdate;
        error = status?.error;

        let currentActivity = playerData?.currentActivity;
        if (currentActivity?.activityInfo) {
            activityInfoMap[currentActivity.activityHash] =
                currentActivity.activityInfo;
        }
    }

    async function init() {
        handleUpdate(await ipc.getPlayerdata());

        appWindow.listen(
            "playerdata_update",
            (e: TauriEvent<PlayerDataStatus>) => handleUpdate(e.payload)
        );

        // Refresh '*m ago' text
        setInterval(() => (playerData = playerData), 30000);

        showBanner =
            new URLSearchParams(window.location.search).get("welcome") == "";
    }

    init();
</script>

<main>
    {#if playerData || error}
        {#if showBanner}
            <div class="banner margin">
                <div class="text">
                    <p class="title">Welcome to threepole!</p>
                    <p>
                        The overlay is disabled by default, and can be enabled
                        in preferences.
                    </p>
                </div>
                <button on:click={() => (showBanner = false)}>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        height="20"
                        width="20"
                    >
                        <path
                            d="M6.062 15 5 13.938 8.938 10 5 6.062 6.062 5 10 8.938 13.938 5 15 6.062 11.062 10 15 13.938 13.938 15 10 11.062Z"
                        />
                    </svg>
                </button>
            </div>
        {/if}
        <div class="header margin">
            <div class="status">
                {#if playerData}
                    {#if playerData.currentActivity?.activityInfo?.activityModes.includes(RAID_ACTIVITY_MODE)}
                        <h1>
                            {timeText}<span class="small grey">{msText}</span>
                        </h1>
                        <h2 class="grey">
                            {playerData.currentActivity.activityInfo.name.toUpperCase()}
                        </h2>
                    {:else}
                        <h1 class="small">
                            {playerData.profileInfo.displayName}<span
                                class="grey"
                                >#{playerData.profileInfo.displayTag}</span
                            >
                        </h1>
                        <h2 class="grey">NOT IN RAID</h2>
                    {/if}
                {:else}
                    <h1 class="small">Error</h1>
                    <p class="error">{error}</p>
                    <div class="error-actions">
                        <p>If this persists, consider:</p>
                        <li>
                            Joining the <a
                                href={DISCORD_INVITE}
                                target="_blank"
                                rel="noreferrer">Discord</a
                            > for support
                        </li>
                        <li>
                            Opening an issue on <a
                                href={REPOSITORY_LINK}
                                target="_blank"
                                rel="noreferrer">GitHub</a
                            >
                        </li>
                    </div>
                {/if}
            </div>
            <div class="actions">
                <button on:click={() => ipc.openProfiles()}>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        height="24"
                        width="24"
                        ><path
                            d="M12 12q-1.65 0-2.825-1.175Q8 9.65 8 8q0-1.65 1.175-2.825Q10.35 4 12 4q1.65 0 2.825 1.175Q16 6.35 16 8q0 1.65-1.175 2.825Q13.65 12 12 12Zm-8 8v-2.8q0-.85.438-1.563.437-.712 1.162-1.087 1.55-.775 3.15-1.163Q10.35 13 12 13t3.25.387q1.6.388 3.15 1.163.725.375 1.162 1.087Q20 16.35 20 17.2V20Z"
                        /></svg
                    >
                </button>
                <button on:click={() => ipc.openPreferences()}>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        height="24"
                        width="24"
                        ><path
                            d="m10.125 21-.35-2.9q-.475-.125-1.037-.437Q8.175 17.35 7.8 17l-2.675 1.125-1.875-3.25 2.325-1.75q-.05-.25-.087-.538-.038-.287-.038-.562 0-.25.038-.538.037-.287.087-.612L3.25 9.125l1.875-3.2 2.65 1.1q.45-.375.963-.675.512-.3 1.012-.45l.375-2.9h3.75l.35 2.9q.575.225 1.013.475.437.25.912.65l2.725-1.1 1.875 3.2-2.4 1.8q.1.3.1.563V12q0 .225-.012.488-.013.262-.088.637l2.35 1.75-1.875 3.25-2.675-1.15q-.475.4-.937.675-.463.275-.988.45l-.35 2.9Zm1.85-6.5q1.05 0 1.775-.725.725-.725.725-1.775 0-1.05-.725-1.775-.725-.725-1.775-.725-1.05 0-1.775.725-.725.725-.725 1.775 0 1.05.725 1.775.725.725 1.775.725Z"
                        /></svg
                    >
                </button>
            </div>
        </div>
        {#if playerData}
            <div class="margin">
                <p class="summary">
                    <span>Today's raids</span>
                    <span class="key">
                        <span class="item">
                            <Dot completed={true} />{countedClears}
                        </span>
                        <span class="item">
                            <Dot completed={false} />{playerData.activityHistory
                                .length - countedClears}
                        </span>
                    </span>
                </p>
                {#each playerData.activityHistory as activity}
                    {#await getActivityInfo(activity.activityHash) then activityInfo}
                        <PreviousRaid {activity} {activityInfo} />
                    {/await}
                {/each}
                {#if playerData.activityHistory.length == 0}
                    <p class="list-empty">No raids completed today.</p>
                {/if}
            </div>
        {/if}
    {:else}
        <div class="loader">
            <Loader />
        </div>
    {/if}
</main>

<style>
    .loader {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }

    .margin {
        margin: 24px 48px;
    }

    .banner {
        display: flex;
        padding: 12px 20px;
        background: linear-gradient(
            45deg,
            var(--secondary-highlight-light),
            var(--primary-highlight-light)
        );
        align-items: center;
    }

    .banner .text {
        flex: 1;
        font-weight: 300;
        font-size: 14px;
    }

    .banner .text .title {
        margin-bottom: 6px;
        font-weight: 500;
        font-size: 16px;
    }

    .banner button {
        width: 24px;
        height: 24px;
        margin-left: 8px;
        fill: #fff;
        font-size: 0;
        transition: background-color 0.1s;
    }

    .banner button:hover {
        background-color: rgba(0, 0, 0, 0.2);
    }

    .grey {
        color: #aaa;
    }

    h1 {
        font-size: 56px;
        font-weight: 600;
        margin-bottom: 4px;
    }

    h1.small,
    h1 .small {
        font-size: 36px;
    }

    h2 {
        font-size: 20px;
        font-weight: 300;
    }

    .header {
        display: flex;
    }

    .status {
        flex: 1;
    }

    .error {
        color: var(--error);
        margin-top: 8px;
    }

    .error-actions {
        margin-top: 20px;
        line-height: 150%;
    }

    .error-actions p {
        margin-bottom: 8px;
        font-size: 16px;
        font-weight: 500;
    }

    .error-actions li {
        font-size: 16px;
        margin-left: 12px;
        font-weight: 300;
    }

    .actions button {
        padding: 4px;
        fill: #fff;
        transition: background-color 0.1s;
        font-size: 0;
    }

    .actions button:hover {
        background-color: rgba(255, 255, 255, 0.05);
    }

    .summary {
        font-size: 16px;
        padding-bottom: 8px;
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    }

    .key {
        float: right;
    }

    .key .item {
        margin-right: 8px;
    }

    .key * {
        vertical-align: middle;
    }

    .list-empty {
        text-align: center;
        color: #aaa;
        margin-top: 12px;
        font-size: 14px;
    }
</style>
