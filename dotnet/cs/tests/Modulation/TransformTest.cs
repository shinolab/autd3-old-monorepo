/*
 * File: TransformTest.cs
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

public class TransformTest
{
    [Fact]
    public void Transform()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(new Audit());

        Assert.True(autd.Send(new Sine(150).WithTransform((i, v) => v / 2)));
        foreach (var dev in autd.Geometry)
        {
            var mod = Audit.Modulation(autd, dev.Idx);
            var modExpext = new byte[] {
                41,
                50,
                60,
                69,
                76,
                81,
                84,
                84,
                82,
                78,
                71,
                63,
                54,
                44,
                34,
                25,
                16,
                9,
                4,
                1,
                0,
                1,
                4,
                9,
                16,
                25,
                34,
                44,
                54,
                63,
                71,
                78,
                82,
                84,
                84,
                81,
                76,
                69,
                60,
                50,
                41,
                31,
                22,
                14,
                7,
                3,
                0,
                0,
                1,
                5,
                11,
                19,
                28,
                37,
                47,
                57,
                66,
                73,
                79,
                83,
                85,
                83,
                79,
                73,
                66,
                57,
                47,
                37,
                28,
                19,
                11,
                5,
                1,
                0,
                0,
                3,
                7,
                14,
                22,
                31 };
            Assert.Equal(modExpext, mod);
            Assert.Equal(40960u, Audit.ModulationFrequencyDivision(autd, dev.Idx));
        }
    }
}
