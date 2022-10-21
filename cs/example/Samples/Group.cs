/*
 * File: Group.cs
 * Project: Test
 * Created Date: 23/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Gain.Holo;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;

namespace Samples;

internal static class GroupTest
{
    public static void Test(Controller autd)
    {
        var config = new SilencerConfig();
        autd.Send(config);

        var center = autd.Geometry.Center + new Vector3d(0, 0, 150);

        var g1 = new Focus(center);
        var g2 = new GSPAT();
        g2.Add(center + new Vector3d(30.0, 0.0, 0.0), 1.0);
        g2.Add(center - new Vector3d(30.0, 0.0, 0.0), 1.0);

        var gain = new Grouped(autd);
        gain.Add(0, g1);
        gain.Add(1, g2);
        var mod = new Sine(150); // AM sin 150 Hz
        autd.Send(mod, gain);
    }
}
