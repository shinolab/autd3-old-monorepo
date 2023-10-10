/*
 * File: AUTDTest.cs
 * Project: tests
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests;

public class AUTDTest
{

    public static Controller CreateController()
    {
        return Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity)).OpenWith(Audit.Builder());
    }

    [Fact]
    public void TestSilencer()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
            Assert.Equal(10, autd.Link<Audit>().SilencerStep(dev.Idx));

        Assert.True(autd.Send(new Silencer(20)));
        foreach (var dev in autd.Geometry)
            Assert.Equal(20, autd.Link<Audit>().SilencerStep(dev.Idx));

        Assert.True(autd.Send(Silencer.Disable()));
        foreach (var dev in autd.Geometry)
            Assert.Equal(0xFFFF, autd.Link<Audit>().SilencerStep(dev.Idx));

        Assert.True(autd.Send(new Silencer()));
        foreach (var dev in autd.Geometry)
            Assert.Equal(10, autd.Link<Audit>().SilencerStep(dev.Idx));
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
            autd.Link<Audit>().AssertThermalSensor(0);
            autd.Link<Audit>().Update(0);
            autd.Link<Audit>().Update(1);

            var infos = autd.FPGAInfo;
            Assert.True(infos[0].IsThermalAssert);
            Assert.False(infos[1].IsThermalAssert);
        }

        {
            autd.Link<Audit>().DeassertThermalSensor(0);
            autd.Link<Audit>().AssertThermalSensor(1);
            autd.Link<Audit>().Update(0);
            autd.Link<Audit>().Update(1);

            var infos = autd.FPGAInfo;
            Assert.False(infos[0].IsThermalAssert);
            Assert.True(infos[1].IsThermalAssert);
        }

        {
            autd.Link<Audit>().BreakDown();
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
                Assert.Equal(info.Info, $"{i}: CPU = v3.0.2, FPGA = v3.0.2 [Emulator]");
            }
        }

        {
            autd.Link<Audit>().BreakDown();
            Assert.Throws<AUTDException>(() => _ = autd.FirmwareInfoList().Last());
        }
    }

    [Fact]
    public void TestClose()
    {
        {
            var autd = CreateController();
            Assert.True(autd.Link<Audit>().IsOpen());

            autd.Close();
            Assert.False(autd.Link<Audit>().IsOpen());
        }

        {
            var autd = CreateController();

            autd.Link<Audit>().BreakDown();
            Assert.Throws<AUTDException>(() => autd.Close());
        }
    }

    [Fact]
    public void TestSendTimeout()
    {
        {
            var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
                .OpenWith(Audit.Builder().WithTimeout(TimeSpan.FromMicroseconds(0)));

            autd.Send(new UpdateFlags());
            Assert.Equal(0ul, autd.Link<Audit>().LastTimeoutNs());

            autd.Send(new UpdateFlags(), TimeSpan.FromMicroseconds(1));
            Assert.Equal(1000ul, autd.Link<Audit>().LastTimeoutNs());

            autd.Send((new UpdateFlags(), new UpdateFlags()), TimeSpan.FromMicroseconds(2));
            Assert.Equal(2000ul, autd.Link<Audit>().LastTimeoutNs());

            autd.Send(new Stop(), TimeSpan.FromMicroseconds(3));
            Assert.Equal(3000ul, autd.Link<Audit>().LastTimeoutNs());
        }

        {
            var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
                .OpenWith(Audit.Builder().WithTimeout(TimeSpan.FromMicroseconds(10)));

            autd.Send(new UpdateFlags());
            Assert.Equal(10000ul, autd.Link<Audit>().LastTimeoutNs());

            autd.Send(new UpdateFlags(), TimeSpan.FromMicroseconds(1));
            Assert.Equal(1000ul, autd.Link<Audit>().LastTimeoutNs());

            autd.Send((new UpdateFlags(), new UpdateFlags()), TimeSpan.FromMicroseconds(2));
            Assert.Equal(2000ul, autd.Link<Audit>().LastTimeoutNs());

            autd.Send(new Stop(), TimeSpan.FromMicroseconds(3));
            Assert.Equal(3000ul, autd.Link<Audit>().LastTimeoutNs());
        }
    }

    [Fact]
    public void TestSendSingle()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
        }
        Assert.True(autd.Send(new Static()));

        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
        }

        autd.Link<Audit>().Down();
        Assert.False(autd.Send(new Static()));

        autd.Link<Audit>().BreakDown();
        Assert.Throws<AUTDException>(() => autd.Send(new Static()));
    }

    [Fact]
    public void TestSendDouble()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        Assert.True(autd.Send(new Static(), new Uniform(1)));
        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }

        autd.Link<Audit>().Down();
        Assert.False(autd.Send((new Static(), new Uniform(1))));

        autd.Link<Audit>().BreakDown();
        Assert.Throws<AUTDException>(() => autd.Send(new Static(), new Uniform(1)));
    }

    [Fact]
    public void TestSendSpecial()
    {
        var autd = CreateController();
        Assert.True(autd.Send(new Uniform(1.0)));

        foreach (var dev in autd.Geometry)
        {
            var (duties, _) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
        }
        Assert.True(autd.Send(new Stop()));

        foreach (var dev in autd.Geometry)
        {
            var (duties, _) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
        }

        autd.Link<Audit>().Down();
        Assert.False(autd.Send(new Stop()));

        autd.Link<Audit>().BreakDown();
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
            var m = autd.Link<Audit>().Modulation(0);
            Assert.Equal(2, m.Length);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(0, 0);
            Assert.All(duties, d => Assert.Equal(8, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var m = autd.Link<Audit>().Modulation(1);
            Assert.Equal(80, m.Length);
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(1, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }


        autd.Group(dev => dev.Idx.ToString())
            .Set("1", new Stop())
            .Set("0", (new Sine(150), new Uniform(1)))
            .Send();
        {
            var m = autd.Link<Audit>().Modulation(0);
            Assert.Equal(80, m.Length);
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(0, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (duties, _) = autd.Link<Audit>().DutiesAndPhases(1, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
        }
    }

    [Fact]
    public void TestGroupCheckOnlyForEnabled()
    {
        var autd = CreateController();
        autd.Geometry[0].Enable = false;

        var check = new bool[autd.Geometry.NumDevices];
        autd.Group(dev =>
        {
            check[dev.Idx] = true;
            return "0";
        })
                 .Set("0", new Sine(150), new Uniform(0.5).WithPhase(Math.PI))
                 .Send();

        Assert.False(check[0]);
        Assert.True(check[1]);

        {
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(0, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(1, 0);
            Assert.All(duties, d => Assert.Equal(680, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }
    }

    [Fact]
    public void TestAmplitudes()
    {
        var autd = Controller.Builder().AdvancedPhase().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWith(Audit.Builder());

        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }

        Assert.True(autd.Send(new Uniform(1).WithPhase(Math.PI)));
        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }

        Assert.True(autd.Send(new Amplitudes()));
        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
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
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }

        Assert.True(autd.Send(new Clear()));
        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
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
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(2048, d));
            Assert.All(phases, p => Assert.Equal(2048, p));
        }

        Assert.True(autd.Send(new Stop()));
        foreach (var dev in autd.Geometry)
        {
            var (duties, _) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
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
            Assert.Equal(0, autd.Link<Audit>().FpgaFlags(dev.Idx));
        }

        Assert.True(autd.Send(new UpdateFlags()));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(1, autd.Link<Audit>().FpgaFlags(dev.Idx));
        }
    }

    [Fact]
    public void TestSynchronize()
    {
        var autd = Controller.Builder().Advanced().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWith(Audit.Builder());

        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().Cycles(dev.Idx), c => Assert.Equal(4096, c));
            foreach (var tr in dev)
                tr.Cycle = 4000;
            Assert.All(autd.Link<Audit>().Cycles(dev.Idx), c => Assert.Equal(4096, c));
        }

        Assert.True(autd.Send(new Synchronize()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().Cycles(dev.Idx), c => Assert.Equal(4000, c));
        }
    }

    [Fact]
    public void TestConfigureModDelay()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().ModDelays(dev.Idx), d => Assert.Equal(0, d));
            foreach (var tr in dev)
                tr.ModDelay = 1;
            Assert.All(autd.Link<Audit>().ModDelays(dev.Idx), d => Assert.Equal(0, d));
        }

        Assert.True(autd.Send(new ConfigureModDelay()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().ModDelays(dev.Idx), d => Assert.Equal(1, d));
        }
    }

    [Fact]
    public void TestConfigureAmpFilter()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().DutyFilters(dev.Idx), d => Assert.Equal(0, d));
            foreach (var tr in dev)
                tr.AmpFilter = -1;
            Assert.All(autd.Link<Audit>().DutyFilters(dev.Idx), d => Assert.Equal(0, d));
        }

        Assert.True(autd.Send(new ConfigureAmpFilter()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().DutyFilters(dev.Idx), d => Assert.Equal(-2048, d));
        }
    }

    [Fact]
    public void TestConfigurePhaseFilter()
    {
        var autd = CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().PhaseFilters(dev.Idx), d => Assert.Equal(0, d));
            foreach (var tr in dev)
                tr.PhaseFilter = -Math.PI;
            Assert.All(autd.Link<Audit>().PhaseFilters(dev.Idx), d => Assert.Equal(0, d));
        }

        Assert.True(autd.Send(new ConfigurePhaseFilter()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().PhaseFilters(dev.Idx), d => Assert.Equal(-2048, d));
        }
    }

    [Fact]
    public void TestLegacy()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWith(Audit.Builder());

        Assert.True(autd.Send(new Uniform(1)));

        foreach (var dev in autd.Geometry)
        {
            Assert.True(autd.Link<Audit>().IsLegacy(dev.Idx));
        }
    }

    [Fact]
    public void TestAdvanced()
    {
        var autd = Controller.Builder().Advanced().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWith(Audit.Builder());

        Assert.True(autd.Send(new Uniform(1)));

        foreach (var dev in autd.Geometry)
        {
            Assert.False(autd.Link<Audit>().IsLegacy(dev.Idx));
        }
    }

    [Fact]
    public void TestAdvancedPhase()
    {
        var autd = Controller.Builder().AdvancedPhase().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWith(Audit.Builder());

        Assert.True(autd.Send(new Uniform(1)));

        foreach (var dev in autd.Geometry)
        {
            Assert.False(autd.Link<Audit>().IsLegacy(dev.Idx));
        }
    }
}
