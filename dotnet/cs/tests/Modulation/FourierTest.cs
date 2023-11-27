/*
 * File: FourierTest.cs
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
                128,
                157,
                183,
                205,
                220,
                227,
                227,
                219,
                206,
                189,
                171,
                153,
                140,
                129,
                124,
                124,
                127,
                133,
                141,
                147,
                153,
                155,
                155,
                151,
                146,
                139,
                131,
                125,
                121,
                119,
                120,
                123,
                127,
                132,
                137,
                140,
                142,
                141,
                138,
                133,
                128,
                121,
                116,
                113,
                112,
                114,
                117,
                122,
                127,
                132,
                135,
                135,
                133,
                129,
                123,
                116,
                108,
                103,
                100,
                99,
                102,
                107,
                114,
                121,
                127,
                130,
                130,
                125,
                115,
                101,
                84,
                66,
                49,
                35,
                27,
                27,
                34,
                49,
                71,
                98
            };
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }
}