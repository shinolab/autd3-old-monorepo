"""
File: test_geometry.py
Project: tests
Created Date: 18/09/2023
Author: Shun Suzuki
-----
Last Modified: 11/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import numpy as np
import pytest
from numpy.typing import ArrayLike

from pyautd3 import AUTD3, Controller
from pyautd3.geometry import Angle, EulerAngles, deg, rad
from pyautd3.link.audit import Audit

from .test_autd import create_controller


def test_angle():
    assert (np.pi / 2 * rad).radian == np.pi / 2
    assert (90 * deg).radian == np.pi / 2


def test_angle_ctr():
    with pytest.raises(NotImplementedError):
        _ = Angle()

    with pytest.raises(NotImplementedError):
        _ = Angle._UnitDegree()

    with pytest.raises(NotImplementedError):
        _ = Angle._UnitRad()

    with pytest.raises(NotImplementedError):
        _ = EulerAngles()


@pytest.mark.asyncio()
async def test_with_rotation():
    async def open_with_rotation(q: ArrayLike) -> Controller[Audit]:
        return await Controller[Audit].builder().add_device(AUTD3([0.0, 0.0, 0.0]).with_rotation(q)).open_with_async(Audit.builder())

    autd = await open_with_rotation(EulerAngles.from_zyz(90 * deg, 0 * deg, 0 * deg))
    assert np.allclose(autd.geometry[0][0].x_direction, [0.0, 1.0, 0.0])
    assert np.allclose(autd.geometry[0][0].y_direction, [-1.0, 0.0, 0.0])
    assert np.allclose(autd.geometry[0][0].z_direction, [0.0, 0.0, 1.0])

    autd = await open_with_rotation(EulerAngles.from_zyz(0 * deg, 90 * deg, 0 * deg))
    assert np.allclose(autd.geometry[0][0].x_direction, [0.0, 0.0, -1.0])
    assert np.allclose(autd.geometry[0][0].y_direction, [0.0, 1.0, 0.0])
    assert np.allclose(autd.geometry[0][0].z_direction, [1.0, 0.0, 0.0])

    autd = await open_with_rotation(EulerAngles.from_zyz(0 * deg, 0 * deg, 90 * deg))
    assert np.allclose(autd.geometry[0][0].x_direction, [0.0, 1.0, 0.0])
    assert np.allclose(autd.geometry[0][0].y_direction, [-1.0, 0.0, 0.0])
    assert np.allclose(autd.geometry[0][0].z_direction, [0.0, 0.0, 1.0])

    autd = await open_with_rotation(EulerAngles.from_zyz(0 * deg, 90 * deg, 90 * deg))
    assert np.allclose(autd.geometry[0][0].x_direction, [0.0, 1.0, 0.0])
    assert np.allclose(autd.geometry[0][0].y_direction, [0.0, 0.0, 1.0])
    assert np.allclose(autd.geometry[0][0].z_direction, [1.0, 0.0, 0.0])

    autd = await open_with_rotation(EulerAngles.from_zyz(90 * deg, 90 * deg, 0 * deg))
    assert np.allclose(autd.geometry[0][0].x_direction, [0.0, 0.0, -1.0])
    assert np.allclose(autd.geometry[0][0].y_direction, [-1.0, 0.0, 0.0])
    assert np.allclose(autd.geometry[0][0].z_direction, [0.0, 1.0, 0.0])


def test_autd3_props():
    assert AUTD3.transducer_spacing() == 10.16
    assert AUTD3.device_width() == 192.0
    assert AUTD3.device_height() == 151.4
    assert AUTD3.num_transducer_in_x() == 18
    assert AUTD3.num_transducer_in_y() == 14
    assert AUTD3.num_transducer_in_unit() == 249
    assert AUTD3.fpga_clk_freq() == 20.48e6


@pytest.mark.asyncio()
async def test_geometry_num_devices():
    autd = await create_controller()
    assert autd.geometry.num_devices == 2


@pytest.mark.asyncio()
async def test_geometry_center():
    autd = await create_controller()
    center = autd.geometry.center
    assert len(center) == 3
    assert center[0] == 86.62522088353406
    assert center[1] == 66.71325301204821
    assert center[2] == 0.0


@pytest.mark.asyncio()
async def test_device_idx():
    autd = await create_controller()
    assert autd.geometry[0].idx == 0
    assert autd.geometry[1].idx == 1


@pytest.mark.asyncio()
async def test_device_sound_speed():
    autd = await create_controller()
    for dev in autd.geometry:
        assert dev.sound_speed == 340e3
        dev.sound_speed = 350e3
        assert dev.sound_speed == 350e3


@pytest.mark.asyncio()
async def test_device_set_sound_speed_from_temp():
    autd = await create_controller()
    for dev in autd.geometry:
        dev.set_sound_speed_from_temp(15)
        assert dev.sound_speed == 340.2952640537549e3

    autd.geometry.set_sound_speed(350e3)
    for dev in autd.geometry:
        assert dev.sound_speed == 350e3

    autd.geometry.set_sound_speed_from_temp(15)
    for dev in autd.geometry:
        assert dev.sound_speed == 340.2952640537549e3


@pytest.mark.asyncio()
async def test_device_attenuation():
    autd = await create_controller()
    for dev in autd.geometry:
        assert dev.attenuation == 0.0
        dev.attenuation = 1.0
        assert dev.attenuation == 1.0


@pytest.mark.asyncio()
async def test_device_enable():
    autd = await create_controller()
    for dev in autd.geometry:
        assert dev.enable
        dev.enable = False
        assert not dev.enable


@pytest.mark.asyncio()
async def test_device_num_transducers():
    autd = await create_controller()
    assert autd.geometry.num_transducers == 249 * autd.geometry.num_devices
    for dev in autd.geometry:
        assert dev.num_transducers == 249


@pytest.mark.asyncio()
async def test_device_center():
    autd = await create_controller()
    for dev in autd.geometry:
        center = dev.center
        assert len(center) == 3
        assert center[0] == 86.62522088353406
        assert center[1] == 66.71325301204821
        assert center[2] == 0.0


@pytest.mark.asyncio()
async def test_device_translate():
    autd = await create_controller()

    for dev in autd.geometry:
        original_pos = [tr.position for tr in dev]
        t = [1, 2, 3]
        dev.translate(t)
        for tr in dev:
            assert np.allclose(tr.position, original_pos[tr.idx] + t)


@pytest.mark.asyncio()
async def test_device_rotate():
    autd = await create_controller()

    for dev in autd.geometry:
        r = [0.70710678, 0.0, 0.0, 0.70710678]
        dev.rotate(r)
        for tr in dev:
            assert np.allclose(tr.rotation, r)


@pytest.mark.asyncio()
async def test_device_affine():
    autd = await create_controller()

    for dev in autd.geometry:
        original_pos = [tr.position for tr in dev]
        t = [1, 2, 3]
        r = [0.70710678, 0.0, 0.0, 0.70710678]
        dev.affine(t, r)
        for tr in dev:
            op = original_pos[tr.idx]
            expected = np.array([-op[1], op[0], op[2]]) + t
            assert np.allclose(tr.position, expected)
            assert np.allclose(tr.rotation, r)


@pytest.mark.asyncio()
async def test_transducer_idx():
    autd = await create_controller()

    for dev in autd.geometry:
        for i, tr in enumerate(dev):
            assert tr.idx == i


@pytest.mark.asyncio()
async def test_transducer_position():
    autd = await create_controller()

    assert np.allclose(autd.geometry[0][0].position, [0.0, 0.0, 0.0])
    assert np.allclose(
        autd.geometry[0][-1].position,
        [(AUTD3.num_transducer_in_x() - 1) * AUTD3.transducer_spacing(), (AUTD3.num_transducer_in_y() - 1) * AUTD3.transducer_spacing(), 0.0],
    )

    assert np.allclose(autd.geometry[1][0].position, [0.0, 0.0, 0.0])
    assert np.allclose(
        autd.geometry[1][-1].position,
        [(AUTD3.num_transducer_in_x() - 1) * AUTD3.transducer_spacing(), (AUTD3.num_transducer_in_y() - 1) * AUTD3.transducer_spacing(), 0.0],
    )


@pytest.mark.asyncio()
async def test_transducer_rotation():
    autd = await create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert np.allclose(tr.rotation, [1.0, 0.0, 0.0, 0.0])


@pytest.mark.asyncio()
async def test_transducer_x_direction():
    autd = await create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert np.allclose(tr.x_direction, [1.0, 0.0, 0.0])


@pytest.mark.asyncio()
async def test_transducer_y_direction():
    autd = await create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert np.allclose(tr.y_direction, [0.0, 1.0, 0.0])


@pytest.mark.asyncio()
async def test_transducer_z_direction():
    autd = await create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert np.allclose(tr.z_direction, [0.0, 0.0, 1.0])


@pytest.mark.asyncio()
async def test_transducer_wavelength():
    autd = await create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert tr.wavelength(340e3) == 340e3 / 40e3


@pytest.mark.asyncio()
async def test_transducer_wavenum():
    autd = await create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert tr.wavenumber(340e3) == 2.0 * np.pi * 40e3 / 340e3
