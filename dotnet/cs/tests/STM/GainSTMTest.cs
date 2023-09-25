/*
 * File: GainSTMTest.cs
 * Project: STM
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.STM;

namespace tests.STM;

public class GainSTMTest
{
    [Fact]
    public void TestGainSTMLegacy()
    {
        var autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(new Audit());

        const double radius = 30.0;
        const int size = 2;
        var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
        var stm = new GainSTM(1)
            .AddGainsFromIter(Enumerable.Range(0, size).Select(i => 2 * Math.PI * i / size).Select(theta =>
                new Focus(center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0))));
        Assert.True(autd.Send(stm));

        foreach (var dev in autd.Geometry)
        {
            Assert.True(Audit.IsStmGainMode(autd, dev.Idx));
        }

        Assert.Equal(1, stm.Frequency);
        Assert.Equal(2, stm.SamplingFrequency);
        Assert.Equal(10240000u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(500000), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(81920000u, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        Assert.Null(stm.StartIdx);
        Assert.Null(stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(-1, Audit.StmStartIdx(autd, dev.Idx));
            Assert.Equal(-1, Audit.StmFinishIdx(autd, dev.Idx));
        }

        stm = stm.WithStartIdx(0);
        Assert.True(autd.Send(stm));
        Assert.Equal((ushort)0, stm.StartIdx);
        Assert.Null(stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(0, Audit.StmStartIdx(autd, dev.Idx));
            Assert.Equal(-1, Audit.StmFinishIdx(autd, dev.Idx));
        }

        stm = stm.WithStartIdx(null).WithFinishIdx(0);
        Assert.True(autd.Send(stm));
        Assert.Null(stm.StartIdx);
        Assert.Equal((ushort)0, stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(-1, Audit.StmStartIdx(autd, dev.Idx));
            Assert.Equal(0, Audit.StmFinishIdx(autd, dev.Idx));
        }

        stm = GainSTM.WithSamplingFrequencyDivision(512).AddGain(new Uniform(1)).AddGain(new Uniform(0.5));
        Assert.True(autd.Send(stm));
        Assert.Equal(20000.0, stm.Frequency);
        Assert.Equal(2 * 20000.0, stm.SamplingFrequency);
        Assert.Equal(512u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(25), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        stm = GainSTM.WithSamplingFrequency(20e3).AddGain(new Uniform(1)).AddGain(new Uniform(0.5));
        Assert.True(autd.Send(stm));
        Assert.Equal(10000, stm.Frequency);
        Assert.Equal(2 * 10000, stm.SamplingFrequency);
        Assert.Equal(1024u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(50), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u * 2, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        stm = GainSTM.WithSamplingPeriod(TimeSpan.FromMicroseconds(25)).AddGain(new Uniform(1)).AddGain(new Uniform(0.5));
        Assert.True(autd.Send(stm));
        Assert.Equal(20000.0, stm.Frequency);
        Assert.Equal(2 * 20000.0, stm.SamplingFrequency);
        Assert.Equal(512u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(25), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(2u, Audit.StmCycle(autd, dev.Idx));
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 1);
                Assert.All(duties, d => Assert.Equal(680, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
        }

        stm = stm.WithMode(GainSTMMode.PhaseFull);
        Assert.True(autd.Send(stm));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(2u, Audit.StmCycle(autd, dev.Idx));
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 1);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
        }

        stm = stm.WithMode(GainSTMMode.PhaseHalf);
        Assert.True(autd.Send(stm));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(2u, Audit.StmCycle(autd, dev.Idx));
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 1);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
        }
    }

    [Fact]
    public void TestGainSTMAdvanced()
    {
        var autd = Controller.Builder()
            .Advanced()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(new Audit());

        const double radius = 30.0;
        const int size = 2;
        var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
        var stm = new GainSTM(1)
            .AddGainsFromIter(Enumerable.Range(0, size).Select(i => 2 * Math.PI * i / size).Select(theta =>
                new Focus(center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0))));
        Assert.True(autd.Send(stm));

        foreach (var dev in autd.Geometry)
        {
            Assert.True(Audit.IsStmGainMode(autd, dev.Idx));
        }

        Assert.Equal(1, stm.Frequency);
        Assert.Equal(2, stm.SamplingFrequency);
        Assert.Equal(10240000u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(500000), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(81920000u, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        Assert.Null(stm.StartIdx);
        Assert.Null(stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(-1, Audit.StmStartIdx(autd, dev.Idx));
            Assert.Equal(-1, Audit.StmFinishIdx(autd, dev.Idx));
        }

        stm = stm.WithStartIdx(0);
        Assert.True(autd.Send(stm));
        Assert.Equal((ushort)0, stm.StartIdx);
        Assert.Null(stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(0, Audit.StmStartIdx(autd, dev.Idx));
            Assert.Equal(-1, Audit.StmFinishIdx(autd, dev.Idx));
        }

        stm = stm.WithStartIdx(null).WithFinishIdx(0);
        Assert.True(autd.Send(stm));
        Assert.Null(stm.StartIdx);
        Assert.Equal((ushort)0, stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(-1, Audit.StmStartIdx(autd, dev.Idx));
            Assert.Equal(0, Audit.StmFinishIdx(autd, dev.Idx));
        }

        stm = GainSTM.WithSamplingFrequencyDivision(512).AddGain(new Uniform(1)).AddGain(new Uniform(0.5));
        Assert.True(autd.Send(stm));
        Assert.Equal(20000.0, stm.Frequency);
        Assert.Equal(2 * 20000.0, stm.SamplingFrequency);
        Assert.Equal(512u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(25), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        stm = GainSTM.WithSamplingFrequency(20e3).AddGain(new Uniform(1)).AddGain(new Uniform(0.5));
        Assert.True(autd.Send(stm));
        Assert.Equal(10000, stm.Frequency);
        Assert.Equal(2 * 10000, stm.SamplingFrequency);
        Assert.Equal(1024u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(50), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u * 2, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        stm = GainSTM.WithSamplingPeriod(TimeSpan.FromMicroseconds(25)).AddGain(new Uniform(1)).AddGain(new Uniform(0.5));
        Assert.True(autd.Send(stm));
        Assert.Equal(20000.0, stm.Frequency);
        Assert.Equal(2 * 20000.0, stm.SamplingFrequency);
        Assert.Equal(512u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(25), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(2u, Audit.StmCycle(autd, dev.Idx));
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 1);
                Assert.All(duties, d => Assert.Equal(683, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
        }

        stm = stm.WithMode(GainSTMMode.PhaseFull);
        Assert.True(autd.Send(stm));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(2u, Audit.StmCycle(autd, dev.Idx));
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 1);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
        }

        stm = stm.WithMode(GainSTMMode.PhaseHalf);
        Assert.Throws<AUTDException>(() => autd.Send(stm));
    }

    [Fact]
    public void TestGainSTMAdvancedPhase()
    {
        var autd = Controller.Builder()
            .AdvancedPhase()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(new Audit());

        const double radius = 30.0;
        const int size = 2;
        var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
        var stm = new GainSTM(1)
            .AddGainsFromIter(Enumerable.Range(0, size).Select(i => 2 * Math.PI * i / size).Select(theta =>
                new Focus(center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0))));
        Assert.True(autd.Send(stm));

        foreach (var dev in autd.Geometry)
        {
            Assert.True(Audit.IsStmGainMode(autd, dev.Idx));
        }

        Assert.Equal(1, stm.Frequency);
        Assert.Equal(2, stm.SamplingFrequency);
        Assert.Equal(10240000u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(500000), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(81920000u, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        Assert.Null(stm.StartIdx);
        Assert.Null(stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(-1, Audit.StmStartIdx(autd, dev.Idx));
            Assert.Equal(-1, Audit.StmFinishIdx(autd, dev.Idx));
        }

        stm = stm.WithStartIdx(0);
        Assert.True(autd.Send(stm));
        Assert.Equal((ushort)0, stm.StartIdx);
        Assert.Null(stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(0, Audit.StmStartIdx(autd, dev.Idx));
            Assert.Equal(-1, Audit.StmFinishIdx(autd, dev.Idx));
        }

        stm = stm.WithStartIdx(null).WithFinishIdx(0);
        Assert.True(autd.Send(stm));
        Assert.Null(stm.StartIdx);
        Assert.Equal((ushort)0, stm.FinishIdx);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(-1, Audit.StmStartIdx(autd, dev.Idx));
            Assert.Equal(0, Audit.StmFinishIdx(autd, dev.Idx));
        }

        stm = GainSTM.WithSamplingFrequencyDivision(512).AddGain(new Uniform(1)).AddGain(new Uniform(0.5));
        Assert.True(autd.Send(stm));
        Assert.Equal(20000.0, stm.Frequency);
        Assert.Equal(2 * 20000.0, stm.SamplingFrequency);
        Assert.Equal(512u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(25), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        stm = GainSTM.WithSamplingFrequency(20e3).AddGain(new Uniform(1)).AddGain(new Uniform(0.5));
        Assert.True(autd.Send(stm));
        Assert.Equal(10000, stm.Frequency);
        Assert.Equal(2 * 10000, stm.SamplingFrequency);
        Assert.Equal(1024u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(50), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u * 2, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        stm = GainSTM.WithSamplingPeriod(TimeSpan.FromMicroseconds(25)).AddGain(new Uniform(1)).AddGain(new Uniform(0.5));
        Assert.True(autd.Send(stm));
        Assert.Equal(20000.0, stm.Frequency);
        Assert.Equal(2 * 20000.0, stm.SamplingFrequency);
        Assert.Equal(512u, stm.SamplingFrequencyDivision);
        Assert.Equal(TimeSpan.FromMicroseconds(25), stm.SamplingPeriod);
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u, Audit.StmFrequencyDivision(autd, dev.Idx));
        }

        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(2u, Audit.StmCycle(autd, dev.Idx));
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 1);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
        }

        stm = stm.WithMode(GainSTMMode.PhaseFull);
        Assert.True(autd.Send(stm));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(2u, Audit.StmCycle(autd, dev.Idx));
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
            {
                var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 1);
                Assert.All(duties, d => Assert.Equal(2048, d));
                Assert.All(phases, p => Assert.Equal(0, p));
            }
        }

        stm = stm.WithMode(GainSTMMode.PhaseHalf);
        Assert.Throws<AUTDException>(() => autd.Send(stm));
    }
}