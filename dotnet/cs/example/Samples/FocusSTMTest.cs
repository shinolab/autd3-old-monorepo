/*
 * File: FocusSTMTest.cs
 * Project: Test
 * Created Date: 30/04/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
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
        var config = Silencer.Disable();
        autd.Send(config);

        var mod = new Static();
        autd.Send(mod);

        var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
        const int pointNum = 200;
        const double radius = 30.0;
        var stm = new FocusSTM(1).AddFociFromIter(Enumerable.Range(0, pointNum).Select(i =>
        {
            var theta = 2.0 * Math.PI * i / pointNum;
            return center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0);
        }));

        Console.WriteLine($"Actual frequency is {stm.Frequency}");
        autd.Send(stm);
    }
}
