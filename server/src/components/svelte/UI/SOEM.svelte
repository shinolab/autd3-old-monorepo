<!--
File: SOEM.svelte
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
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";
    import { writable } from "svelte/store";
    import { Command } from "@tauri-apps/api/shell";
    import { console_output_queue } from "./console_output.js";

    import Button from "./utils/Button.svelte";
    import Select from "./utils/Select.svelte";
    import CheckBox from "./utils/CheckBox.svelte";
    import NumberInput from "./utils/NumberInput.svelte";

    import { msToDuration, msFromDuration } from "./utils/duration.js";

    import { SyncMode, TimerStrategy } from "./ecat.js";

    export let soemOptions;

    let command;
    let child = null;

    let parse_mode = (mode) => {
        switch (mode.toString().toLowerCase()) {
            case "dc":
                return "dc";
            case "freerun":
                return "free-run";
            default:
                return "free-run";
        }
    };

    let parse_strategy = (strategy) => {
        switch (strategy.toString().toLowerCase()) {
            case "nativetimer":
                return "native-timer";
            case "sleep":
                return "sleep";
            case "busywait":
                return "busy-wait";
            default:
                return "sleep";
        }
    };

    let state_check_interval_ms = msFromDuration(
        soemOptions.state_check_interval
    );
    $: {
        soemOptions.state_check_interval = msToDuration(
            state_check_interval_ms
        );
    }
    let timeout_ms = msFromDuration(soemOptions.timeout);
    $: {
        soemOptions.timeout = msToDuration(timeout_ms);
    }

    const cachedAdapters = writable(null);
    let adapter_names;

    let handleRunClick = async () => {
        const args = [
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
            parse_mode(soemOptions.mode),
            "-w",
            parse_strategy(soemOptions.timer_strategy),
            "-e",
            state_check_interval_ms.toString(),
            "-t",
            timeout_ms.toString(),
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
        let adapters = null;
        cachedAdapters.subscribe((v) => {
            adapters = v;
        })();
        if (!adapters) {
            adapters = await invoke("fetch_ifnames", {});
            cachedAdapters.set(adapters);
        }
        adapter_names = ["Auto"].concat(
            adapters.map((adapter) => adapter.split(",")[1].trim())
        );
    });
</script>

<div class="ui">
    <label for="ifname">Interface name:</label>
    <Select
        id="ifname"
        bind:value={soemOptions.ifname}
        values={adapter_names}
    />

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
    <Select
        id="mode"
        bind:value={soemOptions.mode}
        values={Object.keys(SyncMode)}
    />

    <label for="timer_strategy">Timer strategy:</label>
    <Select
        id="timer_strategy"
        bind:value={soemOptions.timer_strategy}
        values={Object.keys(TimerStrategy)}
    />

    <label for="state_check_interval_ms">State check interval [ms]:</label>
    <NumberInput
        id="state_check_interval_ms"
        bind:value={state_check_interval_ms}
        min="1"
        step="1"
    />

    <label for="timeout_ms">Timeout [ms]:</label>
    <NumberInput id="timeout_ms" bind:value={timeout_ms} min="1" step="1" />

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
