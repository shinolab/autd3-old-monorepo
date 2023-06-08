/*
 * File: Custom.cs
 * Project: Test
 * Created Date: 14/10/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/06/2023
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
            return Transform(geometry, tr =>
            {
                var tp = tr.Position;
                var dist = (tp - _point).L2Norm;
                var phase = dist * tr.Wavenumber;
                return new Drive(1.0, phase);
            });
        }
    }

    public static void Test(Controller autd)
    {
        var config = new SilencerConfig();
        autd.Send(config);

        var mod = new Sine(150);
        var gain = new Focus(autd.Geometry.Center + new Vector3d(0, 0, 150));
        autd.Send(mod, gain);
    }
}
