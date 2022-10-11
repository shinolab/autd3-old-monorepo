/*
 * File: Program.cs
 * Project: example
 * Created Date: 25/08/2019
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2019-2020 Shun Suzuki. All rights reserved.
 * 
 */

namespace example
{
    internal class Program
    {
        private static void Main()
        {
            SOEMTest.Test();
            //TwinCATTest.Test();

            //// If you use emulator, execute `autd-emulator.exe` before
            //// `autd-emulator.exe` is available on https://github.com/shinolab/autd-emulator/releases
            // SimulatorTest.Test();
        }
    }
}
