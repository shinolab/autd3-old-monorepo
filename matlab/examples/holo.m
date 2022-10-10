%{
%File: holo.m
%Project: examples
%Created Date: 11/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 22/06/2022
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

    TRANS_SPACING_MM = 10.16;
    NUM_TRANS_X = 18;
    NUM_TRANS_Y = 14;
    x = TRANS_SPACING_MM * ((NUM_TRANS_X - 1.0) / 2.0);
    y = TRANS_SPACING_MM * ((NUM_TRANS_Y - 1.0) / 2.0);
    z = 150.0;

    center = [x y z];

    g.add(center + [30 0 0], 1)
    g.add(center - [30 0 0], 1)
    m = Sine(150);

    cnt.send(m, g);

    g.delete();
    m.delete();

end
