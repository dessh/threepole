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

type DisplayProfile = {
    displayName: string;
    displayTag: number;
};

export type { BungieProfile, Profiles, Profile, DisplayProfile };
