/*
 * File: SOEMTest.cs
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

using AUTD3Sharp.Internal;
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
    public void SOEM()
    {
        var onLost = new SOEM.OnLostCallbackDelegate(msg =>
        {
            _testOutputHelper.WriteLine(msg);
            Environment.Exit(-1);
        });
        var onLog = new OnLogOutputCallback(_testOutputHelper.WriteLine);
        var onFlush = new OnLogFlushCallback(() => { });

        var autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(new SOEM()
                .WithIfname("")
                .WithBufSize(32)
                .WithSendCycle(2)
                .WithSync0Cycle(2)
                .WithOnLost(onLost)
                .WithTimerStrategy(TimerStrategy.Sleep)
                .WithSyncMode(SyncMode.FreeRun)
                .WithStateCheckInterval(TimeSpan.FromMilliseconds(100))
                .WithLogLevel(Level.Off)
                .WithLogFunc(onLog, onFlush)
                .WithTimeout(TimeSpan.FromMilliseconds(200)));

        autd.Close();
    }

    [Fact(Skip = "SOEM is required")]
    public void RemoteSOEM()
    {
        var autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(new RemoteSOEM(new IPEndPoint(IPAddress.Parse("172.0.0.1"), 8080))
                .WithTimeout(TimeSpan.FromMilliseconds(200)));

        autd.Close();
    }
}