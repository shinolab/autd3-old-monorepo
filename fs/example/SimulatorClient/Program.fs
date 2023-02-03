open AUTD3Sharp.Utils
open AUTD3Sharp
open AUTD3Sharp.Link
open Samples

let geometry = 
    GeometryBuilder()
        .AddDevice(Vector3d.zero, Vector3d.zero)
        .AddDevice(Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d.zero)
        .Build()

let link = Simulator().Build()

let autd = Controller.Open (geometry, link)

autd.ToNormal()
for tr in autd.Geometry do
    tr.Frequency <- 70e3

SampleRunner.Run autd
