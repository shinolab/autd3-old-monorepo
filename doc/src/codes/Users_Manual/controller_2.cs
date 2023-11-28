using AUTD3Sharp.Gain;
using AUTD3Sharp.Utils;

await autd.Group(dev =>
    {
        return dev.Idx switch
        {
            0 => "null",
            1 => "focus",
            _ => null
        };
    })
    .Set("null", new Null())
    .Set("focus", new Focus(new Vector3d(x, y, z)))
    .SendAsync();