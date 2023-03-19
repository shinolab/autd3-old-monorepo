// File: timer.hpp
// Project: win32
// Created Date: 01/05/2021
// Author: Shun Suzuki
// -----
// Last Modified: 19/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#include <Windows.h>
#include <mmsystem.h>

#include <exception>
#include <memory>
#include <string>
#include <thread>
#include <utility>

namespace autd3::core {

template <typename T>
class Timer {
 public:
  Timer(std::unique_ptr<T> handler, const uint32_t timer_id) : _handler(std::move(handler)), _timer_id(timer_id), _is_closed(false) {}
  ~Timer() { const auto _ = this->stop(); }

  [[nodiscard]] static std::unique_ptr<Timer> start(std::unique_ptr<T> handler, uint32_t interval_ns) {
    const auto timer_id = timeSetEvent((std::max)(1u, interval_ns / 1000 / 1000), 1u, timer_thread, reinterpret_cast<DWORD_PTR>(handler.get()),
                                       TIME_PERIODIC | TIME_CALLBACK_FUNCTION | TIME_KILL_SYNCHRONOUS);
    if (timer_id == 0) throw std::runtime_error("timeSetEvent failed");

    return std::make_unique<Timer>(std::move(handler), timer_id);
  }

  [[nodiscard]] std::unique_ptr<T> stop() {
    if (_is_closed) return std::unique_ptr<T>(nullptr);
    if (timeKillEvent(_timer_id) != TIMERR_NOERROR) throw std::runtime_error("timeKillEvent failed");
    _is_closed = true;
    return std::move(this->_handler);
  }

  Timer(const Timer &) = delete;
  Timer(Timer &&) = delete;
  Timer &operator=(const Timer &) = delete;
  Timer &operator=(Timer &&) = delete;

 private:
  static void CALLBACK timer_thread(UINT, UINT, DWORD_PTR dw_user, DWORD_PTR, DWORD_PTR) {
    auto *const handler = reinterpret_cast<T *>(dw_user);
    handler->callback();
  }

  std::unique_ptr<T> _handler;
  uint32_t _timer_id;
  bool _is_closed;
};
}  // namespace autd3::core
