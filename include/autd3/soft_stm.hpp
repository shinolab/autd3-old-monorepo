// File: soft_stm.hpp
// Project: autd3
// Created Date: 07/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/controller.hpp"
#include "autd3/core/utils.hpp"

namespace autd3 {

/**
 * @brief Software Spatio-Temporal Modulation Controller
 */
class SoftSTM {
 public:
#pragma warning(push)
#pragma warning(disable : 26812)
  class TimerStrategy final {
   public:
    enum VALUE : uint8_t {
      NONE = 0,
      BUSY_WAIT = 1 << 1,
    };

    TimerStrategy() = default;
    explicit TimerStrategy(const VALUE value) noexcept : _value(value) {}

    ~TimerStrategy() = default;
    TimerStrategy(const TimerStrategy& v) noexcept = default;
    TimerStrategy& operator=(const TimerStrategy& obj) = default;
    TimerStrategy& operator=(const VALUE v) noexcept {
      _value = v;
      return *this;
    }
    TimerStrategy(TimerStrategy&& obj) = default;
    TimerStrategy& operator=(TimerStrategy&& obj) = default;

    constexpr bool operator==(const TimerStrategy a) const { return _value == a._value; }
    constexpr bool operator!=(const TimerStrategy a) const { return _value != a._value; }
    constexpr bool operator==(const VALUE a) const { return _value == a; }
    constexpr bool operator!=(const VALUE a) const { return _value != a; }

    void set(const VALUE v) noexcept { _value = static_cast<VALUE>(_value | v); }
    void remove(const VALUE v) noexcept { _value = static_cast<VALUE>(_value & ~v); }
    bool contains(const VALUE v) const noexcept { return (_value & v) == v; }

    [[nodiscard]] VALUE value() const noexcept { return _value; }

   private:
    VALUE _value;
  };
#pragma warning(pop)

  struct SoftSTMThreadHandle {
    friend class SoftSTM;

    ~SoftSTMThreadHandle() = default;
    SoftSTMThreadHandle(const SoftSTMThreadHandle& v) = delete;
    SoftSTMThreadHandle& operator=(const SoftSTMThreadHandle& obj) = delete;
    SoftSTMThreadHandle(SoftSTMThreadHandle&& obj) = default;
    SoftSTMThreadHandle& operator=(SoftSTMThreadHandle&& obj) = default;

    Controller finish() {
      if (!_run) throw std::runtime_error("STM has been already finished.");
      _run = false;
      if (_th.joinable()) _th.join();
      _cnt.check_trials = _trials;
      return std::move(_cnt);
    }

   private:
    SoftSTMThreadHandle(Controller cnt, const std::vector<std::shared_ptr<core::Gain>>& bodies, const uint64_t period, const TimerStrategy strategy)
        : _cnt(std::move(cnt)), _trials(_cnt.check_trials) {
      _run = true;
      const auto interval = std::chrono::nanoseconds(period);
      _cnt.check_trials = 0;
      if (strategy.contains(TimerStrategy::BUSY_WAIT))
        _th = std::thread([this, interval, bodies]() {
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
        _th = std::thread([this, interval, bodies]() {
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

    bool _run;
    std::thread _th;
    Controller _cnt;
    size_t _trials;
  };

  SoftSTM() noexcept : _sample_period_ns(0), timer_strategy(TimerStrategy::NONE) {}
  ~SoftSTM() = default;
  SoftSTM(const SoftSTM& v) = default;
  SoftSTM& operator=(const SoftSTM& obj) = default;
  SoftSTM(SoftSTM&& obj) = default;
  SoftSTM& operator=(SoftSTM&& obj) = default;

  [[nodiscard]] size_t size() const { return _bodies.size(); }

  /**
   * @brief Set frequency
   * @param[in] freq Frequency
   * @return double Actual frequency
   */
  double set_frequency(const double freq) {
    const auto sample_freq = static_cast<double>(size()) * freq;
    const auto sample_period_ns = static_cast<uint64_t>(std::round(1000000000.0 / sample_freq));
    _sample_period_ns = sample_period_ns;
    return frequency();
  }

  /**
   * @brief Add data to send
   * @param[in] b data
   */
  template <typename T>
  void add(T&& b) {
    static_assert(std::is_base_of_v<core::Gain, std::remove_reference_t<T>>, "This is not Gain.");
    add_impl(std::forward<T>(b));
  }

  SoftSTMThreadHandle start(Controller cnt) {
    if (size() == 0) throw std::runtime_error("No data was added.");
    return SoftSTMThreadHandle(std::move(cnt), std::move(_bodies), _sample_period_ns, timer_strategy);
  }

  /**
   * @return Frequency
   */
  [[nodiscard]] double frequency() const { return sampling_frequency() / static_cast<double>(size()); }

  /**
   * @return Period
   */
  [[nodiscard]] uint64_t period() const { return _sample_period_ns * static_cast<uint64_t>(size()); }

  /**
   * @brief Sampling frequency
   */
  [[nodiscard]] double sampling_frequency() const noexcept { return 1000000000.0 / static_cast<double>(_sample_period_ns); }

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
  template <typename T>
  void add_impl(T& b) {
    _bodies.emplace_back(std::make_shared<std::remove_reference_t<T>>(b));
  }

  template <typename T>
  void add_impl(T&& b) {
    _bodies.emplace_back(std::make_shared<std::remove_reference_t<T>>(std::move(b)));
  }

  std::vector<std::shared_ptr<core::Gain>> _bodies;
  uint64_t _sample_period_ns;
};

}  // namespace autd3