'''
File: gain.py
Project: internal
Created Date: 29/08/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from abc import ABCMeta, abstractmethod

from pyautd3.autd import Datagram
from pyautd3.geometry import Geometry

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import DatagramPtr, GainPtr


class IGain(Datagram, metaclass=ABCMeta):
    def __init__(self):
        super().__init__()

    def ptr(self, geometry: Geometry) -> DatagramPtr:
        return Base().gain_into_datagram(self.gain_ptr(geometry))

    @abstractmethod
    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        pass
