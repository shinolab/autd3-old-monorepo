'''
File: test_geometry.py
Project: tests
Created Date: 18/09/2023
Author: Shun Suzuki
-----
Last Modified: 19/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import numpy as np
from pyautd3 import AUTD3, UpdateFlags
from pyautd3.link import Audit

from .test_autd import create_controller


def test_autd3_props():
    assert AUTD3.trans_spacing() == 10.16
    assert AUTD3.device_width() == 192.0
    assert AUTD3.device_height() == 151.4
    assert AUTD3.num_trans_in_x() == 18
    assert AUTD3.num_trans_in_y() == 14
    assert AUTD3.num_trans_in_unit() == 249
    assert AUTD3.fpga_clk_freq() == 163.84e6
    assert AUTD3.fpga_sub_clk_freq() == 20.48e6


def test_geometry_num_devices():
    autd = create_controller()
    assert autd.geometry.num_devices == 2


def test_geometry_center():
    autd = create_controller()
    center = autd.geometry.center
    assert len(center) == 3
    assert center[0] == 86.62522088353406
    assert center[1] == 66.71325301204821
    assert center[2] == 0.0


def test_device_idx():
    autd = create_controller()
    assert autd.geometry[0].idx == 0
    assert autd.geometry[1].idx == 1


def test_device_sound_speed():
    autd = create_controller()
    for dev in autd.geometry:
        assert dev.sound_speed == 340e3
        dev.sound_speed = 350e3
        assert dev.sound_speed == 350e3


def test_device_set_sound_speed_from_temp():
    autd = create_controller()
    for dev in autd.geometry:
        dev.set_sound_speed_from_temp(15)
        assert dev.sound_speed == 340.2952640537549e3


def test_device_attenuation():
    autd = create_controller()
    for dev in autd.geometry:
        assert dev.attenuation == 0.0
        dev.attenuation = 1.0
        assert dev.attenuation == 1.0


def test_device_num_transducers():
    autd = create_controller()
    for dev in autd.geometry:
        assert dev.num_transducers == 249


def test_device_center():
    autd = create_controller()
    for dev in autd.geometry:
        center = dev.center
        assert len(center) == 3
        assert center[0] == 86.62522088353406
        assert center[1] == 66.71325301204821
        assert center[2] == 0.0


def test_device_force_fan():
    autd = create_controller()
    for dev in autd.geometry:
        assert Audit.fpga_flags(autd._ptr, dev.idx) == 0

    autd.geometry[0].force_fan = True
    autd.geometry[1].force_fan = False

    autd.send(UpdateFlags())

    assert Audit.fpga_flags(autd._ptr, 0) == 1
    assert Audit.fpga_flags(autd._ptr, 1) == 0

    autd.geometry[0].force_fan = False
    autd.geometry[1].force_fan = True

    autd.send(UpdateFlags())

    assert Audit.fpga_flags(autd._ptr, 0) == 0
    assert Audit.fpga_flags(autd._ptr, 1) == 1


def test_device_reads_fpga_info():
    autd = create_controller()
    for dev in autd.geometry:
        assert Audit.fpga_flags(autd._ptr, dev.idx) == 0

    autd.geometry[0].reads_fpga_info = True
    autd.geometry[1].reads_fpga_info = False

    autd.send(UpdateFlags())

    assert Audit.fpga_flags(autd._ptr, 0) == 2
    assert Audit.fpga_flags(autd._ptr, 1) == 0

    autd.geometry[0].reads_fpga_info = False
    autd.geometry[1].reads_fpga_info = True

    autd.send(UpdateFlags())

    assert Audit.fpga_flags(autd._ptr, 0) == 0
    assert Audit.fpga_flags(autd._ptr, 1) == 2


def test_device_translate():
    autd = create_controller()

    for dev in autd.geometry:
        original_pos = [tr.position for tr in dev]
        t = [1, 2, 3]
        dev.translate(t)
        for tr in dev:
            assert np.allclose(tr.position, original_pos[tr.local_idx] + t)


def test_device_rotate():
    autd = create_controller()

    for dev in autd.geometry:
        r = [0.70710678, 0., 0., 0.70710678]
        dev.rotate(r)
        for tr in dev:
            assert np.allclose(tr.rotation, r)


def test_device_affine():
    autd = create_controller()

    for dev in autd.geometry:
        original_pos = [tr.position for tr in dev]
        t = [1, 2, 3]
        r = [0.70710678, 0., 0., 0.70710678]
        dev.affine(t, r)
        for tr in dev:
            op = original_pos[tr.local_idx]
            expected = np.array([-op[1], op[0], op[2]]) + t
            assert np.allclose(tr.position, expected)
            assert np.allclose(tr.rotation, r)


def test_transducer_local_idx():
    autd = create_controller()

    for dev in autd.geometry:
        for i, tr in enumerate(dev):
            assert tr.local_idx == i


def test_transducer_position():
    autd = create_controller()

    assert np.allclose(autd.geometry[0][0].position, [0.0, 0.0, 0.0])
    assert np.allclose(autd.geometry[0][-1].position, [(AUTD3.num_trans_in_x() - 1) *
                       AUTD3.trans_spacing(), (AUTD3.num_trans_in_y() - 1) * AUTD3.trans_spacing(), 0.0])

    assert np.allclose(autd.geometry[1][0].position, [0.0, 0.0, 0.0])
    assert np.allclose(autd.geometry[1][-1].position, [(AUTD3.num_trans_in_x() - 1) *
                       AUTD3.trans_spacing(), (AUTD3.num_trans_in_y() - 1) * AUTD3.trans_spacing(), 0.0])


def test_transducer_rotation():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert np.allclose(tr.rotation, [1.0, 0.0, 0.0, 0.0])


def test_transducer_x_direction():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert np.allclose(tr.x_direction, [1.0, 0.0, 0.0])


def test_transducer_y_direction():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert np.allclose(tr.y_direction, [0.0, 1.0, 0.0])


def test_transducer_z_direction():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert np.allclose(tr.z_direction, [0.0, 0.0, 1.0])


def test_transducer_frequency():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert tr.frequency == 40e3
            tr.frequency = 69.98718496369073e3
            assert tr.frequency == 69.98718496369073e3


def test_transducer_cycle():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert tr.cycle == 4096
            tr.cycle = 3000
            assert tr.cycle == 3000


def test_transducer_mod_delay():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert tr.mod_delay == 0
            tr.mod_delay = 1
            assert tr.mod_delay == 1


def test_transducer_amp_filter():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert tr.amp_filter == 0
            tr.amp_filter = -1
            assert tr.amp_filter == -1


def test_transducer_phase_filter():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert tr.phase_filter == 0
            tr.phase_filter = -1
            assert tr.phase_filter == -1


def test_transducer_wavelength():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert tr.wavelength(340e3) == 340e3 / 40e3


def test_transducer_wavenum():
    autd = create_controller()

    for dev in autd.geometry:
        for tr in dev:
            assert tr.wavenumber(340e3) == 2.0 * np.pi * 40e3 / 340e3
