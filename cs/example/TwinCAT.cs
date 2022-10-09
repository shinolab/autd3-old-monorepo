/*
 * File: TwinCAT.cs
 * Project: example
 * Created Date: 20/05/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3Sharp;
using System;
using AUTD3Sharp.Utils;
using example.Test;

namespace example
{
    internal static class TwinCATTest
    {
        public static void Test()
        {
            Console.WriteLine("Test with TwinCAT");

            var autd = new Controller();
            autd.AddDevice(Vector3d.Zero, Vector3d.Zero);

            var link = new Link.TwinCAT().Build();
            if (!autd.Open(link))
            {
                Console.WriteLine(Controller.LastError);
                return;
            }

            TestRunner.Run(autd);
        }
    }
}
