import "../global.css"
import "./overlay.css"
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import createPopup from "./popups";

const loaderElem = document.querySelector<HTMLElement>("#loader")!;
const widgetElem = document.querySelector<HTMLElement>("#widget")!;
const stopwatchElem = document.querySelector<HTMLElement>("#stopwatch")!;
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
let shouldNotifyRaid = true;

function init() {
    appWindow.listen("show", () => {
        appWindow.show();
        shown = true;
    });

    appWindow.listen("hide", () => {
        appWindow.hide();
        shown = false;
    });

    appWindow.listen("update_profiles", (p) => forceRefresh(p.payload));
    appWindow.listen("update_preferences", (p) => applyPreferences(p.payload));

    setInterval(stopwatchTick, 1 / 60);

    invoke("get_profiles").then((p) => forceRefresh(p));
    invoke("get_preferences").then((p) => applyPreferences(p));
}

function applyPreferences(p: any) {
    console.log(p);

    if (p.displayDailyClears) {
        counterElem.classList.remove("hidden");
    } else {
        counterElem.classList.add("hidden");
    }

    shouldNotifyRaid = p.displayClearNotifications;
}

async function forceRefresh(p: any) {
    console.log(p);

    currentActivity = null;
    latestActivityCompleted = null;

    loaderElem.classList.remove("hidden");
    widgetElem.classList.add("hidden");

    try {
        await refreshActivity(true);
        await refreshHistory(true);

        let selectedProfile = p.selectedProfile;

        if (selectedProfile) {
            createPopup({ title: `${selectedProfile.displayName}#${selectedProfile.displayTag}`, subtext: "Threepole is active." });
        }
    } catch (e: any) {
        createPopup({ title: "Failed to fetch initial stats", subtext: e.message });
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
        stopwatchElem.classList.remove("hidden");
    } else {
        stopwatchElem.classList.add("hidden");
    }
}

async function refreshHistory(force: boolean) {
    if (!shown && !force) {
        return;
    }

    let res: any = await invoke("get_history");

    let newTime = new Date(res.latest_activity_completed?.period);
    if (!latestActivityCompleted || newTime > latestActivityCompleted?.period) { // In case Bungie API returns an old activity history list
        if (latestActivityCompleted && res.latest_activity_completed && latestActivityCompleted.instanceId != res.latest_activity_completed.instance_id && shouldNotifyRaid) {
            createPopup({ title: "Raid clear result", subtext: `API Time: <strong>${res.latest_activity_completed.activity_duration}</strong>` });
        }

        latestActivityCompleted = {
            period: newTime,
            instanceId: res.latest_activity_completed?.instance_id,
        };

        dailyElem.innerText = res.total_today;
    }
}

function stopwatchTick() {
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
