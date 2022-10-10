%{
%File: runner.m
%Project: autd3-matlab
%Created Date: 11/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 11/06/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

function runner(cnt)

    addpath('examples');

    tests = ["focus(x)", "Single focus Test"
        "bessel(x)", "BesselBeam Test"
        "point_stm(x)", "PointSTM Test"
        "gain_stm(x)", "GainSTM Test"
        "holo(x)", "Holo Test"
        "custom(x)", "Custom Test"];
    n = size(tests, 1);
    test_names = strings(n);

    for i = 1:n
        test_names(i) = tests(i, 2);
    end

    cnt.sound_speed = 340.0;

    firm_list = cnt.firmware_info_list();

    for i = 1:length(firm_list)
        disp(firm_list(i));
    end

    cnt.clear();
    cnt.synchronize();

    while true
        [i, ok] = listdlg('ListString', test_names, 'PromptString', 'Select one test', 'SelectionMode', 'single', 'ListSize', [600, 600]);

        if ~ok || i > n
            break;
        end

        f = str2func(sprintf('@(x)%s', tests(i, 1)));
        f(cnt);

        prompt = 'press any key to finish...';
        input(prompt);

        cnt.stop();
    end

    cnt.clear();
    cnt.close();

    disp('finish');

    cnt.delete();
end
