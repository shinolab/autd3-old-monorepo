/*
 * File: GainTest.cs
 * Project: Gain
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Gain;

public class GainTest
{
    public class MyUniform : AUTD3Sharp.Gain.Gain
    {
        private readonly double _amp;
        private readonly double _phase;

        public MyUniform(double amp, double phase)
        {
            _amp = amp;
            _phase = phase;
        }

        public override Dictionary<int, Drive[]> Calc(Geometry geometry)
        {
            return Transform(geometry, (_, _) => new Drive { Phase = _phase, Amp = _amp });
        }
    }

    [Fact]
    public void Gain()
    {
        var autd = AUTDTest.CreateController();

        Assert.True(autd.Send(new MyUniform(0.5, Math.PI)));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }
    }
}