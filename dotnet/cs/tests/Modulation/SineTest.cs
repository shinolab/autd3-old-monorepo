/*
 * File: SineTest.cs
 * Project: Modulation
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

namespace tests.Modulation;

public class SineTest
{
    [Fact]
    public async Task Sine()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWithAsync(Audit.Builder());

        Assert.True(await autd.SendAsync(new Sine(150).WithAmp(0.5).WithOffset(0.25).WithPhase(Math.PI / 2.0)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
            var modExpect = new byte[] {
                128,
                126,
                121,
                112,
                101,
                88,
                74,
                59,
                44,
                30,
                19,
                9,
                3,
                0,
                1,
                5,
                12,
                22,
                35,
                49,
                64,
                79,
                93,
                105,
                115,
                123,
                127,
                127,
                124,
                118,
                109,
                97,
                83,
                69,
                54,
                39,
                26,
                15,
                7,
                2,
                0,
                2,
                7,
                15,
                26,
                39,
                54,
                69,
                83,
                97,
                109,
                118,
                124,
                127,
                127,
                123,
                115,
                105,
                93,
                79,
                64,
                49,
                35,
                22,
                12,
                5,
                1,
                0,
                3,
                9,
                19,
                30,
                44,
                59,
                74,
                88,
                101,
                112,
                121,
                126};
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }


        Assert.True(await autd.SendAsync(new Sine(150).WithSamplingConfiguration(SamplingConfiguration.NewWithFrequencyDivision(512))));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(512u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }
}