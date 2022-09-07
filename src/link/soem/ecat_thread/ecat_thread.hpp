// File: ecat_thread.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <limits>
#include <queue>
#include <string>
#include <vector>

extern "C" {
#include "./ethercat.h"
}
#include "error_handler.hpp"

#if WIN32
#include "win.hpp"
#elif __APPLE__
#include "mac.hpp"
#else
#include "unix.hpp"
#endif

#include "spdlog/spdlog.h"

namespace autd3::link {

inline int64_t ec_sync(const int64_t reftime, const int64_t cycletime, int64_t* integral) {
  auto delta = (reftime - 50000) % cycletime;
  if (delta > (cycletime / 2)) delta -= cycletime;
  if (delta > 0) *integral += 1;
  if (delta < 0) *integral -= 1;
  return -(delta / 100) - (*integral / 20);
}

inline void print_stats(const std::string& header, std::vector<int64_t> stats) {
  int64_t min = std::numeric_limits<int64_t>::max();
  int64_t max = std::numeric_limits<int64_t>::min();
  int64_t sum = 0;
  for (const auto s : stats) {
    min = std::min(min, s);
    max = std::max(max, s);
    sum += s;
  }
  int64_t ave = sum / stats.size();
  int64_t std = 0;
  for (const auto s : stats) std += (s - ave) * (s - ave);
  std = std::sqrt(static_cast<double>(std / stats.size()));
  spdlog::debug("{}: {}+/-{} (Max.{} Min.{}) [us]", header, ave / 1000, std / 1000.0, max / 1000, min / 1000);
}

using wait_func = void(const timespec&);

template <wait_func W>
void ecat_run_(std::atomic<bool>* is_open, bool* is_running, int32_t expected_wkc, int64_t cycletime_ns, std::mutex& mtx,
               std::queue<driver::TxDatagram>& send_queue, IOMap& io_map, std::function<void(std::string)> on_lost) {
  ecat_init();

#if WIN32
  const auto u_resolution = 1;
  timeBeginPeriod(u_resolution);

  auto* h_process = GetCurrentProcess();
  const auto priority = GetPriorityClass(h_process);
  SetPriorityClass(h_process, REALTIME_PRIORITY_CLASS);
#endif

  auto ts = ecat_setup(cycletime_ns);
  int64_t toff = 0;
  std::vector<int64_t> stats;
  constexpr size_t STATS_SIZE = 2000;
  stats.reserve(STATS_SIZE);
  auto start = std::chrono::high_resolution_clock::now();
  while (*is_running) {
    add_timespec(ts, cycletime_ns + toff);

    W(ts);

    if (spdlog::get_level() <= spdlog::level::debug) {
      auto now = std::chrono::high_resolution_clock::now();
      const auto itvl = std::chrono::duration_cast<std::chrono::nanoseconds>(now - start).count();
      stats.emplace_back(itvl);
      if (stats.size() == STATS_SIZE) {
        print_stats("EC send interval", stats);
        stats.clear();
        stats.reserve(STATS_SIZE);
      }
      start = now;
    }

    if (ec_slave[0].state != EC_STATE_OPERATIONAL) {
      spdlog::warn("EC_STATE changed: {}", ec_slave[0].state);
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

#if WIN32
  timeEndPeriod(u_resolution);
  SetPriorityClass(h_process, priority);
#endif
}

void ecat_run(const bool high_precision, std::atomic<bool>* is_open, bool* is_running, const int32_t expected_wkc, const int64_t cycletime_ns,
              std::mutex& mtx, std::queue<driver::TxDatagram>& send_queue, IOMap& io_map, std::function<void(std::string)> on_lost) {
  if (high_precision)
    ecat_run_<timed_wait_h>(is_open, is_running, expected_wkc, cycletime_ns, mtx, send_queue, io_map, on_lost);
  else
    ecat_run_<timed_wait>(is_open, is_running, expected_wkc, cycletime_ns, mtx, send_queue, io_map, on_lost);
}

}  // namespace autd3::link
