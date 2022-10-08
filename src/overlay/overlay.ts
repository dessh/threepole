import "../global.css"
import "./overlay.css"
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import createPopup from "./popups";
import type { RustResult, TauriEvent, Profiles, ProfileInfo, Preferences, PlayerData, CurrentActivity } from "../types";
import { RAID_ACTIVITY_TYPE } from "../consts";

const loaderElem = document.querySelector<HTMLElement>("#loader")!;
const widgetElem = document.querySelector<HTMLElement>("#widget")!;
const timerElem = document.querySelector<HTMLElement>("#timer")!;
const timeElem = document.querySelector<HTMLElement>("#time")!;
const msElem = document.querySelector<HTMLElement>("#ms")!;
const counterElem = document.querySelector<HTMLElement>("#counter")!;
const dailyElem = document.querySelector<HTMLElement>("#daily")!;

let currentActivity: CurrentActivity;
let lastRaidId = null;
let doneInitialRefresh = false;

let shown = false;
let prefs: Preferences;
let timerInterval;

async function init() {
    appWindow.listen("show", () => {
        if (!timerInterval) {
            timerInterval = setInterval(timerTick, 1000 / 30);
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

    appWindow.listen("playerdata_update", (e: TauriEvent<RustResult<PlayerData> | null>) => {
        // if the initial refresh returned null (meaning refresh was in prog), and this returns err, overlay won't get latest data till next successful refresh
        if (e.payload?.Err) {
            if (!doneInitialRefresh) {
                createPopup({ title: "Failed to fetch initial stats", subtext: e.payload.Err });
            }
            return;
        }

        refresh(e.payload?.Ok);
    });

    refresh(await invoke("get_playerdata")); // the initial refresh

    let prefs: Preferences = await invoke("get_preferences");
    applyPreferences(prefs);
}

function refresh(playerData: PlayerData | null) {
    if (!playerData) {
        loaderElem.classList.remove("hidden");
        widgetElem.classList.add("hidden");

        currentActivity = null;
        doneInitialRefresh = false;
        return;
    } else {
        loaderElem.classList.add("hidden");
        widgetElem.classList.remove("hidden");
    }

    currentActivity = playerData.currentActivity;

    if (currentActivity && currentActivity.activityInfo.activityTypeHash == RAID_ACTIVITY_TYPE) {
        timerElem.classList.remove("hidden");
    } else {
        timerElem.classList.add("hidden");
    }

    dailyElem.innerText = String(playerData.activityHistory.length);

    if (doneInitialRefresh && playerData.activityHistory[0] && lastRaidId != playerData.activityHistory[0].instanceId && prefs.displayClearNotifications) {
        createPopup({ title: "Raid clear result", subtext: `API Time: <strong>${playerData.activityHistory[0].activityDuration}</strong>` });
    }

    lastRaidId = playerData.activityHistory[0]?.instanceId;

    if (!doneInitialRefresh) {
        showWelcomePopup();
    }

    doneInitialRefresh = true;
}

async function showWelcomePopup() {
    let profiles: Profiles = await invoke("get_profiles");

    if (!profiles.selectedProfile) {
        return;
    }

    let profileInfo: ProfileInfo = await invoke("get_profile_info", {
        profile: profiles.selectedProfile,
    });

    createPopup({ title: `${profileInfo.displayName}#${profileInfo.displayTag}`, subtext: "Threepole is active." });
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
    if (!shown || !currentActivity || currentActivity.activityInfo.activityTypeHash != RAID_ACTIVITY_TYPE) {
        return;
    }

    let millis = Number(new Date()) - Number(new Date(currentActivity.startDate));
    timeElem.innerHTML = formatTime(millis);
    msElem.innerHTML = formatMillis(millis);
}

function formatTime(millis: number): string {
    let seconds = Math.floor(millis / 1000);

    let minutes = Math.floor(seconds / 60);
    seconds = seconds - (minutes * 60);

    let hours = Math.floor(minutes / 60);
    minutes = minutes - (hours * 60);

    return (hours > 0 ? (hours + ":") : "") + String(minutes).padStart(2, "0") + ":" + String(seconds).padStart(2, "0");
}

function formatMillis(millis: number): string {
    return ":" + String(millis % 1000).padStart(3, "0").substring(0, 2);
}

init();

export { shown };
