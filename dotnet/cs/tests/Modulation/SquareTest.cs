/*
 * File: SquareTest.cs
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

public class SquareTest
{
    [Fact]
    public async Task Square()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWithAsync(Audit.Builder());

        Assert.True(await autd.SendAsync(new Square(200).WithLow(0.2).WithHigh(0.5).WithDuty(0.1)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
#pragma warning disable IDE0230
            var modExpext = new byte[] { 85, 85, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32 };
#pragma warning restore IDE0230
            Assert.Equal(modExpext, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new Square(150).WithSamplingFrequencyDivision(512)));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(512u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new Square(150).WithSamplingFrequency(8e3)));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(2560u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(await autd.SendAsync(new Square(150).WithSamplingPeriod(TimeSpan.FromMicroseconds(100))));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(2048u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }
}