import { appWindow } from "@tauri-apps/api/window";
import { shown } from "./overlay";

type Popup = {
    title: string,
    subtext: string,
};

const popupPanel = document.querySelector<HTMLElement>("#popup-panel")!;

let queuedPopups: Popup[] = [];

appWindow.listen("show", showQueuedPopups);

function showQueuedPopups() {
    for (let queuedPopup of queuedPopups) {
        let popup = document.createElement("div");
        popup.classList.add("popup");

        let glyph = document.createElement("img");
        glyph.src = "/glyph.png";

        let contents = document.createElement("div");

        let title = document.createElement("p");
        title.classList.add("title");
        title.innerText = queuedPopup.title;

        let subtext = document.createElement("p");
        subtext.innerHTML = queuedPopup.subtext;

        contents.appendChild(title);
        contents.appendChild(subtext);

        popup.appendChild(glyph);
        popup.appendChild(contents);

        popupPanel.appendChild(popup);

        setTimeout(() => {
            popup.classList.add("fade-out");
        }, 7600);

        setTimeout(() => {
            popupPanel.removeChild(popup);
        }, 8000);
    }

    queuedPopups = [];
}

function createPopup(popup: Popup) {
    queuedPopups.push(popup);

    if (shown) {
        showQueuedPopups();
    }
}

export default createPopup;
