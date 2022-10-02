import "../global.css";
import ProfileWindow from "./ProfileWindow.svelte";

const app = new ProfileWindow({
    target: document.querySelector("body")
});

export default app;
