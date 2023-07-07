<!--
File: SOEM.svelte
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

    import Button from "./utils/Button.svelte";
    import Select from "./utils/Select.svelte";
    import CheckBox from "./utils/CheckBox.svelte";
    import NumberInput from "./utils/NumberInput.svelte";

    import { msToDuration, msFromDuration } from "./utils/duration.js";

    import { SyncMode, TimerStrategy } from "./ecat.js";

    let ifname = "";
    let port = 8080;
    let sync0 = 2;
    let send = 2;
    let buf_size = 32;
    let mode = Object.keys(SyncMode)[1];
    let timer_strategy = Object.keys(TimerStrategy)[1];
    let state_check_interval_ms = 500;
    let timeout_ms = 200;
    let debug = false;
    let lightweight = false;

    let adapters = [];
    $: adapter_names = ["Auto"].concat(
        adapters.map((adapter) => adapter.split(",")[1].trim())
    );

    let options = () => {
        const timeout = msToDuration(timeout_ms);
        const state_check_interval = msToDuration(state_check_interval_ms);
        return JSON.stringify({
            ifname,
            port,
            sync0,
            send,
            buf_size,
            mode,
            timer_strategy,
            state_check_interval,
            timeout,
            debug,
            lightweight,
        });
    };

    let handleRunClick = async () => {
        const soemOptions = options();
        try {
            await invoke("run_soem_server", { soemOptions });
        } catch (err) {
            alert(err);
        }
    };

    onMount(async () => {
        const options = await invoke("load_settings", {});
        if (options.soem) {
            ifname = !!options.soem.ifname ? options.soem.ifname : "Auto";
            port = options.soem.port;
            sync0 = options.soem.sync0;
            send = options.soem.send;
            buf_size = options.soem.buf_size;
            mode = options.soem.mode;
            timer_strategy = options.soem.timer_strategy;
            state_check_interval_ms = msFromDuration(
                options.soem.state_check_interval
            );
            timeout_ms = msFromDuration(options.soem.timeout);
            debug = options.soem.debug;
            lightweight = options.soem.lightweight;
        }

        adapters = await invoke("fetch_ifnames", {});
    });

    onDestroy(async () => {
        const soemOptions = options();
        await invoke("save_soem_settings", { soemOptions });
    });
</script>

<div class="ui">
    <label for="ifname">Interface name:</label>
    <Select id="ifname" bind:value={ifname} values={adapter_names} />

    <label for="port">Port:</label>
    <NumberInput id="port" bind:value={port} min="0" max="65535" step="1" />

    <label for="buf_size">Buffer size:</label>
    <NumberInput id="buf_size" bind:value={buf_size} min="1" step="1" />

    <label for="sync0">Sync0 cycle:</label>
    <NumberInput id="sync0" bind:value={sync0} min="1" step="1" />

    <label for="send">Send cycle:</label>
    <NumberInput id="send" bind:value={send} min="1" step="1" />

    <label for="mode">Sync mode:</label>
    <Select id="mode" bind:value={mode} values={Object.keys(SyncMode)} />

    <label for="timer_strategy">Timer strategy:</label>
    <Select
        id="timer_strategy"
        bind:value={timer_strategy}
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
    <CheckBox id="debug" bind:checked={debug} />

    <label for="lightweight">Enable lightweight:</label>
    <CheckBox id="lightweight" bind:checked={lightweight} />

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
