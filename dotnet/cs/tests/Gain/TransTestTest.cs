/*
 * File: TransTestTest.cs
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

public class TransTestTest
{
    [Fact]
    public async Task TransTest()
    {
        var autd = await AUTDTest.CreateController();

        Assert.True(await autd.SendAsync(new TransducerTest().Set(autd.Geometry[0][0], Math.PI, new EmitIntensity(0x80)).Set(autd.Geometry[1][248], Math.PI, new EmitIntensity(0x80))));

        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(0, 0);
            Assert.Equal(0x80, intensities[0]);
            Assert.Equal(128, phases[0]);
            Assert.All(intensities.Skip(1), d => Assert.Equal(0, d));
            Assert.All(phases.Skip(1), p => Assert.Equal(0, p));
        }

        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(1, 0);
            Assert.Equal(0x80, intensities[autd.Geometry[1].NumTransducers - 1]);
            Assert.Equal(128, phases[autd.Geometry[1].NumTransducers - 1]);
            Assert.All(intensities.Take(autd.Geometry[1].NumTransducers - 1), d => Assert.Equal(0, d));
            Assert.All(phases.Take(autd.Geometry[1].NumTransducers - 1), p => Assert.Equal(0, p));
        }
    }
}