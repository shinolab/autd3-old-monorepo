/*
 * File: main.ts
 * Project: autd3-server
 * Created Date: 24/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

import "the-new-css-reset/css/reset.css";
import "./styles.css";
import App from "./App.svelte";

const app = new App({
    target: document.getElementById("app"),
});

export default app;
