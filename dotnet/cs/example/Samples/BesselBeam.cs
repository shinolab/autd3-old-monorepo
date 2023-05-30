﻿/*
 * File: BesselBeam.cs
 * Project: Test
 * Created Date: 30/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;

namespace Samples;

internal static class BesselBeamTest
{
    public static void Test(Controller autd)
    {
        var config = new SilencerConfig();
        autd.Send(config);

        var mod = new Sine(150); // AM sin 150 Hz

        var start = autd.Geometry.Center;
        var dir = Vector3d.UnitZ;
        var gain = new BesselBeam(start, dir, 13.0 / 180.0 * AUTD3.Pi); // BesselBeam from (x, y, 0), theta = 13 deg
        autd.Send(gain, mod);
    }
}
