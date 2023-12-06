/*
 * File: RawPCM.cs
 * Project: AudioFile
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp.Modulation.AudioFile;

namespace tests.Modulation.AudioFile;

public class RawPCMTest
{
    [Fact]
    public async Task RawPCM()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        Assert.True(await autd.SendAsync(new RawPCM("sin150.dat", 4000)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
            var modExpect = new byte[] {
                157,
                185,
                210,
                231,
                245,
                253,
                255,
                249,
                236,
                218,
                194,
                167,
                138,
                108,
                79,
                53,
                31,
                14,
                4,
                0,
                4,
                14,
                31,
                53,
                79,
                108,
                138,
                167,
                194,
                218,
                236,
                249,
                255,
                253,
                245,
                231,
                210,
                185,
                157,
                128,
                98,
                70,
                45,
                24,
                10,
                2,
                0,
                6,
                19,
                37,
                61,
                88,
                117,
                147,
                176,
                202,
                224,
                241,
                251,
                255,
                251,
                241,
                224,
                202,
                176,
                147,
                117,
                88,
                61,
                37,
                19,
                6,
                0,
                2,
                10,
                24,
                45,
                70,
                98,
                128};
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }

        var m = new RawPCM("sin150.dat", 4000).WithSamplingConfig(SamplingConfiguration.FromFrequencyDivision(512));
        Assert.Equal(SamplingConfiguration.FromFrequencyDivision(512), m.SamplingConfiguration);
        Assert.True(await autd.SendAsync(m));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(512u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }
}
