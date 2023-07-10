/*
 * File: console_output.ts
 * Project: AUTD Server
 * Created Date: 09/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

import { writable } from 'svelte/store';

export const consoleOutputQueue = writable<string[]>([]);
