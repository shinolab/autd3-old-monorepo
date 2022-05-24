// File: custom.hpp
// Project: base
// Created Date: 19/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <algorithm>
#include <vector>

#include "autd3.hpp"
#include "autd3/core/geometry/dynamic_transducer.hpp"

/**
 * @brief Gain that can set the phase and duty ratio freely
 */
template <typename T = autd3::core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<autd3::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class CustomGain final : public autd3::Gain<T> {
 public:
  /**
   * @param[in] amp pointer to data of amplitude of each transducer
   * @param[in] phase pointer to data of phase of each transducer
   * @param[in] size length of the data
   * @details The data length should be the same as the number of transducers.
   */
  explicit CustomGain(const double* amp, const double* phase, const size_t size) : _amp(size), _phase(size) {
    std::memcpy(_amp.data(), amp, size * sizeof(double));
    std::memcpy(_phase.data(), phase, size * sizeof(double));
  }

  void calc(const autd3::core::Geometry<T>& geometry) override {
    std::for_each(geometry.begin(), geometry.end(), [&](const auto& dev) {
      std::for_each(dev.begin(), dev.end(), [&](const auto& tr) { this->_props.drives.set_drive(tr, _phase[tr.id()], _amp[tr.id()]); });
    });
  }

  ~CustomGain() override = default;
  CustomGain(const CustomGain& v) noexcept = delete;
  CustomGain& operator=(const CustomGain& obj) = delete;
  CustomGain(CustomGain&& obj) = default;
  CustomGain& operator=(CustomGain&& obj) = default;

 private:
  std::vector<double> _amp;
  std::vector<double> _phase;
};

/**
 * @brief Custom wave modulation
 */
class CustomModulation final : public autd3::Modulation {
 public:
  /**
   * @brief Generate function
   * @param[in] buffer data of modulation
   * @param size size of buffer
   * @param freq_div sampling frequency division ratio
   */
  explicit CustomModulation(const uint8_t* buffer, const size_t size, const uint32_t freq_div = 40960) : Modulation() {
    this->_props.freq_div = freq_div;
    this->_props.buffer.resize(size);
    std::memcpy(this->_props.buffer.data(), buffer, size);
  }

  void calc() override {}

  ~CustomModulation() override = default;
  CustomModulation(const CustomModulation& v) noexcept = delete;
  CustomModulation& operator=(const CustomModulation& obj) = delete;
  CustomModulation(CustomModulation&& obj) = default;
  CustomModulation& operator=(CustomModulation&& obj) = default;
};
