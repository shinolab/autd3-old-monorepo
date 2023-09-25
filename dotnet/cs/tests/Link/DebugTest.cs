/*
 * File: DebugTest.cs
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
using Xunit.Abstractions;

namespace tests.Link;

public class DebugTest
{
    private readonly ITestOutputHelper _testOutputHelper;

    public DebugTest(ITestOutputHelper testOutputHelper)
    {
        _testOutputHelper = testOutputHelper;
    }

    [Fact]
    public void Debug()
    {
        var onLog = new OnLogOutputCallback(_testOutputHelper.WriteLine);
        var onFlush = new OnLogFlushCallback(() => { });

        var autd = Controller.Builder()
            .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
            .OpenWith(new Debug()
                    .WithLogFunc(onLog, onFlush)
                    .WithLogLevel(Level.Trace)
                    .WithTimeout(TimeSpan.FromMilliseconds(20)));

       autd.Close();
    }
}