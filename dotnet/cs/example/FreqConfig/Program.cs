/*
 * File: Program.cs
 * Project: FreqConfig
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Link;
using AUTD3Sharp.Utils;

var autd = Controller.Builder()
    .Advanced()
    .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
    .OpenWith(new Debug().WithLogLevel(Level.Off));

foreach (var dev in autd.Geometry)
    foreach (var tr in dev)
        tr.Frequency = 70e3;

autd.Send(new Synchronize());
