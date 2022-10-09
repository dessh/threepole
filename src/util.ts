import type { CompletedActivity } from "./types";

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

function countClears(activityHistory: CompletedActivity[]): number {
    let clearCount = 0;
    for (let activity of activityHistory) {
        if (activity.completed) {
            clearCount++;
        }
    }

    return clearCount;
}

export { formatTime, formatMillis, countClears };
