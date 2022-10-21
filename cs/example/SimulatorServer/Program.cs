/*
 * File: Program.cs
 * Project: SimulatorServer
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

new AUTD3Sharp.Extra.Simulator().SettingsPath("settings.json").Ip("127.0.0.1").Port(50632).Vsync(true).GpuIdx(0).Run();
