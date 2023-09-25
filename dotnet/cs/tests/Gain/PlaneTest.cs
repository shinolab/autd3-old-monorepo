/*
 * File: PlaneTest.cs
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

public class PlaneTest
{
    [Fact]
    public void Plane()
    {
        var autd = AUTDTest.CreateController();

        Assert.True(autd.Send(new Plane(new Vector3d(0, 0, 1)).WithAmp(0.5)));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
    }
}
