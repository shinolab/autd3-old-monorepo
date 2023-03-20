open System
open AUTD3Sharp
open AUTD3Sharp.Utils
open AUTD3Sharp.Gain
open AUTD3Sharp.Modulation
open AUTD3Sharp.Link

let geometry = 
    GeometryBuilder()
        .AddDevice(Vector3d.zero, Vector3d.zero)
        .Build()

let link = (new SOEM()).Build()

let autd = Controller.Open (geometry, link)

(new Clear(), TimeSpan.FromMilliseconds(20)) |> autd.Send |> ignore;
(new Synchronize(), TimeSpan.FromMilliseconds(20)) |> autd.Send |> ignore;

let print_firm firm = printfn $"{firm}" 
printfn "==================================== Firmware information ======================================"
autd.FirmwareInfoList() |> Seq.iter print_firm
printfn "================================================================================================"

(new SilencerConfig(), TimeSpan.FromMilliseconds(20)) |> autd.Send |> ignore;

let m = new Sine 150;
let g = new Focus(autd.Geometry.Center + Vector3d(0, 0, 150));
autd.Send(m, g, TimeSpan.FromMilliseconds(20)) |> ignore

System.Console.ReadKey true |> ignore;

autd.Close() |> ignore;