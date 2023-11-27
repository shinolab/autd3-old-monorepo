"""
File: amplitude.py
Project: holo
Created Date: 23/11/2023
Author: Shun Suzuki
-----
Last Modified: 23/11/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class Amplitude:
    """Amplitude in pascal."""

    _value: float

    def __new__(cls: type["Amplitude"]) -> "Amplitude":
        """DO NOT USE THIS CONSTRUCTOR."""
        raise NotImplementedError

    @classmethod
    def __private_new__(cls: type["Amplitude"], value: float) -> "Amplitude":
        ins = super().__new__(cls)
        ins._value = value
        return ins

    @staticmethod
    def new_pascal(value: float) -> "Amplitude":
        """Create by pascal."""
        return Amplitude.__private_new__(value)

    @staticmethod
    def new_spl(value: float) -> "Amplitude":
        """Create by sound pressure level."""
        return Amplitude.__private_new__(float(GainHolo().gain_holo_spl_to_pascal(value)))

    @property
    def pascal(self: "Amplitude") -> float:
        """Amplitude in pascal."""
        return self._value

    @property
    def spl(self: "Amplitude") -> float:
        """Amplitude in sound pressure level."""
        return float(GainHolo().gain_holo_pascal_to_spl(self._value))

    class _UnitPascal:
        def __new__(cls: type["Amplitude._UnitPascal"]) -> "Amplitude._UnitPascal":
            """DO NOT USE THIS CONSTRUCTOR."""
            raise NotImplementedError

        @classmethod
        def __private_new__(cls: type["Amplitude._UnitPascal"]) -> "Amplitude._UnitPascal":
            return super().__new__(cls)

        def __rmul__(self: "Amplitude._UnitPascal", other: float) -> "Amplitude":
            return Amplitude.new_pascal(other)

    class _UnitSPL:
        def __new__(cls: type["Amplitude._UnitSPL"]) -> "Amplitude._UnitSPL":
            """DO NOT USE THIS CONSTRUCTOR."""
            raise NotImplementedError

        @classmethod
        def __private_new__(cls: type["Amplitude._UnitSPL"]) -> "Amplitude._UnitSPL":
            return super().__new__(cls)

        def __rmul__(self: "Amplitude._UnitSPL", other: float) -> "Amplitude":
            return Amplitude.new_spl(other)


pascal: Amplitude._UnitPascal = Amplitude._UnitPascal.__private_new__()
dB: Amplitude._UnitSPL = Amplitude._UnitSPL.__private_new__()  # noqa: N816
