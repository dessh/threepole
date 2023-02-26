export type TauriEvent<T> = {
    payload: T
};

export type BungieProfile = {
    membershipType: number;
    membershipId: string;
    bungieGlobalDisplayName: string;
    bungieGlobalDisplayNameCode: number;
    crossSaveOverride: number;
};

export type Profiles = {
    savedProfiles: Profile[],
    selectedProfile: Profile,
}

export type Profile = {
    accountPlatform: number;
    accountId: string;
};

export type ProfileInfo = {
    privacy: number;
    displayName: string;
    displayTag: number;
    characterIds: string[];
};

export type Preferences = {
    enableOverlay: boolean;
    displayDailyClears: boolean;
    displayClearNotifications: boolean;
    displayMilliseconds: boolean;
};

export type PlayerDataStatus = {
    lastUpdate: PlayerData,
    error: string,
}

export type PlayerData = {
    currentActivity: CurrentActivity;
    activityHistory: CompletedActivity[];
    profileInfo: ProfileInfo;
};

export type CurrentActivity = {
    startDate: string;
    activityHash: number;
    activityInfo: ActivityInfo;
};

export type ActivityInfo = {
    name: string;
    activityModes: number[];
    backgroundImage: string;
};

export type CompletedActivity = {
    period: string;
    instanceId: string;
    completed: boolean;
    activityDuration: string;
    activityDurationSeconds: number;
    activityHash: number;
    modes: number[];
};
