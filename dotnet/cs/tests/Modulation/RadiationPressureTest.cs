/*
 * File: RadiationPressureTest.cs
 * Project: Modulation
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
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
                180,
                200,
                217,
                231,
                242,
                249,
                253,
                254,
                251,
                245,
                235,
                222,
                206,
                187,
                165,
                141,
                115,
                87,
                60,
                32,
                0,
                32,
                60,
                87,
                115,
                141,
                165,
                187,
                206,
                222,
                235,
                245,
                251,
                254,
                253,
                249,
                242,
                231,
                217,
                200,
                180,
                157,
                133,
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
                247,
                252,
                254,
                252,
                247,
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
                133,
                157};
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }
}
