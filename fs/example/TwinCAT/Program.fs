
open AUTD3Sharp.Utils
open AUTD3Sharp
open AUTD3Sharp.Link
open Samples

let geometry = 
    GeometryBuilder()
        .AddDevice(Vector3d.zero, Vector3d.zero)
        .Build()

let link = (new TwinCAT()).Build()

let autd = Controller.Open (geometry, link)

SampleRunner.Run autd
