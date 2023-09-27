// File: soem.cpp
// Project: link
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 28/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/internal/controller.hpp>
#include <autd3/link/soem.hpp>

#ifdef RUN_LINK_SOEM
[[noreturn]] void test_soem_on_lost(const char* msg) {
  std::cerr << msg;
#ifdef __APPLE__
  exit(-1);
#else
  std::quick_exit(-1);
#endif
}
void test_soem_log_out(const char* msg) { std::cerr << msg; }
void test_soem_log_flush() {}

TEST(Link, SOEM) {
  auto link = autd3::link::SOEM()
                  .with_ifname("")
                  .with_buf_size(32)
                  .with_send_cycle(2)
                  .with_sync0_cycle(2)
                  .with_on_lost(&test_soem_on_lost)
                  .with_timer_strategy(autd3::internal::native_methods::TimerStrategy::Sleep)
                  .with_sync_mode(autd3::internal::native_methods::SyncMode::FreeRun)
                  .with_state_check_interval(std::chrono::milliseconds(100))
                  .with_log_level(autd3::internal::native_methods::Level::Off)
                  .with_log_func(&test_soem_log_out, &test_soem_log_flush)
                  .with_timeout(std::chrono::milliseconds(200));

  auto autd = autd3::internal::Controller::builder()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                  .open_with(std::move(link));

  autd.close();
}
#endif

#ifdef RUN_LINK_REMOTE_SOEM
TEST(Link, RemoteSOEM) {
  auto link = autd3::link::RemoteSOEM("127.0.0.1:8080").with_timeout(std::chrono::milliseconds(200));
  auto autd = autd3::internal::Controller::builder()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                  .open_with(std::move(link));

  autd.close();
}
#endif
