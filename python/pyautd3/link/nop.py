"""
File: nop.py
Project: link
Created Date: 10/10/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pyautd3.internal.link import LinkBuilder
from pyautd3.native_methods.autd3capi import (
    NativeMethods as LinkNop,
)
from pyautd3.native_methods.autd3capi_def import LinkBuilderPtr


class Nop:
    """Link which do nothing."""

    class _Builder(LinkBuilder):
        def __init__(self: "Nop._Builder") -> None:
            pass

        def _link_builder_ptr(self: "Nop._Builder") -> LinkBuilderPtr:
            return LinkNop().link_nop()

    @staticmethod
    def builder() -> _Builder:
        """Create Nop link builder."""
        return Nop._Builder()
