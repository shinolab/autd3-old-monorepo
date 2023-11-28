using AUTD3Sharp.Gain.Holo;
using AUTD3Sharp.Utils;
using static AUTD3Sharp.Gain.Holo.Amplitude.Units;

var backend = new NalgebraBackend();
var g = new GSPAT<NalgebraBackend>(backend)
            .AddFocus(new Vector3d(x1, y1, z1), 5e3 * Pascal)
            .AddFocus(new Vector3d(x2, y2, z2), 5e3 * Pascal);