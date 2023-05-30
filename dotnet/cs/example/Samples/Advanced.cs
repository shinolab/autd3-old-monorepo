/*
 * File: Advanced.cs
 * Project: Test
 * Created Date: 23/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;

namespace Samples;

public class AdvancedTest
{
    public static void Test(Controller autd)
    {
        var config = SilencerConfig.None();
        autd.Send(config);

        var amp = new double[autd.Geometry.NumTransducers];
        var phase = new double[autd.Geometry.NumTransducers];
        for (var i = 0; i < autd.Geometry.NumTransducers; i++)
        {
            amp[i] = 1.0;
            phase[i] = 0.0;
        }

        var burst = new double[4000];
        burst[0] = 1;

        var g = new AUTD3Sharp.Gain.Custom(amp, phase);
        var m = new AUTD3Sharp.Modulation.Custom(burst, 40960);

        autd.Send(m, g);
    }
}
