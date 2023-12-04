"""
File: test_sampling_config.py
Project: tests
Created Date: 25/11/2023
Author: Shun Suzuki
-----
Last Modified: 25/11/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from datetime import timedelta

import pytest

from pyautd3 import SamplingConfiguration
from pyautd3.autd_error import AUTDError
from pyautd3.native_methods.autd3capi_def import FPGA_CLK_FREQ

SAMPLING_FREQ_DIV_MIN = 512
SAMPLING_FREQ_DIV_MAX = 0xFFFFFFFF
FREQ_MIN = FPGA_CLK_FREQ / SAMPLING_FREQ_DIV_MAX
FREQ_MAX = FPGA_CLK_FREQ / SAMPLING_FREQ_DIV_MIN
PERIOD_MIN = int(1000000000 / FPGA_CLK_FREQ * SAMPLING_FREQ_DIV_MIN)
PERIOD_MAX = int(1000000000 / FPGA_CLK_FREQ * SAMPLING_FREQ_DIV_MAX)


def test_sampl_config_from_freq_div():
    config = SamplingConfiguration.from_frequency_division(SAMPLING_FREQ_DIV_MIN)
    assert config.frequency_division == 512
    assert config.frequency == 40e3
    assert config.period == timedelta(microseconds=25)

    with pytest.raises(AUTDError) as e:
        config = SamplingConfiguration.from_frequency_division(SAMPLING_FREQ_DIV_MIN - 1)
    assert e.value.msg == "Sampling frequency division (511) is out of range ([512, 4294967295])"


def test_sampl_config_from_freq():
    config = SamplingConfiguration.from_frequency(40e3)
    assert config.frequency_division == 512
    assert config.frequency == 40e3
    assert config.period == timedelta(microseconds=25)

    with pytest.raises(AUTDError) as e:
        config = SamplingConfiguration.from_frequency(FREQ_MIN - 0.1)
    assert e.value.msg == "Sampling frequency (-0.09523162841685853) is out of range ([0.004768371583141473, 40000])"

    with pytest.raises(AUTDError) as e:
        config = SamplingConfiguration.from_frequency(FREQ_MAX + 0.1)
    assert e.value.msg == "Sampling frequency (40000.1) is out of range ([0.004768371583141473, 40000])"


def test_sampl_config_from_period():
    config = SamplingConfiguration.from_period(timedelta(microseconds=25))
    assert config.frequency_division == 512
    assert config.frequency == 40e3
    assert config.period == timedelta(microseconds=25)

    with pytest.raises(AUTDError) as e:
        config = SamplingConfiguration.from_period(timedelta(microseconds=PERIOD_MIN / 1000 - 1))
    assert e.value.msg == "Sampling period (24000 ns) is out of range ([25000, 209715199951])"

    with pytest.raises(AUTDError) as e:
        config = SamplingConfiguration.from_period(timedelta(microseconds=PERIOD_MAX / 1000 + 1))
    assert e.value.msg == "Sampling period (209715201000 ns) is out of range ([25000, 209715199951])"


def test_sampling_config_ctr():
    with pytest.raises(NotImplementedError):
        _ = SamplingConfiguration()
