/*
 * File: Group.cs
 * Project: Test
 * Created Date: 23/05/2021
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
    internal static class GroupTest
    {
        public static void Test(Controller autd)
        {
            var config = new SilencerConfig();
            autd.Send(config);

            const double x = Controller.DeviceWidth / 2;
            const double y = Controller.DeviceHeight / 2;
            const double z = 150;

            var center = new Vector3d(x, y, z);

            var g1 = new Gain.Focus(center);
            var g2 = new Gain.Holo.GSPAT();
            g2.Add(center + new Vector3d(30.0, 0.0, 0.0), 1.0);
            g2.Add(center - new Vector3d(30.0, 0.0, 0.0), 1.0);

            var gain = new Gain.Grouped(autd);
            gain.Add(0, g1);
            gain.Add(1, g2);
            var mod = new Modulation.Sine(150); // AM sin 150 Hz
            autd.Send(mod, gain);
        }
    }
}
