using AUTD3Sharp;
using AUTD3Sharp.Link;
using AUTD3Sharp.Utils;
using Samples;

var autd = Controller.Builder()
    .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
    .OpenWith(new Debug());

SampleRunner.Run(autd);
