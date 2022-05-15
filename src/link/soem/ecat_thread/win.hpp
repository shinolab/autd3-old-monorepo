// File: win.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <queue>
#include <string>

extern "C" {
#include "./ethercat.h"
#include "./osal_win32.h"
}
#include "error_handler.hpp"
#include "utils.hpp"

namespace autd3::link {

static LARGE_INTEGER PERFORMANCE_FREQUENCY{};

inline void ecat_init() {
  LARGE_INTEGER f;
  QueryPerformanceFrequency(&f);
  PERFORMANCE_FREQUENCY = f;
}

inline void nanosleep(const int64_t t) {
  LARGE_INTEGER start;
  QueryPerformanceCounter(&start);

  const auto sleep = t * PERFORMANCE_FREQUENCY.QuadPart / (1000L * 1000L * 1000L);
  while (true) {
    LARGE_INTEGER now;
    QueryPerformanceCounter(&now);
    if (now.QuadPart - start.QuadPart > sleep) break;
  }
}

inline void add_timespec(timespec& ts, const int64_t addtime) {
  const auto nsec = addtime % 1000000000;
  const auto sec = (addtime - nsec) / 1000000000;
  ts.tv_sec += sec;
  ts.tv_nsec += nsec;
  if (ts.tv_nsec >= 1000000000) {
    const auto nsec_ = ts.tv_nsec % 1000000000;
    ts.tv_sec += (ts.tv_nsec - nsec_) / 1000000000;
    ts.tv_nsec = nsec_;
  }
}

void timed_wait(const timespec& abs_time) {
  auto tp = timeval{0, 0};
  osal_gettimeofday(&tp, nullptr);

  const auto sleep = (abs_time.tv_sec - tp.tv_sec) * 1000000000 + (abs_time.tv_nsec - tp.tv_usec * 1000);

  if (sleep > 0) std::this_thread::sleep_for(std::chrono::nanoseconds(sleep));
}

void timed_wait_h(const timespec& abs_time) {
  auto tp = timeval{0, 0};
  osal_gettimeofday(&tp, nullptr);

  const auto sleep = (abs_time.tv_sec - tp.tv_sec) * 1000000000 + (abs_time.tv_nsec - tp.tv_usec * 1000);

  if (sleep > 0) nanosleep(sleep);
}

void ecat_run_(std::atomic<bool>* is_open, bool* is_running, int32_t expected_wkc, int64_t cycletime_ns, std::mutex& mtx,
               std::queue<driver::TxDatagram>& send_queue, IOMap& io_map, std::function<void(std::string)> on_lost) {
  const auto u_resolution = 1;
  timeBeginPeriod(u_resolution);

  auto* h_process = GetCurrentProcess();
  const auto priority = GetPriorityClass(h_process);
  SetPriorityClass(h_process, REALTIME_PRIORITY_CLASS);

  auto ts = timespec{0, 0};

  auto tp = timeval{0, 0};
  osal_gettimeofday(&tp, nullptr);

  const auto cyctime_us = cycletime_ns / 1000;

  ts.tv_sec = tp.tv_sec;
  const auto ht = ((tp.tv_usec / cyctime_us) + 1) * cyctime_us;
  ts.tv_nsec = ht * 1000;

  int64_t toff = 0;
  while (*is_running) {
    add_timespec(ts, cycletime_ns + toff);

    timed_wait(ts);

    if (ec_slave[0].state == EC_STATE_SAFE_OP) {
      ec_slave[0].state = EC_STATE_OPERATIONAL;
      ec_writestate(0);
    }

    if (!send_queue.empty()) {
      std::lock_guard<std::mutex> lock(mtx);
      io_map.copy_from(send_queue.front());
      send_queue.pop();
    }

    ec_send_processdata();
    if (ec_receive_processdata(EC_TIMEOUTRET) != expected_wkc && !error_handle(is_open, on_lost)) return;

    ec_sync(ec_DCtime, cycletime_ns, &toff);
  }

  timeEndPeriod(1);
  SetPriorityClass(h_process, priority);
}

void ecat_run_h(std::atomic<bool>* is_open, bool* is_running, int32_t expected_wkc, int64_t cycletime_ns, std::mutex& mtx,
                std::queue<driver::TxDatagram>& send_queue, IOMap& io_map, std::function<void(std::string)> on_lost) {
  const auto u_resolution = 1;
  timeBeginPeriod(u_resolution);

  auto* h_process = GetCurrentProcess();
  const auto priority = GetPriorityClass(h_process);
  SetPriorityClass(h_process, REALTIME_PRIORITY_CLASS);

  auto ts = timespec{0, 0};

  auto tp = timeval{0, 0};
  osal_gettimeofday(&tp, nullptr);

  const auto cyctime_us = cycletime_ns / 1000;

  ts.tv_sec = tp.tv_sec;
  const auto ht = ((tp.tv_usec / cyctime_us) + 1) * cyctime_us;
  ts.tv_nsec = ht * 1000;

  int64_t toff = 0;
  while (*is_running) {
    add_timespec(ts, cycletime_ns + toff);

    timed_wait_h(ts);

    if (ec_slave[0].state == EC_STATE_SAFE_OP) {
      ec_slave[0].state = EC_STATE_OPERATIONAL;
      ec_writestate(0);
    }

    if (!send_queue.empty()) {
      std::lock_guard<std::mutex> lock(mtx);
      io_map.copy_from(send_queue.front());
      send_queue.pop();
    }

    ec_send_processdata();

    if (ec_receive_processdata(EC_TIMEOUTRET) != expected_wkc && !error_handle(is_open, on_lost)) return;

    ec_sync(ec_DCtime, cycletime_ns, &toff);
  }

  timeEndPeriod(1);
  SetPriorityClass(h_process, priority);
}

void ecat_run(bool high_precision, std::atomic<bool>* is_open, bool* is_running, int32_t expected_wkc, int64_t cycletime_ns, std::mutex& mtx,
              std::queue<driver::TxDatagram>& send_queue, IOMap& io_map, std::function<void(std::string)> on_lost) {
  if (high_precision)
    ecat_run_h(is_open, is_running, expected_wkc, cycletime_ns, mtx, send_queue, io_map, on_lost);
  else
    ecat_run_(is_open, is_running, expected_wkc, cycletime_ns, mtx, send_queue, io_map, on_lost);
}

}  // namespace autd3::link
