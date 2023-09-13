using AUTD3Sharp;
using AUTD3Sharp.Link;
using AUTD3Sharp.Utils;

var autd = Controller.Builder()
    .Advanced()
    .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
    .OpenWith(new Debug().WithLogLevel(Level.Off));

foreach (var dev in autd.Geometry)
    foreach (var tr in dev)
        tr.Frequency = 70e3;

autd.Send(new Synchronize());
