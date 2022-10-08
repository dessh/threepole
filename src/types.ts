type TauriEvent<T> = {
    payload: T
};

type RustResult<T> = {
    Ok: T | null,
    Err: string | null,
}

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
    displayDailyClears: boolean;
    displayClearNotifications: boolean;
    displayMilliseconds: boolean;
};

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

export type { TauriEvent, RustResult, BungieProfile, Profiles, Profile, ProfileInfo, Preferences, PlayerData, CurrentActivity, ActivityInfo, CompletedActivity };
