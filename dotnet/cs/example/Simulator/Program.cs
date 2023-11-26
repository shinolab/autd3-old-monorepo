/*
 * File: Program.cs
 * Project: Simulator
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Utils;
using AUTD3Sharp;
using AUTD3Sharp.Link;
using Samples;


using var autd = await new ControllerBuilder()
    .AddDevice(new AUTD3(Vector3d.zero))
    .AddDevice(new AUTD3(new Vector3d(AUTD3.DeviceWidth, 0, 0)))
    .OpenWithAsync(Simulator.Builder(8080));

await SampleRunner.Run(autd);
