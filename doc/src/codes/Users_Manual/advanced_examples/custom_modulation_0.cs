public class Burst : Modulation
{
    private readonly int _length;

    public Burst(int length) : base(SamplingConfiguration.NewWithFrequency(4e3))
    {
        _length = length;
    }

    public override EmitIntensity[] Calc()
    {
        var buf = Enumerable.Repeat<EmitIntensity>(EmitIntensity.Min, _length).ToArray();
        buf[0] = EmitIntensity.Max;
        return buf;
    }
}