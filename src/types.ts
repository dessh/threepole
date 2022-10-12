type TauriEvent<T> = {
    payload: T
};

type BungieProfile = {
    membershipType: number;
    membershipId: string;
    bungieGlobalDisplayName: string;
    bungieGlobalDisplayNameCode: number;
    crossSaveOverride: number;
};

type Profiles = {
    savedProfiles: Profile[],
    selectedProfile: Profile,
}

type Profile = {
    accountPlatform: number;
    accountId: string;
};

type ProfileInfo = {
    privacy: number;
    displayName: string;
    displayTag: number;
    characterIds: string[];
};

type Preferences = {
    enableOverlay: boolean;
    displayDailyClears: boolean;
    displayClearNotifications: boolean;
    displayMilliseconds: boolean;
};

type PlayerDataStatus = {
    lastUpdate: PlayerData,
    error: string,
}

type PlayerData = {
    currentActivity: CurrentActivity;
    activityHistory: CompletedActivity[];
    profileInfo: ProfileInfo;
};

type CurrentActivity = {
    startDate: string;
    activityHash: number;
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
    activityDurationSeconds: number;
    activityHash: number;
}

export type { TauriEvent, BungieProfile, Profiles, Profile, ProfileInfo, Preferences, PlayerDataStatus, PlayerData, CurrentActivity, ActivityInfo, CompletedActivity };
