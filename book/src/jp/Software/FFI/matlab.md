# MATLAB

[autd3-matlab](https://github.com/shinolab/autd3-matlab)はpythonに対応したラッパーを提供している.

## Usage

基本的には, C++版と同じになるように設計している.

たとえば, [Getting Started](../Users_Manual/getting_started.md)と等価なコードは以下のようになる.

```matlab
addpath('autd3')

Error = [];

use_link_soem = true;
use_backend_cuda = false;

try
    init_autd(use_link_soem, use_backend_cuda);

    adapters = SOEM.enumerate_adapters();

    adapters_list = strings(length(adapters));

    for i = 1:length(adapters)
        s = sprintf('[%d]: %s, %s', i, adapters(i, 1), adapters(i, 2));
        adapters_list(i) = s;
    end

    [i, ok] = listdlg('ListString', adapters_list, 'PromptString', 'Select one interface', 'SelectionMode', 'single', 'ListSize', [600, 600]);

    if ~ok || ~isnumeric(i) || i > length(adapters)
        throw(MException('MATLAB:BadIndex', 'Index out of range'));
    end

    ifname = adapters(i, 2);
    l = SOEM(ifname, 1);
    link = l.build();

    cnt = Controller();
    cnt.add_device([0 0 0], [0 0 0]);

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

より詳細なサンプルは[autd3-matlabのexample](https://github.com/shinolab/autd3-matlab/tree/master/example)を参照されたい.

その他, 質問があれば[GitHubのissue](https://github.com/shinolab/autd3-matlab/issues)にてお願いします.
