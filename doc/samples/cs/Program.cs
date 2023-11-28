using AUTD3Sharp;
using AUTD3Sharp.Utils;
using AUTD3Sharp.Link;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;

var onLost = new SOEM.OnErrCallbackDelegate((string msg) =>
{
    Console.WriteLine($"Unrecoverable error occurred: {msg}");
    Environment.Exit(-1);
});

using var autd = await new ControllerBuilder()
        .AddDevice(new AUTD3(Vector3d.zero))
        .OpenWithAsync(SOEM.Builder().WithOnLost(onLost));

var firmList = await autd.FirmwareInfoListAsync();
foreach (var firm in firmList)
    Console.WriteLine(firm);

await autd.SendAsync(new Silencer());

var g = new Focus(autd.Geometry.Center + new Vector3d(0, 0, 150));
var m = new Sine(150);
await autd.SendAsync(m, g);

Console.ReadKey(true);

await autd.CloseAsync();