<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";
    import LineButton from "../widgets/LineButton.svelte";
    import StyledCheckbox from "./StyledCheckbox.svelte";
    import type { Preferences } from "../../core/types";
    import * as ipc from "../../core/ipc";

    let preferences: Preferences;
    let error: string;

    function init() {
        ipc.getPreferences().then((p: Preferences) => (preferences = p));
    }

    function confirm() {
        ipc.setPreferences(preferences)
            .then(() => appWindow.close())
            .catch((e) => {
                error = e.message ?? e;
                appWindow.show();
            });

        appWindow.hide();
    }

    init();
</script>

<main>
    <h1>Preferences</h1>
    {#if preferences}
        <div class="preferences">
            {#if error}
                <p class="error">{error}</p>
            {/if}
            <div class="preference">
                <StyledCheckbox bind:checked={preferences.enableOverlay}
                    >Enable overlay</StyledCheckbox
                >
            </div>
            <div class="preference-group">
                <div class="preference">
                    <StyledCheckbox
                        bind:checked={preferences.displayDailyClears}
                        disabled={!preferences.enableOverlay}
                        >Display daily clears</StyledCheckbox
                    >
                </div>
                <div class="preference">
                    <StyledCheckbox
                        bind:checked={preferences.displayClearNotifications}
                        disabled={!preferences.enableOverlay}
                        >Display activity clear notifications</StyledCheckbox
                    >
                </div>
                <div class="preference">
                    <StyledCheckbox
                        bind:checked={preferences.displayMilliseconds}
                        disabled={!preferences.enableOverlay}
                        >Display timer milliseconds</StyledCheckbox
                    >
                </div>
            </div>
            <div class="actions">
                <LineButton clickCallback={confirm}>Confirm</LineButton>
            </div>
        </div>
    {/if}
</main>

<style>
    h1 {
        margin: 24px 48px;
    }

    .preferences {
        margin: 16px 48px;
    }

    .preference-group {
        padding: 8px 12px;
        border: 1px solid rgba(255, 255, 255, 0.1);
    }

    .preference {
        margin: 12px 8px;
    }

    .error {
        color: var(--error);
    }

    .actions {
        margin-top: 24px;
        float: right;
    }
</style>
