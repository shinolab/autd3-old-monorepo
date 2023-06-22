/*
 * File: Custom.cs
 * Project: Test
 * Created Date: 14/10/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;

namespace Samples;
internal static class CustomTest
{

    private class Focus : Gain
    {
        private readonly Vector3d _point;

        public Focus(Vector3d point)
        {
            _point = point;
        }

        public override Drive[] Calc(Geometry geometry)
        {
            var soundSpeed = geometry.SoundSpeed;
            return Transform(geometry, tr =>
            {
                var tp = tr.Position;
                var dist = (tp - _point).L2Norm;
                var phase = dist * tr.Wavenumber(soundSpeed);
                return new Drive{Phase = phase, Amp= 1.0};
            });
        }
    }

    public static void Test(Controller autd)
    {
        var config = new SilencerConfig();
        autd.Send(config);

        var m = new Sine(150);
        var g = new Focus(autd.Geometry.Center + new Vector3d(0, 0, 150));

        autd.Send((m, g));
    }
}
