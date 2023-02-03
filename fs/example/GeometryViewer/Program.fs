open AUTD3Sharp
open AUTD3Sharp.Extra
open AUTD3Sharp.Utils

let geometry =
    GeometryBuilder()
        .AddDevice(Vector3d.zero, Vector3d.zero)
        .AddDevice(Vector3d(0, 0, AUTD3.DeviceWidth), Vector3d(0, AUTD3.Pi / 2.0, 0))
        .AddDevice(Vector3d(AUTD3.DeviceWidth, 0, AUTD3.DeviceWidth), Vector3d(0, AUTD3.Pi, 0))
        .AddDevice(Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d(0, -AUTD3.Pi / 2.0, 0))
        .Build()

GeometryViewer().WindowSize(800, 600).Vsync(true).View(geometry)
