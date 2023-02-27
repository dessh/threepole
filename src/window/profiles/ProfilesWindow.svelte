<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";
    import LineButton from "../widgets/LineButton.svelte";
    import Loader from "../widgets/Loader.svelte";
    import ProfileWidget from "./ProfileWidget.svelte";
    import ProfileAddWidget from "./ProfileAddWidget.svelte";
    import type { BungieProfile, ProfileInfo, Profile } from "../../core/types";
    import { rrPlatforms } from "./platforms/platforms";
    import * as ipc from "../../core/ipc";

    type State = {
        addPage: boolean;
        hasSearched: boolean;
        error: string;
        searchResults: BungieProfile[];
        searchSelectedProfile: BungieProfile;
    };

    let wasNoSavedProfiles = true;

    let selectedProfile: BungieProfile;
    let savedProfiles: BungieProfile[];

    let input = "";
    let placeholder = { hidden: "", shown: "Profile#0000" };

    let state: State = defaultState(false);

    let searchButton: HTMLButtonElement;

    $: updatePlaceholder(input);

    function defaultState(addPage: boolean): State {
        return {
            addPage,
            hasSearched: false,
            error: null,
            searchResults: null,
            searchSelectedProfile: null,
        };
    }

    async function init() {
        let p = await ipc.getProfiles();

        let profiles: BungieProfile[] = [];

        for (let profile of p.savedProfiles) {
            let profileInfo: ProfileInfo;

            try {
                profileInfo = await ipc.getProfileInfo(profile);
            } catch (e) {
                continue;
            }

            let bungieProfile = {
                membershipType: profile.accountPlatform,
                membershipId: profile.accountId,
                bungieGlobalDisplayName: profileInfo.displayName,
                bungieGlobalDisplayNameCode: profileInfo.displayTag,
                crossSaveOverride: null,
            };

            profiles.push(bungieProfile);

            if (
                p.selectedProfile.accountId == profile.accountId &&
                p.selectedProfile.accountPlatform == profile.accountPlatform
            ) {
                selectedProfile = bungieProfile;
            }
        }

        wasNoSavedProfiles = profiles.length == 0;

        savedProfiles = profiles;
    }

    function areProfilesEqual(p1: BungieProfile, p2: BungieProfile): boolean {
        return (
            p1?.membershipType == p2?.membershipType &&
            p1?.membershipId == p2?.membershipId
        );
    }

    function addSavedProfile(profile: BungieProfile) {
        selectedProfile = profile;

        if (savedProfiles.some((p) => areProfilesEqual(p, profile))) {
            return;
        }

        savedProfiles.push(profile);
    }

    function deleteSavedProfile(profile: BungieProfile) {
        savedProfiles = savedProfiles.filter(
            (p) => !areProfilesEqual(p, profile)
        );

        if (areProfilesEqual(profile, selectedProfile)) {
            selectedProfile = null;
        }
    }

    function inputKeyDown(e: KeyboardEvent) {
        if (e.code == "Enter") {
            searchButton.click();
        }
    }

    async function search() {
        state = defaultState(true);
        state.hasSearched = true;

        try {
            let segments = input.split("#");
            if (segments.length > 1) {
                let tag = Number(segments.pop());

                if (!isNaN(tag) && tag >= 1 && tag <= 9999) {
                    let profiles = await ipc.searchProfile(
                        segments.join("#"),
                        tag
                    );

                    state.searchResults = profiles.filter(
                        (p) =>
                            p.crossSaveOverride == 0 ||
                            p.membershipType == p.crossSaveOverride
                    );

                    return;
                }
            }

            let path = input.split("/").filter((e) => e);
            let platform = rrPlatforms[path[path.length - 2]];

            if (path.length > 1 && platform) {
                let profile: Profile = {
                    accountPlatform: platform,
                    accountId: path[path.length - 1],
                };

                let profileInfo = await ipc.getProfileInfo(profile);

                state.searchResults = [
                    {
                        membershipType: profile.accountPlatform,
                        membershipId: profile.accountId,
                        bungieGlobalDisplayName: profileInfo.displayName,
                        bungieGlobalDisplayNameCode: profileInfo.displayTag,
                        crossSaveOverride: null,
                    },
                ];

                return;
            }

            throw "Enter a valid Bungie ID or Raid/Dungeon Report link.";
        } catch (e) {
            state.error = e.message ?? e;
        }
    }

    function updatePlaceholder(newValue) {
        let split = newValue.split("#");

        if (!newValue.trim()) {
            input = "";
            placeholder = { hidden: "", shown: "Profile#0000" };
        } else if (split.length == 1) {
            placeholder = {
                hidden: input,
                shown: "#0000",
            };
        } else {
            placeholder = { hidden: "", shown: "" };
        }
    }

    function convertProfile(profile: BungieProfile): Profile {
        return {
            accountPlatform: profile.membershipType,
            accountId: profile.membershipId,
        };
    }

    function confirm() {
        let newSavedProfiles = savedProfiles.map((p) => convertProfile(p));
        let newSelectedProfile = convertProfile(selectedProfile);

        ipc.setProfiles({
            savedProfiles: newSavedProfiles,
            selectedProfile: newSelectedProfile,
        }).then(() => appWindow.close());
    }

    init();
