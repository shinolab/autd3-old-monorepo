'''
File: test_autd.py
Project: tests
Created Date: 18/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''

from datetime import timedelta
import numpy as np
from pyautd3 import Clear, Controller, AUTD3, FirmwareInfo, Silencer, Stop, Synchronize, TimerStrategy, UpdateFlags, Amplitudes
from pyautd3 import ConfigureModDelay, ConfigureAmpFilter, ConfigurePhaseFilter
from pyautd3.link.audit import Audit
from pyautd3.modulation import Static, Sine
from pyautd3.gain import Uniform, Null
from pyautd3.autd_error import AUTDError

import pytest


def create_controller():
    return Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))\
        .open_with(Audit.builder())


def test_silencer():
    autd = create_controller()

    for dev in autd.geometry:
        assert autd.link().silencer_step(dev.idx) == 10

    autd.send(Silencer(20))

    for dev in autd.geometry:
        assert autd.link().silencer_step(dev.idx) == 20

    autd.send(Silencer.disable())

    for dev in autd.geometry:
        assert autd.link().silencer_step(dev.idx) == 0xFFFF

    autd.send(Silencer())

    for dev in autd.geometry:
        assert autd.link().silencer_step(dev.idx) == 10


def test_fpga_info():
    autd = create_controller()

    for dev in autd.geometry:
        dev.reads_fpga_info = True

    autd.send(UpdateFlags())

    infos = autd.fpga_info
    for info in infos:
        assert not info.is_thermal_assert()

    autd.link().assert_thermal_sensor(0)
    autd.link().update(0)
    autd.link().update(1)

    infos = autd.fpga_info
    assert infos[0].is_thermal_assert()
    assert not infos[1].is_thermal_assert()

    autd.link().deassert_thermal_sensor(0)
    autd.link().assert_thermal_sensor(1)
    autd.link().update(0)
    autd.link().update(1)

    infos = autd.fpga_info
    assert not infos[0].is_thermal_assert()
    assert infos[1].is_thermal_assert()

    autd.link().break_down()
    with pytest.raises(AUTDError) as e:
        autd.fpga_info
    assert str(e.value) == "broken"


def test_firmware_info():
    autd = create_controller()

    assert FirmwareInfo.latest_version() == "v3.0.2"

    for i, firm in enumerate(autd.firmware_info_list()):
        assert firm.info == f"{i}: CPU = v3.0.2, FPGA = v3.0.2 [Emulator]"

    autd.link().break_down()
    with pytest.raises(AUTDError) as e:
        autd.firmware_info_list()
    assert str(e.value) == "broken"


def test_close():
    autd = create_controller()

    assert autd.link().is_open()

    autd.close()

    assert not autd.link().is_open()

    autd = create_controller()

    autd.link().break_down()
    with pytest.raises(AUTDError) as e:
        autd.close()
    assert str(e.value) == "broken"


def test_send_timeout():
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))\
        .open_with(Audit.builder().with_timeout(timeout=timedelta(microseconds=0)))

    autd.send(UpdateFlags())

    assert autd.link().last_timeout_ns() == 0

    autd.send(UpdateFlags(), timeout=timedelta(microseconds=1))

    assert autd.link().last_timeout_ns() == 1000

    autd.send((UpdateFlags(), UpdateFlags()), timeout=timedelta(microseconds=2))

    assert autd.link().last_timeout_ns() == 2000

    autd.send(Stop(), timeout=timedelta(microseconds=3))

    assert autd.link().last_timeout_ns() == 3000

    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))\
        .open_with(Audit.builder().with_timeout(timeout=timedelta(microseconds=10)))

    autd.send(UpdateFlags())

    assert autd.link().last_timeout_ns() == 10 * 1000

    autd.send(UpdateFlags(), timeout=timedelta(microseconds=1))

    assert autd.link().last_timeout_ns() == 1000

    autd.send((UpdateFlags(), UpdateFlags()), timeout=timedelta(microseconds=2))

    assert autd.link().last_timeout_ns() == 2000

    autd.send(Stop(), timeout=timedelta(microseconds=3))

    assert autd.link().last_timeout_ns() == 3000


def test_send_single():
    autd = create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0)

    assert autd.send(Static())

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0xFF)

    autd.link().down()
    assert not autd.send(Static())

    autd.link().break_down()
    with pytest.raises(AUTDError) as e:
        autd.send(Static())
    assert str(e.value) == "broken"


def test_send_double():
    autd = create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0)
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 0)
        assert np.all(phases == 0)

    assert autd.send((Static(), Uniform(1.0)))

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0xFF)
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 2048)
        assert np.all(phases == 0)

    autd.link().down()
    assert not autd.send((Static(), Uniform(1.0)))

    autd.link().break_down()
    with pytest.raises(AUTDError) as e:
        autd.send((Static(), Uniform(1.0)))
    assert str(e.value) == "broken"


def test_send_special():
    autd = create_controller()

    assert autd.send((Static(), Uniform(1.0)))

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0xFF)
        duties, _ = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 2048)

    autd.send(Stop())

    for dev in autd.geometry:
        duties, _ = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 0)

    autd.link().down()
    assert not autd.send(Stop())

    autd.link().break_down()
    with pytest.raises(AUTDError) as e:
        autd.send(Stop())
    assert str(e.value) == "broken"


def test_software_stm():
    autd = create_controller()

    cnt = 0

    def callback(autd: Controller, i: int, elapsed: timedelta):
        nonlocal cnt
        cnt += 1
        return False

    autd.software_stm(callback).with_timer_strategy(TimerStrategy.Sleep).start(timedelta(milliseconds=1.0))
    assert cnt == 1

    cnt = 0
    autd.software_stm(callback).with_timer_strategy(TimerStrategy.BusyWait).start(timedelta(milliseconds=1.0))
    assert cnt == 1

    cnt = 0
    autd.software_stm(callback).with_timer_strategy(TimerStrategy.NativeTimer).start(timedelta(milliseconds=1.0))
    assert cnt == 1


def test_group():
    autd = create_controller()

    autd.group(lambda dev: dev.idx)\
        .set(0, (Static(), Null()))\
        .set(1, (Sine(150), Uniform(1.0)))\
        .send()

    mod = autd.link().modulation(0)
    assert len(mod) == 2
    assert np.all(mod == 0xFF)
    duties, phases = autd.link().duties_and_phases(0, 0)
    assert np.all(duties == 8)
    assert np.all(phases == 0)

    mod = autd.link().modulation(1)
    assert len(mod) == 80
    duties, phases = autd.link().duties_and_phases(1, 0)
    assert np.all(duties == 2048)
    assert np.all(phases == 0)

    autd.group(lambda dev: dev.idx)\
        .set(1, Stop())\
        .set(0, (Sine(150), Uniform(1.0)))\
        .send()

    mod = autd.link().modulation(0)
    assert len(mod) == 80
    duties, phases = autd.link().duties_and_phases(0, 0)
    assert np.all(duties == 2048)
    assert np.all(phases == 0)

    mod = autd.link().modulation(1)
    duties, phases = autd.link().duties_and_phases(1, 0)
    assert np.all(duties == 0)


def test_group_check_only_for_enabled():
    autd = create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(autd.geometry.num_devices, dtype=bool)

    def f(dev):
        check[dev.idx] = True
        return 0
    autd.group(f)\
        .set(0, (Sine(150), Uniform(0.5).with_phase(np.pi)))\
        .send()

    assert not check[0]
    assert check[1]

    mod = autd.link().modulation(0)
    duties, phases = autd.link().duties_and_phases(0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    mod = autd.link().modulation(1)
    assert len(mod) == 80
    duties, phases = autd.link().duties_and_phases(1, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048)


def test_amplitudes():
    autd = Controller.builder()\
        .advanced_phase()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))\
        .open_with(Audit.builder())

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0)
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 0)
        assert np.all(phases == 0)

    assert autd.send(Uniform(1.0).with_phase(np.pi))

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0)
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 0)
        assert np.all(phases == 2048)

    assert autd.send(Amplitudes(1.0))

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0)
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 2048)
        assert np.all(phases == 2048)

    assert autd.send(Amplitudes(1.0))


def test_clear():
    autd = create_controller()

    assert autd.send((Static(), Uniform(1.0).with_phase(np.pi)))

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0xFF)
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 2048)
        assert np.all(phases == 2048)

    autd.send(Clear())

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0)
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 0)
        assert np.all(phases == 0)


def test_stop():
    autd = create_controller()

    assert autd.send((Static(), Uniform(1.0)))

    for dev in autd.geometry:
        assert np.all(autd.link().modulation(dev.idx) == 0xFF)
        duties, _ = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 2048)

    autd.send(Stop())

    for dev in autd.geometry:
        duties, _ = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 0)


def test_update_flags():
    autd = create_controller()

    for dev in autd.geometry:
        dev.force_fan = True
        assert autd.link().fpga_flags(dev.idx) == 0

    autd.send(UpdateFlags())

    for dev in autd.geometry:
        assert autd.link().fpga_flags(dev.idx) == 1


def test_synchronize():
    autd = Controller.builder()\
        .advanced()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))\
        .open_with(Audit.builder())

    for dev in autd.geometry:
        assert np.all(autd.link().cycles(dev.idx) == 4096)

    for dev in autd.geometry:
        for tr in dev:
            tr.cycle = 4000

    assert autd.send(Synchronize())

    for dev in autd.geometry:
        assert np.all(autd.link().cycles(dev.idx) == 4000)


def test_configure_mod_delay():
    autd = create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link().mod_delays(dev.idx) == 0)

    for dev in autd.geometry:
        for tr in dev:
            tr.mod_delay = 1

    assert autd.send(ConfigureModDelay())

    for dev in autd.geometry:
        assert np.all(autd.link().mod_delays(dev.idx) == 1)


def test_configure_amp_filter():
    autd = create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link().duty_filters(dev.idx) == 0)

    for dev in autd.geometry:
        for tr in dev:
            tr.amp_filter = -1

    assert autd.send(ConfigureAmpFilter())

    for dev in autd.geometry:
        assert np.all(autd.link().duty_filters(dev.idx) == -2048)


def test_configure_phase_filter():
    autd = create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link().phase_filters(dev.idx) == 0)

    for dev in autd.geometry:
        for tr in dev:
            tr.phase_filter = -np.pi

    assert autd.send(ConfigurePhaseFilter())

    for dev in autd.geometry:
        assert np.all(autd.link().phase_filters(dev.idx) == -2048)


def test_legacy():
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))\
        .open_with(Audit.builder())

    assert autd.send(Uniform(1.0))

    for dev in autd.geometry:
        assert autd.link().is_legacy(dev.idx)


def test_advanced():
    autd = Controller.builder()\
        .advanced()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))\
        .open_with(Audit.builder())

    assert autd.send(Uniform(1.0))

    for dev in autd.geometry:
        assert not autd.link().is_legacy(dev.idx)


def test_advanced_phase():
    autd = Controller.builder()\
        .advanced_phase()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))\
        .open_with(Audit.builder())

    assert autd.send(Uniform(1.0))

    for dev in autd.geometry:
        assert not autd.link().is_legacy(dev.idx)
