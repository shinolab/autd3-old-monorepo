// File: soft_stm.cpp
// Project: src
// Created Date: 22/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/soft_stm.hpp"

#include "autd3/core/utils.hpp"
#include "spdlog.hpp"

namespace autd3 {

constexpr driver::autd3_float_t NANO_SECONDS = static_cast<driver::autd3_float_t>(1000000000);

bool SoftwareSTM::SoftwareSTMThreadHandle::finish() {
  if (!_run) {
    spdlog::error("STM has been already finished.");
    return false;
  }
  _run = false;
  if (_th.joinable()) _th.join();
  _cnt.set_ack_check_timeout(_timeout);
  return true;
}
SoftwareSTM::SoftwareSTMThreadHandle::SoftwareSTMThreadHandle(Controller& cnt, std::vector<std::shared_ptr<core::Gain>> bodies, const uint64_t period,
                                                              const TimerStrategy strategy)
    : _cnt(cnt), _timeout(_cnt.get_ack_check_timeout()) {
  _run = true;
  if (bodies.empty()) return;
  const auto interval = std::chrono::nanoseconds(period);
  _cnt.set_ack_check_timeout(std::chrono::high_resolution_clock::duration::zero());
  if (strategy.contains(TimerStrategy::BusyWait))
    _th = std::thread([this, interval, bodies = std::move(bodies)] {
      size_t i = 0;
      auto next = std::chrono::high_resolution_clock::now();
      while (_run) {
        next += interval;
        bodies[i]->build(this->_cnt.geometry());
        for (;; core::spin_loop_hint())
          if (std::chrono::high_resolution_clock::now() >= next) break;
        this->_cnt.send(*bodies[i]);
        i = (i + 1) % bodies.size();
      }
    });
  else
    _th = std::thread([this, interval, bodies = std::move(bodies)] {
      size_t i = 0;
      auto next = std::chrono::high_resolution_clock::now();
      while (_run) {
        next += interval;
        bodies[i]->build(this->_cnt.geometry());
        std::this_thread::sleep_until(next);
        this->_cnt.send(*bodies[i]);
        i = (i + 1) % bodies.size();
      }
    });
}

driver::autd3_float_t SoftwareSTM::set_frequency(const driver::autd3_float_t freq) {
  const auto sample_freq = static_cast<driver::autd3_float_t>(size()) * freq;
  const auto sample_period_ns = static_cast<uint64_t>(std::round(NANO_SECONDS / sample_freq));
  _sample_period_ns = sample_period_ns;
  return frequency();
}

SoftwareSTM::SoftwareSTMThreadHandle SoftwareSTM::start(Controller& cnt) {
  if (size() == 0) spdlog::warn("No data was added.");
  return {cnt, std::move(_bodies), _sample_period_ns, timer_strategy};
}

driver::autd3_float_t SoftwareSTM::frequency() const { return sampling_frequency() / static_cast<driver::autd3_float_t>(size()); }

uint64_t SoftwareSTM::period() const { return _sample_period_ns * size(); }

driver::autd3_float_t SoftwareSTM::sampling_frequency() const noexcept {
  return NANO_SECONDS / static_cast<driver::autd3_float_t>(_sample_period_ns);
}

uint64_t SoftwareSTM::sampling_period_ns() const noexcept { return _sample_period_ns; }

uint64_t& SoftwareSTM::sampling_period_ns() noexcept { return _sample_period_ns; }
}  // namespace autd3
