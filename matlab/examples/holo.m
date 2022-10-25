%{
%File: holo.m
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

function holo(cnt)
    config = SilencerConfig();
    cnt.send(config);
    config.delete();

    backend = BackendEigen();

    opts = ["SDP(x)", "SDP"
        "EVD(x)", "EVD"
        "GS(x)", "GS"
        "GSPAT(x)", "GSPAT"
        "Naive(x)", "Naive"
        "LM(x)", "LM"
        "Greedy(x)", "Greedy"];
    n = size(opts, 1);
    opt_names = strings(n);

    for i = 1:n
        opt_names(i) = opts(i, 2);
    end

    [i, ok] = listdlg('ListString', opt_names, 'PromptString', 'Select one method', 'SelectionMode', 'single', 'ListSize', [600, 600]);

    if ~ok || i > n
        return;
    end

    f = str2func(sprintf('@(x)%s', opts(i, 1)));
    g = f(backend);

    x = 90.0;
    y = 70.0;
    z = 150.0;

    center = [x y z];

    g.add(center + [30 0 0], 1)
    g.add(center - [30 0 0], 1)
    m = Sine(150);

    cnt.send(m, g);

    g.delete();
    m.delete();

end
