/*
 * File: SquareTest.cs
 * Project: Modulation
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Modulation;

public class SquareTest
{
    [Fact]
    public void Square()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(Audit.Builder());

        Assert.True(autd.Send(new Square(200).WithLow(0.2).WithHigh(0.5).WithDuty(0.1)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link<Audit>().Modulation(dev.Idx);
#pragma warning disable IDE0230
            var modExpext = new byte[] { 85, 85, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32 };
#pragma warning restore IDE0230
            Assert.Equal(modExpext, mod);
            Assert.Equal(40960u, autd.Link<Audit>().ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(autd.Send(new Square(150).WithSamplingFrequencyDivision(4096 / 8)));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u, autd.Link<Audit>().ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(autd.Send(new Square(150).WithSamplingFrequency(8e3)));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(20480u, autd.Link<Audit>().ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(autd.Send(new Square(150).WithSamplingPeriod(TimeSpan.FromMicroseconds(100))));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(16384u, autd.Link<Audit>().ModulationFrequencyDivision(dev.Idx));
        }
    }
}