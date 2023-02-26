<script lang="ts">
    import type { BungieProfile } from "../../core/types";
    import { iconPaths } from "./platforms/platforms";

    export let profile: BungieProfile;
    export let selected: boolean;
    export let clickCallback: (e: MouseEvent) => void;
    export let deleteCallback: (e: MouseEvent) => void = null;

    function click(e: MouseEvent) {
        let target = e.target;
        if (
            target instanceof Element &&
            target.classList.contains("delete-button-component")
        ) {
            e.preventDefault();
            return;
        }

        clickCallback(e);
    }
</script>

<div class="profile {selected ? 'selected' : ''}" on:click={click}>
    <div
        class="platform-icon"
        style="background-image: url('{iconPaths[profile.membershipType]}')"
    />
    <span
        >{profile.bungieGlobalDisplayName}#{profile.bungieGlobalDisplayNameCode}</span
    >
    {#if deleteCallback}
        <button class="delete-button-component" on:click={deleteCallback}
            ><svg
                xmlns="http://www.w3.org/2000/svg"
                class="delete-button-component"
                width="20"
                height="20"
            >
                <path
                    class="delete-button-component"
                    d="M6.062 15 5 13.938 8.938 10 5 6.062 6.062 5 10 8.938 13.938 5 15 6.062 11.062 10 15 13.938 13.938 15 10 11.062Z"
                />
            </svg></button
        >
    {/if}
</div>

<style>
    .profile {
        display: flex;
        padding: 16px;
        padding-right: 12px;
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
        align-items: center;
    }

    .profile:hover {
        border-image-source: linear-gradient(
            45deg,
            var(--primary-highlight),
            var(--primary-highlight)
        );
    }

    .profile.selected {
        border-image-source: linear-gradient(
            45deg,
            var(--secondary-highlight-light),
            var(--primary-highlight-light)
        );
    }

    .profile .platform-icon {
        display: inline-block;
        width: 24px;
        height: 24px;
        background-size: contain;
        background-repeat: no-repeat;
        background-position: center;
    }

    .profile span {
        font-size: 16px;
        flex: 1;
        margin: 0 12px;
    }

    .profile button {
        width: 24px;
        height: 24px;
        float: right;
        padding: 2px;
        transition: background-color 0.1s, fill 0.1s;
        cursor: default;
        fill: #aaa;
    }

    .profile button:hover {
        background-color: rgba(255, 255, 255, 0.05);
        fill: #fff;
    }
</style>
