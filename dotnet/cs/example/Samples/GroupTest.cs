/*
 * File: GroupTest.cs
 * Project: Samples
 * Created Date: 15/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;

namespace Samples;

internal static class GroupTest
{
    public static async Task Test<T>(Controller<T> autd)
    {
        var config = new Silencer();
        await autd.SendAsync(config);

        await autd.Group(dev =>
            {
                return dev.Idx switch
                {
                    0 => "null",
                    1 => "focus",
                    _ => null
                };
            })
            .Set("null", (new Static(), new Null()))
            .Set("focus", (new Sine(150), new Focus(autd.Geometry.Center + new Vector3d(0, 0, 150))))
            .SendAsync();
    }
}
