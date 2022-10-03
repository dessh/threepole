import "../global.css"
import "./overlay.css"
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import createPopup from "./popups";

const loaderElem = document.querySelector<HTMLElement>("#loader")!;
const widgetElem = document.querySelector<HTMLElement>("#widget")!;
const timerElem = document.querySelector<HTMLElement>("#timer")!;
const timeElem = document.querySelector<HTMLElement>("#time")!;
const msElem = document.querySelector<HTMLElement>("#ms")!;
const counterElem = document.querySelector<HTMLElement>("#counter")!;
const dailyElem = document.querySelector<HTMLElement>("#daily")!;

let currentActivity: {
    startTime: Date;
    isRaid: boolean;
} | null;

let latestActivityCompleted: {
    period: Date;
    instanceId: string | null;
} | null;

let shown = false;
let prefs: any;
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

    appWindow.listen("update_profiles", (p) => forceRefresh(p.payload));
    appWindow.listen("update_preferences", (p) => applyPreferences(p.payload));

    let prefs: any = await invoke("get_preferences");
    applyPreferences(prefs);

    let profiles: any = await invoke("get_profiles");
    forceRefresh(profiles);
}

function applyPreferences(p: any) {
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

async function forceRefresh(p: any) {
    currentActivity = null;
    latestActivityCompleted = null;

    loaderElem.classList.remove("hidden");
    widgetElem.classList.add("hidden");

    try {
        await refreshActivity(true);
        await refreshHistory(true);

        let selectedProfile = p.selectedProfile;

        if (selectedProfile) {
            let displayProfile: any = await invoke("get_display_profile", {
                profile: selectedProfile,
            });

            createPopup({ title: `${displayProfile.displayName}#${displayProfile.displayTag}`, subtext: "Threepole is active." });
        }
    } catch (e: any) {
        let message = e.message ?? e;

        createPopup({ title: "Failed to fetch initial stats", subtext: message });
    }

    loaderElem.classList.add("hidden");
    widgetElem.classList.remove("hidden");

    setInterval(() => refreshActivity(false), 2000);
    setInterval(() => refreshHistory(false), 10000);
}

async function refreshActivity(force: boolean) {
    if (!shown && !force) {
        return;
    }

    let res: any = await invoke("get_current_activity");

    let newTime = new Date(res.latest_activity_started);
    if (!currentActivity || newTime > currentActivity.startTime) { // In case Bungie API returns an old current activity (can happen)
        currentActivity = { startTime: newTime, isRaid: res.is_raid };
    }

    if (currentActivity?.isRaid) {
        timerElem.classList.remove("hidden");
    } else {
        timerElem.classList.add("hidden");
    }
}

async function refreshHistory(force: boolean) {
    if (!shown && !force) {
        return;
    }

    let res: any = await invoke("get_history");

    let newTime = new Date(res.latest_activity_completed?.period);
    if (!latestActivityCompleted || newTime > latestActivityCompleted?.period) { // In case Bungie API returns an old activity history list
        if (latestActivityCompleted && res.latest_activity_completed && latestActivityCompleted.instanceId != res.latest_activity_completed.instance_id && prefs?.displayClearNotifications) {
            createPopup({ title: "Raid clear result", subtext: `API Time: <strong>${res.latest_activity_completed.activity_duration}</strong>` });
        }

        latestActivityCompleted = {
            period: newTime,
            instanceId: res.latest_activity_completed?.instance_id,
        };

        dailyElem.innerText = res.total_today;
    }
}

function timerTick() {
    if (!shown || !currentActivity) {
        return;
    }

    let millis = Number(new Date()) - Number(currentActivity.startTime);
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
