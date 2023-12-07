/*
 * File: CacheTest.cs
 * Project: Modulation
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Modulation;

public class CacheTest
{
    [Fact]
    public async Task Cache()
    {
        var autd1 = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());
        var autd2 = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        Assert.True(await autd1.SendAsync(new Sine(150)));
        Assert.True(await autd2.SendAsync(new Sine(150).WithCache()));
        foreach (var dev in autd2.Geometry)
        {
            var modExpect = autd1.Link.Modulation(dev.Idx);
            var mod = autd2.Link.Modulation(dev.Idx);
            Assert.Equal(modExpect, mod);
            Assert.Equal(autd1.Link.ModulationFrequencyDivision(dev.Idx), autd2.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }

    [Fact]
    public async Task CacheBuffer()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        using var m = new Static().WithCache();
        Assert.Equal(new EmitIntensity(0xFF), m[0]);
        Assert.Equal(new EmitIntensity(0xFF), m[1]);
        var buffer = m.Buffer;
        Assert.Equal(new EmitIntensity(0xFF), buffer[0]);
        Assert.Equal(new EmitIntensity(0xFF), buffer[1]);

        foreach (var buf in m)
        {
            Assert.Equal(new EmitIntensity(0xFF), buf);
        }
    }


    public class ForCacheTest : AUTD3Sharp.Modulation.Modulation
    {
        internal int CalcCnt;

        public ForCacheTest() : base(SamplingConfiguration.FromFrequencyDivision(5120))
        {
            CalcCnt = 0;
        }

        public override EmitIntensity[] Calc()
        {
            CalcCnt++;
            return new[] { EmitIntensity.Max, EmitIntensity.Max };
        }
    }

    [Fact]
    public async Task CacheCheckOnce()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        {
            var m = new ForCacheTest();
            Assert.True(await autd.SendAsync(m));
            Assert.Equal(1, m.CalcCnt);
            Assert.True(await autd.SendAsync(m));
            Assert.Equal(2, m.CalcCnt);
        }

        {
            var m = new ForCacheTest();
            var mc = m.WithCache();
            Assert.True(await autd.SendAsync(mc));
            Assert.Equal(1, m.CalcCnt);
            Assert.True(await autd.SendAsync(mc));
            Assert.Equal(1, m.CalcCnt);
        }
    }



    [Fact]
    public async Task CacheCheckFree()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        var mc = new ForCacheTest().WithCache();
        {
            var mc2 = mc;
            Assert.True(await autd.SendAsync(mc2));
        }

        Assert.True(await autd.SendAsync(mc));
    }
}