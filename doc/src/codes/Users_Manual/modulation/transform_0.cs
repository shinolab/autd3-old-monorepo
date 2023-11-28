using AUTD3Sharp;
using AUTD3Sharp.Modulation;

var m = new Sine(150).WithTransform((i, d) => new EmitIntensity((byte)(d.Value / 2)));