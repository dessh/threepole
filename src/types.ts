type TauriEvent<T> = {
    payload: T
};

type BungieProfile = {
    membershipType: number;
    membershipId: string;
    bungieGlobalDisplayName: string;
    bungieGlobalDisplayNameCode: number;
};

type Profiles = {
    savedProfiles: Profile[],
    selectedProfile: Profile | null,
}

type Profile = {
    accountPlatform: number;
    accountId: string;
};

type ProfileInfo = {
    displayName: string;
    displayTag: number;
};

type Preferences = {
    enableOverlay: boolean;
    displayDailyClears: boolean;
    displayClearNotifications: boolean;
    displayMilliseconds: boolean;
};

type PlayerDataStatus = {
    lastUpdate: PlayerData | null,
    error: string | null,
}

type PlayerData = {
    currentActivity: CurrentActivity;
    activityHistory: CompletedActivity[];
    profileInfo: ProfileInfo;
};

type CurrentActivity = {
    startDate: string;
    activityInfo: ActivityInfo | null;
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
    activityDurationSeconds: number;
    activityHash: number;
}

export type { TauriEvent, BungieProfile, Profiles, Profile, ProfileInfo, Preferences, PlayerDataStatus, PlayerData, CurrentActivity, ActivityInfo, CompletedActivity };
