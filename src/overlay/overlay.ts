import "../global.css"
import "./overlay.css"
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import createPopup from "./popups";
import type { TauriEvent, Preferences, CurrentActivity, PlayerDataStatus } from "../types";
import { RAID_ACTIVITY_TYPE } from "../consts";
import { countClears, formatMillis, formatTime } from "../util";

const loaderElem = document.querySelector<HTMLElement>("#loader")!;
const errorElem = document.querySelector<HTMLElement>("#error")!;
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
        if (shown) {
            return;
        }

        startTimerInterval();

        appWindow.show();
        shown = true;
    });

    appWindow.listen("hide", () => {
        if (!shown) {
            return;
        }

        appWindow.hide();
        shown = false;

        stopTimerInterval();
    });

    appWindow.listen("preferences_update", (p: TauriEvent<Preferences>) => applyPreferences(p.payload));
    appWindow.listen("playerdata_update", (e: TauriEvent<PlayerDataStatus>) => refresh(e.payload));

    applyPreferences(await invoke("get_preferences"));
    refresh(await invoke("get_playerdata"));
}

function startTimerInterval() {
    if (!timerInterval && prefs) {
        timerInterval = setInterval(() => requestAnimationFrame(timerTick), 1000 / (prefs.displayMilliseconds ? 30 : 2));
    }
}

function stopTimerInterval() {
    if (timerInterval) {
        clearTimeout(timerInterval);
        timerInterval = null;
    }
}

function refresh(playerDataStatus: PlayerDataStatus) {
    let playerData = playerDataStatus.lastUpdate;

    if (!playerData) {
        widgetElem.classList.add("hidden");

        currentActivity = null;
        doneInitialRefresh = false;

        if (playerDataStatus.error) {
            loaderElem.classList.add("hidden");
            errorElem.classList.remove("hidden");
            createPopup({ title: "Failed to fetch initial stats", subtext: playerDataStatus.error });
        } else {
            errorElem.classList.add("hidden");
            loaderElem.classList.remove("hidden");
        }

        return;
    }

    loaderElem.classList.add("hidden");
    errorElem.classList.add("hidden");
    widgetElem.classList.remove("hidden");

    currentActivity = playerData.currentActivity;

    if (currentActivity?.activityInfo?.activityTypeHash == RAID_ACTIVITY_TYPE) {
        timerElem.classList.remove("hidden");
    } else {
        timerElem.classList.add("hidden");
    }


    dailyElem.innerText = String(countClears(playerData.activityHistory));

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

    stopTimerInterval();
    startTimerInterval();
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
