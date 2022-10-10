<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";

    window.addEventListener("load", () => {
        appWindow.show();
        appWindow.setFocus();
    });

    document.addEventListener("contextmenu", (e) => e.preventDefault());
</script>

<main>
    <div class="background" />
    <div class="titlebar" data-tauri-drag-region>
        <button id="exit-button" on:click={() => appWindow.close()}
            ><svg xmlns="http://www.w3.org/2000/svg">
                <path
                    d="M6.062 15 5 13.938 8.938 10 5 6.062 6.062 5 10 8.938 13.938 5 15 6.062 11.062 10 15 13.938 13.938 15 10 11.062Z"
                />
            </svg></button
        >
    </div>
    <div class="content">
        <slot />
    </div>
</main>

<style>
    main {
        width: 100vw;
        height: 100vh;
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        border: 1px solid rgba(255, 255, 255, 0.1);
        color: #fff;
    }

    .background {
        position: absolute;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: linear-gradient(
            200deg,
            var(--primary-background),
            var(--secondary-background)
        );
        z-index: -10;
    }

    .titlebar {
        height: 28px;
    }

    .titlebar button {
        float: right;
        width: 32px;
        height: 28px;
        transition: background-color 0.1s;
        cursor: default;
    }

    .titlebar button svg {
        padding: 4px 6px;
        fill: #aaa;
        transition: fill 0.1s;
    }

    .titlebar button:hover {
        background-color: var(--primary-highlight);
    }

    .titlebar button:hover svg {
        fill: #fff;
    }

    .content {
        flex: 1;
        overflow-y: auto;
    }
</style>
