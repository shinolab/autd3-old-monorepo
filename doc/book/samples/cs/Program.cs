using System;
using AUTD3Sharp;
using AUTD3Sharp.Utils;
using AUTD3Sharp.Link;
using AUTD3Sharp.Gain;
using AUTD3Sharp.Modulation;

var geometry = new Geometry.Builder()
    .AddDevice(Vector3d.zero, Vector3d.zero)
    .Build();

var link = new SOEM().Build();

var autd = Controller.Open(geometry, link);

autd.Send(new Clear());
autd.Send(new Synchronize());

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