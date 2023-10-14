/*
 * File: FocusTest.cs
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

public class FocusTest
{
    [Fact]
    public void Focus()
    {
        var autd = AUTDTest.CreateController();

        Assert.True(autd.Send(new Focus(autd.Geometry.Center).WithAmp(0.5)));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.Contains(phases, p => p != 0);
        }
    }
}