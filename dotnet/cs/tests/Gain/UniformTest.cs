/*
 * File: UniformTest.cs
 * Project: Gain
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Gain;

public class UniformTest
{
    [Fact]
    public async Task Uniform()
    {
        var autd = await AUTDTest.CreateController();

        Assert.True(await autd.SendAsync(new Uniform(0x80).WithPhase(new Phase(0x90))));
        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.All(phases, p => Assert.Equal(0x90, p));
        }

        Assert.True(await autd.SendAsync(new Uniform(new EmitIntensity(0x81)).WithPhase(new Phase(0x91))));
        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0x81, d));
            Assert.All(phases, p => Assert.Equal(0x91, p));
        }
    }
}