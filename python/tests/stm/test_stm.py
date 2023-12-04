"""
File: test_stm.py
Project: stm
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 11/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

from datetime import timedelta

import numpy as np
import pytest

from pyautd3 import AUTD3, Controller, SamplingConfiguration
from pyautd3.gain import Uniform
from pyautd3.link.audit import Audit
from pyautd3.stm import FocusSTM, GainSTM, GainSTMMode
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_focus_stm():
    autd = await create_controller()

    radius = 30.0
    size = 2
    center = np.array([0.0, 0.0, 150.0])
    stm = FocusSTM(1.0).add_foci_from_iter(
        center + radius * np.array([np.cos(theta), np.sin(theta), 0]) for theta in (2.0 * np.pi * i / size for i in range(size))
    )
    assert await autd.send_async(stm)
    for dev in autd.geometry:
        assert not autd.link.is_stm_gain_mode(dev.idx)
    assert stm.frequency == 1.0
    assert stm.period == timedelta(microseconds=1000000)
    assert stm.sampling_config.frequency == 2 * 1.0
    assert stm.sampling_config.frequency_division == 10240000
    assert stm.sampling_config.period == timedelta(microseconds=500000)
    for dev in autd.geometry:
        assert autd.link.stm_freqency_division(dev.idx) == 10240000
    assert stm.start_idx is None
    assert stm.finish_idx is None
    for dev in autd.geometry:
        assert autd.link.stm_start_idx(dev.idx) == -1
        assert autd.link.stm_finish_idx(dev.idx) == -1
    stm = stm.with_start_idx(0)
    assert stm.start_idx == 0
    assert stm.finish_idx is None
    assert await autd.send_async(stm)
    for dev in autd.geometry:
        assert autd.link.stm_start_idx(dev.idx) == 0
        assert autd.link.stm_finish_idx(dev.idx) == -1
    stm = stm.with_start_idx(None).with_finish_idx(0)
    assert stm.start_idx is None
    assert stm.finish_idx == 0
    assert await autd.send_async(stm)
    for dev in autd.geometry:
        assert autd.link.stm_start_idx(dev.idx) == -1
        assert autd.link.stm_finish_idx(dev.idx) == 0

    stm = FocusSTM.from_period(timedelta(microseconds=1000000)).add_focus(center).add_focus(center)
    assert await autd.send_async(stm)
    assert stm.frequency == 1
    assert stm.period == timedelta(microseconds=1000000)
    assert stm.sampling_config.frequency == 2 * 1.0
    assert stm.sampling_config.frequency_division == 10240000
    assert stm.sampling_config.period == timedelta(microseconds=500000)
    for dev in autd.geometry:
        assert autd.link.stm_freqency_division(dev.idx) == 10240000
    for dev in autd.geometry:
        assert autd.link.stm_cycle(dev.idx) == 2
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert not np.all(intensities == 0)
        assert not np.all(phases == 0)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 1)
        assert not np.all(intensities == 0)
        assert not np.all(phases == 0)

    stm = FocusSTM.from_sampling_config(SamplingConfiguration.from_frequency_division(512)).add_focus(center).add_focus(center)
    assert await autd.send_async(stm)
    assert stm.frequency == 20000.0
    assert stm.sampling_config.frequency == 2 * 20000.0
    assert stm.sampling_config.frequency_division == 512
    assert stm.sampling_config.period == timedelta(microseconds=25)
    for dev in autd.geometry:
        assert autd.link.stm_freqency_division(dev.idx) == 512
    for dev in autd.geometry:
        assert autd.link.stm_cycle(dev.idx) == 2
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert not np.all(intensities == 0)
        assert not np.all(phases == 0)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 1)
        assert not np.all(intensities == 0)
        assert not np.all(phases == 0)

    stm = FocusSTM.from_sampling_config(SamplingConfiguration.from_period(timedelta(microseconds=25))).add_focus(center).add_focus(center)
    assert await autd.send_async(stm)
    assert stm.frequency == 20000.0
    assert stm.sampling_config.frequency == 2 * 20000.0
    assert stm.sampling_config.frequency_division == 512
    assert stm.sampling_config.period == timedelta(microseconds=25)
    for dev in autd.geometry:
        assert autd.link.stm_freqency_division(dev.idx) == 512
    for dev in autd.geometry:
        assert autd.link.stm_cycle(dev.idx) == 2
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert not np.all(intensities == 0)
        assert not np.all(phases == 0)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 1)
        assert not np.all(intensities == 0)
        assert not np.all(phases == 0)


@pytest.mark.asyncio()
async def test_gain_stm():
    autd: Controller[Audit] = (
        await Controller.builder().add_device(AUTD3([0.0, 0.0, 0.0])).add_device(AUTD3([0.0, 0.0, 0.0])).open_with_async(Audit.builder())
    )

    size = 2
    stm = GainSTM(1.0).add_gains_from_iter(Uniform(0xFF // (i + 1)) for i in range(size))
    assert await autd.send_async(stm)
    for dev in autd.geometry:
        assert autd.link.is_stm_gain_mode(dev.idx)
    assert stm.frequency == 1.0
    assert stm.period == timedelta(microseconds=1000000)
    assert stm.sampling_config.frequency == 2 * 1.0
    assert stm.sampling_config.frequency_division == 10240000
    assert stm.sampling_config.period == timedelta(microseconds=500000)
    for dev in autd.geometry:
        assert autd.link.stm_freqency_division(dev.idx) == 10240000
    assert stm.start_idx is None
    assert stm.finish_idx is None
    for dev in autd.geometry:
        assert autd.link.stm_start_idx(dev.idx) == -1
        assert autd.link.stm_finish_idx(dev.idx) == -1
    stm = stm.with_start_idx(0)
    assert stm.start_idx == 0
    assert stm.finish_idx is None
    assert await autd.send_async(stm)
    for dev in autd.geometry:
        assert autd.link.stm_start_idx(dev.idx) == 0
        assert autd.link.stm_finish_idx(dev.idx) == -1
    stm = stm.with_start_idx(None).with_finish_idx(0)
    assert stm.start_idx is None
    assert stm.finish_idx == 0
    assert await autd.send_async(stm)
    for dev in autd.geometry:
        assert autd.link.stm_start_idx(dev.idx) == -1
        assert autd.link.stm_finish_idx(dev.idx) == 0

    stm = GainSTM.from_period(timedelta(microseconds=1000000)).add_gain(Uniform(0xFF)).add_gain(Uniform(0x80))
    assert await autd.send_async(stm)
    assert stm.frequency == 1
    assert stm.period == timedelta(microseconds=1000000)
    assert stm.sampling_config.frequency == 2 * 1.0
    assert stm.sampling_config.frequency_division == 10240000
    assert stm.sampling_config.period == timedelta(microseconds=500000)
    for dev in autd.geometry:
        assert autd.link.stm_freqency_division(dev.idx) == 10240000
    for dev in autd.geometry:
        assert autd.link.stm_cycle(dev.idx) == 2
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0xFF)
        assert np.all(phases == 0)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 1)
        assert np.all(intensities == 0x80)
        assert np.all(phases == 0)

    stm = GainSTM.from_sampling_config(SamplingConfiguration.from_frequency_division(512)).add_gain(Uniform(0xFF)).add_gain(Uniform(0x80))
    assert await autd.send_async(stm)
    assert stm.frequency == 20000.0
    assert stm.sampling_config.frequency == 2 * 20000.0
    assert stm.sampling_config.frequency_division == 512
    assert stm.sampling_config.period == timedelta(microseconds=25)
    for dev in autd.geometry:
        assert autd.link.stm_freqency_division(dev.idx) == 512
    for dev in autd.geometry:
        assert autd.link.stm_cycle(dev.idx) == 2
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0xFF)
        assert np.all(phases == 0)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 1)
        assert np.all(intensities == 0x80)
        assert np.all(phases == 0)

    stm = GainSTM.from_sampling_config(SamplingConfiguration.from_period(timedelta(microseconds=25))).add_gain(Uniform(0xFF)).add_gain(Uniform(0x80))
    assert await autd.send_async(stm)
    assert stm.frequency == 20000.0
    assert stm.sampling_config.frequency == 2 * 20000.0
    assert stm.sampling_config.frequency_division == 512
    assert stm.sampling_config.period == timedelta(microseconds=25)
    for dev in autd.geometry:
        assert autd.link.stm_freqency_division(dev.idx) == 512
    for dev in autd.geometry:
        assert autd.link.stm_cycle(dev.idx) == 2
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0xFF)
        assert np.all(phases == 0)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 1)
        assert np.all(intensities == 0x80)
        assert np.all(phases == 0)

    stm = stm.with_mode(GainSTMMode.PhaseFull)
    assert await autd.send_async(stm)
    for dev in autd.geometry:
        assert autd.link.stm_cycle(dev.idx) == 2
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0xFF)
        assert np.all(phases == 0)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 1)
        assert np.all(intensities == 0xFF)
        assert np.all(phases == 0)

    stm = stm.with_mode(GainSTMMode.PhaseHalf)
    assert await autd.send_async(stm)
    for dev in autd.geometry:
        assert autd.link.stm_cycle(dev.idx) == 2
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0xFF)
        assert np.all(phases == 0)
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 1)
        assert np.all(intensities == 0xFF)
        assert np.all(phases == 0)
