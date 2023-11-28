using AUTD3Sharp;
using AUTD3Sharp.Utils;
using AUTD3Sharp.STM;
using AUTD3Sharp.Gain;

var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
const int pointNum = 200;
const double radius = 30.0;
var stm = new GainSTM(1.0).AddGainsFromIter(Enumerable.Range(0, pointNum).Select(i =>
{
    var theta = 2.0 * Math.PI * i / pointNum;
    return new Focus(center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0));
}));