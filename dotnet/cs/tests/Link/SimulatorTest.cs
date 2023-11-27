/*
 * File: SimulatorTest.cs
 * Project: Link
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/11/2023
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
    public async Task TestSimulator()
    {
        var autd = await new ControllerBuilder()
            .AddDevice(new AUTD3(Vector3d.zero))
            .OpenWithAsync(Simulator.Builder(8080)
                .WithServerIp(IPAddress.Parse("127.0.0.1"))
                .WithTimeout(TimeSpan.FromMilliseconds(200)));

        await autd.CloseAsync();
    }
}