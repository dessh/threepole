import "../global.css";
import ProfileWindow from "./ProfileWindow.svelte";

const window = new ProfileWindow({
    target: document.querySelector("body")
});

export default window;
