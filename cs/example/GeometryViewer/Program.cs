/*
 * File: Program.cs
 * Project: GeometryViewer
 * Created Date: 21/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

using AUTD3Sharp;
using AUTD3Sharp.Extra;
using AUTD3Sharp.Utils;

var autd = new Controller();

autd.Geometry.AddDevice(Vector3d.zero, Vector3d.zero);
autd.Geometry.AddDevice(new Vector3d(0, 0, AUTD3.DeviceWidth), new Vector3d(0, AUTD3.Pi / 2.0, 0));
autd.Geometry.AddDevice(new Vector3d(AUTD3.DeviceWidth, 0, AUTD3.DeviceWidth), new Vector3d(0, AUTD3.Pi, 0));
autd.Geometry.AddDevice(new Vector3d(AUTD3.DeviceWidth, 0, 0), new Vector3d(0, -AUTD3.Pi / 2.0, 0));

new GeometryViewer().WindowSize(800, 600).Vsync(true).View(autd.Geometry);
