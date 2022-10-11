/*
 * File: FocusTest.cs
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
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;

namespace example.Test
{
    internal static class FocusTest
    {
        public static void Test(Controller autd)
        {
            var config = new SilencerConfig();
            autd.Send(config);

            const double x = Controller.DeviceWidth / 2;
            const double y = Controller.DeviceHeight / 2;
            const double z = 150;

            var mod = new Sine(150); // AM sin 150 Hz
            var gain = new Focus(new Vector3d(x, y, z)); // Focal point @ (x, y, z) [mm]
            autd.Send(mod, gain);
        }
    }
}
