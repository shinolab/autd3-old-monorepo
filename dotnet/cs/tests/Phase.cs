/*
 * File: Phase.cs
 * Project: tests
 * Created Date: 06/12/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */


namespace tests;

public class PhaseTest
{
    [Fact]
    public void PhaseNew()
    {
        for (var i = 0; i <= 0xFF; i++)
        {
            var phase = new Phase((byte)i);
            Assert.Equal(i, phase.Value);
        }
    }

    [Fact]
    public void PhaseFromRad()
    {
        var phase = Phase.FromRad(0.0);
        Assert.Equal(0, phase.Value);
        Assert.Equal(0.0, phase.Radian);

        phase = Phase.FromRad(Math.PI);
        Assert.Equal(128, phase.Value);
        Assert.Equal(Math.PI, phase.Radian);

        phase = Phase.FromRad(2 * Math.PI);
        Assert.Equal(0, phase.Value);
        Assert.Equal(0, phase.Radian);
    }
}