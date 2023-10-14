/*
 * File: SOEMTest.cs
 * Project: Link
 * Created Date: 25/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

using System.Net;
using Xunit.Abstractions;

namespace tests.Link;

public class SOEMTest
{
    private readonly ITestOutputHelper _testOutputHelper;

    public SOEMTest(ITestOutputHelper testOutputHelper)
    {
        _testOutputHelper = testOutputHelper;
    }

    [Fact(Skip = "SOEM is required")]
    public void TestSOEM()
    {
        var onLost = new SOEM.OnErrCallbackDelegate(msg =>
        {
            _testOutputHelper.WriteLine(msg);
            Environment.Exit(-1);
        });
        var onErr = new SOEM.OnErrCallbackDelegate(msg =>
        {
            _testOutputHelper.WriteLine(msg);
        });

        var autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(SOEM.Builder()
                .WithIfname("")
                .WithBufSize(32)
                .WithSendCycle(2)
                .WithSync0Cycle(2)
                .WithOnLost(onLost)
                .WithOnErr(onErr)
                .WithTimerStrategy(TimerStrategy.Sleep)
                .WithSyncMode(SyncMode.FreeRun)
                .WithStateCheckInterval(TimeSpan.FromMilliseconds(100))
                .WithTimeout(TimeSpan.FromMilliseconds(200)));

        autd.Close();
    }

    [Fact(Skip = "SOEM is required")]
    public void TestRemoteSOEM()
    {
        var autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(RemoteSOEM.Builder(new IPEndPoint(IPAddress.Parse("172.0.0.1"), 8080))
                .WithTimeout(TimeSpan.FromMilliseconds(200)));

        autd.Close();
    }
}