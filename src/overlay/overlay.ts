import "../core/global.css"
import "./overlay.css"
import { appWindow } from "@tauri-apps/api/window";
import { createPopup as _createPopup, type Popup } from "./popups";
import type { TauriEvent, Preferences, CurrentActivity, PlayerDataStatus } from "../core/types";
import { countClears, determineActivityType, formatMillis, formatTime } from "../core/util";
import { getPlayerdata, getPreferences } from "../core/ipc";

const widgetElem = document.querySelector<HTMLElement>("#widget")!;
const loaderElem = document.querySelector<HTMLElement>("#widget-loader")!;
const errorElem = document.querySelector<HTMLElement>("#widget-error")!;
const widgetContentElem = document.querySelector<HTMLElement>("#widget-content")!;
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

        appWindow.show();
        shown = true;

        checkTimerInterval();
    });

    appWindow.listen("hide", () => {
        if (!shown) {
            return;
        }

        appWindow.hide();
        shown = false;

        checkTimerInterval();
    });

    applyPreferences(await getPreferences());
    refresh(await getPlayerdata());

    appWindow.listen("preferences_update", (p: TauriEvent<Preferences>) => applyPreferences(p.payload));
    appWindow.listen("playerdata_update", (e: TauriEvent<PlayerDataStatus>) => refresh(e.payload));
}

function createPopup(popup: Popup) {
    _createPopup(popup, shown);
}

function checkTimerInterval() {
    if (!prefs || !shown || !determineActivityType(currentActivity?.activityInfo?.activityModes)) {
        clearTimeout(timerInterval);
        timerInterval = null;
        timerElem.classList.add("hidden");
        return;
    }

    timerElem.classList.remove("hidden");

    if (!timerInterval) {
        timerInterval = setInterval(() => requestAnimationFrame(timerTick), 1000 / (prefs.displayMilliseconds ? 30 : 2));
    }
}

function refresh(playerDataStatus: PlayerDataStatus) {
    let playerData = playerDataStatus?.lastUpdate;

    if (!playerData) {
        widgetContentElem.classList.add("hidden");

        currentActivity = null;
        doneInitialRefresh = false;

        if (playerDataStatus?.error) {
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
    widgetContentElem.classList.remove("hidden");

    currentActivity = playerData.currentActivity;

    checkTimerInterval();

    dailyElem.innerText = String(countClears(playerData.activityHistory));

    let latestRaid = playerData.activityHistory[0];

    if (doneInitialRefresh && latestRaid?.completed && lastRaidId != latestRaid.instanceId && prefs.displayClearNotifications) {
        const type = determineActivityType(latestRaid.modes);

        if (type) {
            const typeFormatted = type.charAt(0).toUpperCase() + type.slice(1);
            createPopup({ title: `${typeFormatted} clear result`, subtext: `API Time: <strong>${latestRaid.activityDuration}</strong>` });
        }
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

    clearTimeout(timerInterval);
    timerInterval = null;

    checkTimerInterval();
}

function timerTick() {
    let millis = Number(new Date()) - Number(new Date(currentActivity.startDate));
    timeElem.innerHTML = formatTime(millis);
    msElem.innerHTML = formatMillis(millis);
}

init();
