/*
 * File: SampleRunner.cs
 * Project: Samples
 * Created Date: 13/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 * 
 */


using AUTD3Sharp;

namespace Samples;

internal delegate Task TestFn<T>(Controller<T> autd);

public class SampleRunner
{
    public static async Task Run<T>(Controller<T> autd)
    {
        var examples = new List<(TestFn<T>, string)> { (FocusTest.Test, "Single focus test"),
            (BesselBeamTest.Test, "Bessel beam test"),
            (PlaneWaveTest.Test, "Plane wave test"),
            (WavTest.Test, "Wav modulation test"),
            (FocusSTMTest.Test, "FocusSTM test"),
            (GainSTMTest.Test, "GainSTM test"),
            (GainHoloTest.Test, "Multiple foci test"),
            (CustomTest.Test, "Custom Gain & Modulation test"),
            (FlagTest.Test, "Flag test"),
            (TransTest.Test, "TransducerTest test"),
            (GroupByTransducerTest.Test, "Group (by Transducer) test")
        };
        if (autd.Geometry.NumDevices >= 2) examples.Add((GroupByDeviceTest.Test, "Group (by Device) test"));

        Console.WriteLine("======== AUTD3 firmware information ========");
        Console.WriteLine(string.Join("\n", await autd.FirmwareInfoListAsync()));
        Console.WriteLine("============================================");

        while (true)
        {
            Console.WriteLine(string.Join("\n", examples.Select((example, i) => $"[{i}]: {example.Item2}")));
            Console.WriteLine("[Others]: finish");
            Console.Write("Choose number: ");

            if (!int.TryParse(Console.ReadLine(), out var idx) || idx >= examples.Count) break;

            var fn = examples[idx].Item1;
            await fn(autd);

            Console.WriteLine("press any key to finish...");
            Console.ReadKey(true);

            Console.WriteLine("finish.");
            await autd.SendAsync(new Stop());
        }

        await autd.CloseAsync();
    }
}
