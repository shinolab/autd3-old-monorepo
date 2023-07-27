/*
 * File: Group.cs
 * Project: Test
 * Created Date: 23/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/07/2023
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

internal static class GroupTest
{
    public static void Test(Controller autd)
    {
        var config = new SilencerConfig();
        autd.Send(config);

        var g1 = new Focus(autd.Geometry.CenterOf(0) + new Vector3d(0, 0, 150));
        var g2 = new Bessel(autd.Geometry.CenterOf(1), Vector3d.UnitZ, 13.0 / 180.0 * Math.PI);

        var gain = new Grouped().Add(0, g1).Add(1, g2);
        var mod = new Sine(150); // AM sin 150 Hz
        autd.Send(mod, gain);
    }
}
