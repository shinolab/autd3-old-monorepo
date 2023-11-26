/*
 * File: RadiationPressureTest.cs
 * Project: Modulation
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Modulation;

public class RadiationPressureTest
{
    [Fact]
    public async Task RadiationPressure()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        Assert.True(await autd.SendAsync(new Sine(150).WithRadiationPressure()));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
            var modExpect = new byte[] {
                181,
                200,
                217,
                231,
                243,
                250,
                254,
                255,
                252,
                245,
                236,
                222,
                206,
                188,
                166,
                142,
                116,
                89,
                60,
                32,
                0,
                32,
                60,
                89,
                116,
                142,
                166,
                188,
                206,
                222,
                236,
                245,
                252,
                255,
                254,
                250,
                243,
                231,
                217,
                200,
                181,
                158,
                134,
                107,
                78,
                50,
                23,
                0,
                39,
                70,
                97,
                125,
                150,
                173,
                194,
                212,
                227,
                239,
                248,
                253,
                255,
                253,
                248,
                239,
                227,
                212,
                194,
                173,
                150,
                125,
                97,
                70,
                39,
                0,
                23,
                50,
                78,
                107,
                134,
                158 };
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }
}
