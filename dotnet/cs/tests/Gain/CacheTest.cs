/*
 * File: CacheTest.cs
 * Project: Gain
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Gain;

public class CacheTest
{
    [Fact]
    public async Task Cache()
    {
        var autd = await AUTDTest.CreateController();

        Assert.True(await autd.SendAsync(new Uniform(0.5).WithPhase(Math.PI).WithCache()));

        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = autd.Link.DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(85, d));
            Assert.All(phases, p => Assert.Equal(256, p));
        }
    }

    public class ForCacheTest : AUTD3Sharp.Gain.Gain
    {
        internal int CalcCnt;

        public override Dictionary<int, Drive[]> Calc(Geometry geometry)
        {
            CalcCnt++;
            return Transform(geometry, (_, _) => new Drive { Phase = Math.PI, Amp = EmitIntensity.NewNormalized(0.5).PulseWidth });
        }
    }

    [Fact]
    public async Task CacheCheckOnce()
    {
        var autd = await AUTDTest.CreateController();
        {
            var g = new ForCacheTest();
            Assert.True(await autd.SendAsync(g));
            Assert.Equal(1, g.CalcCnt);
            Assert.True(await autd.SendAsync(g));
            Assert.Equal(2, g.CalcCnt);
        }

        {
            var g = new ForCacheTest();
            var gc = g.WithCache();
            Assert.True(await autd.SendAsync(gc));
            Assert.Equal(1, g.CalcCnt);
            Assert.True(await autd.SendAsync(gc));
            Assert.Equal(1, g.CalcCnt);
        }
    }

    [Fact]
    public async Task CacheCheckOnlyForEnabled()
    {
        var autd = await AUTDTest.CreateController();
        autd.Geometry[0].Enable = false;

        var g = new ForCacheTest();
        var gc = g.WithCache();
        Assert.True(await autd.SendAsync(gc));

        Assert.False(gc.Drives().ContainsKey(0));
        Assert.True(gc.Drives().ContainsKey(1));

        {
            var (duties, phases) = autd.Link.DutiesAndPhases(0, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (duties, phases) = autd.Link.DutiesAndPhases(1, 0);
            Assert.All(duties, d => Assert.Equal(85, d));
            Assert.All(phases, p => Assert.Equal(256, p));
        }
    }
}