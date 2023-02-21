/*
 * File: SampleRunner.cs
 * Project: Samples
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/02/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3Sharp;

namespace Samples;

internal delegate void TestFn(Controller autd);

public class SampleRunner
{
    public static void Run(Controller autd)
    {
        var examples = new List<(TestFn, string)> { (FocusTest.Test, "Single Focal Point Test"),
            (BesselBeamTest.Test, "BesselBeam Test"),
            (GainHoloTest.Test, "Multiple Focal Points Test"),
            (FocusSTMTest.Test, "FocusSTM Test"),
            (GainSTMTest.Test, "GainSTM Test"),
            (AdvancedTest.Test, "Advanced Test (Custom gain/modulation)"),
            (CustomTest.Test, "Custom Test (Custom Focus)")
        };

        if (autd.Geometry.NumDevices == 2)
            examples.Add((GroupTest.Test, "Grouped gain Test"));

        autd.Send(new Clear());
        autd.Send(new Synchronize());

        var firmList = autd.FirmwareInfoList().ToArray();
        if (!firmList.All((firm) => firm.MatchesVersion))
        {
            Console.ForegroundColor = ConsoleColor.Yellow;
            Console.WriteLine("WARN: FPGA and CPU firmware version do not match.");
            Console.ResetColor();
        }
        if (!firmList.All((firm) => firm.IsSupported))
        {
            Console.ForegroundColor = ConsoleColor.Yellow;
            Console.WriteLine($"WARN: You are using old firmware. Please consider updating to {FirmwareInfo.LatestVersion}.");
            Console.ResetColor();
        }

        Console.WriteLine("==================================== Firmware information ======================================");
        foreach (var firm in firmList)
            Console.WriteLine($"{firm}");
        Console.WriteLine("================================================================================================");

        while (true)
        {
            for (var i = 0; i < examples.Count; i++)
                Console.WriteLine($"[{i}]: {examples[i].Item2}");

            Console.WriteLine("[Others]: finish");
            Console.Write("Choose number: ");

            if (!int.TryParse(Console.ReadLine(), out var idx) || idx >= examples.Count) break;

            var fn = examples[idx].Item1;
            fn(autd);

            Console.WriteLine("press any key to finish...");
            Console.ReadKey(true);

            Console.WriteLine("finish.");
            autd.Send(new Stop());
        }

        autd.Close();
        autd.Dispose();
    }
}
