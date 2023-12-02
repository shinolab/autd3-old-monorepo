public class FocalPoint : Gain
{
    private readonly Vector3d _point;

    public FocalPoint(Vector3d point)
    {
        _point = point;
    }

    public override Dictionary<int, Drive[]> Calc(Geometry geometry)
    {
        return Transform(geometry, (dev, tr) =>
        {
            var tp = tr.Position;
            var dist = (tp - _point).L2Norm;
            var phase = dist * tr.Wavenumber(dev.SoundSpeed);
            return new Drive { Phase = Phase.FromRad(phase), Intensity = EmitIntensity.Max };
        });
    }
}