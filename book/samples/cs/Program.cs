using AUTD3Sharp;
using AUTD3Sharp.Utils;
using AUTD3Sharp.Link;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;

var autd = new Controller();
autd.AddDevice(Vector3d.Zero, Vector3d.Zero);

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
foreach (var (firm, index) in firmList.Select((firm, i) => (firm, i)))
    Console.WriteLine($"AUTD {index}: {firm}");

var config = new SilencerConfig();
autd.Send(config);

const double x = Controller.TransSpacing * ((Controller.NumTransInX - 1) / 2.0);
const double y = Controller.TransSpacing * ((Controller.NumTransInY - 1) / 2.0);
const double z = 150.0;
var g = new Focus(new Vector3d(x, y, z));
var m = new Sine(150);
autd.Send(m, g);

Console.ReadKey(true);

autd.Close();