using AUTD3Sharp;
using AUTD3Sharp.Utils;
using AUTD3Sharp.Link;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;

var onLost = new SOEM.OnLostCallbackDelegate((string msg) =>
{
    Console.WriteLine($"Unrecoverable error occurred: {msg}");
    Environment.Exit(-1);
});

var autd = Controller.Builder()
        .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
        .OpenWith(new SOEM().WithOnLost(onLost));

autd.Send(new Clear());
autd.Send(new Synchronize());

var firmList = autd.FirmwareInfoList().ToArray();
foreach (var firm in firmList)
    Console.WriteLine(firm);

autd.Send(new SilencerConfig());

var g = new Focus(autd.Geometry.Center + new Vector3d(0, 0, 150));
var m = new Sine(150);
autd.Send(m, g);

Console.ReadKey(true);

autd.Close();