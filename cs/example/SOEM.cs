/*
 * File: SOEM.cs
 * Project: example
 * Created Date: 30/04/2021
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
using AUTD3Sharp.Link;
using AUTD3Sharp.Utils;
using example.Test;

namespace example
{
    internal static class SOEMTest
    {
        public static void Test()
        {
            Console.WriteLine("Test with SOEM");

            var autd = new Controller();
            autd.AddDevice(Vector3d.Zero, Vector3d.Zero);
            //autd.AddDevice(Vector3d.Zero, Vector3d.Zero, 1);

            // Controller.ToNormal();
            // for (int i = 0; i < Controller.NumTransInDevice; i++)
            //     autd.SetTransFrequency(0, i, 70e3);

            var link = new SOEM()
                .HighPrecision(true)
                .OnLost(x =>
                    {
                        Console.WriteLine($"Unrecoverable error occurred: {x}");
                        Environment.Exit(-1);
                    })
                .Build();
            if (!autd.Open(link))
            {
                Console.WriteLine(Controller.LastError);
                return;
            }

            autd.CheckTrials = 50;

            TestRunner.Run(autd);
        }
    }
}
