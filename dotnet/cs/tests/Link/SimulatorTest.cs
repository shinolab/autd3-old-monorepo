/*
 * File: SimulatorTest.cs
 * Project: Link
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System.Net;

namespace tests.Link;

public class SimulatorTest
{
    [Fact(Skip = "Simulator is required")]
    public void Simulator()
    {
        var autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(new Simulator(8080)
                .WithServerIp(IPAddress.Parse("127.0.0.1"))
                .WithTimeout(TimeSpan.FromMilliseconds(200)));

        autd.Close();
    }
}