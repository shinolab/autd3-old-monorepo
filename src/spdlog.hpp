// File: spdlog.hpp
// Project: autd3
// Created Date: 18/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <string>
#include <utility>

#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#endif
#include "spdlog/async.h"
#include "spdlog/sinks/base_sink.h"
#include "spdlog/spdlog.h"
#ifdef WIN32
#include "spdlog/sinks/wincolor_sink.h"
#else
#include "spdlog/sinks/ansicolor_sink.h"
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

namespace autd3 {

template <typename Mutex>
class CustomSink final : public spdlog::sinks::base_sink<Mutex> {
 public:
  explicit CustomSink(std::function<void(std::string)> out, std::function<void()> flush) : _out(std::move(out)), _flush(std::move(flush)) {}

 protected:
  void sink_it_(const spdlog::details::log_msg& msg) override {
    spdlog::memory_buf_t formatted;
    spdlog::sinks::base_sink<Mutex>::formatter_->format(msg, formatted);
    _out(fmt::to_string(formatted));
  }
  void flush_() override { _flush(); }

 private:
  std::function<void(std::string)> _out;
  std::function<void()> _flush;
};

inline spdlog::sink_ptr get_default_sink() {
#ifdef WIN32
  return std::make_shared<spdlog::sinks::wincolor_stdout_sink_mt>();
#else
  return std::make_shared<spdlog::sinks::ansicolor_stdout_sink_mt>();
#endif
}

}  // namespace autd3
