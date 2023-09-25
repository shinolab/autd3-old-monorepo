/*
 * File: TransformTest.cs
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

public class TransformTest
{
    [Fact]
    public void Transform()
    {
        var autd = AUTDTest.CreateController();

        Assert.True(autd.Send(new Uniform(0.5).WithPhase(Math.PI).WithTransform((dev, _, d) =>
            dev.Idx == 0 ? d with { Phase = d.Phase + Math.PI / 4 } : d with { Phase = d.Phase - Math.PI / 4 })));
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, 0, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.All(phases, p => Assert.Equal(2048 + 512, p));
        }

        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, 1, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.All(phases, p => Assert.Equal(2048 - 512, p));
        }
    }
}