using AUTD3Sharp;
using AUTD3Sharp.Utils;
using AUTD3Sharp.Link;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;

var autd = new Controller();
autd.Geometry.AddDevice(Vector3d.Zero, Vector3d.Zero);

var link = new SOEM().HighPrecision(true).Build();
if (!autd.Open(link))
{
    Console.WriteLine(Controller.LastError);
    return;
}

autd.CheckTrials = 50;

autd.Clear();

autd.Synchronize();

var firmList = autd.FirmwareInfoList().ToArray();
foreach (var firm in firmList)
    Console.WriteLine(firm);

var config = new SilencerConfig();
autd.Send(config);

var g = new Focus(autd.Geometry.Center + new Vector3d(0, 0, 150));
var m = new Sine(150);
autd.Send(m, g);

Console.ReadKey(true);

autd.Close();