using AUTD3Sharp;
using AUTD3Sharp.Utils;
using AUTD3Sharp.STM;

var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
const int pointNum = 200;
const double radius = 30.0;
var stm = new FocusSTM(1).AddFociFromIter(Enumerable.Range(0, pointNum).Select(i =>
{
    var theta = 2.0 * Math.PI * i / pointNum;
    return center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0);
}));