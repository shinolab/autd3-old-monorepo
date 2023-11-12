/*
 * File: AUTDTest.cs
 * Project: tests
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests;

public class AUTDTest
{

    public static async Task<Controller> CreateController()
    {
        return await Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity)).OpenWithAsync(Audit.Builder());
    }

    [Fact]
    public async Task TestSilencer()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
            Assert.Equal(10, autd.Link<Audit>().SilencerStep(dev.Idx));

        Assert.True(await autd.SendAsync(new Silencer(20)));
        foreach (var dev in autd.Geometry)
            Assert.Equal(20, autd.Link<Audit>().SilencerStep(dev.Idx));

        Assert.True(await autd.SendAsync(Silencer.Disable()));
        foreach (var dev in autd.Geometry)
            Assert.Equal(0xFFFF, autd.Link<Audit>().SilencerStep(dev.Idx));

        Assert.True(await autd.SendAsync(new Silencer()));
        foreach (var dev in autd.Geometry)
            Assert.Equal(10, autd.Link<Audit>().SilencerStep(dev.Idx));
    }

    [Fact]
    public async Task TestFPGAInfo()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
            dev.ReadsFPGAInfo = true;

        Assert.True(await autd.SendAsync(new UpdateFlags()));

        {
            var infos = await autd.FPGAInfoAsync();
            foreach (var info in infos)
            {
                Assert.False(info.IsThermalAssert);
            }
        }

        {
            autd.Link<Audit>().AssertThermalSensor(0);
            autd.Link<Audit>().Update(0);
            autd.Link<Audit>().Update(1);

            var infos = await autd.FPGAInfoAsync();
            Assert.True(infos[0].IsThermalAssert);
            Assert.False(infos[1].IsThermalAssert);
        }

        {
            autd.Link<Audit>().DeassertThermalSensor(0);
            autd.Link<Audit>().AssertThermalSensor(1);
            autd.Link<Audit>().Update(0);
            autd.Link<Audit>().Update(1);

            var infos = await autd.FPGAInfoAsync();
            Assert.False(infos[0].IsThermalAssert);
            Assert.True(infos[1].IsThermalAssert);
        }

        {
            autd.Link<Audit>().BreakDown();
            await Assert.ThrowsAsync<AUTDException>(async () => _ = await autd.FPGAInfoAsync());
        }
    }

    [Fact]
    public async Task TestFirmwareInfoList()
    {
        var autd = await CreateController();

        Assert.Equal("v4.0.0", FirmwareInfo.LatestVersion);

        {
            foreach (var (info, i) in (await autd.FirmwareInfoListAsync()).Select((info, i) => (info, i)))
            {
                Assert.Equal(info.Info, $"{i}: CPU = v4.0.0, FPGA = v4.0.0 [Emulator]");
            }
        }

        {
            autd.Link<Audit>().BreakDown();
            await Assert.ThrowsAsync<AUTDException>(async () => _ = (await autd.FirmwareInfoListAsync()).Last());
        }
    }

    [Fact]
    public async Task TestClose()
    {
        {
            var autd = await CreateController();
            Assert.True(autd.Link<Audit>().IsOpen());

            await autd.CloseAsync();
            Assert.False(autd.Link<Audit>().IsOpen());
        }

        {
            var autd = await CreateController();

            autd.Link<Audit>().BreakDown();
            await Assert.ThrowsAsync<AUTDException>(async () => await autd.CloseAsync());
        }
    }

    [Fact]
    public async Task TestSendTimeout()
    {
        {
            var autd = await Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
                .OpenWithAsync(Audit.Builder().WithTimeout(TimeSpan.FromMicroseconds(0)));

            await autd.SendAsync(new UpdateFlags());
            Assert.Equal(0ul, autd.Link<Audit>().LastTimeoutNs());

            await autd.SendAsync(new UpdateFlags(), TimeSpan.FromMicroseconds(1));
            Assert.Equal(1000ul, autd.Link<Audit>().LastTimeoutNs());

            await autd.SendAsync((new UpdateFlags(), new UpdateFlags()), TimeSpan.FromMicroseconds(2));
            Assert.Equal(2000ul, autd.Link<Audit>().LastTimeoutNs());

            await autd.SendAsync(new Stop(), TimeSpan.FromMicroseconds(3));
            Assert.Equal(3000ul, autd.Link<Audit>().LastTimeoutNs());
        }

        {
            var autd = await Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
                .OpenWithAsync(Audit.Builder().WithTimeout(TimeSpan.FromMicroseconds(10)));

            await autd.SendAsync(new UpdateFlags());
            Assert.Equal(10000ul, autd.Link<Audit>().LastTimeoutNs());

            await autd.SendAsync(new UpdateFlags(), TimeSpan.FromMicroseconds(1));
            Assert.Equal(1000ul, autd.Link<Audit>().LastTimeoutNs());

            await autd.SendAsync((new UpdateFlags(), new UpdateFlags()), TimeSpan.FromMicroseconds(2));
            Assert.Equal(2000ul, autd.Link<Audit>().LastTimeoutNs());

            await autd.SendAsync(new Stop(), TimeSpan.FromMicroseconds(3));
            Assert.Equal(3000ul, autd.Link<Audit>().LastTimeoutNs());
        }
    }

    [Fact]
    public async Task TestSendSingle()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
        }
        Assert.True(await autd.SendAsync(new Static()));

        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
        }

        autd.Link<Audit>().Down();
        Assert.False(await autd.SendAsync(new Static()));

        autd.Link<Audit>().BreakDown();
        await Assert.ThrowsAsync<AUTDException>(async () => await autd.SendAsync(new Static()));
    }

    [Fact]
    public async Task TestSendDouble()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        Assert.True(await autd.SendAsync(new Static(), new Uniform(1.0)));
        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(256, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }

        autd.Link<Audit>().Down();
        Assert.False(await autd.SendAsync((new Static(), new Uniform(1.0))));

        autd.Link<Audit>().BreakDown();
        await Assert.ThrowsAsync<AUTDException>(async () => await autd.SendAsync(new Static(), new Uniform(1.0)));
    }

    [Fact]
    public async Task TestSendSpecial()
    {
        var autd = await CreateController();
        Assert.True(await autd.SendAsync(new Uniform(1.0)));

        foreach (var dev in autd.Geometry)
        {
            var (duties, _) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(256, d));
        }
        Assert.True(await autd.SendAsync(new Stop()));

        foreach (var dev in autd.Geometry)
        {
            var (duties, _) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
        }

        autd.Link<Audit>().Down();
        Assert.False(await autd.SendAsync(new Stop()));

        autd.Link<Audit>().BreakDown();
        await Assert.ThrowsAsync<AUTDException>(async () => await autd.SendAsync(new Stop()));
    }

    [Fact]
    public async Task TestGroup()
    {
        var autd = await CreateController();

        await autd.Group(dev => dev.Idx.ToString())
             .Set("0", (new Static(), new Null()))
             .Set("1", new Sine(150), new Uniform(1.0))
             .SendAsync();
        {
            var m = autd.Link<Audit>().Modulation(0);
            Assert.Equal(2, m.Length);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(0, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var m = autd.Link<Audit>().Modulation(1);
            Assert.Equal(80, m.Length);
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(1, 0);
            Assert.All(duties, d => Assert.Equal(256, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }


        await autd.Group(dev => dev.Idx.ToString())
             .Set("1", new Stop())
             .Set("0", (new Sine(150), new Uniform(1.0)))
             .SendAsync();
        {
            var m = autd.Link<Audit>().Modulation(0);
            Assert.Equal(80, m.Length);
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(0, 0);
            Assert.All(duties, d => Assert.Equal(256, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (duties, _) = autd.Link<Audit>().DutiesAndPhases(1, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
        }
    }

    [Fact]
    public async Task TestGroupCheckOnlyForEnabled()
    {
        var autd = await CreateController();
        autd.Geometry[0].Enable = false;

        var check = new bool[autd.Geometry.NumDevices];
        await autd.Group(dev =>
        {
            check[dev.Idx] = true;
            return "0";
        })
                 .Set("0", new Sine(150), new Uniform(0.5).WithPhase(Math.PI))
                 .SendAsync();

        Assert.False(check[0]);
        Assert.True(check[1]);

        {
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(0, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(1, 0);
            Assert.All(duties, d => Assert.Equal(85, d));
            Assert.All(phases, p => Assert.Equal(256, p));
        }
    }

    [Fact]
    public async Task TestClear()
    {
        var autd = await CreateController();
        Assert.True(await autd.SendAsync(new Uniform(1.0).WithPhase(Math.PI)));
        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link<Audit>().Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0, d));
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(256, d));
            Assert.All(phases, p => Assert.Equal(256, p));
        }

        Assert.True(await autd.SendAsync(new Clear()));
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
    public async Task TestStop()
    {
        var autd = await CreateController();
        Assert.True(await autd.SendAsync(new Uniform(1.0).WithPhase(Math.PI)));
        foreach (var dev in autd.Geometry)
        {
            var (duties, phases) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(256, d));
            Assert.All(phases, p => Assert.Equal(256, p));
        }

        Assert.True(await autd.SendAsync(new Stop()));
        foreach (var dev in autd.Geometry)
        {
            var (duties, _) = autd.Link<Audit>().DutiesAndPhases(dev.Idx, 0);
            Assert.All(duties, d => Assert.Equal(0, d));
        }
    }

    [Fact]
    public async Task TestUpdateFlags()
    {
        var autd = await CreateController();
        foreach (var dev in autd.Geometry)
        {
            dev.ForceFan = true;
            Assert.Equal(0, autd.Link<Audit>().FpgaFlags(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new UpdateFlags()));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(1, autd.Link<Audit>().FpgaFlags(dev.Idx));
        }
    }

    [Fact]
    public async Task TestSynchronize()
    {
        var autd = await Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero, Quaterniond.identity))
            .OpenWithAsync(Audit.Builder());

        Assert.True(await autd.SendAsync(new Synchronize()));
    }

    [Fact]
    public async Task TestConfigureModDelay()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().ModDelays(dev.Idx), d => Assert.Equal(0, d));
            foreach (var tr in dev)
                tr.ModDelay = 1;
            Assert.All(autd.Link<Audit>().ModDelays(dev.Idx), d => Assert.Equal(0, d));
        }

        Assert.True(await autd.SendAsync(new ConfigureModDelay()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().ModDelays(dev.Idx), d => Assert.Equal(1, d));
        }
    }

    [Fact]
    public async Task TestConfigureAmpFilter()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().DutyFilters(dev.Idx), d => Assert.Equal(0, d));
            foreach (var tr in dev)
                tr.AmpFilter = -1;
            Assert.All(autd.Link<Audit>().DutyFilters(dev.Idx), d => Assert.Equal(0, d));
        }

        Assert.True(await autd.SendAsync(new ConfigureAmpFilter()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().DutyFilters(dev.Idx), d => Assert.Equal(-256, d));
        }
    }

    [Fact]
    public async Task TestConfigurePhaseFilter()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().PhaseFilters(dev.Idx), d => Assert.Equal(0, d));
            foreach (var tr in dev)
                tr.PhaseFilter = -Math.PI;
            Assert.All(autd.Link<Audit>().PhaseFilters(dev.Idx), d => Assert.Equal(0, d));
        }

        Assert.True(await autd.SendAsync(new ConfigurePhaseFilter()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link<Audit>().PhaseFilters(dev.Idx), d => Assert.Equal(-256, d));
        }
    }
}
