using static AUTD3Sharp.Angle.Units;

new ControllerBuilder()
   .AddDevice(new AUTD3(Vector3d.zero))
   .AddDevice(
      new AUTD3(new Vector3d(AUTD3.DeviceWidth, 0, 0))
         .WithRotation(EulerAngles.FromZYZ(0 * Rad, AUTD3.Pi / 2 * Rad, 0 * Rad)))