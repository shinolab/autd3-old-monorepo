// File: unix.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <queue>
#include <string>

#include "./ethercat.h"
#include "error_handler.hpp"
#include "utils.hpp"

namespace autd3::link {

inline void ecat_init() {}

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

void ecat_run_(std::atomic<bool>* is_open, bool* is_running, int32_t expected_wkc, int64_t cycletime_ns, std::mutex& mtx,
               std::queue<driver::TxDatagram>& send_queue, IOMap& io_map, std::function<void(std::string)> on_lost) {
  auto ts = timespec{0, 0};

  auto tleft = timespec{0, 0};
  clock_gettime(CLOCK_MONOTONIC, &ts);

  const auto ht = ((ts.tv_nsec / cycletime_ns) + 1) * cycletime_ns;
  ts.tv_nsec = ht;

  int64_t toff = 0;
  while (*is_running) {
    add_timespec(ts, cycletime_ns + toff);

    clock_nanosleep(CLOCK_MONOTONIC, TIMER_ABSTIME, &ts, &tleft);

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
}

void ecat_run(bool, std::atomic<bool>* is_open, bool* is_running, int32_t expected_wkc, int64_t cycletime_ns, std::mutex& mtx,
              std::queue<driver::TxDatagram>& send_queue, IOMap& io_map, std::function<void(std::string)> on_lost) {
  ecat_run_(is_open, is_running, expected_wkc, cycletime_ns, mtx, send_queue, io_map, on_lost);
}

}  // namespace autd3::link
