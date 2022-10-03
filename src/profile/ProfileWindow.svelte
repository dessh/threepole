<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow } from "@tauri-apps/api/window";
    import { prevent_default } from "svelte/internal";
    import Window from "../svelte/Window.svelte";
    import Loader from "./Loader.svelte";

    type BungieProfile = {
        membershipType: number;
        membershipId: string;
        bungieGlobalDisplayName: string;
        bungieGlobalDisplayNameCode: number;
    };

    type ProfileResults = {
        profiles: BungieProfile[];
        wasSearch: boolean;
    };

    let headerText = {
        main: "Welcome",
        sub: "Enter your Bungie ID below.",
    };

    let selectedProfile: BungieProfile;
    let input = "";
    let placeholder = { hidden: "", shown: "Profile#0000" };
    let state: ProfileResults;
    let error: string;
    let savedProfiles: any[];
    let initialLoad = false;

    let searchButton: HTMLButtonElement;

    $: updatePlaceholder(input);

    async function init() {
        let p: any = await invoke("get_profiles");

        savedProfiles = p.savedProfiles;

        let results: ProfileResults = {
            profiles: [],
            wasSearch: false,
        };

        let selectedBungieProfile: BungieProfile;

        for (let profile of p.savedProfiles) {
            let displayProfile: any = await invoke("get_display_profile", {
                profile,
            });

            let bungieProfile = {
                membershipType: profile.accountPlatform,
                membershipId: profile.accountId,
                bungieGlobalDisplayName: displayProfile.displayName,
                bungieGlobalDisplayNameCode: displayProfile.displayTag,
            };

            results.profiles.push(bungieProfile);

            if (
                p.selectedProfile.accountId == profile.accountId &&
                p.selectedProfile.accountPlatform == profile.accountPlatform
            ) {
                selectedBungieProfile = bungieProfile;
            }
        }

        if (selectedBungieProfile) {
            selectedProfile = selectedBungieProfile;

            headerText.main = "Welcome back";
            headerText.sub = "Want to switch accounts?";
        }

        state = results;

        initialLoad = true;
    }

    function getIconPath(membershipType: number): string {
        let name = "default";

        switch (membershipType) {
            case 3:
                name = "steam";
                break;
            case 4:
                name = "blizzard";
                break;
            case 6:
                name = "epic";
                break;
        }

        return `/platforms/${name}.svg`;
    }

    function deleteSavedProfile(profile: BungieProfile) {
        savedProfiles = savedProfiles.filter(
            (p) =>
                p.accountPlatform != profile.membershipType ||
                p.accountId != profile.membershipId
        );

        state.profiles = state.profiles.filter((p) => p != profile);
    }

    function inputKeyDown(e: KeyboardEvent) {
        if (e.code == "Enter") {
            searchButton.click();
        }
    }

    function isProfileResults(s: any): boolean {
        return s && s.wasSearch !== null;
    }

    function search() {
        state = null;
        error = null;
        selectedProfile = null;

        let args = input.split("#");

        invoke("search_profile", {
            displayName: args[0],
            displayNameCode: parseInt(args[1]),
        })
            .then(
                (p: BungieProfile[]) =>
                    (state = { profiles: p, wasSearch: true })
            )
            .catch((e) => (error = e.message ?? e));
    }

    function updatePlaceholder(newValue) {
        let split = newValue.split("#");

        if (!newValue.trim()) {
            input = "";
            placeholder = { hidden: "", shown: "Profile#0000" };
        } else if (split.length == 1) {
            placeholder = {
                hidden: input.replaceAll(" ", "&nbsp;"),
                shown: "#0000",
            };
        } else {
            input =
                split[0] + "#" + split[1].replaceAll(/\D/g, "").substring(0, 4);
            placeholder = { hidden: "", shown: "" };
        }
    }

    function confirm() {
        savedProfiles.push({
            accountPlatform: selectedProfile.membershipType,
            accountId: selectedProfile.membershipId,
        });

        invoke("set_profiles", {
            profiles: {
                savedProfiles,
                selectedProfile: {
                    accountPlatform: selectedProfile.membershipType,
                    accountId: selectedProfile.membershipId,
                },
            },
        })
            .then(() => appWindow.close())
            .catch((e) => {
                error = e.message ?? e;
                appWindow.show();
            });

        appWindow.hide();
    }

    init();
