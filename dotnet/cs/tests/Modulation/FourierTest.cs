/*
 * File: FourierTest.cs
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

public class FourierTest
{
    [Fact]
    public void Fourier()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(Audit.Builder());

        var m = (new Sine(50) + new Sine(100)).AddComponent(new Sine(150))
            .AddComponentsFromIter(new[] { 200 }.Select(x => new Sine(x))) + new Sine(250);

        Assert.True(autd.Send(m));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link<Audit>().Modulation(dev.Idx);
            var modExpext = new byte[] { 85, 107, 130, 152, 169, 179, 178, 168, 152, 135, 119, 105, 94, 86, 82, 82, 85, 89, 95, 100, 104, 106, 106, 103, 98, 93, 88, 83, 80, 79, 79, 81, 85, 88, 92, 94, 96, 95, 93, 89, 85, 80, 77, 74, 74, 75, 77, 81, 85, 88, 90, 91, 89, 86, 81, 76, 71, 67, 65, 65, 66, 70, 75, 80, 85, 87, 87, 83, 76, 66, 54, 42, 31, 22, 17, 17, 21, 31, 45, 63 };
            Assert.Equal(modExpext, mod);
            Assert.Equal(40960u, autd.Link<Audit>().ModulationFrequencyDivision(dev.Idx));
        }
    }
}