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

namespace autd3 {

/**
 * @brief Software Spatio-Temporal Modulation Controller
 */
class SoftSTM {
 public:
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
    SoftSTMThreadHandle(Controller cnt, const std::vector<std::shared_ptr<core::Gain>>& bodies, const uint64_t period)
        : _cnt(std::move(cnt)), _trials(_cnt.check_trials) {
      _run = true;
      const auto interval = std::chrono::nanoseconds(period);
      _cnt.check_trials = 0;
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

  SoftSTM() noexcept : _sample_period_ns(0) {}
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
    return SoftSTMThreadHandle(std::move(cnt), std::move(_bodies), _sample_period_ns);
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