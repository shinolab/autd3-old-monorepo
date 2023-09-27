// File: log.cpp
// Project: link
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/internal/controller.hpp>
#include <autd3/link/debug.hpp>

void test_log_out(const char* msg) { std::cerr << msg; }
void test_log_flush() {}

TEST(Link, Log) {
  const auto autd = autd3::internal::Controller::builder()
                        .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                        .open_with(autd3::link::Debug()
                                       .with_log()
                                       .with_log_func(&test_log_out, &test_log_flush)
                                       .with_log_level(autd3::internal::native_methods::Level::Trace));

  autd.close();
}
