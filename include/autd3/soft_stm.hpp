// File: soft_stm.hpp
// Project: autd3
// Created Date: 07/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/12/2022
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
   *
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

    bool finish();

   private:
    SoftwareSTMThreadHandle(Controller& cnt, std::vector<std::shared_ptr<core::Gain>> bodies, uint64_t period, TimerStrategy strategy);

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
  driver::autd3_float_t set_frequency(driver::autd3_float_t freq);

  /**
   * @brief Add data to send
   * @param[in] b data
   */
  template <typename T>
  void add(T&& b) {
    static_assert(std::is_base_of_v<core::Gain, std::remove_reference_t<T>>, "This is not Gain.");
    add_impl(std::forward<T>(b));
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
  SoftwareSTMThreadHandle start(Controller& cnt);

  /**
   * @return Frequency
   */
  [[nodiscard]] driver::autd3_float_t frequency() const;

  /**
   * @return Period
   */
  [[nodiscard]] uint64_t period() const;

  /**
   * @brief Sampling frequency
   */
  [[nodiscard]] driver::autd3_float_t sampling_frequency() const noexcept;

  /**
   * @brief Sampling period in ns
   */
  [[nodiscard]] uint64_t sampling_period_ns() const noexcept;

  /**
   * @brief Sampling period in ns
   */
  uint64_t& sampling_period_ns() noexcept;

  TimerStrategy timer_strategy;

 private:
  template <typename T>
  void add_impl(T& b) {
    _bodies.emplace_back(std::make_shared<std::remove_reference_t<T>>(b));
  }

  template <typename T>
  void add_impl(T&& b) {
    _bodies.emplace_back(std::make_shared<std::remove_reference_t<T>>(std::forward<T>(b)));
  }

  std::vector<std::shared_ptr<core::Gain>> _bodies;
  uint64_t _sample_period_ns;
};

}  // namespace autd3
