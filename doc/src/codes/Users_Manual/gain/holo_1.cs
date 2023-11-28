using AUTD3Sharp.Gain.Holo;

var backend = new NalgebraBackend();
var g = new GSPAT<NalgebraBackend>(backend).WithConstraint(new Uniform(EmitIntensity.Max));