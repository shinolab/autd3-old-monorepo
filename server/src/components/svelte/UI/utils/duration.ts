/*
 * File: duration.ts
 * Project: AUTD server
 * Created Date: 07/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

export interface Duration {
    secs: number;
    nanos: number;
}

export let msToDuration = (ms: number) => {
    let secs = Math.floor(ms / 1000);
    let nanos = (ms % 1000) * 1000000;
    return { secs: secs, nanos };
}

export let msFromDuration = (duration: Duration) => {
    return duration.secs * 1000 + duration.nanos / 1000000;
}
