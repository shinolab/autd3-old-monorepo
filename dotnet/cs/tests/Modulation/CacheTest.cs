/*
 * File: CacheTest.cs
 * Project: Modulation
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
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
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWithAsync(Audit.Builder());

        Assert.True(await autd.SendAsync(new Sine(150).WithCache()));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
            var modExpext = new byte[] {
                85,
                107,
                132,
                157,
                183,
                210,
                236,
                245,
                219,
                192,
                166,
                140,
                115,
                92,
                70,
                50,
                33,
                19,
                8,
                2,
                0,
                2,
                8,
                19,
                33,
                50,
                70,
                92,
                115,
                140,
                166,
                192,
                219,
                245,
                236,
                210,
                183,
                157,
                132,
                107,
                85,
                63,
                44,
                28,
                15,
                6,
                0,
                0,
                3,
                11,
                23,
                39,
                57,
                77,
                100,
                123,
                148,
                174,
                201,
                227,
                255,
                227,
                201,
                174,
                148,
                123,
                100,
                77,
                57,
                39,
                23,
                11,
                3,
                0,
                0,
                6,
                15,
                28,
                44,
                63 };
            Assert.Equal(modExpext, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }

    public class ForCacheTest : AUTD3Sharp.Modulation.Modulation
    {
        internal int CalcCnt;

        public ForCacheTest() : base(5120)
        {
            CalcCnt = 0;
        }

        public override double[] Calc()
        {
            CalcCnt++;
            return new double[] { 1, 1 };
        }
    }

    [Fact]
    public async Task CacheCheckOnce()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWithAsync(Audit.Builder());

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
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWithAsync(Audit.Builder());

        var mc = new ForCacheTest().WithCache();
        {
            var mc2 = mc;
            Assert.True(await autd.SendAsync(mc2));
        }

        Assert.True(await autd.SendAsync(mc));
    }
}