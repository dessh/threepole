type CurrentActivity = {
    latestActivityStarted: string;
    isRaid: boolean;
};

type ActivityHistory = {
    totalToday: number;
    latestActivityCompleted: CompletedActivity | null;
}

type CompletedActivity = {
    period: string;
    instanceId: string;
    completed: boolean;
    activityDuration: string;
}

export type { CurrentActivity, ActivityHistory, CompletedActivity };
