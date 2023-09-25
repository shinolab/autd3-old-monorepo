/*
 * File: RadiationPressureTest.cs
 * Project: Modulation
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Modulation;

public class RadiationPressureTest
{
    [Fact]
    public void RadiationPressure()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(new Audit());

        Assert.True(autd.Send(new Sine(150).WithRadiationPressure()));
        foreach (var dev in autd.Geometry)
        {
            var mod = Audit.Modulation(autd, dev.Idx);
            var modExpext = new byte[] {
                127,
                146,
                165,
                184,
                204,
                223,
                242,
                248,
                229,
                210,
                191,
                172,
                153,
                133,
                114,
                95,
                76,
                57,
                38,
                19,
                0,
                19,
                38,
                57,
                76,
                95,
                114,
                133,
                153,
                172,
                191,
                210,
                229,
                248,
                242,
                223,
                204,
                184,
                165,
                146,
                127,
                108,
                89,
                70,
                51,
                31,
                12,
                6,
                25,
                44,
                63,
                82,
                101,
                121,
                140,
                159,
                178,
                197,
                216,
                235,
                255,
                235,
                216,
                197,
                178,
                159,
                140,
                121,
                102,
                82,
                63,
                44,
                25,
                6,
                12,
                31,
                50,
                70,
                89,
                108 };
            Assert.Equal(modExpext, mod);
            Assert.Equal(40960u, Audit.ModulationFrequencyDivision(autd, dev.Idx));
        }
    }
}
