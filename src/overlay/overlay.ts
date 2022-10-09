import "../global.css"
import "./overlay.css"
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import createPopup from "./popups";
import type { TauriEvent, Profiles, ProfileInfo, Preferences, PlayerData, CurrentActivity, PlayerDataStatus } from "../types";
import { RAID_ACTIVITY_TYPE } from "../consts";
import { formatMillis, formatTime } from "../timer";

const loaderElem = document.querySelector<HTMLElement>("#loader")!;
const widgetElem = document.querySelector<HTMLElement>("#widget")!;
const timerElem = document.querySelector<HTMLElement>("#timer")!;
const timeElem = document.querySelector<HTMLElement>("#time")!;
const msElem = document.querySelector<HTMLElement>("#ms")!;
const counterElem = document.querySelector<HTMLElement>("#counter")!;
const dailyElem = document.querySelector<HTMLElement>("#daily")!;

let currentActivity: CurrentActivity;
let lastRaidId;
let doneInitialRefresh = false;

let shown = false;
let prefs: Preferences;
let timerInterval;

async function init() {
    appWindow.listen("show", () => {
        if (!timerInterval) {
            timerInterval = setInterval(() => requestAnimationFrame(timerTick), 1000 / 30);
        }

        appWindow.show();
        shown = true;
    });

    appWindow.listen("hide", () => {
        appWindow.hide();
        shown = false;

        if (timerInterval) {
            clearTimeout(timerInterval);
            timerInterval = null;
        }
    });

    appWindow.listen("preferences_update", (p: TauriEvent<Preferences>) => applyPreferences(p.payload));
    appWindow.listen("playerdata_update", (e: TauriEvent<PlayerDataStatus>) => refresh(e.payload));

    refresh(await invoke("get_playerdata"));
    applyPreferences(await invoke("get_preferences"));
}

function refresh(playerDataStatus: PlayerDataStatus) {
    let playerData = playerDataStatus.lastUpdate;

    if (!playerData) {
        loaderElem.classList.remove("hidden");
        widgetElem.classList.add("hidden");

        currentActivity = null;
        doneInitialRefresh = false;

        if (playerDataStatus.error) {
            createPopup({ title: "Failed to fetch initial stats", subtext: playerDataStatus.error });
        }

        return;
    }

    loaderElem.classList.add("hidden");
    widgetElem.classList.remove("hidden");

    currentActivity = playerData.currentActivity;

    if (currentActivity?.activityInfo?.activityTypeHash == RAID_ACTIVITY_TYPE) {
        timerElem.classList.remove("hidden");
    } else {
        timerElem.classList.add("hidden");
    }

    let clearCount = 0;
    for (let activity of playerData.activityHistory) {
        if (activity.completed) {
            clearCount++;
        }
    }

    dailyElem.innerText = String(clearCount);

    let latestRaid = playerData.activityHistory[0];

    if (doneInitialRefresh && latestRaid && lastRaidId != latestRaid.instanceId && latestRaid.completed && prefs.displayClearNotifications) {
        createPopup({ title: "Raid clear result", subtext: `API Time: <strong>${latestRaid.activityDuration}</strong>` });
    }

    lastRaidId = latestRaid?.instanceId;

    if (!doneInitialRefresh) {
        createPopup({ title: `${playerData.profileInfo.displayName}#${playerData.profileInfo.displayTag}`, subtext: "Threepole is active." });
    }

    doneInitialRefresh = true;
}

function applyPreferences(p: Preferences) {
    prefs = p;

    if (p.displayDailyClears) {
        counterElem.classList.remove("hidden");
    } else {
        counterElem.classList.add("hidden");
    }

    if (p.displayMilliseconds) {
        msElem.classList.remove("hidden");
    } else {
        msElem.classList.add("hidden");
    }
}

function timerTick() {
    if (!shown || currentActivity?.activityInfo?.activityTypeHash != RAID_ACTIVITY_TYPE) {
        return;
    }

    let millis = Number(new Date()) - Number(new Date(currentActivity.startDate));
    timeElem.innerHTML = formatTime(millis);
    msElem.innerHTML = formatMillis(millis);
}

init();

export { shown };
