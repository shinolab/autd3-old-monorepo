/*
 * File: Program.cs
 * Project: SimulatorServer
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */

class Program
{
    [STAThread]
    static void Main()
    {
        new AUTD3Sharp.Extra.Simulator().Port(8080).SettingsPath("settings.json").Vsync(true).GpuIdx(0).Run();
    }
}
