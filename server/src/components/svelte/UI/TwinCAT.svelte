<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount, onDestroy } from "svelte";

    import Button from "./utils/Button.svelte";
    import Select from "./utils/Select.svelte";
    import CheckBox from "./utils/CheckBox.svelte";
    import NumberInput from "./utils/NumberInput.svelte";
    import IpInput from "./utils/IpInput.svelte";

    import { SyncMode } from "./ecat.js";

    let client = "";
    let sync0 = 2;
    let task = 2;
    let base = 1;
    let mode = Object.keys(SyncMode)[0];
    let keep = false;

    let options = () =>
        JSON.stringify({
            client,
            sync0,
            task,
            base,
            mode,
            keep,
        });

    let handleRunClick = async () => {
        const twincatOptions = options();
        try {
            await invoke("run_twincat_server", { twincatOptions });
        } catch (err) {
            alert(err);
        }
    };

    let handleCopyAUTDXmlClick = async () => {
        try {
            await invoke("copy_autd_xml", {});
        } catch (err) {
            alert(err);
        }
    };

    onMount(async () => {
        const options = await invoke("load_settings", {});
        if (options.twincat) {
            client = options.twincat.client;
            sync0 = options.twincat.sync0;
            task = options.twincat.task;
            base = options.twincat.base;
            mode = SyncMode[options.twincat.mode];
            keep = options.twincat.keep;
        }
    });

    onDestroy(async () => {
        const twincatOptions = options();
        await invoke("save_twincat_settings", { twincatOptions });
    });
</script>

<div class="ui">
    <label for="client">Client IP address:</label>
    <IpInput id="client" bind:value={client} />

    <label for="sync0">Sync0 cycle time:</label>
    <NumberInput id="sync0" bind:value={sync0} min="1" step="1" />

    <label for="task">Send task cycle time:</label>
    <NumberInput id="task" bind:value={task} min="1" step="1" />

    <label for="base">CPU base time:</label>
    <NumberInput id="base" bind:value={base} min="1" step="1" />

    <label for="mode">Sync mode:</label>
    <Select id="mode" bind:value={mode} values={Object.keys(SyncMode)} />

    <label for="keep">Keep XAE Shell open:</label>
    <CheckBox id="keep" bind:checked={keep} />

    <Button label="Run" click={handleRunClick} />
    <Button label="Copy AUTD.xml" click={handleCopyAUTDXmlClick} />
</div>

<style>
    .ui {
        display: grid;
        grid-template-columns: auto 120px;
        grid-gap: 10px 0px;
        align-items: right;
    }
    label {
        text-align: right;
        padding-right: 10px;
    }
</style>
