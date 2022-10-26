// File: custom_sink.hpp
// Project: link_soem
// Created Date: 26/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "spdlog/details/null_mutex.h"
#include "spdlog/sinks/base_sink.h"

namespace autd3::capi {

typedef void (*OutCallback)(const char*);
typedef void (*FlushCallback)();

template <typename Mutex>
class CustomSink final : public spdlog::sinks::base_sink<Mutex> {
 public:
  explicit CustomSink(void* out, void* flush)
      : _out([out](const std::string& msg) { reinterpret_cast<OutCallback>(out)(msg.c_str()); }),
        _flush([flush] { reinterpret_cast<FlushCallback>(flush)(); }) {}

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

using custom_sink_st = CustomSink<spdlog::details::null_mutex>;

}  // namespace autd3::capi
