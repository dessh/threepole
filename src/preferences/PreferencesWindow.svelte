<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow } from "@tauri-apps/api/window";
    import LineButton from "../svelte/LineButton.svelte";
    import Window from "../svelte/Window.svelte";
    import StyledCheckbox from "./StyledCheckbox.svelte";
    import type { Preferences } from "./types";

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
                error = e.message ?? e;
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
            <p class="subheader">Overlay</p>
            <div class="preference-group">
                <div class="preference">
                    <StyledCheckbox
                        bind:checked={preferences.displayDailyClears}
                        >Display daily clears</StyledCheckbox
                    >
                </div>
                <div class="preference">
                    <StyledCheckbox
                        bind:checked={preferences.displayClearNotifications}
                        >Display raid clear notifications</StyledCheckbox
                    >
                </div>
                <div class="preference">
                    <StyledCheckbox
                        bind:checked={preferences.displayMilliseconds}
                        >Display timer milliseconds</StyledCheckbox
                    >
                </div>
            </div>
            <div class="actions">
                <LineButton clickCallback={confirm}>Confirm</LineButton>
            </div>
        </div>
    {/if}
</Window>

<style>
    h1 {
        margin: 24px 48px;
    }

    .preferences {
        margin: 16px 48px;
    }

    .subheader {
        font-size: 18px;
        margin-bottom: 8px;
        font-weight: 500;
    }

    .preference-group {
        padding: 8px 12px;
        border: 1px solid rgba(255, 255, 255, 0.1);
    }

    .preference {
        margin: 8px;
    }

    .error {
        color: #b53e3e;
    }

    .actions {
        margin-top: 24px;
        float: right;
    }
</style>
