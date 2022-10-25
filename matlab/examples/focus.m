%{
%File: focus.m
%Project: examples
%Created Date: 11/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 24/10/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

function focus(cnt)
    config = SilencerConfig();
    cnt.send(config);
    config.delete();

    x = 90.0;
    y = 70.0;
    z = 150.0;

    g = Focus([x y z]);
    m = Sine(150);

    cnt.send(m, g);

    g.delete();
    m.delete();

end
