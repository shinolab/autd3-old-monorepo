// File: macosx.hpp
// Project: osal_timer
// Created Date: 18/03/2023
// Author: Shun Suzuki
// -----
// Last Modified: 20/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <dispatch/dispatch.h>

#include <memory>
#include <string>
#include <thread>
#include <utility>

namespace autd3::core {

template <typename T>
class Timer {
 public:
  Timer(std::unique_ptr<T> handler, dispatch_queue_t queue, dispatch_source_t timer)
      : _handler(std::move(handler)), _queue(queue), _timer(timer), _is_closed(false) {}
  ~Timer() { const auto _ = this->stop(); }

  [[nodiscard]] static std::unique_ptr<Timer> start(std::unique_ptr<T> handler, const uint32_t interval_ns) {
    auto queue = dispatch_queue_create("timerQueue", 0);

    auto *ptr = handler.get();
    auto timer = dispatch_source_create(DISPATCH_SOURCE_TYPE_TIMER, 0, 0, queue);
    dispatch_source_set_event_handler(timer, ^{
      main_loop(ptr);
    });

    dispatch_source_set_cancel_handler(timer, ^{
      dispatch_release(timer);
      dispatch_release(queue);
    });

    dispatch_time_t start = dispatch_time(DISPATCH_TIME_NOW, 0);
    dispatch_source_set_timer(timer, start, interval_ns, 0);
    dispatch_resume(timer);

    return std::make_unique<Timer>(std::move(handler), queue, timer);
  }

  [[nodiscard]] std::unique_ptr<T> stop() {
    if (_is_closed) return std::unique_ptr<T>(nullptr);
    dispatch_source_cancel(_timer);
    _is_closed = true;
    return std::move(this->_handler);
  }

  Timer(const Timer &) = delete;
  Timer(Timer &&) = delete;
  Timer &operator=(const Timer &) = delete;
  Timer &operator=(Timer &&) = delete;

 private:
  std::unique_ptr<T> _handler;

  dispatch_queue_t _queue;
  dispatch_source_t _timer;

  bool _is_closed;

  static void main_loop(T *ptr) { ptr->callback(); }
};
}  // namespace autd3::core
