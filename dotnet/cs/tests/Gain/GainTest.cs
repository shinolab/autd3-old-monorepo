/*
 * File: GainTest.cs
 * Project: Gain
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/11/2023
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
        private readonly EmitIntensity _intensity;
        private readonly Phase _phase;
        public bool[] Check;

        public MyUniform(EmitIntensity intensity, Phase phase, bool[] check)
        {
            _intensity = intensity;
            _phase = phase;
            Check = check;
        }

        public override Dictionary<int, Drive[]> Calc(Geometry geometry)
        {
            return Transform(geometry, (dev, _) =>
            {
                Check[dev.Idx] = true;
                return new Drive { Phase = _phase, Intensity = _intensity };
            });
        }
    }

    [Fact]
    public async Task Gain()
    {
        var autd = await AUTDTest.CreateController();

        var check = new bool[autd.Geometry.NumDevices];
        Assert.True(await autd.SendAsync(new MyUniform(new EmitIntensity(0x80), new Phase(0x90), check)));

        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.All(phases, p => Assert.Equal(0x90, p));
        }
    }

    [Fact]
    public async Task GainCheckOnlyForEnabled()
    {
        var autd = await AUTDTest.CreateController();
        autd.Geometry[0].Enable = false;

        var check = new bool[autd.Geometry.NumDevices];
        Assert.True(await autd.SendAsync(new MyUniform(new EmitIntensity(0x80), new Phase(0x90), check)));

        Assert.False(check[0]);
        Assert.True(check[1]);

        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(0, 0);
            Assert.All(intensities, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(1, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.All(phases, p => Assert.Equal(0x90, p));
        }
    }
}