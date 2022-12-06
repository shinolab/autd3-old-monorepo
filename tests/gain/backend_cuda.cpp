// File: backend_cuda.cpp
// Project: gain
// Created Date: 02/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/gain/backend_cuda.hpp"

#include "backend_base.hpp"

AUTD3_BACKEND_TEST(GainBackendCUDA, autd3::gain::holo::CUDABackend)
