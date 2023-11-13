/*
 * File: GainTest.cs
 * Project: Gain
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
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
        private readonly EmitIntensity _amp;
        private readonly double _phase;
        public bool[] Check;

        public MyUniform(double amp, double phase, bool[] check)
        {
            _amp = EmitIntensity.NewNormalized(amp);
            _phase = phase;
            Check = check;
        }

        public override Dictionary<int, Drive[]> Calc(Geometry geometry)
        {
            return Transform(geometry, (dev, _) =>
            {
                Check[dev.Idx] = true;
                return new Drive { Phase = _phase, Amp = _amp.PulseWidth };
            });
        }
    }

    [Fact]
    public async Task Gain()
    {
        var autd = await AUTDTest.CreateController();

        var check = new bool[autd.Geometry.NumDevices];
        Assert.True(await autd.SendAsync(new MyUniform(0.5, Math.PI, check)));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = autd.Link.DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(85, d));
            Assert.All(phases, p => Assert.Equal(256, p));
        }
    }

    [Fact]
    public async Task GainCheckOnlyForEnabled()
    {
        var autd = await AUTDTest.CreateController();
        autd.Geometry[0].Enable = false;

        var check = new bool[autd.Geometry.NumDevices];
        Assert.True(await autd.SendAsync(new MyUniform(0.5, Math.PI, check)));

        Assert.False(check[0]);
        Assert.True(check[1]);

        {
            var (duties, phases) = autd.Link.DutiesAndPhases(0, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (duties, phases) = autd.Link.DutiesAndPhases(1, 0);
            Assert.All(duties, d => Assert.Equal(85, d));
            Assert.All(phases, p => Assert.Equal(256, p));
        }
    }
}