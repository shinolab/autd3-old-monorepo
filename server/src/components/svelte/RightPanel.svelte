<script>
    import { console_output_queue } from "./UI/console_output.js";
    import { listen } from "@tauri-apps/api/event";
    import { onMount } from "svelte";

    $: console_output = $console_output_queue.join("\n");

    onMount(async () => {
        await listen("console-emu", (event) => {
            console_output_queue = [...console_output_queue, event.payload];
        });
    });
</script>

<div>
    <textarea readonly>{console_output}</textarea>
</div>

<style>
    div {
        display: flex;
        align-items: flex-start;
        gap: 10px;
        flex: 1 0 0;
        align-self: stretch;

        width: 100%;
        box-sizing: border-box;
    }

    textarea {
        resize: none;

        display: flex;
        padding: 8px;
        align-items: flex-start;
        flex: 1 0 0;
        align-self: stretch;

        border-radius: 3px;
        border: 1px solid var(--color-border-interactive-muted, #2b659b);
        background: var(--color-background-base-default, #101923);
        color: var(--color-text-base-default, #ffffff);
    }
</style>
