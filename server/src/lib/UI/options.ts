/*
 * File: options.ts
 * Project: AUTD Server
 * Created Date: 10/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

import type { Duration } from "./utils/duration.js";

export const SyncModeValues = ["DC", "FreeRun"] as const
export type SyncMode = typeof SyncModeValues[number]


export const TimerStrategyValues = ["NativeTimer", "Sleep", "BusyWait"] as const
export type TimerStrategy = typeof TimerStrategyValues[number]

export interface TwinCATOptions {
    client: string;
    sync0: number;
    task: number;
    base: number;
    mode: SyncMode;
    keep: boolean;
}

export interface SOEMOptions {
    ifname: string;
    port: number;
    sync0: number;
    send: number;
    buf_size: number;
    mode: SyncMode;
    timer_strategy: TimerStrategy;
    state_check_interval: Duration;
    timeout: Duration;
    debug: boolean;
    lightweight: boolean;
}

export interface SimulatorOptions {
    vsync: boolean;
    port: number;
    gpu_idx: number;
    window_width: number;
    window_height: number;
}

export interface Options {
    twincat: TwinCATOptions;
    soem: SOEMOptions;
    simulator: SimulatorOptions;
}
