// File: soft_stm.hpp
// Project: autd3
// Created Date: 07/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <utility>
#include <vector>

#include "autd3/controller.hpp"
#include "autd3/core/gain.hpp"
#include "autd3/core/utils/hint.hpp"
#include "autd3/core/utils/osal_timer.hpp"

namespace autd3 {

using core::TimerStrategy;

/**
 * @brief Software Spatio-Temporal Modulation
 */
class SoftwareSTM {
 public:
  /**
   * @brief Handler of SoftwareSTM
   */
  struct SoftwareSTMThreadHandle {
    friend class SoftwareSTM;

    struct SoftwareSTMCallback final : core::CallbackHandler {
      ~SoftwareSTMCallback() override = default;
      SoftwareSTMCallback(const SoftwareSTMCallback& v) noexcept = delete;
      SoftwareSTMCallback& operator=(const SoftwareSTMCallback& obj) = delete;
      SoftwareSTMCallback(SoftwareSTMCallback&& obj) = delete;
      SoftwareSTMCallback& operator=(SoftwareSTMCallback&& obj) = delete;

      explicit SoftwareSTMCallback(Controller& cnt, std::vector<std::shared_ptr<core::Gain>> bodies)
          : _rt_lock(false), _cnt(cnt), _bodies(std::move(bodies)), _i(0), _size(_bodies.size()) {}

      void callback() override {
        if (auto expected = false; _rt_lock.compare_exchange_weak(expected, true)) {
          _cnt.send(*_bodies[_i], core::Duration::zero());
          _i = (_i + 1) % _size;
          _rt_lock.store(false, std::memory_order_release);
        }
      }

     private:
      std::atomic<bool> _rt_lock;

      Controller& _cnt;
      std::vector<std::shared_ptr<core::Gain>> _bodies;
      size_t _i;
      size_t _size;
    };

    ~SoftwareSTMThreadHandle() = default;
    SoftwareSTMThreadHandle(const SoftwareSTMThreadHandle& v) = delete;
    SoftwareSTMThreadHandle& operator=(const SoftwareSTMThreadHandle& obj) = delete;
    SoftwareSTMThreadHandle(SoftwareSTMThreadHandle&& obj) = default;
    SoftwareSTMThreadHandle& operator=(SoftwareSTMThreadHandle&& obj) = delete;

    bool finish() {
      if (!_run) return false;
      _run = false;
      switch (_strategy) {
        case TimerStrategy::BusyWait:
        case TimerStrategy::Sleep:
          if (_th.joinable()) _th.join();
          break;
        case TimerStrategy::NativeTimer:
          const auto _ = _timer->stop();
          break;
      }

      return true;
    }

   private:
    SoftwareSTMThreadHandle(Controller& cnt, std::vector<std::shared_ptr<core::Gain>> bodies, const uint32_t period, const TimerStrategy strategy)
        : _strategy(strategy), _cnt(cnt) {
      _run = true;
      if (bodies.empty()) return;
      const auto interval = std::chrono::nanoseconds(period);
      switch (strategy) {
        case TimerStrategy::BusyWait:
          _th = std::thread([this, interval, bodies = std::move(bodies)] {
            size_t i = 0;
            auto next = core::Clock::now();
            while (_run) {
              next += interval;
              for (;; core::spin_loop_hint())
                if (core::Clock::now() >= next) break;
              this->_cnt.send(*bodies[i], core::Duration::zero());
              i = (i + 1) % bodies.size();
            }
          });
          break;
        case TimerStrategy::Sleep:
          _th = std::thread([this, interval, bodies = std::move(bodies)] {
            size_t i = 0;
            auto next = core::Clock::now();
            while (_run) {
              next += interval;
              std::this_thread::sleep_until(next);
              this->_cnt.send(*bodies[i], core::Duration::zero());
              i = (i + 1) % bodies.size();
            }
          });
          break;
        case TimerStrategy::NativeTimer:
          _timer = core::Timer<SoftwareSTMCallback>::start(std::make_unique<SoftwareSTMCallback>(cnt, std::move(bodies)), period);
          break;
      }
    }

    std::unique_ptr<core::Timer<SoftwareSTMCallback>> _timer;
    bool _run;
    TimerStrategy _strategy;
    std::thread _th;
    Controller& _cnt;
  };

  explicit SoftwareSTM(const TimerStrategy timer_strategy = TimerStrategy::Sleep) noexcept : sampling_period_ns(0), _timer_strategy(timer_strategy) {}
  ~SoftwareSTM() = default;
  SoftwareSTM(const SoftwareSTM& v) = default;
  SoftwareSTM& operator=(const SoftwareSTM& obj) = default;
  SoftwareSTM(SoftwareSTM&& obj) = default;
  SoftwareSTM& operator=(SoftwareSTM&& obj) = default;

  [[nodiscard]] size_t size() const { return _bodies.size(); }

  /**
   * @brief Set frequency
   * @param[in] freq Frequency
   * @return float_t Actual frequency
   */
  driver::float_t set_frequency(const driver::float_t freq) {
    constexpr auto nanoseconds = static_cast<driver::float_t>(1000000000);
    const auto sample_freq = static_cast<driver::float_t>(size()) * freq;
    sampling_period_ns = static_cast<uint32_t>(std::round(nanoseconds / sample_freq));
    return frequency();
  }

  /**
   * @brief Add data to send
   * @param[in] b data
   */
  template <typename G>
  void add(G&& b) {
    static_assert(std::is_base_of_v<core::Gain, std::remove_reference_t<G>>, "This is not Gain.");
    _bodies.emplace_back(std::make_shared<std::remove_reference_t<G>>(std::forward<G>(b)));
  }

  /**
   * @brief Add data to send
   * @param[in] b data
   */
  void add(std::shared_ptr<core::Gain> b) { _bodies.emplace_back(std::move(b)); }

  /**
   * @brief Start STM
   * @param[in] cnt autd3::Controller
   * @details Never use cnt after calling this function.
   */
  SoftwareSTMThreadHandle start(Controller& cnt) {
    if (size() == 0) throw std::runtime_error("No Gains ware added.");
    return {cnt, std::move(_bodies), sampling_period_ns, _timer_strategy};
  }

  /**
   * @return Frequency
   */
  [[nodiscard]] driver::float_t frequency() const { return sampling_frequency() / static_cast<driver::float_t>(size()); }

  /**
   * @return Period
   */
  [[nodiscard]] uint64_t period() const { return sampling_period_ns * size(); }

  /**
   * @brief Sampling frequency
   */
  [[nodiscard]] driver::float_t sampling_frequency() const noexcept {
    constexpr auto nanoseconds = static_cast<driver::float_t>(1000000000);
    return nanoseconds / static_cast<driver::float_t>(sampling_period_ns);
  }

  /**
   * @brief Sampling period in ns
   */
  uint32_t sampling_period_ns;

 private:
  TimerStrategy _timer_strategy;
  std::vector<std::shared_ptr<core::Gain>> _bodies;
};

}  // namespace autd3
