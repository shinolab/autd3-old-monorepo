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
    ConfigureDebugOutputIdx,
    ConfigureModDelay,
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
    return await Controller[Audit].builder().add_device(AUTD3([0.0, 0.0, 0.0])).add_device(AUTD3([0.0, 0.0, 0.0])).open_with_async(Audit.builder())


@pytest.mark.asyncio()
async def test_silencer():
    autd = await create_controller()

    for dev in autd.geometry:
        assert autd.link.silencer_step_intensity(dev.idx) == 256
        assert autd.link.silencer_step_phase(dev.idx) == 256

    await autd.send_async(Silencer(2000, 1000))

    for dev in autd.geometry:
        assert autd.link.silencer_step_intensity(dev.idx) == 2000
        assert autd.link.silencer_step_phase(dev.idx) == 1000

    await autd.send_async(Silencer.disable())

    for dev in autd.geometry:
        assert autd.link.silencer_step_intensity(dev.idx) == 0xFFFF
        assert autd.link.silencer_step_phase(dev.idx) == 0xFFFF

    await autd.send_async(Silencer())

    for dev in autd.geometry:
        assert autd.link.silencer_step_intensity(dev.idx) == 256
        assert autd.link.silencer_step_phase(dev.idx) == 256


@pytest.mark.asyncio()
async def test_debug_output_idx():
    autd = await create_controller()

    for dev in autd.geometry:
        assert autd.link.debug_output_idx(dev.idx) == 0xFF

    await autd.send_async(ConfigureDebugOutputIdx(lambda dev: dev[0]))

    for dev in autd.geometry:
        assert autd.link.debug_output_idx(dev.idx) == 0

    await autd.send_async(ConfigureDebugOutputIdx(lambda dev: dev[10] if dev.idx == 0 else None))

    assert autd.link.debug_output_idx(0) == 10
    assert autd.link.debug_output_idx(1) == 0xFF


@pytest.mark.asyncio()
async def test_fpga_info_async():
    autd = await create_controller()

    for dev in autd.geometry:
        dev.reads_fpga_info = True

    await autd.send_async(UpdateFlags())

    infos = await autd.fpga_info_async()
    for info in infos:
        assert not info.is_thermal_assert()

    autd.link.assert_thermal_sensor(0)
    autd.link.update(0)
    autd.link.update(1)

    infos = await autd.fpga_info_async()
    assert infos[0].is_thermal_assert()
    assert not infos[1].is_thermal_assert()

    autd.link.deassert_thermal_sensor(0)
    autd.link.assert_thermal_sensor(1)
    autd.link.update(0)
    autd.link.update(1)

    infos = await autd.fpga_info_async()
    assert not infos[0].is_thermal_assert()
    assert infos[1].is_thermal_assert()

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        _ = await autd.fpga_info_async()
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_firmware_info():
    autd = await create_controller()

    assert FirmwareInfo.latest_version() == "v4.0.1"

    for i, firm in enumerate(await autd.firmware_info_list_async()):
        assert firm.info == f"{i}: CPU = v4.0.1, FPGA = v4.0.1 [Emulator]"

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        await autd.firmware_info_list_async()
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_close_async():
    autd = await create_controller()

    assert autd.link.is_open()

    await autd.close_async()

    assert not autd.link.is_open()

    autd = await create_controller()

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        await autd.close_async()
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_send_async_timeout():
    autd: Controller[Audit] = (
        await Controller.builder()
        .add_device(AUTD3([0.0, 0.0, 0.0]))
        .add_device(AUTD3([0.0, 0.0, 0.0]))
        .open_with_async(Audit.builder().with_timeout(timeout=timedelta(microseconds=0)))
    )

    await autd.send_async(UpdateFlags())

    assert autd.link.last_timeout_ns() == 0

    await autd.send_async(UpdateFlags(), timeout=timedelta(microseconds=1))

    assert autd.link.last_timeout_ns() == 1000

    await autd.send_async((UpdateFlags(), UpdateFlags()), timeout=timedelta(microseconds=2))

    assert autd.link.last_timeout_ns() == 2000

    await autd.send_async(Stop(), timeout=timedelta(microseconds=3))

    assert autd.link.last_timeout_ns() == 3000

    autd: Controller[Audit] = (
        await Controller.builder()
        .add_device(AUTD3([0.0, 0.0, 0.0]))
        .add_device(AUTD3([0.0, 0.0, 0.0]))
        .open_with_async(Audit.builder().with_timeout(timeout=timedelta(microseconds=10)))
    )

    await autd.send_async(UpdateFlags())

    assert autd.link.last_timeout_ns() == 10 * 1000

    await autd.send_async(UpdateFlags(), timeout=timedelta(microseconds=1))

    assert autd.link.last_timeout_ns() == 1000

    await autd.send_async((UpdateFlags(), UpdateFlags()), timeout=timedelta(microseconds=2))

    assert autd.link.last_timeout_ns() == 2000

    await autd.send_async(Stop(), timeout=timedelta(microseconds=3))

    assert autd.link.last_timeout_ns() == 3000


