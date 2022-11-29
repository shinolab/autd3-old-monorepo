%{
%File: focus_stm.m
%Project: examples
%Created Date: 11/06/2022
%Author: Shun Suzuki
%-----
%Last Modified: 29/11/2022
%Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
%-----
%Copyright (c) 2022 Shun Suzuki. All rights reserved.
%
%}

function focus_stm(cnt)
    config = SilencerConfig.none();
    cnt.send(config);
    config.delete();

    x = 90.0;
    y = 70.0;
    z = 150.0;
    center = [x y z];

    sound_speed = cnt.sound_speed;
    stm = FocusSTM(sound_speed);
    points_num = 200;
    radius = 30.0;

    for i = 1:points_num
        theta = 2.0 * pi * i / points_num;
        p = center + radius * [cos(theta) sin(theta) 0];
        stm.add(p);
    end

    stm.freqeuncy = 1;
    fprintf("acutual frequency is %f\n", stm.freqeuncy);

    m = Static();

    cnt.send(m, stm);

    stm.delete();
    m.delete();

end
