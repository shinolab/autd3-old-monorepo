// File: ecat_thread.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
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

#include "iomap.hpp"

extern "C" {
#include "./ethercat.h"
}

#if WIN32
#include "ecat_thread/win32.hpp"
#elif __APPLE__
#include "ecat_thread/macosx.hpp"
#else
#include "ecat_thread/linux.hpp"
#endif

#include "../../spdlog.hpp"

#if WIN32
#include <timeapi.h>
#endif

namespace autd3::link {

inline int64_t ec_sync(const int64_t reftime, const int64_t cycletime, int64_t* integral) {
  auto delta = (reftime - 50000) % cycletime;
  if (delta > cycletime / 2) delta -= cycletime;
  if (delta > 0) *integral += 1;
  if (delta < 0) *integral -= 1;
  return -(delta / 100) - *integral / 20;
}

inline void print_stats(const std::string& header, const std::vector<int64_t>& stats) {
  const int64_t min = std::accumulate(stats.begin(), stats.end(), std::numeric_limits<int64_t>::max(),
                                      [](const int64_t min, const int64_t s) { return std::min(min, s); });
  const int64_t max = std::accumulate(stats.begin(), stats.end(), std::numeric_limits<int64_t>::min(),
                                      [](const int64_t max, const int64_t s) { return std::max(max, s); });
  const int64_t ave = std::accumulate(stats.begin(), stats.end(), int64_t{0}) / static_cast<int64_t>(stats.size());
  const int64_t stdd = std::sqrt(static_cast<double>(std::accumulate(stats.begin(), stats.end(), int64_t{0},
                                                                     [ave](const int64_t acc, const int64_t s) { return (s - ave) * (s - ave); })) /
                                 static_cast<double>(stats.size()));
  spdlog::debug("{}: {}+/-{} (Max.{} Min.{}) [us]", header, ave / 1000, stdd / 1000.0, max / 1000, min / 1000);
}

using WaitFunc = void(const timespec&);

template <WaitFunc W>
void ecat_run_(std::atomic<bool>* is_open, std::atomic<int32_t>* wkc, const int64_t cycletime_ns, std::mutex& mtx,
               std::queue<driver::TxDatagram>& send_queue, IOMap& io_map) {
  ecat_init();

#if WIN32
  constexpr auto u_resolution = 1;
  timeBeginPeriod(u_resolution);

  auto* h_process = GetCurrentProcess();
  const auto priority = GetPriorityClass(h_process);
  SetPriorityClass(h_process, REALTIME_PRIORITY_CLASS);
#endif

  auto ts = ecat_setup(cycletime_ns);
  int64_t toff = 0;
  std::vector<int64_t> stats;
  constexpr size_t stats_size = 10000;
  stats.reserve(stats_size);
  auto start = std::chrono::high_resolution_clock::now();
  ec_send_processdata();
  while (is_open->load()) {
    add_timespec(ts, cycletime_ns + toff);

    W(ts);

    if (spdlog::get_level() <= spdlog::level::debug) {
      auto now = std::chrono::high_resolution_clock::now();
      const auto itvl = std::chrono::duration_cast<std::chrono::nanoseconds>(now - start).count();
      stats.emplace_back(itvl);
      if (stats.size() == stats_size) {
        ec_readstate();
        for (size_t slave = 1; slave <= static_cast<size_t>(ec_slavecount); slave++)
          spdlog::debug("Slave[{}]: {} (State={:#02x}, StatusCode={:#04x})", slave, ec_ALstatuscode2string(ec_slave[slave].ALstatuscode),
                        ec_slave[slave].state, ec_slave[slave].ALstatuscode);

        print_stats("EC send interval", stats);
        stats.clear();
        stats.reserve(stats_size);
      }
      start = now;
    }

    wkc->store(ec_receive_processdata(EC_TIMEOUTRET));

    ec_sync(ec_DCtime, cycletime_ns, &toff);

    if (!send_queue.empty()) {
      std::lock_guard lock(mtx);
      io_map.copy_from(send_queue.front());
      send_queue.pop();
    }

    ec_send_processdata();
  }

#if WIN32
  timeEndPeriod(u_resolution);
  SetPriorityClass(h_process, priority);
#endif
}

inline void ecat_run(const bool high_precision, std::atomic<bool>* is_open, std::atomic<int32_t>* wkc, const int64_t cycletime_ns, std::mutex& mtx,
                     std::queue<driver::TxDatagram>& send_queue, IOMap& io_map) {
  if (high_precision)
    ecat_run_<timed_wait_h>(is_open, wkc, cycletime_ns, mtx, send_queue, io_map);
  else
    ecat_run_<timed_wait>(is_open, wkc, cycletime_ns, mtx, send_queue, io_map);
}

}  // namespace autd3::link
