type PlayerData = {
    currentActivity: CurrentActivity | null;
    activityHistory: CompletedActivity[];
};

type CurrentActivity = {
    startDate: string;
    activityInfo: ActivityInfo;
};

type ActivityInfo = {
    name: string;
    activityTypeHash: number;
    backgroundImage: string;
}

type CompletedActivity = {
    period: string;
    instanceId: string;
    completed: boolean;
    activityDuration: string;
    activityHash: number;
}

export type { PlayerData, CurrentActivity, ActivityInfo, CompletedActivity };
