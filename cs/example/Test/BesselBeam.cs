/*
 * File: BesselBeam.cs
 * Project: Test
 * Created Date: 30/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3Sharp;
using AUTD3Sharp.Utils;

namespace example.Test
{
    internal static class BesselBeamTest
    {
        public static void Test(Controller autd)
        {
            var config = new SilencerConfig();
            autd.Send(config);

            const double x = Controller.DeviceWidth / 2;
            const double y = Controller.DeviceHeight / 2;

            var mod = new Modulation.Sine(150); // AM sin 150 Hz

            var start = new Vector3d(x, y, 0);
            var dir = Vector3d.UnitZ;
            var gain = new Gain.BesselBeam(start, dir, 13.0f / 180f * Controller.Pi); // BesselBeam from (x, y, 0), theta = 13 deg
            autd.Send(gain, mod);
        }
    }
}
