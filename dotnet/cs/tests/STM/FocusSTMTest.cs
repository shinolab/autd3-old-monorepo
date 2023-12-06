/*
 * File: FocusSTMTest.cs
 * Project: STM
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.STM;

namespace tests.STM;

public class FocusSTMTest
{
    [Fact]
    public async Task TestFocusSTM()
    {
        var autd = await AUTDTest.CreateController();

        ; const double radius = 30.0;
        const int size = 2;
        var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
        var stm = new FocusSTM(1)
            .AddFociFromIter(Enumerable.Range(0, size).Select(i => 2 * Math.PI * i / size).Select(theta =>
                (center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0), EmitIntensity.Max)));
        Assert.True(await autd.SendAsync(stm));

        foreach (var dev in autd.Geometry) Assert.False(autd.Link.IsStmGainMode(dev.Idx));

        Assert.Equal(1, stm.Frequency);
        Assert.Equal(TimeSpan.FromMicroseconds(1000000), stm.Period);
        Assert.Equal(2, stm.SamplingConfiguration.Frequency);
        Assert.Equal(10240000u, stm.SamplingConfiguration.FrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(500000), stm.SamplingConfiguration.Period);
        foreach (var dev in autd.Geometry) Assert.Equal(10240000u, autd.Link.StmFrequencyDivision(dev.Idx));

        Assert.Null(stm.StartIdx);
        Assert.Null(stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(-1, autd.Link.StmStartIdx(dev.Idx));
            Assert.Equal(-1, autd.Link.StmFinishIdx(dev.Idx));
        }

        stm = stm.WithStartIdx(0).WithFinishIdx(null);
        Assert.True(await autd.SendAsync(stm));
        Assert.Equal((ushort)0, stm.StartIdx);
        Assert.Null(stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(0, autd.Link.StmStartIdx(dev.Idx));
            Assert.Equal(-1, autd.Link.StmFinishIdx(dev.Idx));
        }

        stm = stm.WithStartIdx(null).WithFinishIdx(0);
        Assert.True(await autd.SendAsync(stm));
        Assert.Null(stm.StartIdx);
        Assert.Equal((ushort)0, stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(-1, autd.Link.StmStartIdx(dev.Idx));
            Assert.Equal(0, autd.Link.StmFinishIdx(dev.Idx));
        }

        stm = FocusSTM.FromPeriod(TimeSpan.FromMicroseconds(1000000))
                   .AddFociFromIter(Enumerable.Range(0, size).Select(i => 2 * Math.PI * i / size).Select(theta =>
                       center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0)));
        Assert.True(await autd.SendAsync(stm));
        Assert.Equal(1, stm.Frequency);
        Assert.Equal(TimeSpan.FromMicroseconds(1000000), stm.Period);
        Assert.Equal(2, stm.SamplingConfiguration.Frequency);
        Assert.Equal(10240000u, stm.SamplingConfiguration.FrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(500000), stm.SamplingConfiguration.Period);
        foreach (var dev in autd.Geometry) Assert.Equal(10240000u, autd.Link.StmFrequencyDivision(dev.Idx));

        stm = FocusSTM.FromSamplingConfig(SamplingConfiguration.FromFrequencyDivision(512)).AddFocus(center).AddFocus(center);
        Assert.True(await autd.SendAsync(stm));
        Assert.Equal(20000.0, stm.Frequency);
        Assert.Equal(2 * 20000.0, stm.SamplingConfiguration.Frequency);
        Assert.Equal(512u, stm.SamplingConfiguration.FrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(25), stm.SamplingConfiguration.Period);
        foreach (var dev in autd.Geometry) Assert.Equal(512u, autd.Link.StmFrequencyDivision(dev.Idx));

        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(2u, autd.Link.StmCycle(dev.Idx));
            {
                var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
                Assert.Contains(intensities, d => d != 0);
                Assert.Contains(phases, p => p != 0);
            }
            {
                var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 1);
                Assert.Contains(intensities, d => d != 0);
                Assert.Contains(phases, p => p != 0);
            }
        }
    }
}