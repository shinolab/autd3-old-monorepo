
open AUTD3Sharp.Utils
open AUTD3Sharp
open AUTD3Sharp.Link
open Samples

let geometry = 
    GeometryBuilder()
        .AddDevice(Vector3d.zero, Vector3d.zero)
        .Build()

let serverAmsNetId = "your TwinCATAUTDServer AMS net id (e.g. 172.16.99.2.1.1)"

let link = (RemoteTwinCAT serverAmsNetId).Build()

let autd = Controller.Open (geometry, link)

SampleRunner.Run autd
