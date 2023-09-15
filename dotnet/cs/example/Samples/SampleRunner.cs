/*
 * File: SampleRunner.cs
 * Project: Samples
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3Sharp;

namespace Samples;

internal delegate void TestFn(Controller autd);

public class SampleRunner
{
    public static void Run(Controller autd)
    {
        var examples = new List<(TestFn, string)> { (FocusTest.Test, "Single focus test"),
            (BesselBeamTest.Test, "Bessel beam test"),
            (PlaneWaveTest.Test, "Plane wave test"),
            (WavTest.Test, "Wav modulation test"),
            (FocusSTMTest.Test, "FocusSTM test"),
            (GainSTMTest.Test, "GainSTM test"),
            (SoftwareSTMTest.Test, "SoftwareSTM test"),
            (GainHoloTest.Test, "Multiple foci test"),
            (CustomTest.Test, "Custom Gain & Modulation test"),
            (FlagTest.Test, "Flag test"),
            (TransTest.Test, "TransducerTest test")
        };

        Console.WriteLine("======== AUTD3 firmware information ========");
        Console.WriteLine(string.Join("\n", autd.FirmwareInfoList()));
        Console.WriteLine("============================================");

        while (true)
        {
            Console.WriteLine(string.Join("\n", examples.Select((example, i) => $"[{i}]: {example.Item2}")));
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