@pytest.mark.asyncio()
async def test_send_async_single():
    autd = await create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)

    assert await autd.send_async(Static())

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)

    autd.link.down()
    assert not await autd.send_async(Static())

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        await autd.send_async(Static())
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_send_async_double():
    autd = await create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0)
        assert np.all(phases == 0)

    assert await autd.send_async((Static(), Uniform(0xFF)))

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0xFF)
        assert np.all(phases == 0)

    autd.link.down()
    assert not await autd.send_async((Static(), Uniform(0xFF)))

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        await autd.send_async((Static(), Uniform(0xFF)))
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_send_async_special():
    autd = await create_controller()

    assert await autd.send_async((Static(), Uniform(0xFF)))

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)
        intensities, _ = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0xFF)

    await autd.send_async(Stop())

    for dev in autd.geometry:
        intensities, _ = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0)

    autd.link.down()
    assert not await autd.send_async(Stop())

    autd.link.break_down()
    with pytest.raises(AUTDError) as e:
        await autd.send_async(Stop())
    assert str(e.value) == "broken"


@pytest.mark.asyncio()
async def test_group():
    autd = await create_controller()

    await autd.group(lambda dev: dev.idx).set_data(0, Static(), Null()).set_data(1, Sine(150), Uniform(0xFF)).send_async()

    mod = autd.link.modulation(0)
    assert len(mod) == 2
    assert np.all(mod == 0xFF)
    intensities, phases = autd.link.intensities_and_phases(0, 0)
    assert np.all(intensities == 0)
    assert np.all(phases == 0)

    mod = autd.link.modulation(1)
    assert len(mod) == 80
    intensities, phases = autd.link.intensities_and_phases(1, 0)
    assert np.all(intensities == 0xFF)
    assert np.all(phases == 0)

    await autd.group(lambda dev: dev.idx).set_data(1, Stop()).set_data(0, (Sine(150), Uniform(0xFF))).send_async()

    mod = autd.link.modulation(0)
    assert len(mod) == 80
    intensities, phases = autd.link.intensities_and_phases(0, 0)
    assert np.all(intensities == 0xFF)
    assert np.all(phases == 0)

    mod = autd.link.modulation(1)
    intensities, phases = autd.link.intensities_and_phases(1, 0)
    assert np.all(intensities == 0)


@pytest.mark.asyncio()
async def test_group_check_only_for_enabled():
    autd = await create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(autd.geometry.num_devices, dtype=bool)

    def f(dev: Device) -> int:
        check[dev.idx] = True
        return 0

    await autd.group(f).set_data(0, Sine(150), Uniform(0x80).with_phase(np.pi)).send_async()

    assert not check[0]
    assert check[1]

    mod = autd.link.modulation(0)
    intensities, phases = autd.link.intensities_and_phases(0, 0)
    assert np.all(intensities == 0)
    assert np.all(phases == 0)

    mod = autd.link.modulation(1)
    assert len(mod) == 80
    intensities, phases = autd.link.intensities_and_phases(1, 0)
    assert np.all(intensities == 0x80)
    assert np.all(phases == 128)


@pytest.mark.asyncio()
async def test_clear():
    autd = await create_controller()

    assert await autd.send_async((Static(), Uniform(0xFF).with_phase(np.pi)))

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0xFF)
        assert np.all(phases == 128)

    await autd.send_async(Clear())

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0)
        assert np.all(phases == 0)


@pytest.mark.asyncio()
async def test_stop():
    autd = await create_controller()

    assert await autd.send_async((Static(), Uniform(0xFF)))

    for dev in autd.geometry:
        assert np.all(autd.link.modulation(dev.idx) == 0xFF)
        intensities, _ = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0xFF)

    await autd.send_async(Stop())

    for dev in autd.geometry:
        intensities, _ = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0)


@pytest.mark.asyncio()
async def test_update_flags():
    autd = await create_controller()

    for dev in autd.geometry:
        dev.force_fan = True
        assert autd.link.fpga_flags(dev.idx) == 0

    await autd.send_async(UpdateFlags())

    for dev in autd.geometry:
        assert autd.link.fpga_flags(dev.idx) == 1


@pytest.mark.asyncio()
async def test_synchronize():
    autd = await Controller.builder().add_device(AUTD3([0.0, 0.0, 0.0])).add_device(AUTD3([0.0, 0.0, 0.0])).open_with_async(Audit.builder())

    assert await autd.send_async(Synchronize())


@pytest.mark.asyncio()
async def test_configure_mod_delay():
    autd = await create_controller()

    for dev in autd.geometry:
        assert np.all(autd.link.mod_delays(dev.idx) == 0)

    for dev in autd.geometry:
        for tr in dev:
            tr.mod_delay = 1

    assert await autd.send_async(ConfigureModDelay())

    for dev in autd.geometry:
        assert np.all(autd.link.mod_delays(dev.idx) == 1)
