/*
 * File: GroupTest.cs
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

public class GroupTest
{
    [Fact]
    public void Group()
    {
        var autd = AUTDTest.CreateController();

        var cx = autd.Geometry.Center.x;

        Assert.True(autd.Send(new Group<string>((_, tr) => tr.Position.x switch
        {
            var x when x < cx => "uniform",
            _ => "null"
        }).Set("uniform", new Uniform(0.5).WithPhase(Math.PI)).Set("null", new Null())));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            foreach (var tr in dev)
            {
                if (tr.Position.x < cx)
                {
                    Assert.Equal(680, duties[tr.LocalIdx]);
                    Assert.Equal(2048, phases[tr.LocalIdx]);
                }
                else
                {
                    Assert.Equal(8, duties[tr.LocalIdx]);
                    Assert.Equal(0, phases[tr.LocalIdx]);
                }
            }
        }
    }
}
