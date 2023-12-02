/*
 * File: CacheTest.cs
 * Project: Gain
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
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

        Assert.True(await autd.SendAsync(new Uniform(new EmitIntensity(0x80)).WithPhase(new Phase(0x90)).WithCache()));

        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.All(phases, p => Assert.Equal(0x90, p));
        }
    }

    public class ForCacheTest : AUTD3Sharp.Gain.Gain
    {
        internal int CalcCnt;

        public override Dictionary<int, Drive[]> Calc(Geometry geometry)
        {
            CalcCnt++;
            return Transform(geometry, (_, _) => new Drive { Phase = new Phase(0x90), Intensity = new EmitIntensity(0x80) });
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
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(0, 0);
            Assert.All(intensities, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(1, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.All(phases, p => Assert.Equal(0x90, p));
        }
    }
}