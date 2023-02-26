import { invoke } from "@tauri-apps/api/tauri";
import type { ActivityInfo, BungieProfile, PlayerDataStatus, Preferences, Profile, ProfileInfo, Profiles } from "./types";

export function openPreferences(): Promise<void> {
    return invoke("open_preferences");
}

export function openProfiles(): Promise<void> {
    return invoke("open_profiles");
}

export function getPreferences(): Promise<Preferences> {
    return invoke("get_preferences");
}

export function setPreferences(preferences: Preferences): Promise<void> {
    return invoke("set_preferences", { preferences });
}

export function getProfiles(): Promise<Profiles> {
    return invoke("get_profiles");
}

export function setProfiles(profiles: Profiles): Promise<void> {
    return invoke("set_profiles", { profiles });
}

export function getProfileInfo(profile: Profile): Promise<ProfileInfo> {
    return invoke("get_profile_info", { profile });
}

export function getActivityInfo(activityHash: number): Promise<ActivityInfo> {
    return invoke("get_activity_info", { activityHash });
}

export function searchProfile(displayName: string, displayNameCode: number): Promise<BungieProfile[]> {
    return invoke("search_profile", { displayName, displayNameCode });
}

export function getPlayerdata(): Promise<PlayerDataStatus | null> {
    return invoke("get_playerdata");
}
