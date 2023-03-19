// File: osal_timer.hpp
// Project: core
// Created Date: 11/05/2021
// Author: Shun Suzuki
// -----
// Last Modified: 19/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#if WIN32
#include "autd3/core/osal_timer/win32.hpp"
#elif __APPLE__
#include "autd3/core/osal_timer/macosx.hpp"
#else
#include "autd3/core/osal_timer/linux.hpp"
#endif

#include "autd3/core/osal_timer/callback.hpp"
#include "autd3/core/osal_timer/timer_strategy.hpp"
