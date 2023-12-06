/*
 * File: SineTest.cs
 * Project: Modulation
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
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
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        var modExpect = new byte[] {
                126,
                124,
                119,
                111,
                100,
                87,
                73,
                58,
                44,
                30,
                18,
                9,
                3,
                0,
                1,
                5,
                12,
                22,
                34,
                48,
                63,
                78,
                92,
                104,
                114,
                121,
                125,
                126,
                123,
                117,
                108,
                96,
                82,
                68,
                53,
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
                53,
                68,
                82,
                96,
                108,
                117,
                123,
                126,
                125,
                121,
                114,
                104,
                92,
                78,
                63,
                48,
                34,
                22,
                12,
                5,
                1,
                0,
                3,
                9,
                18,
                30,
                44,
                58,
                73,
                87,
                100,
                111,
                119,
                124};

        Assert.True(await autd.SendAsync(new Sine(150).WithIntensity(EmitIntensity.Max / 2).WithOffset(EmitIntensity.Max / 4).WithPhase(Math.PI / 2.0)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }


        Assert.True(await autd.SendAsync(new Sine(150).WithIntensity(127).WithOffset(63).WithPhase(Math.PI / 2.0)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }

        var m = new Sine(150).WithSamplingConfig(SamplingConfiguration.FromFrequencyDivision(512));
        Assert.Equal(800, m.Length);
        Assert.Equal(SamplingConfiguration.FromFrequencyDivision(512), m.SamplingConfiguration);
        Assert.True(await autd.SendAsync(m));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(512u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }
}