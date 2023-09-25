/*
 * File: AUTDTest.cs
 * Project: tests
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using Xunit;

namespace tests;

public class AUTDTest
{

    public static Controller CreateController()
    {
        return Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity)).OpenWith(new Audit());
    }

    [Fact]
    public void TestSilencer()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
            Assert.Equal(10, Audit.SilencerStep(autd, dev.Idx));

        Assert.True(autd.Send(new Silencer(20)));
        foreach (var dev in autd.Geometry)
            Assert.Equal(20, Audit.SilencerStep(autd, dev.Idx));

        Assert.True(autd.Send(Silencer.Disable()));
        foreach (var dev in autd.Geometry)
            Assert.Equal(0xFFFF, Audit.SilencerStep(autd, dev.Idx));

        Assert.True(autd.Send(new Silencer()));
        foreach (var dev in autd.Geometry)
            Assert.Equal(10, Audit.SilencerStep(autd, dev.Idx));
    }

    [Fact]
    public void TestFPGAInfo()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
            dev.ReadsFPGAInfo = true;

        Assert.True(autd.Send(new UpdateFlags()));

        {
            var infos = autd.FPGAInfo;
            foreach (var info in infos)
            {
                Assert.False(info.IsThermalAssert);
            }
        }

        {
            Audit.AssertThermalSensor(autd, 0);
            Audit.Update(autd, 0);
            Audit.Update(autd, 1);

            var infos = autd.FPGAInfo;
            Assert.True(infos[0].IsThermalAssert);
            Assert.False(infos[1].IsThermalAssert);
        }

        {
            Audit.DeassertThermalSensor(autd, 0);
            Audit.AssertThermalSensor(autd, 1);
            Audit.Update(autd, 0);
            Audit.Update(autd, 1);

            var infos = autd.FPGAInfo;
            Assert.False(infos[0].IsThermalAssert);
            Assert.True(infos[1].IsThermalAssert);
        }

        {
            Audit.BreakDown(autd);
            Assert.Throws<AUTDException>(() => _ = autd.FPGAInfo);
        }
    }

    [Fact]
    public void TestFirmwareInfoList()
    {
        var autd = CreateController();

        Assert.Equal("v3.0.2", FirmwareInfo.LatestVersion);

        {
            var infos = autd.FirmwareInfoList();
            foreach (var (info, i) in infos.Select((info, i) => (info, i)))
            {
                Assert.Equal(info.Info, $"{i}: CPU = v3.0.2, FPGA = v3.0.2  [Emulator]");
            }
        }

        {
            Audit.BreakDown(autd);
            Assert.Throws<AUTDException>(() => _ = autd.FirmwareInfoList().Last());
        }
    }

    [Fact]
    public void TestClose()
    {
        {
            var autd = CreateController();
            Assert.True(Audit.IsOpen(autd));

            autd.Close();
            Assert.False(Audit.IsOpen(autd));
        }

        {
            var autd = CreateController();

            Audit.BreakDown(autd);
            Assert.Throws<AUTDException>(() => autd.Close());
        }
    }

    [Fact]
    public void TestSendTimeout()
    {
        {
            var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
                .OpenWith(new Audit().WithTimeout(TimeSpan.FromMicroseconds(0)));

            autd.Send(new UpdateFlags());
            Assert.Equal(0ul, Audit.LastTimeoutNs(autd));

            autd.Send(new UpdateFlags(), TimeSpan.FromMicroseconds(1));
            Assert.Equal(1000ul, Audit.LastTimeoutNs(autd));

            autd.Send((new UpdateFlags(), new UpdateFlags()), TimeSpan.FromMicroseconds(2));
            Assert.Equal(2000ul, Audit.LastTimeoutNs(autd));

            autd.Send(new Stop(), TimeSpan.FromMicroseconds(3));
            Assert.Equal(3000ul, Audit.LastTimeoutNs(autd));
        }

        {
            var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
                .OpenWith(new Audit().WithTimeout(TimeSpan.FromMicroseconds(10)));

            autd.Send(new UpdateFlags());
            Assert.Equal(10000ul, Audit.LastTimeoutNs(autd));

            autd.Send(new UpdateFlags(), TimeSpan.FromMicroseconds(1));
            Assert.Equal(1000ul, Audit.LastTimeoutNs(autd));

            autd.Send((new UpdateFlags(), new UpdateFlags()), TimeSpan.FromMicroseconds(2));
            Assert.Equal(2000ul, Audit.LastTimeoutNs(autd));

            autd.Send(new Stop(), TimeSpan.FromMicroseconds(3));
            Assert.Equal(3000ul, Audit.LastTimeoutNs(autd));
        }
    }

    [Fact]
    public void TestSendSingle()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
        {
            var m = Audit.Modulation(autd, dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
        }
        Assert.True(autd.Send(new Static()));

        foreach (var dev in autd.Geometry)
        {
            var m = Audit.Modulation(autd, dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
        }

        Audit.Down(autd);
        Assert.False(autd.Send(new Static()));

        Audit.BreakDown(autd);
        Assert.Throws<AUTDException>(() => autd.Send(new Static()));
    }

    [Fact]
    public void TestSendDouble()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
        {
            var m = Audit.Modulation(autd, dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        Assert.True(autd.Send(new Static(), new Uniform(1)));
        foreach (var dev in autd.Geometry)
        {
            var m = Audit.Modulation(autd, dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }

        Audit.Down(autd);
        Assert.False(autd.Send((new Static(), new Uniform(1))));

        Audit.BreakDown(autd);
        Assert.Throws<AUTDException>(() => autd.Send(new Static(), new Uniform(1)));
    }

    [Fact]
    public void TestSendSpecial()
    {
        var autd = CreateController();
        Assert.True(autd.Send(new Uniform(1.0)));

        foreach (var dev in autd.Geometry)
        {
            var (duties, _) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
        }
        Assert.True(autd.Send(new Stop()));

        foreach (var dev in autd.Geometry)
        {
            var (duties, _) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
        }

        Audit.Down(autd);
        Assert.False(autd.Send(new Stop()));

        Audit.BreakDown(autd);
        Assert.Throws<AUTDException>(() => autd.Send(new Stop()));
    }

    [Fact]
    public void TestSoftwareSTM()
    {
        {
            var autd = CreateController();
            var cnt = 0;
            autd.SoftwareSTM((_, _, _) =>
            {
                cnt++;
                return false;
            }).WithTimerStrategy(TimerStrategy.Sleep).Start(TimeSpan.FromMilliseconds(1));
            Assert.Equal(1, cnt);
        }

        {
            var autd = CreateController();
            var cnt = 0;
            autd.SoftwareSTM((_, _, _) =>
            {
                cnt++;
                return false;
            }).WithTimerStrategy(TimerStrategy.BusyWait).Start(TimeSpan.FromMilliseconds(1));
            Assert.Equal(1, cnt);
        }

        {
            var autd = CreateController();
            var cnt = 0;
            autd.SoftwareSTM((_, _, _) =>
            {
                cnt++;
                return false;
            }).WithTimerStrategy(TimerStrategy.NativeTimer).Start(TimeSpan.FromMilliseconds(1));
            Assert.Equal(1, cnt);
        }
    }

    [Fact]
    public void TestGroup()
    {
        var autd = CreateController();

        autd.Group(dev => dev.Idx.ToString())
            .Set("0", (new Static(), new Null()))
            .Set("1", new Sine(150), new Uniform(1))
            .Send();
        {
            var m = Audit.Modulation(autd, 0);
            Assert.Equal(2, m.Length);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (duties, phases) = Audit.DutiesAndPhases(autd, 0, 0);
            Assert.All(duties, d => Assert.Equal(8, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var m = Audit.Modulation(autd, 1);
            Assert.Equal(80, m.Length);
            var (duties, phases) = Audit.DutiesAndPhases(autd, 1, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }


        autd.Group(dev => dev.Idx.ToString())
            .Set("1", new Stop())
            .Set("0", (new Sine(150), new Uniform(1)))
            .Send();
        {
            var m = Audit.Modulation(autd, 0);
            Assert.Equal(80, m.Length);
            var (duties, phases) = Audit.DutiesAndPhases(autd, 0, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var m = Audit.Modulation(autd, 1);
            var (duties, phases) = Audit.DutiesAndPhases(autd, 1, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
        }
    }

    [Fact]
    public void TestAmplitudes()
    {
        var autd = Controller.Builder().AdvancedPhase().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWith(new Audit());

        foreach (var dev in autd.Geometry)
        {
            var m = Audit.Modulation(autd, dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }

        Assert.True(autd.Send(new Uniform(1).WithPhase(Math.PI)));
        foreach (var dev in autd.Geometry)
        {
            var m = Audit.Modulation(autd, dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }

        Assert.True(autd.Send(new Amplitudes(1.0)));
        foreach (var dev in autd.Geometry)
        {
            var m = Audit.Modulation(autd, dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }
    }

    [Fact]
    public void TestClear()
    {
        var autd = CreateController();
        Assert.True(autd.Send(new Uniform(1).WithPhase(Math.PI)));
        foreach (var dev in autd.Geometry)
        {
            var m = Audit.Modulation(autd, dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }

        Assert.True(autd.Send(new Clear()));
        foreach (var dev in autd.Geometry)
        {
            var m = Audit.Modulation(autd, dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
    }

    [Fact]
    public void TestStop()
    {
        var autd = CreateController();
        Assert.True(autd.Send(new Uniform(1).WithPhase(Math.PI)));
        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }

        Assert.True(autd.Send(new Stop()));
        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = Audit.DutiesAndPhases(autd, dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
        }
    }

    [Fact]
    public void TestUpdateFlags()
    {
        var autd = CreateController();
        foreach (var dev in autd.Geometry)
        {
            dev.ForceFan = true;
            Assert.Equal(0 , Audit.FpgaFlags(autd, dev.Idx));
        }

        Assert.True(autd.Send(new UpdateFlags()));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(1, Audit.FpgaFlags(autd, dev.Idx));
        }
    }

    [Fact]
    public void TestSynchronize()
    {
        var autd = Controller.Builder().Advanced().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWith(new Audit());

        foreach (var dev in autd.Geometry)
        {
            Assert.All(Audit.Cycles(autd, dev.Idx), c => Assert.Equal(4096, c));
            foreach (var tr in dev)
                tr.Cycle = 4000;
            Assert.All(Audit.Cycles(autd, dev.Idx), c => Assert.Equal(4096, c));
        }

        Assert.True(autd.Send(new Synchronize()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(Audit.Cycles(autd, dev.Idx), c => Assert.Equal(4000, c));
        }
    }

    [Fact]
    public void TestConfigureModDelay()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.All(Audit.ModDelays(autd, dev.Idx), d => Assert.Equal(0, d));
            foreach (var tr in dev)
                tr.ModDelay = 1;
            Assert.All(Audit.ModDelays(autd, dev.Idx), d => Assert.Equal(0, d));
        }

        Assert.True(autd.Send(new ConfigureModDelay()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(Audit.ModDelays(autd, dev.Idx), d => Assert.Equal(1, d));
        }
    }

    [Fact]
    public void TestConfigureAmpFilter()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.All(Audit.DutyFilters(autd, dev.Idx), d => Assert.Equal(0, d));
            foreach (var tr in dev)
                tr.AmpFilter = -1;
            Assert.All(Audit.DutyFilters(autd, dev.Idx), d => Assert.Equal(0, d));
        }

        Assert.True(autd.Send(new ConfigureAmpFilter()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(Audit.DutyFilters(autd, dev.Idx), d => Assert.Equal(-2048, d));
        }
    }

    [Fact]
    public void TestConfigurePhaseFilter()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.All(Audit.PhaseFilters(autd, dev.Idx), d => Assert.Equal(0, d));
            foreach (var tr in dev)
                tr.PhaseFilter = -Math.PI;
            Assert.All(Audit.PhaseFilters(autd, dev.Idx), d => Assert.Equal(0, d));
        }

        Assert.True(autd.Send(new ConfigurePhaseFilter()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(Audit.PhaseFilters(autd, dev.Idx), d => Assert.Equal(-2048, d));
        }
    }

    [Fact]
    public void TestLegacy()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWith(new Audit());

        Assert.True(autd.Send(new Uniform(1)));

        foreach (var dev in autd.Geometry)
        {
            Assert.True(Audit.IsLegacy(autd, dev.Idx));
        }
    }

    [Fact]
    public void TestAdvanced()
    {
        var autd = Controller.Builder().Advanced().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWith(new Audit());

        Assert.True(autd.Send(new Uniform(1)));

        foreach (var dev in autd.Geometry)
        {
            Assert.False(Audit.IsLegacy(autd, dev.Idx));
        }
    }

    [Fact]
    public void TestAdvancedPhase()
    {
        var autd = Controller.Builder().AdvancedPhase().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWith(new Audit());

        Assert.True(autd.Send(new Uniform(1)));

        foreach (var dev in autd.Geometry)
        {
            Assert.False(Audit.IsLegacy(autd, dev.Idx));
        }
    }
}
