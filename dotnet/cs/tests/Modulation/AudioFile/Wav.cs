/*
 * File: Wav.cs
 * Project: AudioFile
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2023
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
    public void Wav()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(new Audit());

        Assert.True(autd.Send(new Wav("sin150.wav")));
        foreach (var dev in autd.Geometry)
        {
            var mod = Audit.Modulation(autd, dev.Idx);
            var modExpext = new byte[] {
                85,
                107,
                131,
                157,
                182,
                209,
                234,
                240,
                216,
                191,
                165,
                140,
                115,
                92,
                71,
                51,
                34,
                20,
                9,
                3,
                0,
                3,
                9,
                20,
                34,
                51,
                71,
                92,
                115,
                140,
                165,
                191,
                216,
                240,
                234,
                209,
                182,
                157,
                131,
                107,
                85,
                64,
                45,
                29,
                16,
                7,
                1,
                1,
                5,
                12,
                24,
                39,
                57,
                78,
                99,
                123,
                148,
                174,
                200,
                226,
                255,
                226,
                200,
                174,
                148,
                123,
                99,
                78,
                57,
                39,
                24,
                12,
                5,
                1,
                1,
                7,
                16,
                29,
                45,
                64};
            Assert.Equal(modExpext, mod);
            Assert.Equal(40960u, Audit.ModulationFrequencyDivision(autd, dev.Idx));
        }
    }
}
