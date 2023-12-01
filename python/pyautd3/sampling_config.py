"""
File: sampling_config.py
Project: pyautd3
Created Date: 22/11/2023
Author: Shun Suzuki
-----
Last Modified: 22/11/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from datetime import timedelta

from .internal.utils import _validate_sampling_config
from .native_methods.autd3capi_def import NativeMethods as Def
from .native_methods.autd3capi_def import SamplingConfiguration as _SamplingConfiguration


class SamplingConfiguration:
    """Sampling configuration."""

    _internal: _SamplingConfiguration

    def __new__(cls: type["SamplingConfiguration"]) -> "SamplingConfiguration":
        """DO NOT USE THIS CONSTRUCTOR."""
        raise NotImplementedError

    @classmethod
    def __private_new__(cls: type["SamplingConfiguration"], internal: _SamplingConfiguration) -> "SamplingConfiguration":
        ins = super().__new__(cls)
        ins._internal = internal
        return ins

    @staticmethod
    def from_frequency_division(value: int) -> "SamplingConfiguration":
        """Create by sampling frequency division."""
        return SamplingConfiguration.__private_new__(_validate_sampling_config(Def().sampling_config_from_frequency_division(value)))

    @staticmethod
    def from_frequency(f: float) -> "SamplingConfiguration":
        """Create by sampling frequency."""
        return SamplingConfiguration.__private_new__(_validate_sampling_config(Def().sampling_config_from_frequency(f)))

    @staticmethod
    def from_period(p: timedelta) -> "SamplingConfiguration":
        """Create by sampling period."""
        return SamplingConfiguration.__private_new__(
            _validate_sampling_config(
                Def().sampling_config_from_period(int(p.total_seconds() * 1000.0 * 1000.0 * 1000.0)),
            ),
        )

    @property
    def frequency_division(self: "SamplingConfiguration") -> int:
        """Frequency division."""
        return int(Def().sampling_config_frequency_division(self._internal))

    @property
    def frequency(self: "SamplingConfiguration") -> float:
        """Sampling frequency."""
        return float(Def().sampling_config_frequency(self._internal))

    @property
    def period(self: "SamplingConfiguration") -> timedelta:
        """Sampling period."""
        return timedelta(seconds=int(Def().sampling_config_period(self._internal)) / 1000.0 / 1000.0 / 1000.0)
