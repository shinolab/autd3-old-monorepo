// File: linux.hpp
// Project: osal_timer
// Created Date: 16/03/2023
// Author: Shun Suzuki
// -----
// Last Modified: 19/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <signal.h>
#include <time.h>

#include <cstring>
#include <exception>
#include <memory>
#include <string>
#include <thread>
#include <utility>

namespace autd3::core {

template <typename T>
class Timer {
 public:
  Timer(std::unique_ptr<T> handler, const timer_t timer_id) : _handler(std::move(handler)), _timer_id(timer_id), _is_closed(false) {}

  ~Timer() { const auto _ = this->stop(); }

  [[nodiscard]] static std::unique_ptr<Timer> start(std::unique_ptr<T> handler, const uint32_t interval_ns) {
    struct itimerspec itval;
    struct sigevent se;

    itval.it_value.tv_sec = 0;
    itval.it_value.tv_nsec = interval_ns;
    itval.it_interval.tv_sec = 0;
    itval.it_interval.tv_nsec = interval_ns;

    memset(&se, 0, sizeof(se));
    se.sigev_value.sival_ptr = handler.get();
    se.sigev_notify = SIGEV_THREAD;
    se.sigev_notify_function = notify;
    se.sigev_notify_attributes = NULL;

    timer_t timer_id;
    if (timer_create(CLOCK_REALTIME, &se, &timer_id) < 0) throw std::runtime_error("timer_create failed");
    if (timer_settime(timer_id, 0, &itval, NULL) < 0) throw std::runtime_error("timer_settime failed");

    return std::make_unique<Timer>(std::move(handler), timer_id);
  }
  [[nodiscard]] std::unique_ptr<T> stop() {
    if (_is_closed) return std::unique_ptr<T>(nullptr);
    if (timer_delete(_timer_id) < 0) throw std::runtime_error("timer_delete failed");
    _is_closed = true;
    return std::move(this->_handler);
  }

  Timer(const Timer &) = delete;
  Timer(Timer &&) = delete;
  Timer &operator=(const Timer &) = delete;
  Timer &operator=(Timer &&) = delete;

 private:
  std::unique_ptr<T> _handler;
  timer_t _timer_id;
  bool _is_closed;

  static void notify(union sigval sv) {
    auto *timer = reinterpret_cast<T *>(sv.sival_ptr);
    timer->callback();
  }
};
}  // namespace autd3::core
