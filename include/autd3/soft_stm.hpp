// File: soft_stm.hpp
// Project: autd3
// Created Date: 07/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
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
#include "autd3/core/utils.hpp"

namespace autd3 {

/**
 * @brief Software Spatio-Temporal Modulation
 */
class SoftwareSTM {
 public:
#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26812)
#endif
  /**
   * @brief Software timer strategy flag
   */
  class TimerStrategy final {
   public:
    enum Value : uint8_t {
      None = 0,
      /**
       * @brief Use busy wait instead of sleep
       */
      BusyWait = 1 << 1,
    };

    TimerStrategy() = default;
    explicit TimerStrategy(const Value value) noexcept : _value(value) {}

    ~TimerStrategy() = default;
    TimerStrategy(const TimerStrategy& v) noexcept = default;
    TimerStrategy& operator=(const TimerStrategy& obj) = default;
    TimerStrategy& operator=(const Value v) noexcept {
      _value = v;
      return *this;
    }
    TimerStrategy(TimerStrategy&& obj) = default;
    TimerStrategy& operator=(TimerStrategy&& obj) = default;

    constexpr bool operator==(const TimerStrategy a) const { return _value == a._value; }
    constexpr bool operator!=(const TimerStrategy a) const { return _value != a._value; }
    constexpr bool operator==(const Value a) const { return _value == a; }
    constexpr bool operator!=(const Value a) const { return _value != a; }

    void set(const Value v) noexcept { _value = static_cast<Value>(_value | v); }
    void remove(const Value v) noexcept { _value = static_cast<Value>(_value & ~v); }
    [[nodiscard]] bool contains(const Value v) const noexcept { return (_value & v) == v; }

    [[nodiscard]] Value value() const noexcept { return _value; }

   private:
    Value _value;
  };
#ifdef _MSC_VER
#pragma warning(pop)
#endif

  /**
   * @brief Handler of SoftwareSTM
   */
  struct SoftwareSTMThreadHandle {
    friend class SoftwareSTM;

    ~SoftwareSTMThreadHandle() = default;
    SoftwareSTMThreadHandle(const SoftwareSTMThreadHandle& v) = delete;
    SoftwareSTMThreadHandle& operator=(const SoftwareSTMThreadHandle& obj) = delete;
    SoftwareSTMThreadHandle(SoftwareSTMThreadHandle&& obj) = default;
    SoftwareSTMThreadHandle& operator=(SoftwareSTMThreadHandle&& obj) = delete;

    bool finish() {
      if (!_run) return false;
      _run = false;
      if (_th.joinable()) _th.join();
      _cnt.set_ack_check_timeout(_timeout);
      return true;
    }

   private:
    SoftwareSTMThreadHandle(Controller& cnt, std::vector<std::shared_ptr<core::Gain>> bodies, const uint64_t period, const TimerStrategy strategy)
        : _cnt(cnt), _timeout(_cnt.get_ack_check_timeout()) {
      _run = true;
      if (bodies.empty()) return;
      const auto interval = std::chrono::nanoseconds(period);
      _cnt.set_ack_check_timeout(std::chrono::high_resolution_clock::duration::zero());
      const auto mode = cnt.mode();
      if (strategy.contains(TimerStrategy::BusyWait))
        _th = std::thread([this, mode, interval, bodies = std::move(bodies)] {
          size_t i = 0;
          auto next = std::chrono::high_resolution_clock::now();
          while (_run) {
            next += interval;
            bodies[i]->init(mode, this->_cnt.geometry());
            for (;; core::spin_loop_hint())
              if (std::chrono::high_resolution_clock::now() >= next) break;
            this->_cnt.send(*bodies[i]);
            i = (i + 1) % bodies.size();
          }
        });
      else
        _th = std::thread([this, mode, interval, bodies = std::move(bodies)] {
          size_t i = 0;
          auto next = std::chrono::high_resolution_clock::now();
          while (_run) {
            next += interval;
            bodies[i]->init(mode, this->_cnt.geometry());
            std::this_thread::sleep_until(next);
            this->_cnt.send(*bodies[i]);
            i = (i + 1) % bodies.size();
          }
        });
    }

    bool _run;
    std::thread _th;
    Controller& _cnt;
    std::chrono::high_resolution_clock::duration _timeout;
  };

  SoftwareSTM() noexcept : timer_strategy(TimerStrategy::None), _sample_period_ns(0) {}
  ~SoftwareSTM() = default;
  SoftwareSTM(const SoftwareSTM& v) = default;
  SoftwareSTM& operator=(const SoftwareSTM& obj) = default;
  SoftwareSTM(SoftwareSTM&& obj) = default;
  SoftwareSTM& operator=(SoftwareSTM&& obj) = default;

  [[nodiscard]] size_t size() const { return _bodies.size(); }

  /**
   * @brief Set frequency
   * @param[in] freq Frequency
   * @return autd3_float_t Actual frequency
   */
  driver::autd3_float_t set_frequency(const driver::autd3_float_t freq) {
    constexpr auto nanoseconds = static_cast<driver::autd3_float_t>(1000000000);
    const auto sample_freq = static_cast<driver::autd3_float_t>(size()) * freq;
    const auto sample_period_ns = static_cast<uint64_t>(std::round(nanoseconds / sample_freq));
    _sample_period_ns = sample_period_ns;
    return frequency();
  }

  /**
   * @brief Add data to send
   * @param[in] b data
   */
  template <typename G>
  void add(G&& b) {
    static_assert(std::is_base_of_v<core::Gain, std::remove_reference_t<G>>, "This is not Gain.");
    add_impl(std::forward<G>(b));
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
    return {cnt, std::move(_bodies), _sample_period_ns, timer_strategy};
  }

  /**
   * @return Frequency
   */
  [[nodiscard]] driver::autd3_float_t frequency() const { return sampling_frequency() / static_cast<driver::autd3_float_t>(size()); }

  /**
   * @return Period
   */
  [[nodiscard]] uint64_t period() const { return _sample_period_ns * size(); }

  /**
   * @brief Sampling frequency
   */
  [[nodiscard]] driver::autd3_float_t sampling_frequency() const noexcept {
    constexpr auto nanoseconds = static_cast<driver::autd3_float_t>(1000000000);
    return nanoseconds / static_cast<driver::autd3_float_t>(_sample_period_ns);
  }

  /**
   * @brief Sampling period in ns
   */
  [[nodiscard]] uint64_t sampling_period_ns() const noexcept { return _sample_period_ns; }

  /**
   * @brief Sampling period in ns
   */
  uint64_t& sampling_period_ns() noexcept { return _sample_period_ns; }

  TimerStrategy timer_strategy;

 private:
  template <typename G>
  void add_impl(G&& b) {
    _bodies.emplace_back(std::make_shared<std::remove_reference_t<G>>(std::forward<G>(b)));
  }

  std::vector<std::shared_ptr<core::Gain>> _bodies;
  uint64_t _sample_period_ns;
};

}  // namespace autd3
