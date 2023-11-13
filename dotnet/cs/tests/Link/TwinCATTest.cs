/*
 * File: TwinCATTest.cs
 * Project: Link
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System.Net;

namespace tests.Link;

public class TwinCATTest
{
    [Fact(Skip = "TwinCAT is required")]
    public async Task TestTwinCAT()
    {
        var autd = await new ControllerBuilder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWithAsync(TwinCAT.Builder().WithTimeout(TimeSpan.FromMilliseconds(200)));

        await autd.CloseAsync();
    }

    [Fact(Skip = "TwinCAT is required")]
    public async Task TestRemoteTwinCAT()
    {
        var autd = await new ControllerBuilder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWithAsync(RemoteTwinCAT.Builder("xxx.xxx.xxx.xxx.xxx.xxx")
                .WithServerIp(IPAddress.Parse("127.0.0.1"))
                .WithClientAmsNetId("xxx.xxx.xxx.xxx.xxx.xxx")
                .WithTimeout(TimeSpan.FromMilliseconds(200)));

        await autd.CloseAsync();
    }
}
