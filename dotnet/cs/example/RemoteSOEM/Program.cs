using System.Net;
using AUTD3Sharp.Utils;
using AUTD3Sharp;
using AUTD3Sharp.Link;
using Samples;

var autd = Controller.Builder()
    .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
    .OpenWith(new RemoteSOEM(new IPEndPoint(IPAddress.Parse("127.0.0.1"), 8080)));

SampleRunner.Run(autd);
