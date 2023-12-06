/*
 * File: PlaneTest.cs
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

public class PlaneTest
{
    [Fact]
    public async Task Plane()
    {
        var autd = await AUTDTest.CreateController();

        Assert.True(await autd.SendAsync(new Plane(new Vector3d(0, 0, 1)).WithIntensity(0x80)));
        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }

        Assert.True(await autd.SendAsync(new Plane(new Vector3d(0, 0, 1)).WithIntensity(new EmitIntensity(0x81))));
        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0x81, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
    }
}
