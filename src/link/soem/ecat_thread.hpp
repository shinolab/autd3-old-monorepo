// File: ecat_thread.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 27/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <limits>
#include <memory>
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
  ec_send_processdata();
  while (is_open->load()) {
    ec_sync(ec_DCtime, cycletime_ns, &toff);

    wkc->store(ec_receive_processdata(EC_TIMEOUTRET));
    if (!send_queue.empty()) {
      io_map.copy_from(send_queue.front());
      {
        std::lock_guard lock(mtx);
        send_queue.pop();
      }
    }

    add_timespec(ts, cycletime_ns + toff);
    W(ts);

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
