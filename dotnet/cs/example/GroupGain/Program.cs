/*
 * File: Program.cs
 * Project: Group
 * Created Date: 13/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

using AUTD3Sharp;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Link;
using AUTD3Sharp.Modulation;
using AUTD3Sharp.Utils;

var autd = Controller.Builder()
    .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
    .OpenWith(Nop.Builder());

var cx = autd.Geometry.Center.x;
var g1 = new Focus(autd.Geometry.Center + new Vector3d(0, 0, 150));
var g2 = new Null();

var g = new Group(
    (_, tr) => tr.Position.x < cx ? "focus" : "null"
    ).Set("focus", g1).Set("null", g2);

var m = new Sine(150);

autd.Send(m, g);

autd.Close();
