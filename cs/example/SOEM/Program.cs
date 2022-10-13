using AUTD3Sharp;
using AUTD3Sharp.Link;
using AUTD3Sharp.Utils;
using Samples;

Console.WriteLine("Test with SOEM");

var autd = new Controller();
autd.AddDevice(Vector3d.Zero, Vector3d.Zero);
//autd.AddDevice(Vector3d.Zero, Vector3d.Zero);

//autd.ToNormal();
//for (int i = 0; i < Controller.NumTransInDevice; i++)
//    autd.SetTransFrequency(0, i, 70e3);

var link = new SOEM()
    .HighPrecision(true)
    .OnLost(x =>
    {
        Console.WriteLine($"Unrecoverable error occurred: {x}");
        Environment.Exit(-1);
    })
    .Build();
if (!autd.Open(link))
{
    Console.WriteLine(Controller.LastError);
    return;
}

autd.CheckTrials = 50;

SampleRunner.Run(autd);
