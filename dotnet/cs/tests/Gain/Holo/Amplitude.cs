/*
 * File: Amplitude.cs
 * Project: Holo
 * Created Date: 25/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using static AUTD3Sharp.Gain.Holo.Amplitude.Units;

namespace tests.Gain.Holo;


public class AmplitudeTest
{
    [Fact]
    public void HoloAmplitudedB()
    {
        var amp = 121.5 * dB;
        Assert.Equal(23.77004454874038, amp.Pascal);
        Assert.Equal(121.5, amp.SPL);
    }

    [Fact]
    public void HoloAmplitudePascal()
    {
        var amp = 23.77004454874038 * Pascal;
        Assert.Equal(23.77004454874038, amp.Pascal);
        Assert.Equal(121.5, amp.SPL);
    }
}