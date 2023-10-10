'''
File: link.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''

from abc import ABCMeta, abstractmethod

from pyautd3.native_methods.autd3capi_def import LinkBuilderPtr


class LinkBuilder(metaclass=ABCMeta):
    @abstractmethod
    def _ptr(self) -> LinkBuilderPtr:
        pass

    def _resolve_link(self, obj):
        pass
