<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow } from "@tauri-apps/api/window";
    import Window from "../svelte/Window.svelte";
    import StyledCheckbox from "./StyledCheckbox.svelte";

    type Preferences = {
        displayDailyClears: boolean;
        displayClearNotifications: boolean;
    };

    let preferences: Preferences;
    let error: string;

    function init() {
        invoke("get_preferences").then((p: Preferences) => (preferences = p));
    }

    function confirm() {
        invoke("set_preferences", {
            preferences,
        })
            .then(() => appWindow.close())
            .catch((e) => {
                error = e;
                appWindow.show();
            });

        appWindow.hide();
    }

    init();
</script>

<Window>
    <h1>Preferences</h1>
    {#if preferences}
        <div class="preferences">
            {#if error}
                <p class="error">{error}</p>
            {/if}
            <div class="preference">
                <StyledCheckbox bind:checked={preferences.displayDailyClears}
                    >Display daily clears</StyledCheckbox
                >
            </div>
            <div class="preference">
                <StyledCheckbox
                    bind:checked={preferences.displayClearNotifications}
                    >Display raid clear notifications</StyledCheckbox
                >
            </div>
            <div class="actions">
                <button class="confirm" on:click={confirm}>Confirm</button>
            </div>
        </div>
    {/if}
</Window>

<style>
    h1,
    .preferences {
        margin: 24px 48px;
    }

    .preference {
        margin-top: 16px;
        margin-left: 12px;
    }

    .error {
        color: #b53e3e;
    }

    .actions {
        margin-top: 24px;
        float: right;
    }

    button {
        color: #eee;
        padding: 8px 10px;
        font-family: "Inter Tight";
        font-size: 14px;
        transition: border 0.1s, box-shadow 0.2s, color 0.1s;
        height: 36px;
        cursor: pointer;
        margin-left: 16px;
        box-shadow: 0 -1px 0 var(--primary-highlight) inset;
    }

    button:hover {
        color: #fff;

        box-shadow: 0 -4px 0 var(--primary-highlight) inset;
    }
</style>
