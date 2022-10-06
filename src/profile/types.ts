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

export type { BungieProfile, Profiles, Profile, ProfileInfo };
