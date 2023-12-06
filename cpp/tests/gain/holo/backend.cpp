// File: backend.cpp
// Project: holo
// Created Date: 05/12/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/gain/holo/backend_nalgebra.hpp>

TEST(Gain_Holo, BackendNewDelete) {
  auto* backend = new autd3::gain::holo::NalgebraBackend;
  delete backend;
}