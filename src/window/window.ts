import "../global.css";
import ProfileWindow from "./profile/ProfileWindow.svelte";
import PreferencesWindow from "./preferences/PreferencesWindow.svelte";
import { appWindow } from "@tauri-apps/api/window";

const app = getWindowType();

function getWindowType() {
    let windowType = window.location.hash.split("#")[1];
    switch (windowType) {
        case "preferences":
            return new PreferencesWindow({
                target: document.querySelector("body")
            });
        case "profile":
            return new ProfileWindow({
                target: document.querySelector("body")
            });
        default:
            appWindow.close();
            break;
    }
}

export default app;
