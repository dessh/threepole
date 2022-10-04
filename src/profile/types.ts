type BungieProfile = {
    membershipType: number;
    membershipId: string;
    bungieGlobalDisplayName: string;
    bungieGlobalDisplayNameCode: number;
};

type ThreepoleProfiles = {
    savedProfiles: ThreepoleProfile[],
    selectedProfile: ThreepoleProfile | null,
}

type ThreepoleProfile = {
    accountPlatform: number;
    accountId: string;
};

type ThreepoleDisplayProfile = {
    displayName: string;
    displayTag: number;
};

export type { BungieProfile, ThreepoleProfiles, ThreepoleProfile, ThreepoleDisplayProfile };
