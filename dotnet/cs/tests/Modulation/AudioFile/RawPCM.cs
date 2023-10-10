/*
 * File: RawPCM.cs
 * Project: AudioFile
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2023
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
    public void RawPCM()
    {
        var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(Audit.Builder());

        Assert.True(autd.Send(new RawPCM("sin150.dat", 4000)));
        foreach (var dev in autd.Geometry)
        {
            var mod = autd.Link<Audit>().Modulation(dev.Idx);
            var modExpext = new byte[] {
                107,
                131,
                157,
                184,
                209,
                234,
                255,
                219,
                191,
                166,
                140,
                115,
                92,
                70,
                51,
                33,
                19,
                8,
                2,
                0,
                2,
                8,
                19,
                33,
                51,
                70,
                92,
                115,
                140,
                166,
                191,
                219,
                255,
                234,
                209,
                184,
                157,
                131,
                107,
                85,
                64,
                45,
                28,
                15,
                6,
                1,
                0,
                3,
                12,
                23,
                39,
                57,
                77,
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
                77,
                57,
                39,
                23,
                12,
                3,
                0,
                1,
                6,
                15,
                28,
                45,
                64,
                85 };
            Assert.Equal(modExpext, mod);
            Assert.Equal(40960u, autd.Link<Audit>().ModulationFrequencyDivision(dev.Idx));
        }
    }
}
