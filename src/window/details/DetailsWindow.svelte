<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow } from "@tauri-apps/api/window";
    import type {
        ActivityInfo,
        PlayerData,
        PlayerDataStatus,
        TauriEvent,
    } from "../../types";
    import { countClears, formatMillis, formatTime } from "../../util";
    import Window from "../Window.svelte";
    import PreviousRaid from "./PreviousRaid.svelte";
    import { RAID_ACTIVITY_TYPE } from "../../consts";
    import Dot from "./Dot.svelte";
    import Loader from "../widgets/Loader.svelte";

    let timeText = "";
    let msText = "";

    let playerData: PlayerData;
    $: countedClears = playerData ? countClears(playerData.activityHistory) : 0;

    let activityInfoMap: { [hash: number]: ActivityInfo } = {};

    setInterval(() => requestAnimationFrame(timerTick), 1000 / 30);

    function timerTick() {
        if (
            playerData?.currentActivity?.activityInfo?.activityTypeHash !=
            RAID_ACTIVITY_TYPE
        ) {
            return;
        }

        let millis =
            Number(new Date()) -
            Number(new Date(playerData.currentActivity.startDate));

        timeText = formatTime(millis);
        msText = formatMillis(millis);
    }

    appWindow.listen("playerdata_update", (e: TauriEvent<PlayerDataStatus>) => {
        playerData = e.payload.lastUpdate;
    });

    async function getActivityInfo(hash: number): Promise<ActivityInfo> {
        if (activityInfoMap[hash]) {
            return activityInfoMap[hash];
        }

        let activityInfo: ActivityInfo = await invoke("get_activity_info", {
            activityHash: hash,
        });

        activityInfoMap[hash] = activityInfo;

        return activityInfo;
    }

    async function init() {
        playerData = (await invoke<PlayerDataStatus>("get_playerdata"))
            ?.lastUpdate;
    }

    init();
</script>

<Window>
    {#if playerData}
        <div class="margin">
            <div class="status">
                {#if playerData.currentActivity?.activityInfo?.activityTypeHash == RAID_ACTIVITY_TYPE}
                    <h1>{timeText}<span class="small grey">{msText}</span></h1>
                    <h2 class="grey">
                        {playerData.currentActivity.activityInfo.name.toUpperCase()}
                    </h2>
                {:else}
                    <h1 class="small">
                        {playerData.profileInfo.displayName}<span class="grey"
                            >#{playerData.profileInfo.displayTag}</span
                        >
                    </h1>
                    <h2 class="grey">NOT IN RAID</h2>
                {/if}
            </div>
            <div class="actions">
                <button on:click={() => invoke("open_profiles")}>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        height="24"
                        width="24"
                        ><path
                            d="M12 12q-1.65 0-2.825-1.175Q8 9.65 8 8q0-1.65 1.175-2.825Q10.35 4 12 4q1.65 0 2.825 1.175Q16 6.35 16 8q0 1.65-1.175 2.825Q13.65 12 12 12Zm-8 8v-2.8q0-.85.438-1.563.437-.712 1.162-1.087 1.55-.775 3.15-1.163Q10.35 13 12 13t3.25.387q1.6.388 3.15 1.163.725.375 1.162 1.087Q20 16.35 20 17.2V20Z"
                        /></svg
                    >
                </button>
                <button on:click={() => invoke("open_preferences")}>
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
        <div class="margin">
            <p>
                <span>Today's clears</span>
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
        </div>
    {:else}
        <div class="loader">
            <Loader />
        </div>
    {/if}
</Window>

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

    .grey {
        color: #aaa;
    }

    h1 {
        font-size: 56px;
        font-weight: 600;
    }

    h1.small,
    h1 .small {
        font-size: 36px;
    }

    h2 {
        font-size: 20px;
        font-weight: 300;
        margin-top: 4px;
    }

    .status {
        display: inline-block;
    }

    .actions {
        float: right;
    }

    .actions button {
        padding: 4px;
        fill: #fff;
        transition: background-color 0.1s;
        font-size: 0;
    }

    .actions button:hover {
        background-color: rgba(0, 0, 0, 0.2);
    }

    p {
        font-size: 16px;
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
</style>