</script>

<Window>
    <div class="header {initialLoad ? '' : 'invisible'}">
        <h1>{headerText.main}</h1>
        <p>{headerText.sub}</p>
    </div>
    <div class="search {initialLoad ? '' : 'invisible'}">
        <p class="placeholder">
            <span class="invisible">{placeholder.hidden}</span><span
                >{placeholder.shown}</span
            >
        </p>
        <input
            bind:value={input}
            on:keydown={inputKeyDown}
            spellcheck="false"
            maxlength="31"
        /><button
            bind:this={searchButton}
            on:click={search}
            disabled={!state || !input.split("#")[1]}
            ><svg xmlns="http://www.w3.org/2000/svg">
                <path
                    d="m19.6 21-6.3-6.3q-.75.6-1.725.95Q10.6 16 9.5 16q-2.725 0-4.612-1.887Q3 12.225 3 9.5q0-2.725 1.888-4.613Q6.775 3 9.5 3t4.613 1.887Q16 6.775 16 9.5q0 1.1-.35 2.075-.35.975-.95 1.725l6.3 6.3ZM9.5 14q1.875 0 3.188-1.312Q14 11.375 14 9.5q0-1.875-1.312-3.188Q11.375 5 9.5 5 7.625 5 6.312 6.312 5 7.625 5 9.5q0 1.875 1.312 3.188Q7.625 14 9.5 14Z"
                />
            </svg></button
        >
    </div>
    {#if error}
        <div class="results">
            <p class="error">{error}</p>
        </div>
    {:else if isProfileResults(state)}
        <div class="results">
            {#if state.wasSearch}
                <p>
                    {state.profiles.length} result{state.profiles.length != 1
                        ? "s"
                        : ""}
                </p>
            {/if}
            {#each state.profiles as profile}
                <div
                    class="profile {selectedProfile == profile
                        ? 'selected'
                        : ''}"
                    on:click={(e) => {
                        let target = e.target;
                        if (
                            target instanceof Element &&
                            target.classList.contains("delete-button-component")
                        ) {
                            e.preventDefault();
                            return;
                        }

                        selectedProfile = profile;
                    }}
                >
                    <div
                        class="platform-icon"
                        style="background-image: url('{getIconPath(
                            profile.membershipType
                        )}')"
                    />
                    <span
                        >{profile.bungieGlobalDisplayName}#{profile.bungieGlobalDisplayNameCode}</span
                    >
                    {#if !state.wasSearch}
                        <button
                            class="delete-button-component"
                            on:click={() => deleteSavedProfile(profile)}
                            ><svg
                                class="delete-button-component"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    class="delete-button-component"
                                    d="M6.062 15 5 13.938 8.938 10 5 6.062 6.062 5 10 8.938 13.938 5 15 6.062 11.062 10 15 13.938 13.938 15 10 11.062Z"
                                />
                            </svg></button
                        >
                    {/if}
                </div>
            {/each}
            {#if state.profiles.length > 0}
                <div class="confirm-wrapper">
                    <button on:click={confirm} disabled={!selectedProfile}
                        >Confirm</button
                    >
                </div>
            {/if}
        </div>
    {:else}
        <div class="loader">
            <Loader />
        </div>
    {/if}
</Window>

<style>
    button {
        cursor: pointer;
    }

    button:disabled {
        cursor: not-allowed;
    }

    .header {
        margin: 24px 48px;
    }

    .header h1 {
        margin-bottom: 8px;
    }

    .search {
        text-align: center;
        position: relative;
    }

    .search input {
        font-family: "Inter Tight";
        width: 60vw;
        height: 40px;
        font-size: 20px;
        color: #fff;
        border-bottom: 1px solid;
        border-image-slice: 1;
        border-image-source: linear-gradient(
            90deg,
            rgba(255, 255, 255, 0.1),
            rgba(255, 255, 255, 0.1)
        );
        transition: border-image-source 0.1s;
        vertical-align: middle;
        padding-right: 40px;
        box-sizing: border-box;
    }

    .search input:hover,
    .search input:focus {
        border-image-source: linear-gradient(
            90deg,
            var(--secondary-highlight-light),
            var(--primary-highlight-light)
        );
    }

    .placeholder {
        position: absolute;
        left: calc(40vw / 2);
        width: calc(60vw - 40px);
        height: 40px;
        color: #aaa;
        font-size: 20px;
        line-height: 41px;
        text-align: left;
        pointer-events: none;
        white-space: nowrap;
    }

    .invisible {
        opacity: 0;
    }

    .search button {
        position: absolute;
        left: 70vw;
        width: 40px;
        height: 40px;
        vertical-align: middle;
    }

    .search button svg {
        padding: 8px;
        transition: fill 0.1s;
        fill: #aaa;
    }

    .search button:not(:disabled) svg {
        fill: var(--primary-highlight-light);
    }

    .loader {
        position: absolute;
        left: 50%;
        transform: translate(-50%, 20px);
        z-index: -1;
        pointer-events: none;
    }

    .results {
        text-align: center;
        width: 60vw;
        margin: 0 auto;
        margin-top: 8px;
    }

    .results p {
        color: #aaa;
        font-size: 12px;
        text-align: left;
        margin-bottom: 20px;
    }

    .results p.error {
        color: #b53e3e;
    }

    .results .profile {
        padding: 16px;
        padding-right: 12px;
        text-align: left;
        border: 1px solid;
        border-image-source: linear-gradient(
            90deg,
            rgba(255, 255, 255, 0.1),
            rgba(255, 255, 255, 0.1)
        );
        cursor: pointer;
        border-image-slice: 1;
        transition: border-image-source 0.1s;
        margin-top: 12px;
    }

    .results .profile:hover,
    .results .profile.selected {
        border-image-source: linear-gradient(
            90deg,
            var(--secondary-highlight-light),
            var(--primary-highlight-light)
        );
    }

    .profile .platform-icon {
        display: inline-block;
        width: 24px;
        height: 24px;
        vertical-align: middle;
        margin-right: 8px;
        background-size: contain;
    }

    .profile span {
        font-size: 16px;
        vertical-align: middle;
    }

    .profile button {
        vertical-align: middle;
        width: 24px;
        height: 24px;
        float: right;
        padding: 2px;
        transition: background-color 0.1s;
    }

    .profile button:hover {
        background-color: rgba(0, 0, 0, 0.25);
    }

    .profile button svg {
        fill: #aaa;
        transition: fill 0.1s;
    }

    .profile button:hover svg {
        fill: #fff;
    }

    .results .confirm-wrapper {
        margin: 16px 0;
        text-align: right;
    }

    .confirm-wrapper button {
        color: #eee;
        padding: 8px 10px;
        font-family: "Inter Tight";
        font-size: 14px;
        transition: box-shadow 0.2s, color 0.1s;
        box-shadow: 0 -1px 0 var(--primary-highlight) inset;
        height: 36px;
    }

    .confirm-wrapper button:hover:not(:disabled) {
        color: #fff;
        box-shadow: 0 -4px 0 var(--primary-highlight) inset;
    }

    .confirm-wrapper button:disabled {
        color: #777;
        box-shadow: 0 -1px 0 rgba(255, 255, 255, 0.1) inset;
    }
</style>
