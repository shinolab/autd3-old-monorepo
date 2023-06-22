/*
 * File: STMGainTest.cs
 * Project: Test
 * Created Date: 21/07/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.STM;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;

namespace Samples;

internal static class GainSTMTest
{
    public static void Test(Controller autd)
    {
        var config = SilencerConfig.None();
        autd.Send(config);

        var m = new Static();

        var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
        const int pointNum = 50;
        const double radius = 30.0;
        var stm = new GainSTM(1.0).AddGainsFromIter(Enumerable.Range(0, pointNum).Select(i =>
        {
            var theta = 2.0 * Math.PI * i / pointNum;
            return new Focus(center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0));
        }));

        Console.WriteLine($"Actual frequency is {stm.Frequency}");
        autd.Send((m, stm));
    }
}
