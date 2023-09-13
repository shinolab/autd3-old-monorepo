/*
 * File: PlaneWave.cs
 * Project: Samples
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;

namespace Samples;

internal static class PlaneWaveTest
{
    public static void Test(Controller autd)
    {
        var config = new Silencer();
        autd.Send(config);

        var m = new Sine(150); // AM sin 150 Hz

        var dir = Vector3d.UnitZ;
        var g = new Plane(dir);

        autd.Send((m, g));
    }
}
