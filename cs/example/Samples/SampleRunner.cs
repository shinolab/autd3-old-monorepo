/*
 * File: SampleRunner.cs
 * Project: Samples
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/10/2022
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
            (PointSTMTest.Test, "PointSTM Test"),
            (GainSTMTest.Test, "GainSTM Test"),
            (AdvancedTest.Test, "Advanced Test (Custom gain/modulation)"),
            (CustomTest.Test, "Custom Test (Custom Focus)")
        };

        if (autd.NumDevices == 2)
            examples.Add((GroupTest.Test, "Grouped gain Test"));

        autd.Clear();

        autd.Synchronize();

        var firmList = autd.FirmwareInfoList().ToArray();
        Console.WriteLine("============================ Firmware information ==============================");
        foreach (var (firm, index) in firmList.Select((firm, i) => (firm, i)))
            Console.WriteLine($"AUTD {index}: {firm}");
        Console.WriteLine("================================================================================");

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
            autd.Stop();
        }

        autd.Clear();
        autd.Close();
        autd.Dispose();
    }
}
