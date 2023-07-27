<!--
File: Simulator.svelte
Project: AUTD Server
Created Date: 06/07/2023
Author: Shun Suzuki
-----
Last Modified: 10/07/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

-->

<script lang="ts">
  import type { SimulatorOptions } from "./options.ts";

  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { Command, Child } from "@tauri-apps/api/shell";
  import { consoleOutputQueue } from "./console_output.ts";
  import { appConfigDir } from "@tauri-apps/api/path";

  import Button from "./utils/Button.svelte";
  import Select from "./utils/Select.svelte";
  import CheckBox from "./utils/CheckBox.svelte";
  import NumberInput from "./utils/NumberInput.svelte";

  export let simulatorOptions: SimulatorOptions;

  let appConfigDirPath: string;

  let command;
  let child: null | Child = null;

  let gpuName: string;
  $: {
    if (gpuName) {
      const idx = availableGpusNames.indexOf(gpuName);
      if (idx == 0 || idx == -1) {
        simulatorOptions.gpu_idx = -1;
      } else {
        let gpu_idx = availableGpus[idx - 1].split(":")[0].trim();
        simulatorOptions.gpu_idx = parseInt(gpu_idx);
      }
    }
  }

  const cachedGPUs = writable<string | null>(null);
  let availableGpus: string[] = [];
  $: availableGpusNames = ["Auto"].concat(
    availableGpus.map((gpu) => gpu.split(":")[1].trim())
  );

  let handleRunClick = async () => {
    const args: string[] = [
      "run",
      "-w",
      `${simulatorOptions.window_width},${simulatorOptions.window_height}`,
      "-p",
      simulatorOptions.port.toString(),
      "-v",
      simulatorOptions.vsync ? "true" : "false",
      "-s",
      "simulator_settings.json",
      "--config_path",
      appConfigDirPath,
    ];
    if (simulatorOptions.gpu_idx !== -1) {
      args.push("-g");
      args.push(simulatorOptions.gpu_idx.toString());
    }
    command = Command.sidecar("simulator", args);
    child = await command.spawn();
    command.stdout.on("data", (line) =>
      consoleOutputQueue.update((v) => {
        return [...v, line];
      })
    );
    command.stderr.on("data", (line) =>
      consoleOutputQueue.update((v) => {
        return [...v, line];
      })
    );
    command.on("error", () => handleCloseClick());
    command.on("close", () => handleCloseClick());
  };

  let handleCloseClick = async () => {
    if (child !== null) {
      await child.kill();
      child = null;
    }
  };

  onMount(async () => {
    appConfigDirPath = await appConfigDir();

    let gpus: null | string = null;
    cachedGPUs.subscribe((v) => {
      gpus = v;
    })();
    if (!gpus) {
      gpus = (await Command.sidecar("simulator", "list").execute()).stdout;
      cachedGPUs.set(gpus);
    }
    availableGpus = gpus
      .split("\n")
      .map((s) => s.trim().replace(/ \(type .*\)$/g, ""));
    gpuName = (
      availableGpus.find(
        (gpu) => parseInt(gpu.split(":")[0].trim()) === simulatorOptions.gpu_idx
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

  <label for="gpuName">GPU: </label>
  <Select id="gpuName" bind:value={gpuName} values={availableGpusNames} />

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
