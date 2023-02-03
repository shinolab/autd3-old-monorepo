
open AUTD3Sharp.Utils
open AUTD3Sharp
open AUTD3Sharp.Link
open Samples

let geometry = 
    GeometryBuilder()
        .AddDevice(Vector3d.zero, Vector3d.zero)
        .Build()

let onLost (msg:string): unit = 
    printfn $"Unrecoverable error occurred: {msg}"
    System.Environment.Exit(-1)

let link = (new SOEM()).HighPrecision(true).OnLost(new SOEM.OnLostCallbackDelegate(onLost)).Build()

let autd = Controller.Open (geometry, link)

autd.AckCheckTimeoutMs <- 20uL;

SampleRunner.Run autd
