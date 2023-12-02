/*
 * File: TransTest.cs
 * Project: Samples
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;

namespace Samples;

internal static class TransTest
{
    public static async Task Test<T>(Controller<T> autd)
    {
        var config = new Silencer();
        await autd.SendAsync(config);

        var m = new Sine(150);
        var g = new TransducerTest(
            (dev, tr) => (dev.Idx, tr.Idx) switch
            {
                (0, 0) => new Drive(new Phase(0), EmitIntensity.Max),
                (0, 248) => new Drive(new Phase(0), EmitIntensity.Max),
                _ => null
            }
        );
        await autd.SendAsync((m, g));
    }
}
