/*
 * File: Program.cs
 * Project: GeometryViewer
 * Created Date: 21/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Link;
using AUTD3Sharp.Utils;

namespace GeometryViewer;

internal class Program
{
    [STAThread]
    private static void Main()
    {
        var autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .AddDevice(new AUTD3(new Vector3d(0, 0, AUTD3.DeviceWidth), new Vector3d(0, AUTD3.Pi / 2.0, 0)))
            .AddDevice(new AUTD3(new Vector3d(AUTD3.DeviceWidth, 0, AUTD3.DeviceWidth), new Vector3d(0, AUTD3.Pi, 0)))
            .AddDevice(new AUTD3(new Vector3d(AUTD3.DeviceWidth, 0, 0), new Vector3d(0, -AUTD3.Pi / 2.0, 0)))
            .OpenWith(new Debug().WithLogLevel(Level.Off));

        new AUTD3Sharp.Extra.GeometryViewer().WindowSize(800, 600).Vsync(true).Run(autd.Geometry);
    }
}
