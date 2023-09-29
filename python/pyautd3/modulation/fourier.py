'''
File: fourier.py
Project: modulation
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from functools import reduce
import functools
from typing import Iterable

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr
from pyautd3.internal.modulation import IModulation
from .sine import Sine


class Fourier(IModulation):
    """Multi-frequency sine wave modulation

    """

    _components: list[Sine]

    def __init__(self):
        super().__init__()
        self._components = []

    def add_component(self, component: Sine) -> "Fourier":
        """Add a sine wave component

        Arguments:
        - `component` - `Sine` modulation
        """

        self._components.append(component)
        return self

    def add_components_from_iter(self, components: Iterable[Sine]) -> "Fourier":
        """Add sine wave components from iterable

        Arguments:
        - `components` - Iterable of `Sine` modulations
        """

        return functools.reduce(lambda acc, x: acc.add_component(x), components, self)

    def __add__(self, rhs: Sine) -> "Fourier":
        """Add a sine wave component

        Arguments:
        - `rhs` - `Sine` modulation
        """

        return self.add_component(rhs)

    def modulation_ptr(self) -> ModulationPtr:
        return reduce(
            lambda acc, s: Base().modulation_fourier_add_component(
                acc, s.modulation_ptr()
            ),
            self._components,
            Base().modulation_fourier(),
        )
