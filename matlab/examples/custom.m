%{
%File: custom.m
%Project: examples
%Created Date: 11/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 11/06/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

function custom(cnt)
    config = SilencerConfig();
    cnt.send(config);
    config.delete();

    TRANS_SPACING_MM = 10.16;
    NUM_TRANS_X = 18;
    NUM_TRANS_Y = 14;
    x = TRANS_SPACING_MM * ((NUM_TRANS_X - 1.0) / 2.0);
    y = TRANS_SPACING_MM * ((NUM_TRANS_Y - 1.0) / 2.0);
    z = 150.0;
    f = [x; y; z];

    n = cnt.num_devices();

    amps = ones(n * 249);
    phases = zeros(n * 249);

    for i = 1:n

        for j = 1:249
            tp = cnt.trans_position(i - 1, j - 1);
            wavenum = 2 * pi / cnt.wavelength(i - 1, j - 1);
            d = norm(f - tp);
            p = d * wavenum / (2 * pi);
            phases((i - 1) * 249 + j) = p;
        end

    end

    g = CustomGain(amps, phases);

    m = Sine(150);

    cnt.send(m, g);

    g.delete();
    m.delete();

end