</script>

<main>
    {#if savedProfiles}
        {#if !state.addPage}
            <div class="header">
                <h1>{wasNoSavedProfiles ? "Welcome" : "Welcome back"}</h1>
                <p>
                    {wasNoSavedProfiles
                        ? savedProfiles.length == 0
                            ? "Get started by adding your Bungie account."
                            : "Finish setup by hitting confirm."
                        : "Want to switch accounts?"}
                </p>
            </div>
            <div class="padded results">
                {#each savedProfiles as profile}
                    <ProfileWidget
                        {profile}
                        selected={areProfilesEqual(profile, selectedProfile)}
                        clickCallback={() => (selectedProfile = profile)}
                        deleteCallback={() => deleteSavedProfile(profile)}
                    />
                {/each}
                <ProfileAddWidget
                    clickCallback={() => (state = defaultState(true))}
                />
                {#if state.error}
                    <p class="error">{state.error}</p>
                {/if}
                <div class="button-wrapper right">
                    <LineButton
                        clickCallback={confirm}
                        disabled={!selectedProfile}>Confirm</LineButton
                    >
                </div>
            </div>
        {:else}
            <div class="header elem-above">
                <div class="back-wrapper">
                    <LineButton
                        clickCallback={() => (state = defaultState(false))}
                        >Back</LineButton
                    >
                </div>
                <h1>Search</h1>
                <p>Enter your Bungie ID or Raid/Dungeon Report link below.</p>
            </div>
            <div class="search">
                <p class="placeholder">
                    <span class="invisible">{placeholder.hidden}</span><span
                        >{placeholder.shown}</span
                    >
                </p>
                <input
                    bind:value={input}
                    on:keydown={inputKeyDown}
                    spellcheck="false"
                /><button
                    bind:this={searchButton}
                    on:click={search}
                    disabled={(state.hasSearched &&
                        !state.searchResults &&
                        !state.error) ||
                        !input}
                    ><svg xmlns="http://www.w3.org/2000/svg">
                        <path
                            d="m19.6 21-6.3-6.3q-.75.6-1.725.95Q10.6 16 9.5 16q-2.725 0-4.612-1.887Q3 12.225 3 9.5q0-2.725 1.888-4.613Q6.775 3 9.5 3t4.613 1.887Q16 6.775 16 9.5q0 1.1-.35 2.075-.35.975-.95 1.725l6.3 6.3ZM9.5 14q1.875 0 3.188-1.312Q14 11.375 14 9.5q0-1.875-1.312-3.188Q11.375 5 9.5 5 7.625 5 6.312 6.312 5 7.625 5 9.5q0 1.875 1.312 3.188Q7.625 14 9.5 14Z"
                        />
                    </svg></button
                >
            </div>
            <div class="padded">
                {#if state.searchResults}
                    <p class="result-count">
                        {state.searchResults.length} result{state.searchResults
                            .length != 1
                            ? "s"
                            : ""}
                    </p>
                    {#each state.searchResults as profile}
                        <ProfileWidget
                            {profile}
                            selected={areProfilesEqual(
                                profile,
                                state.searchSelectedProfile
                            )}
                            clickCallback={() =>
                                (state.searchSelectedProfile = profile)}
                        />
                    {/each}
                    {#if state.searchResults.length > 0}
                        <div class="button-wrapper right">
                            <LineButton
                                clickCallback={() => {
                                    addSavedProfile(
                                        state.searchSelectedProfile
                                    );
                                    state = defaultState(false);
                                }}
                                disabled={!state.searchSelectedProfile}
                                >Add</LineButton
                            >
                        </div>
                    {/if}
                {:else if state.error}
                    <p class="error">{state.error}</p>
                {:else if state.hasSearched}
                    <div class="loader">
                        <Loader />
                    </div>
                {/if}
            </div>
        {/if}
    {:else}
        <div class="loader centered">
            <Loader />
        </div>
    {/if}
</main>

<style>
    .header {
        margin: 24px 48px;
    }

    .header.elem-above {
        margin-top: 0;
    }

    .header h1 {
        margin-bottom: 8px;
    }

    .search {
        position: relative;
        text-align: center;
    }

    .search input {
        font-family: "Inter Tight";
        width: 60%;
        height: 40px;
        font-size: 20px;
        color: #fff;
        border-bottom: 1px solid;
        border-image-slice: 1;
        border-image-source: linear-gradient(
            45deg,
            rgba(255, 255, 255, 0.1),
            rgba(255, 255, 255, 0.1)
        );
        transition: border-image-source 0.1s;
        vertical-align: middle;
        padding-right: 40px;
        box-sizing: border-box;
    }

    .search input:hover {
        border-image-source: linear-gradient(
            45deg,
            var(--primary-highlight),
            var(--primary-highlight)
        );
    }

    .search input:focus {
        border-image-source: linear-gradient(
            45deg,
            var(--secondary-highlight-light),
            var(--primary-highlight-light)
        );
    }

    .placeholder {
        position: absolute;
        left: calc(40% / 2);
        width: calc(60% - 40px);
        height: 40px;
        color: #aaa;
        font-size: 20px;
        line-height: 40px;
        text-align: left;
        pointer-events: none;
        white-space: pre;
    }

    .invisible {
        opacity: 0;
    }

    .search button {
        position: absolute;
        left: 70%;
        width: 40px;
        height: 40px;
        vertical-align: middle;
    }

    .search button svg {
        padding: 8px;
        transition: fill 0.1s;
        fill: rgba(255, 255, 255, 0.15);
    }

    .search button:not(:disabled) svg {
        fill: var(--primary-highlight-light);
    }

    .loader {
        position: absolute;
        left: 50%;
        transform: translate(-50%, 0);
        z-index: -1;
        pointer-events: none;
        margin-top: 24px;
    }

    .loader.centered {
        margin: 0;
        top: 50%;
        transform: translate(-50%, -50%);
    }

    .padded {
        width: 60%;
        margin-left: auto;
        margin-right: auto;
    }

    .result-count {
        color: #aaa;
        font-size: 12px;
        margin-top: 6px;
        margin-bottom: 20px;
    }

    .error {
        color: var(--error);
        font-size: 14px;
        margin-top: 12px;
    }

    .button-wrapper {
        margin: 16px 0;
    }

    .back-wrapper {
        margin-bottom: 16px;
    }

    .right {
        float: right;
    }
</style>
