import { DUNGEON_ACTIVITY_MODE, RAID_ACTIVITY_MODE, STRIKE_ACTIVITY_MODE } from "./consts";
import type { CompletedActivity } from "./types";

export function formatTime(millis: number): string {
    let seconds = Math.floor(millis / 1000);

    let minutes = Math.floor(seconds / 60);
    seconds = seconds - (minutes * 60);

    let hours = Math.floor(minutes / 60);
    minutes = minutes - (hours * 60);

    return (hours > 0 ? (hours + ":") : "") + String(minutes).padStart(2, "0") + ":" + String(seconds).padStart(2, "0");
}

export function formatMillis(millis: number): string {
    return ":" + String(millis % 1000).padStart(3, "0").substring(0, 2);
}

export function countClears(activityHistory: CompletedActivity[]): number {
    let clearCount = 0;
    for (let activity of activityHistory) {
        if (activity.completed) {
            clearCount++;
        }
    }

    return clearCount;
}

export function determineActivityType(modes: number[]): 'raid' | 'dungeon' | 'strike' | null {
    if (modes?.includes(RAID_ACTIVITY_MODE)) {
        return 'raid';
    } else if (modes?.includes(DUNGEON_ACTIVITY_MODE)) {
        return 'dungeon';
    } else if (modes?.includes(STRIKE_ACTIVITY_MODE)) {
        return 'strike';
    } else {
        return null;
    }
}
