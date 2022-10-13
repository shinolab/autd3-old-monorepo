from pyautd3 import AUTD, SOEM, Focus, Sine, TRANS_SPACING_MM, NUM_TRANS_X, NUM_TRANS_Y, SilencerConfig

if __name__ == '__main__':
    autd = AUTD()

    autd.add_device([0., 0., 0.], [0., 0., 0.])

    link = SOEM().high_precision(True).build()
    if not autd.open(link):
        print(AUTD.last_error())
        exit()

    autd.check_trials = 50

    autd.clear()

    autd.synchronize()

    firm_info_list = autd.firmware_info_list()
    for i, firm in enumerate(firm_info_list):
        print(f'[{i}]: {firm}')

    config = SilencerConfig()
    autd.send(config)

    x = TRANS_SPACING_MM * ((NUM_TRANS_X - 1) / 2.0)
    y = TRANS_SPACING_MM * ((NUM_TRANS_Y - 1) / 2.0)
    z = 150.0
    g = Focus([x, y, z])
    m = Sine(150)
    autd.send(m, g)

    _ = input()

    autd.close()
