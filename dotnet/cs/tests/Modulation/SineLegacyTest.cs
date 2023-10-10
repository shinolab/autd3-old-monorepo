/*
 * File: SineLegacyTest.cs
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

public class SineLegacyTest
{
    [Fact]
    public void SineLegacy()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(Audit.Builder());

        Assert.True(autd.Send(new SineLegacy(150).WithAmp(0.5).WithOffset(0.25)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link<Audit>().Modulation(dev.Idx);
            var modExpext = new byte[] { 41, 50, 60, 68, 75, 81, 84, 84, 83, 78, 72, 64, 55, 45, 36, 26, 18, 11, 5, 1, 0, 0, 3, 8, 14, 22, 0 };
            Assert.Equal(modExpext, mod);
            Assert.Equal(40960u, autd.Link<Audit>().ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(autd.Send(new SineLegacy(150).WithSamplingFrequencyDivision(4096 / 8)));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(4096u, autd.Link<Audit>().ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(autd.Send(new SineLegacy(150).WithSamplingFrequency(8e3)));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(20480u, autd.Link<Audit>().ModulationFrequencyDivision(dev.Idx));
        }

        Assert.True(autd.Send(new SineLegacy(150).WithSamplingPeriod(TimeSpan.FromMicroseconds(100))));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(16384u, autd.Link<Audit>().ModulationFrequencyDivision(dev.Idx));
        }
    }
}