/*
 * File: TransTest.cs
 * Project: Samples
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/11/2023
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
        var g = new TransducerTest().Set(autd.Geometry[0][0], 0, EmitIntensity.Max).Set(autd.Geometry[0][248], 0, EmitIntensity.Max);
        await autd.SendAsync((m, g));
    }
}
