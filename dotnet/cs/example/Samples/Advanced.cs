﻿/*
 * File: Advanced.cs
 * Project: Test
 * Created Date: 23/05/2021
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
using AUTD3Sharp.Modulation;

namespace Samples;

public class AdvancedTest
{
    private class UniformGain : Gain
    {
        public override Drive[] Calc(Geometry geometry)
        {
            return Transform(geometry, _ => new Drive(1.0, 0.0));
        }
    }

    private class Burst : Modulation
    {
        private readonly int _length;

        public Burst(int length, double sampleFreqDiv = 5120) : base(sampleFreqDiv)
        {
            _length = length;
        }

        public override double[] Calc()
        {
            var buf = new double[_length];
            buf[0] = 1;
            return buf;
        }
    }

    public static void Test(Controller autd)
    {
        var config = SilencerConfig.None();
        autd.Send(config);

        var g = new UniformGain();
        var m = new Burst(4000);

        autd.Send(m, g);
    }
}
