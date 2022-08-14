// File: ecat_thread.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/08/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <queue>
#include <string>

extern "C" {
#include "./ethercat.h"
}
#include "error_handler.hpp"
#include "utils.hpp"

#if WIN32
#include "win.hpp"
#elif __APPLE__
#include "mac.hpp"
#else
#include "unix.hpp"
#endif

namespace autd3::link {

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
  while (*is_running) {
    add_timespec(ts, cycletime_ns + toff);

    W(ts);

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
