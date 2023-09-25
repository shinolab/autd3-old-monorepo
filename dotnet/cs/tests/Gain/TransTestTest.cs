/*
 * File: TransTestTest.cs
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

public class TransTestTest
{
    [Fact]
    public void TransTest()
    {
        var autd = AUTDTest.CreateController();

        Assert.True(autd.Send(new TransducerTest().Set(0, 0, Math.PI, 0.5).Set(1, 248, Math.PI, 0.5)));

        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, 0, 0);
            Assert.Equal(680, duties[0]);
            Assert.Equal(2048, phases[0]);
            Assert.All(duties.Skip(1), d => Assert.Equal(8, d));
            Assert.All(phases.Skip(1), p => Assert.Equal(0, p));
        }

        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, 1, 0);
            Assert.Equal(680, duties[autd.Geometry[1].NumTransducers - 1]);
            Assert.Equal(2048, phases[autd.Geometry[1].NumTransducers - 1]);
            Assert.All(duties.Take(autd.Geometry[1].NumTransducers - 1), d => Assert.Equal(8, d));
            Assert.All(phases.Take(autd.Geometry[1].NumTransducers - 1), p => Assert.Equal(0, p));
        }
    }
}