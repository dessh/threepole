import consts from "../consts";
import "../global.css"
import "./setup.css"
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";

const mainHeader = document.querySelector<HTMLElement>("#main-header")!;
const subHeader = document.querySelector<HTMLElement>("#sub-header")!;
const exitButton = document.querySelector<HTMLButtonElement>("#exit-button")!;
const inputElem = document.querySelector<HTMLInputElement>("#input-profile")!;
const inputPlaceholder = document.querySelector<HTMLSpanElement>("#input-placeholder")!;
const inputSubmit = document.querySelector<HTMLButtonElement>("#input-submit")!;
const loader = document.querySelector<HTMLDivElement>("#loader")!;
const results = document.querySelector<HTMLDivElement>("#results")!;

let selected: any;

function init() {
    invoke("get_config").then((c: any) => {
        if (c) {
            mainHeader.innerText = "Welcome back";
            subHeader.innerText = "Want to switch accounts?";
            inputElem.value = `${c.display_name}#${c.display_tag}`;
            updatePlaceholder();
        }
    });

    window.addEventListener("load", () => {
        appWindow.show();
        appWindow.setFocus();
    });

    document.addEventListener("contextmenu", (e) => e.preventDefault());

    exitButton.addEventListener("click", () => appWindow.close());

    inputElem.addEventListener("input", updatePlaceholder);
    inputElem.addEventListener("paste", updatePlaceholder);

    inputSubmit.addEventListener("click", () => {
        clearResults(true);

        let args = inputElem.value.split("#");

        invoke("search_profile", {
            "displayName": args[0],
            "displayNameCode": parseInt(args[1]),
        }).then(displaySearchResults).catch(displayError);
    });

    inputElem.addEventListener("keydown", e => {
        if (e.code == "Enter") {
            inputSubmit.click();
        }
    });

    updatePlaceholder();
}

function updatePlaceholder() {
    let split = inputElem.value.split("#");

    if (!inputElem.value.trim()) {
        inputElem.value = "";
        inputPlaceholder.innerText = "Profile#0000";
    } else if (split.length == 1) {
        inputPlaceholder.innerHTML = `<span class="invisible">${inputElem.value.replaceAll(" ", "&nbsp;")}</span>` + "#0000";
    } else {
        inputElem.value = split[0] + "#" + split[1].replaceAll(/\D/g, "").substring(0, 4);
        inputPlaceholder.innerText = "";
    }

    inputSubmit.disabled = !inputElem.value.split("#")[1];
}

function clearResults(loading: boolean) {
    selected = null;
    results.replaceChildren();

    loading ? loader.classList.remove("invisible") : loader.classList.add("invisible");
    inputSubmit.disabled = loading;
}

function displaySearchResults(array: any) {
    clearResults(false);

    let p = document.createElement("p");
    p.innerText = `${array.length} result${array.length != 1 ? "s" : ""}`;
    results.appendChild(p);


    if (array.length == 0) {
        return;
    }

    let button = document.createElement("button");
    button.classList.add("confirm");
    button.innerText = "Confirm";
    button.disabled = true;

    button.addEventListener("click", () => {
        invoke("set_config", {
            config: {
                account_platform: selected.membershipType,
                account_id: selected.membershipId,
                display_name: selected.bungieGlobalDisplayName,
                display_tag: selected.bungieGlobalDisplayNameCode,
            }
        }).then(() => appWindow.close()).catch(e => {
            appWindow.show();
            displayError(e);
        });

        appWindow.hide();
    });

    array.forEach((e: any) => {
        let elem = createSearchResult(e, button);

        results.appendChild(elem);
    });

    let wrapper = document.createElement("div");
    wrapper.classList.add("confirm-wrapper");
    wrapper.appendChild(button);

    results.appendChild(wrapper);
}

function createSearchResult(entry: any, button: HTMLButtonElement): HTMLElement {
    let elem = document.createElement("div");
    elem.classList.add("result");

    elem.addEventListener("click", () => selectResult(elem, entry, button));

    let img = document.createElement("div");
    img.classList.add("platform-icon");
    img.style.backgroundImage = `url("${consts.resourceBasePath + entry.iconPath}")`;

    let span = document.createElement("span");
    span.innerText = `${entry.bungieGlobalDisplayName}#${entry.bungieGlobalDisplayNameCode}`;

    elem.appendChild(img);
    elem.appendChild(span);

    return elem;
}

function selectResult(e: HTMLDivElement, entry: any, button: HTMLButtonElement) {
    e.classList.add("selected");

    for (let element of results.children) {
        if (element != e) {
            element.classList.remove("selected");
        }
    }

    selected = entry;

    button.disabled = false;
}

function displayError(error: string) {
    clearResults(false);
    loader.classList.add("invisible");

    let p = document.createElement("p");
    p.classList.add("error");
    p.innerText = error;

    results.appendChild(p);
}

init();
