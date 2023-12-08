/*
 * File: SquareTest.cs
 * Project: Modulation
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Modulation;

using AUTD3Sharp.NativeMethods;

public class SquareTest
{
    [Fact]
    public async Task Square()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        Assert.True(await autd.SendAsync(new Square(200).WithLow(new EmitIntensity(32)).WithHigh(new EmitIntensity(85)).WithDuty(0.1)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
#pragma warning disable IDE0230
            var modExpect = new byte[] { 85, 85, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32 };
#pragma warning restore IDE0230
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new Square(200).WithLow(32).WithHigh(85).WithDuty(0.1)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
#pragma warning disable IDE0230
            var modExpect = new byte[] { 85, 85, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32 };
#pragma warning restore IDE0230
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new Square(150).WithSamplingConfig(SamplingConfiguration.FromFrequencyDivision(512))));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(512u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }

    [Fact]
    public async Task SquareMode()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        Assert.True(await autd.SendAsync(new Square(150).WithMode(SamplingMode.SizeOptimized)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
#pragma warning disable IDE0230
            var modExpect = new byte[] { 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 };
#pragma warning restore IDE0230
            Assert.Equal(modExpect, mod);
        }

        await Assert.ThrowsAsync<AUTDException>(async () => _ = await autd.SendAsync(new Square(100.1).WithMode(SamplingMode.ExactFrequency)));
        Assert.True(await autd.SendAsync(new Square(100.1).WithMode(SamplingMode.SizeOptimized)));
    }
}