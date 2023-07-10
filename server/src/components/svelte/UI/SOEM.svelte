<!--
File: SOEM.svelte
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
  import type { SOEMOptions, SyncMode, TimerStrategy } from "./options.ts";
  import { SyncModeValues, TimerStrategyValues } from "./options.ts";

  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { Command, Child } from "@tauri-apps/api/shell";
  import { consoleOutputQueue } from "./console_output.ts";

  import Button from "./utils/Button.svelte";
  import Select from "./utils/Select.svelte";
  import CheckBox from "./utils/CheckBox.svelte";
  import NumberInput from "./utils/NumberInput.svelte";

  import { msToDuration, msFromDuration } from "./utils/duration.ts";

  export let soemOptions: SOEMOptions;

  let command;
  let child: null | Child = null;

  let parseMode = (mode: SyncMode) => {
    switch (mode) {
      case "DC":
        return "dc";
      case "FreeRun":
        return "free-run";
      default:
        return "free-run";
    }
  };

  let parseStrategy = (strategy: TimerStrategy) => {
    switch (strategy) {
      case "NativeTimer":
        return "native-timer";
      case "Sleep":
        return "sleep";
      case "BusyWait":
        return "busy-wait";
      default:
        return "sleep";
    }
  };

  let stateCheckIntervalMs = msFromDuration(soemOptions.state_check_interval);
  $: {
    soemOptions.state_check_interval = msToDuration(stateCheckIntervalMs);
  }
  let timeoutMs = msFromDuration(soemOptions.timeout);
  $: {
    soemOptions.timeout = msToDuration(timeoutMs);
  }

  const cachedAdapters = writable<string[]>([]);
  let adapterNames: string[] = [];

  let handleRunClick = async () => {
    const args: string[] = [
      "run",
      "-i",
      soemOptions.ifname == "Auto" ? "" : soemOptions.ifname,
      "-p",
      soemOptions.port.toString(),
      "-s",
      soemOptions.sync0.toString(),
      "-c",
      soemOptions.send.toString(),
      "-b",
      soemOptions.buf_size.toString(),
      "-m",
      parseMode(soemOptions.mode),
      "-w",
      parseStrategy(soemOptions.timer_strategy),
      "-e",
      stateCheckIntervalMs.toString(),
      "-t",
      timeoutMs.toString(),
    ];
    if (soemOptions.debug) {
      args.push("-d");
    }
    if (soemOptions.lightweight) {
      args.push("-l");
    }
    command = Command.sidecar("SOEMAUTDServer", args);
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
    let adapters: string[] = [];
    cachedAdapters.subscribe((v) => {
      adapters = v;
    })();
    if (adapters.length == 0) {
      adapters = await invoke<string[]>("fetch_ifnames", {});
      cachedAdapters.set(adapters);
    }
    adapterNames = ["Auto"].concat(
      adapters.map((adapter) => adapter.split(",")[1].trim())
    );
  });
</script>

<div class="ui">
  <label for="ifname">Interface name:</label>
  <Select id="ifname" bind:value={soemOptions.ifname} values={adapterNames} />

  <label for="port">Port:</label>
  <NumberInput
    id="port"
    bind:value={soemOptions.port}
    min="0"
    max="65535"
    step="1"
  />

  <label for="buf_size">Buffer size:</label>
  <NumberInput
    id="buf_size"
    bind:value={soemOptions.buf_size}
    min="1"
    step="1"
  />

  <label for="sync0">Sync0 cycle:</label>
  <NumberInput id="sync0" bind:value={soemOptions.sync0} min="1" step="1" />

  <label for="send">Send cycle:</label>
  <NumberInput id="send" bind:value={soemOptions.send} min="1" step="1" />

  <label for="mode">Sync mode:</label>
  <Select id="mode" bind:value={soemOptions.mode} values={SyncModeValues} />

  <label for="timer_strategy">Timer strategy:</label>
  <Select
    id="timer_strategy"
    bind:value={soemOptions.timer_strategy}
    values={TimerStrategyValues}
  />

  <label for="stateCheckIntervalMs">State check interval [ms]:</label>
  <NumberInput
    id="stateCheckIntervalMs"
    bind:value={stateCheckIntervalMs}
    min="1"
    step="1"
  />

  <label for="timeoutMs">Timeout [ms]:</label>
  <NumberInput id="timeoutMs" bind:value={timeoutMs} min="1" step="1" />

  <label for="debug">Enable debug:</label>
  <CheckBox id="debug" bind:checked={soemOptions.debug} />

  <label for="lightweight">Enable lightweight:</label>
  <CheckBox id="lightweight" bind:checked={soemOptions.lightweight} />

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
