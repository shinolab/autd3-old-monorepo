addpath('autd3')

Error = [];

use_link_soem = true;
use_backend_cuda = false;

try
    init_autd(use_link_soem, use_backend_cuda);

    cnt = Controller();
    cnt.add_device([0 0 0], [0 0 0]);

    l = SOEM();
    link = l.build();
    cnt.open(link);

    firm_list = cnt.firmware_info_list();

    for i = 1:length(firm_list)
        disp(firm_list(i));
    end

    clear = Clear();
    cnt.send(clear);
    clear.delete();

    synchronize = Synchronize();
    cnt.send(synchronize);
    synchronize.delete();

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
