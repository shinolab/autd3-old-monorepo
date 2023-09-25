/*
 * File: CacheTest.cs
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

public class CacheTest
{
    [Fact]
    public void Cache()
    {
        var autd = AUTDTest.CreateController();

        Assert.True(autd.Send(new Uniform(0.5).WithPhase(Math.PI).WithCache()));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }
    }

    public class ForCacheTest : AUTD3Sharp.Gain.Gain
    {
        internal int CalcCnt;

        public override Dictionary<int, Drive[]> Calc(Geometry geometry)
        {
            CalcCnt++;
            return Transform(geometry, (_, _) => new Drive { Phase = 0, Amp = 1 });
        }
    }

    [Fact]
    public void CacheCheckOnce()
    {
        var autd = AUTDTest.CreateController();
        {
            var g = new ForCacheTest();
            Assert.True(autd.Send(g));
            Assert.Equal(1, g.CalcCnt);
            Assert.True(autd.Send(g));
            Assert.Equal(2, g.CalcCnt);
        }

        {
            var g = new ForCacheTest();
            var gc = g.WithCache();
            Assert.True(autd.Send(gc));
            Assert.Equal(1, g.CalcCnt);
            Assert.True(autd.Send(gc));
            Assert.Equal(1, g.CalcCnt);
        }
    }
}