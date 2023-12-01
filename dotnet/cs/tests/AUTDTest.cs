/*
 * File: AUTDTest.cs
 * Project: tests
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests;

public class AUTDTest
{

    public static async Task<Controller<Audit>> CreateController()
    {
        return await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());
    }

    [Fact]
    public async Task TestSilencer()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(256, autd.Link.SilencerStepIntensity(dev.Idx));
            Assert.Equal(256, autd.Link.SilencerStepPhase(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new Silencer(10, 20)));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(10, autd.Link.SilencerStepIntensity(dev.Idx));
            Assert.Equal(20, autd.Link.SilencerStepPhase(dev.Idx));
        }

        Assert.True(await autd.SendAsync(Silencer.Disable()));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(0xFFFF, autd.Link.SilencerStepIntensity(dev.Idx));
            Assert.Equal(0xFFFF, autd.Link.SilencerStepPhase(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new Silencer()));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(256, autd.Link.SilencerStepIntensity(dev.Idx));
            Assert.Equal(256, autd.Link.SilencerStepPhase(dev.Idx));
        }
    }

    [Fact]
    public async Task TestDebugOutputIdx()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(0xFF, autd.Link.DebugOutputIdx(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new ConfigureDebugOutputIdx((device) => device[0])));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(0, autd.Link.DebugOutputIdx(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new ConfigureDebugOutputIdx((device) => device.Idx == 0 ? device[10] : null)));
        Assert.Equal(10, autd.Link.DebugOutputIdx(0));
        Assert.Equal(0xFF, autd.Link.DebugOutputIdx(1));
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
            autd.Link.AssertThermalSensor(0);
            autd.Link.Update(0);
            autd.Link.Update(1);

            var infos = await autd.FPGAInfoAsync();
            Assert.True(infos[0].IsThermalAssert);
            Assert.False(infos[1].IsThermalAssert);
        }

        {
            autd.Link.DeassertThermalSensor(0);
            autd.Link.AssertThermalSensor(1);
            autd.Link.Update(0);
            autd.Link.Update(1);

            var infos = await autd.FPGAInfoAsync();
            Assert.False(infos[0].IsThermalAssert);
            Assert.True(infos[1].IsThermalAssert);
        }

        {
            autd.Link.BreakDown();
            await Assert.ThrowsAsync<AUTDException>(async () => _ = await autd.FPGAInfoAsync());
        }
    }

    [Fact]
    public async Task TestFirmwareInfoList()
    {
        var autd = await CreateController();

        Assert.Equal("v4.0.1", FirmwareInfo.LatestVersion);

        {
            foreach (var (info, i) in (await autd.FirmwareInfoListAsync()).Select((info, i) => (info, i)))
            {
                Assert.Equal(info.Info, $"{i}: CPU = v4.0.1, FPGA = v4.0.1 [Emulator]");
            }
        }

        {
            autd.Link.BreakDown();
            await Assert.ThrowsAsync<AUTDException>(async () => _ = (await autd.FirmwareInfoListAsync()).Last());
        }
    }

    [Fact]
    public async Task TestClose()
    {
        {
            var autd = await CreateController();
            Assert.True(autd.Link.IsOpen());

            await autd.CloseAsync();
            Assert.False(autd.Link.IsOpen());
        }

        {
            var autd = await CreateController();

            autd.Link.BreakDown();
            await Assert.ThrowsAsync<AUTDException>(async () => await autd.CloseAsync());
        }
    }

    [Fact]
    public async Task TestSendTimeout()
    {
        {
            var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero))
                .OpenWithAsync(Audit.Builder().WithTimeout(TimeSpan.FromMicroseconds(0)));

            await autd.SendAsync(new UpdateFlags());
            Assert.Equal(0ul, autd.Link.LastTimeoutNs());

            await autd.SendAsync(new UpdateFlags(), TimeSpan.FromMicroseconds(1));
            Assert.Equal(1000ul, autd.Link.LastTimeoutNs());

            await autd.SendAsync((new UpdateFlags(), new UpdateFlags()), TimeSpan.FromMicroseconds(2));
            Assert.Equal(2000ul, autd.Link.LastTimeoutNs());

            await autd.SendAsync(new Stop(), TimeSpan.FromMicroseconds(3));
            Assert.Equal(3000ul, autd.Link.LastTimeoutNs());
        }

        {
            var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero))
                .OpenWithAsync(Audit.Builder().WithTimeout(TimeSpan.FromMicroseconds(10)));

            await autd.SendAsync(new UpdateFlags());
            Assert.Equal(10000ul, autd.Link.LastTimeoutNs());

            await autd.SendAsync(new UpdateFlags(), TimeSpan.FromMicroseconds(1));
            Assert.Equal(1000ul, autd.Link.LastTimeoutNs());

            await autd.SendAsync((new UpdateFlags(), new UpdateFlags()), TimeSpan.FromMicroseconds(2));
            Assert.Equal(2000ul, autd.Link.LastTimeoutNs());

            await autd.SendAsync(new Stop(), TimeSpan.FromMicroseconds(3));
            Assert.Equal(3000ul, autd.Link.LastTimeoutNs());
        }
    }

    [Fact]
    public async Task TestSendSingle()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link.Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
        }
        Assert.True(await autd.SendAsync(new Static()));

        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link.Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
        }

        autd.Link.Down();
        Assert.False(await autd.SendAsync(new Static()));

        autd.Link.BreakDown();
        await Assert.ThrowsAsync<AUTDException>(async () => await autd.SendAsync(new Static()));
    }

    [Fact]
    public async Task TestSendDouble()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link.Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        Assert.True(await autd.SendAsync(new Static(), new Uniform(EmitIntensity.Max)));
        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link.Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0xFF, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }

        autd.Link.Down();
        Assert.False(await autd.SendAsync((new Static(), new Uniform(EmitIntensity.Max))));

        autd.Link.BreakDown();
        await Assert.ThrowsAsync<AUTDException>(async () => await autd.SendAsync(new Static(), new Uniform(EmitIntensity.Max)));
    }

    [Fact]
    public async Task TestSendSpecial()
    {
        var autd = await CreateController();
        Assert.True(await autd.SendAsync(new Uniform(EmitIntensity.Max)));

        foreach (var dev in autd.Geometry)
        {
            var (intensities, _) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0xFF, d));
        }
        Assert.True(await autd.SendAsync(new Stop()));

        foreach (var dev in autd.Geometry)
        {
            var (intensities, _) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0, d));
        }

        autd.Link.Down();
        Assert.False(await autd.SendAsync(new Stop()));

        autd.Link.BreakDown();
        await Assert.ThrowsAsync<AUTDException>(async () => await autd.SendAsync(new Stop()));
    }

    [Fact]
    public async Task TestGroup()
    {
        var autd = await CreateController();

        await autd.Group(dev => dev.Idx.ToString())
             .Set("0", (new Static(), new Null()))
             .Set("1", new Sine(150), new Uniform(EmitIntensity.Max))
             .SendAsync();
        {
            var m = autd.Link.Modulation(0);
            Assert.Equal(2, m.Length);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(0, 0);
            Assert.All(intensities, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var m = autd.Link.Modulation(1);
            Assert.Equal(80, m.Length);
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(1, 0);
            Assert.All(intensities, d => Assert.Equal(0xFF, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }


        await autd.Group(dev => dev.Idx.ToString())
             .Set("1", new Stop())
             .Set("0", (new Sine(150), new Uniform(EmitIntensity.Max)))
             .SendAsync();
        {
            var m = autd.Link.Modulation(0);
            Assert.Equal(80, m.Length);
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(0, 0);
            Assert.All(intensities, d => Assert.Equal(0xFF, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (intensities, _) = autd.Link.IntensitiesAndPhases(1, 0);
            Assert.All(intensities, d => Assert.Equal(0, d));
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
                 .Set("0", new Sine(150), new Uniform(new EmitIntensity(0x80)).WithPhase(Math.PI))
                 .SendAsync();

        Assert.False(check[0]);
        Assert.True(check[1]);

        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(0, 0);
            Assert.All(intensities, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(1, 0);
            Assert.All(intensities, d => Assert.Equal(0x80, d));
            Assert.All(phases, p => Assert.Equal(128, p));
        }
    }

    [Fact]
    public async Task TestClear()
    {
        var autd = await CreateController();
        Assert.True(await autd.SendAsync(new Uniform(EmitIntensity.Max).WithPhase(Math.PI)));
        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link.Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0xFF, d));
            Assert.All(phases, p => Assert.Equal(128, p));
        }

        Assert.True(await autd.SendAsync(new Clear()));
        foreach (var dev in autd.Geometry)
        {
            var m = autd.Link.Modulation(dev.Idx);
            Assert.All(m, d => Assert.Equal(0xFF, d));
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0, d));
            Assert.All(phases, p => Assert.Equal(0, p));
        }
    }

    [Fact]
    public async Task TestStop()
    {
        var autd = await CreateController();
        Assert.True(await autd.SendAsync(new Uniform(EmitIntensity.Max).WithPhase(Math.PI)));
        foreach (var dev in autd.Geometry)
        {
            var (intensities, phases) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0xFF, d));
            Assert.All(phases, p => Assert.Equal(128, p));
        }

        Assert.True(await autd.SendAsync(new Stop()));
        foreach (var dev in autd.Geometry)
        {
            var (intensities, _) = autd.Link.IntensitiesAndPhases(dev.Idx, 0);
            Assert.All(intensities, d => Assert.Equal(0, d));
        }
    }

    [Fact]
    public async Task TestUpdateFlags()
    {
        var autd = await CreateController();
        foreach (var dev in autd.Geometry)
        {
            dev.ForceFan = true;
            Assert.Equal(0, autd.Link.FpgaFlags(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new UpdateFlags()));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(1, autd.Link.FpgaFlags(dev.Idx));
        }
    }

    [Fact]
    public async Task TestSynchronize()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).AddDevice(new AUTD3(Vector3d.zero))
            .OpenWithAsync(Audit.Builder());

        Assert.True(await autd.SendAsync(new Synchronize()));
    }

    [Fact]
    public async Task TestConfigureModDelay()
    {
        var autd = await CreateController();

        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link.ModDelays(dev.Idx), d => Assert.Equal(0, d));
            foreach (var tr in dev)
                tr.ModDelay = 1;
            Assert.All(autd.Link.ModDelays(dev.Idx), d => Assert.Equal(0, d));
        }

        Assert.True(await autd.SendAsync(new ConfigureModDelay()));
        foreach (var dev in autd.Geometry)
        {
            Assert.All(autd.Link.ModDelays(dev.Idx), d => Assert.Equal(1, d));
        }
    }
}
