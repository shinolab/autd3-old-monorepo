/*
 * File: Amplitude.cs
 * Project: Holo
 * Created Date: 25/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using AUTD3Sharp.Gain.Holo;

namespace tests.Gain.Holo;


public class AmplitudeTest
{
    [Fact]
    public void HoloAmplitudedB()
    {
        var amp = Amplitude.NewSPL(121.5);
        Assert.Equal(23.77004454874038, amp.Pascal);
        Assert.Equal(121.5, amp.SPL);
    }

    [Fact]
    public void HoloAmplitudePascal()
    {
        var amp = Amplitude.NewPascal(23.77004454874038);
        Assert.Equal(23.77004454874038, amp.Pascal);
        Assert.Equal(121.5, amp.SPL);
    }
}