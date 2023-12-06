/*
 * File: Wav.cs
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

public class WavTest
{
    [Fact]
    public async Task Wav()
    {
        var autd = await new ControllerBuilder().AddDevice(new AUTD3(Vector3d.zero)).OpenWithAsync(Audit.Builder());

        Assert.True(await autd.SendAsync(new Wav("sin150.wav")));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link.Modulation(dev.Idx);
            var modExpect = new byte[] {
                128,
                157,
                185,
                210,
                230,
                245,
                253,
                254,
                248,
                236,
                217,
                194,
                167,
                137,
                109,
                80,
                54,
                32,
                15,
                5,
                1,
                5,
                15,
                32,
                54,
                80,
                109,
                137,
                167,
                194,
                217,
                236,
                248,
                254,
                253,
                245,
                230,
                210,
                185,
                157,
                128,
                99,
                71,
                46,
                26,
                11,
                3,
                2,
                8,
                20,
                39,
                62,
                89,
                119,
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
                119,
                89,
                62,
                39,
                20,
                8,
                2,
                3,
                11,
                26,
                46,
                71,
                99};
            Assert.Equal(modExpect, mod);
            Assert.Equal(5120u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }

        var m = new Wav("sin150.wav").WithSamplingConfig(SamplingConfiguration.FromFrequencyDivision(512));
        Assert.Equal(SamplingConfiguration.FromFrequencyDivision(512), m.SamplingConfiguration);
        Assert.True(await autd.SendAsync(m));
        foreach (var dev in autd.Geometry)
        {
            Assert.Equal(512u, autd.Link.ModulationFrequencyDivision(dev.Idx));
        }
    }
}
