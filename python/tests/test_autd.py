"""
File: test_autd.py
Project: tests
Created Date: 18/09/2023
Author: Shun Suzuki
-----
Last Modified: 25/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

from datetime import timedelta

import numpy as np
import pytest

from pyautd3 import (
    AUTD3,
    Clear,
    ConfigureAmpFilter,
    ConfigureModDelay,
    ConfigurePhaseFilter,
    Controller,
    Device,
    FirmwareInfo,
    Silencer,
    Stop,
    Synchronize,
    UpdateFlags,
)
from pyautd3.autd_error import AUTDError
from pyautd3.gain import Null, Uniform
from pyautd3.link.audit import Audit
from pyautd3.modulation import Sine, Static


async def create_controller() -> Controller[Audit]:
    return await (
        Controller[Audit]
        .builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))
        .open_with(Audit.builder())
    )


@pytest.mark.asyncio()
async def test_silencer():
    autd = await create_controller()

    for dev in autd.geometry:
        assert autd.link.silencer_step(dev.idx) == 10

    await autd.send(Silencer(20))

    for dev in autd.geometry:
        assert autd.link.silencer_step(dev.idx) == 20

    await autd.send(Silencer.disable())

    for dev in autd.geometry:
        assert autd.link.silencer_step(dev.idx) == 0xFFFF

    await autd.send(Silencer())

    for dev in autd.geometry:
        assert autd.link.silencer_step(dev.idx) == 10


@pytest.mark.asyncio()
async def test_fpga_info():
    autd = await create_controller()

    for dev in autd.geometry:
        dev.reads_fpga_info = True

    await autd.send(UpdateFlags())

    infos = await autd.fpga_info()
    for info in infos:
        assert not info.is_thermal_assert()

    autd.link.assert_thermal_sensor(0)
    autd.link.update(0)
    autd.link.update(1)

    infos = await autd.fpga_info()
    assert infos[0].is_thermal_assert()
    assert not infos[1].is_thermal_assert()

    autd.link.deassert_thermal_sensor(0)
    autd.link.assert_thermal_sensor(1)
    autd.link.update(0)
    autd.link.update(1)

    infos = await autd.fpga_info()
    assert not infos[0].is_thermal_assert()
    assert infos[1].is_thermal_assert()

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        _ = await autd.fpga_info()
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_firmware_info():
    autd = await create_controller()

    assert FirmwareInfo.latest_version() == "v4.0.0"

    for i, firm in enumerate(await autd.firmware_info_list()):
        assert firm.info == f"{i}: CPU = v4.0.0, FPGA = v4.0.0 [Emulator]"

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        await autd.firmware_info_list()
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_close():
    autd = await create_controller()

    assert autd.link.is_open()

    await autd.close()

    assert not autd.link.is_open()

    autd = await create_controller()

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        await autd.close()
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_send_timeout():
    autd: Controller[Audit] = (
        await Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))
        .open_with(Audit.builder().with_timeout(timeout=timedelta(microseconds=0)))
    )

    await autd.send(UpdateFlags())

    assert autd.link.last_timeout_ns() == 0

    await autd.send(UpdateFlags(), timeout=timedelta(microseconds=1))

    assert autd.link.last_timeout_ns() == 1000

    await autd.send((UpdateFlags(), UpdateFlags()), timeout=timedelta(microseconds=2))

    assert autd.link.last_timeout_ns() == 2000

    await autd.send(Stop(), timeout=timedelta(microseconds=3))

    assert autd.link.last_timeout_ns() == 3000

    autd: Controller[Audit] = (
        await Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))
        .open_with(Audit.builder().with_timeout(timeout=timedelta(microseconds=10)))
    )

    await autd.send(UpdateFlags())

    assert autd.link.last_timeout_ns() == 10 * 1000

    await autd.send(UpdateFlags(), timeout=timedelta(microseconds=1))

    assert autd.link.last_timeout_ns() == 1000

    await autd.send((UpdateFlags(), UpdateFlags()), timeout=timedelta(microseconds=2))

    assert autd.link.last_timeout_ns() == 2000

    await autd.send(Stop(), timeout=timedelta(microseconds=3))

    assert autd.link.last_timeout_ns() == 3000


@pytest.mark.asyncio()
async def test_send_single():
    autd = await create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0)

    assert await autd.send(Static())

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)

    autd.link.down()
    assert not await autd.send(Static())

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        print(await autd.send(Static()))
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_send_double():
    autd = await create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0)
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 0)
        assert np.all(phases == 0)

    assert await autd.send((Static(), Uniform(1.0)))

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 256)
        assert np.all(phases == 0)

    autd.link.down()
    assert not await autd.send((Static(), Uniform(1.0)))

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        await autd.send((Static(), Uniform(1.0)))
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_send_special():
    autd = await create_controller()

    assert await autd.send((Static(), Uniform(1.0)))

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)
        duties, _ = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 256)

    await autd.send(Stop())

    for dev in autd.geometry:
        duties, _ = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 0)

    autd.link.down()
    assert not await autd.send(Stop())

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        await autd.send(Stop())
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_group():
    autd = await create_controller()

    await autd.group(lambda dev: dev.idx).set_data(0, Static(), Null()).set_data(1, Sine(150), Uniform(1.0)).send()

    mod = autd.link.modulation(0)
    assert len(mod) == 2
    assert np.all(mod == 0xFF)
    duties, phases = autd.link.duties_and_phases(0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    mod = autd.link.modulation(1)
    assert len(mod) == 80
    duties, phases = autd.link.duties_and_phases(1, 0)
    assert np.all(duties == 256)
    assert np.all(phases == 0)

    await autd.group(lambda dev: dev.idx).set_data(1, Stop()).set_data(0, (Sine(150), Uniform(1.0))).send()

    mod = autd.link.modulation(0)
    assert len(mod) == 80
    duties, phases = autd.link.duties_and_phases(0, 0)
    assert np.all(duties == 256)
    assert np.all(phases == 0)

    mod = autd.link.modulation(1)
    duties, phases = autd.link.duties_and_phases(1, 0)
    assert np.all(duties == 0)


@pytest.mark.asyncio()
async def test_group_check_only_for_enabled():
    autd = await create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(autd.geometry.num_devices, dtype=bool)

    def f(dev: Device) -> int:
        check[dev.idx] = True
        return 0

    await autd.group(f).set_data(0, Sine(150), Uniform(0.5).with_phase(np.pi)).send()

    assert not check[0]
    assert check[1]

    mod = autd.link.modulation(0)
    duties, phases = autd.link.duties_and_phases(0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    mod = autd.link.modulation(1)
    assert len(mod) == 80
    duties, phases = autd.link.duties_and_phases(1, 0)
    assert np.all(duties == 85)
    assert np.all(phases == 256)


@pytest.mark.asyncio()
async def test_clear():
    autd = await create_controller()

    assert await autd.send((Static(), Uniform(1.0).with_phase(np.pi)))

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 256)
        assert np.all(phases == 256)

    await autd.send(Clear())

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0)
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 0)
        assert np.all(phases == 0)


@pytest.mark.asyncio()
async def test_stop():
    autd = await create_controller()

    assert await autd.send((Static(), Uniform(1.0)))

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)
        duties, _ = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 256)

    await autd.send(Stop())

    for dev in autd.geometry:
        duties, _ = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 0)


@pytest.mark.asyncio()
async def test_update_flags():
    autd = await create_controller()

    for dev in autd.geometry:
        dev.force_fan = True
        assert autd.link.fpga_flags(dev.idx) == 0

    await autd.send(UpdateFlags())

    for dev in autd.geometry:
        assert autd.link.fpga_flags(dev.idx) == 1


@pytest.mark.asyncio()
async def test_synchronize():
    autd = (
        await Controller.builder()
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))
        .open_with(Audit.builder())
    )

    assert await autd.send(Synchronize())


@pytest.mark.asyncio()
async def test_configure_mod_delay():
    autd = await create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link.mod_delays(dev.idx) == 0)

    for dev in autd.geometry:
        for tr in dev:
            tr.mod_delay = 1

    assert await autd.send(ConfigureModDelay())

    for dev in autd.geometry:
        assert np.all(autd.link.mod_delays(dev.idx) == 1)


@pytest.mark.asyncio()
async def test_configure_amp_filter():
    autd = await create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link.duty_filters(dev.idx) == 0)

    for dev in autd.geometry:
        for tr in dev:
            tr.amp_filter = -1

    assert await autd.send(ConfigureAmpFilter())

    for dev in autd.geometry:
        assert np.all(autd.link.duty_filters(dev.idx) == -256)


@pytest.mark.asyncio()
async def test_configure_phase_filter():
    autd = await create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link.phase_filters(dev.idx) == 0)

    for dev in autd.geometry:
        for tr in dev:
            tr.phase_filter = -np.pi

    assert await autd.send(ConfigurePhaseFilter())

    for dev in autd.geometry:
        assert np.all(autd.link.phase_filters(dev.idx) == -256)
