/*
 * File: GainHolo.cs
 * Project: Test
 * Created Date: 30/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Gain.Holo;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;

namespace Samples;

internal static class GainHoloTest
{
    public static void Test(Controller autd)
    {
        var config = new SilencerConfig();
        autd.Send(config);

        var center = autd.Geometry.Center + new Vector3d(0, 0, 150);

        var g = new GSPAT().WithConstraint(new Uniform());
        g.AddFocus(center + 20.0 * Vector3d.UnitX, 1.0);
        g.AddFocus(center - 20.0 * Vector3d.UnitX, 1.0);

        var m = new Sine(150);

        autd.Send(m, g);
    }
}
