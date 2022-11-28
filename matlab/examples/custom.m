%{
%File: custom.m
%Project: examples
%Created Date: 11/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 28/11/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

function custom(cnt)
    config = SilencerConfig();
    cnt.send(config);
    config.delete();

    x = 90.0;
    y = 70.0;
    z = 150.0;
    f = [x; y; z];

    n = cnt.num_transducers();

    amps = ones(n);
    phases = zeros(n);

    for i = 1:n
        tp = cnt.trans_position(i - 1);
        wavenum = 2 * pi / cnt.wavelength(i - 1);
        d = norm(f - tp);
        p = d * wavenum / (2 * pi);
        phases(i - 1) = p;
    end

    g = CustomGain(amps, phases);

    m = Sine(150);

    cnt.send(m, g);

    g.delete();
    m.delete();

end
