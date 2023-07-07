<!--
File: Simulator.svelte
Project: AUTD server
Created Date: 06/07/2023
Author: Shun Suzuki
-----
Last Modified: 06/07/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

-->

<script>
    import { invoke } from "@tauri-apps/api";
    import { onMount, onDestroy } from "svelte";
    import { Command } from "@tauri-apps/api/shell";
    import { writable } from "svelte/store";

    import Button from "./utils/Button.svelte";
    import Select from "./utils/Select.svelte";
    import CheckBox from "./utils/CheckBox.svelte";
    import NumberInput from "./utils/NumberInput.svelte";

    let vsync = false;
    let port = 8080;
    let gpu_name = "";
    let gpu_idx = -1;
    $: {
        const idx = availableGpusNames.indexOf(gpu_name);
        if (idx == 0 || idx == -1) {
            gpu_idx = -1;
        } else {
            gpu_idx = parseInt(availableGpus[idx - 1].split(":")[0].trim());
        }
    }
    let window_width = 800;
    let window_height = 600;

    export const cachedGPUs = writable(null);
    let availableGpus = [];
    $: availableGpusNames = ["Auto"].concat(
        availableGpus.map((gpu) => gpu.split(":")[1].trim())
    );

    let options = () =>
        JSON.stringify({
            vsync,
            port,
            gpu_idx,
            window_width,
            window_height,
        });

    let handleRunClick = async () => {
        const simulatorOptions = options();
        try {
            await invoke("run_simulator_server", { simulatorOptions });
        } catch (err) {
            alert(err);
        }
    };

    onMount(async () => {
        let gpus = null;
        cachedGPUs.subscribe((v) => {
            gpus = v;
        })();
        if (gpus === null) {
            gpus = (await Command.sidecar("simulator", "list").execute())
                .stdout;
            cachedGPUs.set(gpus);
        }
        availableGpus = gpus
            .split("\n")
            .map((s) => s.trim().replace(/ \(type .*\)$/g, ""));

        const options = await invoke("load_settings", {});
        if (options.simulator) {
            vsync = options.simulator.vsync;
            port = options.simulator.port;
            gpu_name = (
                availableGpus.find(
                    (gpu) =>
                        gpu.split(":")[0].trim() == options.simulator.gpu_idx
                ) ?? "0:Auto"
            )
                .split(":")[1]
                .trim();
            console.log(gpu_name);
            window_width = options.simulator.window_width;
            window_height = options.simulator.window_height;
        }
    });

    onDestroy(async () => {
        const simulatorOptions = options();
        await invoke("save_simulator_settings", { simulatorOptions });
    });
</script>

<div class="ui">
    <label for="vsync">Vsync:</label>
    <CheckBox id="vsync" bind:checked={vsync} />

    <label for="port">Port:</label>
    <NumberInput id="port" bind:value={port} min="0" max="65535" step="1" />

    <label for="gpu_name">GPU: </label>
    <Select id="gpu_name" bind:value={gpu_name} values={availableGpusNames} />

    <label for="window_width">Window width:</label>
    <NumberInput id="window_width" bind:value={window_width} min="1" step="1" />

    <label for="window_height">Window height:</label>
    <NumberInput
        id="window_height"
        bind:value={window_height}
        min="1"
        step="1"
    />

    <Button label="Run" click={handleRunClick} />
</div>

<style>
    .ui {
        display: grid;
        grid-template-columns: auto 120px;
        grid-gap: 10px 0px;
        align-items: center;
    }
    label {
        text-align: right;
        padding-right: 10px;
    }
</style>
