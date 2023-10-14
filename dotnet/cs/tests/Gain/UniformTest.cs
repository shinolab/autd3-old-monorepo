/*
 * File: UniformTest.cs
 * Project: Gain
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Gain;

public class UniformTest
{
    [Fact]
    public void Uniform()
    {
        var autd = AUTDTest.CreateController();

        Assert.True(autd.Send(new Uniform(0.5).WithPhase(Math.PI)));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }
    }
}