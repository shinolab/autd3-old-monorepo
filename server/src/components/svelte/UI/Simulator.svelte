<!--
File: Simulator.svelte
Project: AUTD server
Created Date: 06/07/2023
Author: Shun Suzuki
-----
Last Modified: 09/07/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

-->

<script>
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { Command } from "@tauri-apps/api/shell";
    import { console_output_queue } from "./console_output.js";

    import Button from "./utils/Button.svelte";
    import Select from "./utils/Select.svelte";
    import CheckBox from "./utils/CheckBox.svelte";
    import NumberInput from "./utils/NumberInput.svelte";

    export let simulatorOptions;

    let command;
    let child = null;

    let gpu_name;
    $: {
        if (gpu_name) {
            const idx = availableGpusNames.indexOf(gpu_name);
            if (idx == 0 || idx == -1) {
                simulatorOptions.gpu_idx = -1;
            } else {
                let gpu_idx = parseInt(
                    availableGpus[idx - 1].split(":")[0].trim()
                );
                simulatorOptions.gpu_idx = gpu_idx;
            }
        }
    }

    const cachedGPUs = writable(null);
    let availableGpus = [];
    $: availableGpusNames = ["Auto"].concat(
        availableGpus.map((gpu) => gpu.split(":")[1].trim())
    );

    let handleRunClick = async () => {
        const args = [
            "run",
            "-w",
            `${simulatorOptions.window_width},${simulatorOptions.window_height}`,
            "-p",
            simulatorOptions.port.toString(),
            "-v",
            simulatorOptions.vsync ? "true" : "false",
            "-s",
            "simulator_settings.json",
        ];
        if (simulatorOptions.gpu_idx !== -1) {
            args.push("-g");
            args.push(simulatorOptions.gpu_idx.toString());
        }
        command = Command.sidecar("simulator", args);
        child = await command.spawn();
        command.stdout.on("data", (line) =>
            console_output_queue.update((v) => {
                return [...v, line];
            })
        );
        command.stderr.on("data", (line) =>
            console_output_queue.update((v) => {
                return [...v, line];
            })
        );
        command.on("error", () => handleCloseClick());
        command.on("close", () => handleCloseClick());
    };

    let handleCloseClick = async () => {
        if (child) {
            child.kill();
            child = null;
        }
    };

    onMount(async () => {
        let gpus = null;
        cachedGPUs.subscribe((v) => {
            gpus = v;
        })();
        if (!gpus) {
            gpus = (await Command.sidecar("simulator", "list").execute())
                .stdout;
            cachedGPUs.set(gpus);
        }
        availableGpus = gpus
            .split("\n")
            .map((s) => s.trim().replace(/ \(type .*\)$/g, ""));
        gpu_name = (
            availableGpus.find(
                (gpu) => gpu.split(":")[0].trim() == simulatorOptions.gpu_idx
            ) ?? "0:Auto"
        )
            .split(":")[1]
            .trim();
    });
</script>

<div class="ui">
    <label for="vsync">Vsync:</label>
    <CheckBox id="vsync" bind:checked={simulatorOptions.vsync} />

    <label for="port">Port:</label>
    <NumberInput
        id="port"
        bind:value={simulatorOptions.port}
        min="0"
        max="65535"
        step="1"
    />

    <label for="gpu_name">GPU: </label>
    <Select id="gpu_name" bind:value={gpu_name} values={availableGpusNames} />

    <label for="window_width">Window width:</label>
    <NumberInput
        id="window_width"
        bind:value={simulatorOptions.window_width}
        min="1"
        step="1"
    />

    <label for="window_height">Window height:</label>
    <NumberInput
        id="window_height"
        bind:value={simulatorOptions.window_height}
        min="1"
        step="1"
    />

    <Button label="Run" click={handleRunClick} disabled={!!child} />
    <Button label="Close" click={handleCloseClick} disabled={!child} />
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
