/*
 * File: FocusSTMTest.cs
 * Project: Test
 * Created Date: 30/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;
using AUTD3Sharp.STM;

namespace Samples;

internal static class FocusSTMTest
{
    public static void Test(Controller autd)
    {
        var config = SilencerConfig.None();
        autd.Send(config, TimeSpan.FromMilliseconds(20));

        var mod = new Static();
        autd.Send(mod, TimeSpan.FromMilliseconds(20));

        var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
        var stm = new FocusSTM();
        const int pointNum = 200;
        for (var i = 0; i < pointNum; i++)
        {
            const double radius = 30.0;
            var theta = 2.0 * Math.PI * i / pointNum;
            var p = radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0);
            stm.Add(center + p);
        }
        stm.Frequency = 1;
        Console.WriteLine($"Actual frequency is {stm.Frequency}");
        autd.Send(stm, TimeSpan.FromMilliseconds(20));
    }
}
