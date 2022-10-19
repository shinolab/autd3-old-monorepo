addpath('autd3')

Error = [];

use_link_soem = true;
use_backend_cuda = false;

try
    init_autd(use_link_soem, use_backend_cuda);

    cnt = Controller();
    cnt.add_device([0 0 0], [0 0 0]);

    l = SOEM();
    l.high_precision(true);
    link = l.build();

    if ~cnt.open(link)
        disp(Controller.last_error());
        throw(MException('MATLAB:RuntimeError', 'Cannot open link'));
    end

    firm_list = cnt.firmware_info_list();

    for i = 1:length(firm_list)
        disp(firm_list(i));
    end

    cnt.clear();
    cnt.synchronize();

    config = SilencerConfig();
    cnt.send(config);
    config.delete();

    TRANS_SPACING_MM = 10.16;
    NUM_TRANS_X = 18;
    NUM_TRANS_Y = 14;
    x = TRANS_SPACING_MM * ((NUM_TRANS_X - 1.0) / 2.0);
    y = TRANS_SPACING_MM * ((NUM_TRANS_Y - 1.0) / 2.0);
    z = 150.0;

    g = Focus([x y z]);
    m = Sine(150);

    cnt.send(m, g);

    g.delete();
    m.delete();

    prompt = 'press any key to finish...';
    input(prompt);

    cnt.close();
    cnt.delete();

catch Error

    for e = Error
        disp(e);
    end

end

deinit_autd(use_link_soem, use_backend_cuda);
