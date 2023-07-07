/*
 * File: Duration.js
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


export let msToDuration = (ms) => {
    let secs = Math.floor(ms / 1000);
    let nanos = (ms % 1000) * 1000000;
    return { secs, nanos };
}

export let msFromDuration = (duration) => {
    return duration.secs * 1000 + duration.nanos / 1000000;
}
