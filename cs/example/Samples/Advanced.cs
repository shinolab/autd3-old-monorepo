/*
 * File: Advanced.cs
 * Project: Test
 * Created Date: 23/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
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

        var amp = new double[autd.NumTransducers];
        var phase = new double[autd.NumTransducers];
        for (var i = 0; i < autd.NumTransducers; i++)
        {
            amp[i] = 1.0;
            phase[i] = 0.0;
        }

        var burst = new byte[4000];
        burst[0] = 0xFF;

        var g = new AUTD3Sharp.Gain.Custom(amp, phase);
        var m = new AUTD3Sharp.Modulation.Custom(burst, 40960);

        autd.Send(m, g);
    }
}
