"""
File: autd_error.py
Project: pyautd3
Created Date: 28/05/2023
Author: Shun Suzuki
-----
Last Modified: 27/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

import ctypes


class AUTDError(Exception):
    """Exception class for pyautd3."""

    msg: str

    def __init__(self: "AUTDError", err: ctypes.Array[ctypes.c_char] | str) -> None:
        self.msg = err if isinstance(err, str) else err.value.decode("utf-8")

    def __str__(self: "AUTDError") -> str:
        return self.msg

    def __repr__(self: "AUTDError") -> str:
        return self.msg


class UnknownGroupKeyError(AUTDError):
    """Exception class for unknown group key."""

    def __init__(self: "UnknownGroupKeyError") -> None:
        super().__init__("Unknown group key")


class InvalidPlotConfigError(AUTDError):
    """Exception class for invalid plot config."""

    def __init__(self: "InvalidPlotConfigError") -> None:
        super().__init__("Invalid plot config type")
