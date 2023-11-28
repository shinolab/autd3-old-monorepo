using AUTD3Sharp.Modulation;

var m = new Fourier(new Sine(100))
        .AddComponent(new Sine(150))
        .AddComponentsFromIter(new[] { new Sine(200) });