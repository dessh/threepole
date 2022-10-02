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
const dailyElem = document.querySelector<HTMLElement>("#daily")!;

let currentActivity: {
    startTime: Date,
    isRaid: boolean,
} | null = null;

let latestActivityCompleted: {
    period: Date,
    instanceId: string,
} | null = null;

let shown = false;

function init() {
    appWindow.listen("show", () => {
        appWindow.show();
        shown = true;
    });

    appWindow.listen("hide", () => {
        appWindow.hide();
        shown = false;
    });

    appWindow.listen("force_refresh", () => {
        currentActivity = null;
        latestActivityCompleted = null;
        forceRefresh().catch(e => {
            createPopup({ title: "Failed to fetch initial stats", subtext: e })
        });
    });

    forceRefresh().catch(e => {
        createPopup({ title: "Failed to fetch initial stats", subtext: e });
    }).finally(() => {
        setInterval(() => refreshActivity(false), 2000);
        setInterval(() => refreshHistory(false), 10000);
        setInterval(stopwatchTick, 1 / 60);
    });

    invoke("get_config").then((c: any) => {
        if (c) {
            createPopup({ title: `${c.display_name}#${c.display_tag}`, subtext: "Threepole is active." });
        }
    });
}

async function forceRefresh() {
    loaderElem.classList.remove("hidden");
    widgetElem.classList.add("hidden");

    await refreshActivity(true);
    await refreshHistory(true);

    loaderElem.classList.add("hidden");
    widgetElem.classList.remove("hidden");
}

async function refreshActivity(force: boolean) {
    if (!shown && !force) {
        return;
    }

    let res: any = await invoke("get_current_activity");

    let newTime = new Date(res.latest_activity_started);
    if (!currentActivity || newTime > currentActivity.startTime) {
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
    if (!latestActivityCompleted || newTime > latestActivityCompleted?.period) {
        if (!force && (!latestActivityCompleted || latestActivityCompleted.instanceId != res.latest_activity_completed?.instance_id)) {
            createPopup({ title: "Raid clear result", subtext: `API Time: <strong>${res.latest_activity_completed.activity_duration}</strong>` });
        }

        latestActivityCompleted = res.latest_activity_completed ? {
            period: newTime,
            instanceId: res.latest_activity_completed.instance_id,
        } : null;

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
