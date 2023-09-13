/*
 * File: Wav.cs
 * Project: Samples
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation.AudioFile;
using AUTD3Sharp.Utils;

namespace Samples;

internal static class WavTest
{
    public static void Test(Controller autd)
    {
        var config = new Silencer();
        autd.Send(config);

        var m = new Wav("sin150.wav");
        var g = new Focus(autd.Geometry.Center + new Vector3d(0, 0, 150));
        autd.Send((m, g));
    }
}
