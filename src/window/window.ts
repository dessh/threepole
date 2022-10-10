import "../global.css";
import ProfilesWindow from "./profiles/ProfilesWindow.svelte";
import PreferencesWindow from "./preferences/PreferencesWindow.svelte";
import DetailsWindow from "./details/DetailsWindow.svelte";
import { appWindow } from "@tauri-apps/api/window";

const app = getWindowType();

function getWindowType() {
    let windowType = window.location.hash.split("#")[1];
    switch (windowType) {
        case "preferences":
            return new PreferencesWindow({
                target: document.querySelector("body")
            });
        case "profiles":
            return new ProfilesWindow({
                target: document.querySelector("body")
            });
        case "details":
            return new DetailsWindow({
                target: document.querySelector("body")
            });
        default:
            appWindow.close();
            break;
    }
}

export default app;
