/*
 * File: FourierTest.cs
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

public class FourierTest
{
    [Fact]
    public async Task Fourier()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        var m = (new Sine(50) + new Sine(100)).AddComponent(new Sine(150))
            .AddComponentsFromIter(new[] { 200 }.Select(x => new Sine(x))) + new Sine(250);

        Assert.True(await autd.SendAsync(m));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
            var modExpect = new byte[]
            {
                127,
                156,
                183,
                205,
                220,
                227,
                226,
                218,
                205,
                188,
                170,
                153,
                139,
                129,
                124,
                123,
                127,
                133,
                140,
                147,
                152,
                155,
                154,
                151,
                145,
                138,
                131,
                125,
                120,
                119,
                119,
                122,
                127,
                132,
                136,
                140,
                141,
                140,
                137,
                133,
                127,
                121,
                116,
                113,
                112,
                113,
                117,
                121,
                127,
                131,
                134,
                135,
                133,
                128,
                122,
                115,
                108,
                103,
                99,
                99,
                101,
                106,
                113,
                121,
                127,
                130,
                130,
                124,
                114,
                100,
                83,
                66,
                48,
                35,
                27,
                27,
                34,
                49,
                70,
                97
            };
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }
}