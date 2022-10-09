/*
 * File: Simulator.cs
 * Project: example
 * Created Date: 21/07/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3Sharp;
using System;
using AUTD3Sharp.Utils;
using example.Test;

namespace example
{
    internal static class SimulatorTest
    {
        public static void Test()
        {
            Console.WriteLine("Test with Simulator");

            var autd = new Controller();
            autd.AddDevice(Vector3d.Zero, Vector3d.Zero);

            var link = new Link.Simulator().Port(50632).Build();
            if (!autd.Open(link))
            {
                Console.WriteLine(Controller.LastError);
                return;
            }

            TestRunner.Run(autd);
        }
    }
}
