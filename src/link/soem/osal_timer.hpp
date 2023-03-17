// File: osal_timer.hpp
// Project: core
// Created Date: 11/05/2021
// Author: Shun Suzuki
// -----
// Last Modified: 04/07/2021
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#if WIN32
#include "osal_timer/win32/timer.hpp"
#elif __APPLE__
#include "osal_timer/macosx/timer.hpp"
#else
#include "osal_timer/linux/timer.hpp"
#endif
