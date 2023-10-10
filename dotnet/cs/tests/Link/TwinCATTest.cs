/*
 * File: TwinCATTest.cs
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

public class TwinCATTest
{
    [Fact(Skip = "TwinCAT is required")]
    public void TestTwinCAT()
    {
        var autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(TwinCAT.Builder().WithTimeout(TimeSpan.FromMilliseconds(200)));

        autd.Close();
    }

    [Fact(Skip = "TwinCAT is required")]
    public void TestRemoteTwinCAT()
    {
        var autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(RemoteTwinCAT.Builder("xxx.xxx.xxx.xxx.xxx.xxx")
                .WithServerIp(IPAddress.Parse("127.0.0.1"))
                .WithClientAmsNetId("xxx.xxx.xxx.xxx.xxx.xxx")
                .WithTimeout(TimeSpan.FromMilliseconds(200)));

        autd.Close();
    }
}
