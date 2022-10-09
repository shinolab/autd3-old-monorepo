/*
 * File: Custom.cs
 * Project: Test
 * Created Date: 14/10/2021
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
    internal static class CustomTest
    {
        private static Gain.Gain Focus(Controller autd, Vector3d point)
        {
            var amps = new double[autd.NumTransducers];
            var phases = new double[autd.NumTransducers];

            var c = 0;
            for (var dev = 0; dev < autd.NumDevices; dev++)
            {
                for (var i = 0; i < Controller.NumTransInDevice; i++)
                {
                    var tp = autd.TransPosition(dev, i);
                    var dist = (tp - point).L2Norm;
                    var wavelength = autd.Wavelength(dev, i);
                    var phase = dist / wavelength;
                    amps[c] = 1.0;
                    phases[c] = phase;
                    c++;
                }
            }

            return new Gain.Custom(amps, phases);
        }

        public static void Test(Controller autd)
        {
            var config = new SilencerConfig();
            autd.Send(config);

            const double x = Controller.DeviceWidth / 2;
            const double y = Controller.DeviceHeight / 2;
            const double z = 150;

            var mod = new Modulation.Sine(150);
            var gain = Focus(autd, new Vector3d(x, y, z));
            autd.Send(mod, gain);
        }
    }
}
