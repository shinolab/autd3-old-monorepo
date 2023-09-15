/*
 * File: STMTest.cs
 * Project: Samples
 * Created Date: 29/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
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
        var config = Silencer.Disable();
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

internal static class SoftwareSTMTest
{
    public static void Test(Controller autd)
    {
        var config = Silencer.Disable();
        autd.Send(config);

        var mod = new Static();
        autd.Send(mod);

        var fin = false;
        var th = Task.Run(() =>
        {
            Console.WriteLine("press enter to stop software stm...");
            Console.ReadKey(true);

            fin = true;
        });


        var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
        const double freq = 1.0;
        const int pointNum = 200;
        const double radius = 30.0;
        autd.SoftwareSTM((cnt, i, _) =>
        {
            if (fin) return false;

            var theta = 2.0 * Math.PI * i / pointNum;
            var p = center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0);
            try
            {
                return cnt.Send(new Focus(center + p));
            }
            catch (Exception)
            {
                return false;
            }
        }).WithTimerStrategy(TimerStrategy.NativeTimer).Start(TimeSpan.FromSeconds(1.0 / freq / pointNum));

        th.Wait();
    }
}
