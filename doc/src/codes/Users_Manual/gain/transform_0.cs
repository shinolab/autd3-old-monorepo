using AUTD3Sharp;
using AUTD3Sharp.Gain;

var g = new Uniform(EmitIntensity.Max).WithTransform((dev, tr, d) =>
{
    d.Intensity = new EmitIntensity((byte)(d.Intensity.Value / 2));
    d.Phase += AUTD3.Pi;
    return d;
});