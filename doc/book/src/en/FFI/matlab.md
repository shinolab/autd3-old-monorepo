# MATLAB

[autd3-matlab](https://github.com/shinolab/autd3/tree/master/matlab) provides a wrapper for MATLAB.

## Usage

For example, the following code is equivalent to [Getting Started](../Users_Manual/getting_started.md).

```matlab
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
```

For more detailed examples, see [autd3-matlab example](https://github.com/shinolab/autd3/tree/master/matlab/examples).

If you have any other questions, please send them to [GitHub issue](https://github.com/shinolab/autd3/issues).
