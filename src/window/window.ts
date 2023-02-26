import "../core/global.css";
import "./window.css";
import ProfilesWindow from "./profiles/ProfilesWindow.svelte";
import PreferencesWindow from "./preferences/PreferencesWindow.svelte";
import DetailsWindow from "./details/DetailsWindow.svelte";
import { appWindow } from "@tauri-apps/api/window";

window.addEventListener("DOMContentLoaded", () => {
    appWindow.show();
    appWindow.setFocus();
});

document.addEventListener("contextmenu", (e) => e.preventDefault());

document.querySelector("#exit-button").addEventListener("click", () => appWindow.close());
document.querySelector("#minimize-button").addEventListener("click", () => appWindow.minimize());

const target = document.querySelector("#content");

const app = getWindowType();

function getWindowType() {
    let windowType = window.location.hash.split("#")[1];
    switch (windowType) {
        case "preferences":
            return new PreferencesWindow({
                target
            });
        case "profiles":
            return new ProfilesWindow({
                target
            });
        case "details":
            return new DetailsWindow({
                target
            });
        default:
            appWindow.close();
            break;
    }
}

export default app;
