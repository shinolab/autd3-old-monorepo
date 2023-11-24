/*
 * File: TransformTest.cs
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

public class TransformTest
{
    [Fact]
    public async Task Transform()
    {
        var autd = await AUTDTest.CreateController();

        Assert.True(await autd.SendAsync(new Uniform(new EmitIntensity(0x80)).WithPhase(Math.PI).WithTransform((dev, _, d) =>
            dev.Idx == 0 ? d with { Phase = d.Phase + Math.PI / 4 } : d with { Phase = d.Phase - Math.PI / 4 })));
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(0, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.All(phases, p => Assert.Equal(128 + 32, p));
        }

        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(1, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.All(phases, p => Assert.Equal(128 - 32, p));
        }
    }

    [Fact]
    public async Task TransformCheckOnlyForEnabled()
    {
        var autd = await AUTDTest.CreateController();
        autd.Geometry[0].Enable = false;

        var check = new bool[autd.Geometry.NumDevices];
        Assert.True(await autd.SendAsync(new Uniform(new EmitIntensity(0x80)).WithPhase(Math.PI)
        .WithTransform((dev, _, d) =>
        {
            check[dev.Idx] = true;
            return d;
        })));

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
            Assert.All(phases, p => Assert.Equal(128, p));
        }
    }
}