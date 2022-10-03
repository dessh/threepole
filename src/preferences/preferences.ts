import "../global.css";
import PreferencesWindow from "./PreferencesWindow.svelte";

const window = new PreferencesWindow({
    target: document.querySelector("body")
});

export default window;
