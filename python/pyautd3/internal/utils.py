"""
File: utils.py
Project: internal
Created Date: 12/11/2023
Author: Shun Suzuki
-----
Last Modified: 12/11/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

import ctypes

from pyautd3.autd_error import AUTDError
from pyautd3.native_methods.autd3capi_def import AUTD3_ERR, ResultI32, ResultSamplingConfig, SamplingConfiguration
from pyautd3.native_methods.autd3capi_def import NativeMethods as Def


def _validate_int(res: ResultI32) -> int:
    if int(res.result) == AUTD3_ERR:
        err = ctypes.create_string_buffer(int(res.err_len))
        Def().get_err(res.err, err)
        raise AUTDError(err)
    return int(res.result)


def _validate_sampling_config(res: ResultSamplingConfig) -> SamplingConfiguration:
    if int(res.result.div) == 0:
        err = ctypes.create_string_buffer(int(res.err_len))
        Def().get_err(res.err, err)
        raise AUTDError(err)
    return res.result


def _validate_ptr(res):  # noqa: ANN001, ANN202
    if res.result._0 is None:
        err = ctypes.create_string_buffer(int(res.err_len))
        Def().get_err(res.err, err)
        raise AUTDError(err)
    return res.result
