// File: Program.fs
// Project: SimulatorServer
// Created Date: 03/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

(new AUTD3Sharp.Extra.Simulator()).SettingsPath("settings.json").Vsync(true).GpuIdx(0).Run();
