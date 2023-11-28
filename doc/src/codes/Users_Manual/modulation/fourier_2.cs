using AUTD3Sharp;
using AUTD3Sharp.Modulation;

var m = new Sine(100) + new Sine(150).WithPhase(AUTD3.Pi / 2.0);