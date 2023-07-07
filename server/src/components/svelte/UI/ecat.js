/*
 * File: Ecat.js
 * Project: AUTD server
 * Created Date: 06/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

export const SyncMode = {
    DC: "DC",
    FreeRun: "FreeRun",
};

export const TimerStrategy = {
    NativeTimer: "NativeTimer",
    Sleep: "Sleep",
    BusyWait: "BusyWait",
};
