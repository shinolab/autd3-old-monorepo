%{
%File: point_stm.m
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

function point_stm(cnt)
    config = SilencerConfig.none();
    cnt.send(config);
    config.delete();

    TRANS_SPACING_MM = 10.16;
    NUM_TRANS_X = 18;
    NUM_TRANS_Y = 14;
    x = TRANS_SPACING_MM * ((NUM_TRANS_X - 1.0) / 2.0);
    y = TRANS_SPACING_MM * ((NUM_TRANS_Y - 1.0) / 2.0);
    z = 150.0;
    center = [x y z];

    stm = PointSTM();
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
