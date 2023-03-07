/*
 * File: Custom.cs
 * Project: Test
 * Created Date: 14/10/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;
using Custom = AUTD3Sharp.Gain.Custom;

namespace Samples;
internal static class CustomTest
{
    private static Gain Focus(Controller autd, Vector3d point)
    {
        var amps = new double[autd.Geometry.NumTransducers];
        var phases = new double[autd.Geometry.NumTransducers];

        foreach (var tr in autd.Geometry)
        {
            var tp = tr.Position;
            var dist = (tp - point).L2Norm;
            var wavenum = 2.0 * AUTD3.Pi / tr.Wavelength;
            var phase = dist * wavenum;
            amps[tr.Id] = 1.0;
            phases[tr.Id] = phase;
        }

        return new Custom(amps, phases);
    }

    public static void Test(Controller autd)
    {
        var config = new SilencerConfig();
        autd.Send(config, TimeSpan.FromMilliseconds(20));

        var mod = new Sine(150);
        var gain = Focus(autd, autd.Geometry.Center + new Vector3d(0, 0, 150));
        autd.Send(mod, gain, TimeSpan.FromMilliseconds(20));
    }
}
