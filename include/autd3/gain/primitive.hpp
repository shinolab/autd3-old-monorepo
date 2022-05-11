// File: primitive.hpp
// Project: gain
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <utility>

#include "autd3/core/gain.hpp"

namespace autd3::gain {

/**
 * @brief Gain to produce nothing
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Null final : public core::Gain<T> {
 public:
  Null() noexcept {}

  void calc(const core::Geometry<T>& geometry) override {
    for (const auto& dev : geometry)
      for (const auto& transducer : dev) {
        this->_props.drives.set_drive(transducer, 0.0, 0.0);
      }
  }

  ~Null() override = default;
  Null(const Null& v) noexcept = delete;
  Null& operator=(const Null& obj) = delete;
  Null(Null&& obj) = default;
  Null& operator=(Null&& obj) = default;
};

/**
 * @brief Gain to produce single focal point
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Focus final : public core::Gain<T> {
 public:
  /**
   * @param[in] point focal point
   * @param[in] amp amplitude of the wave (from 0.0 to 1.0)
   */
  explicit Focus(core::Vector3 point, const double amp) : _point(std::move(point)), _amp(amp) {}

  void calc(const core::Geometry<T>& geometry) override {
    for (const auto& dev : geometry)
      for (const auto& transducer : dev) {
        const auto dist = (_point - transducer.position()).norm();
        const auto phase = transducer.align_phase_at(dist, geometry.sound_speed);
        this->_props.drives.set_drive(transducer, phase, _amp);
      }
  }

  ~Focus() override = default;
  Focus(const Focus& v) noexcept = delete;
  Focus& operator=(const Focus& obj) = delete;
  Focus(Focus&& obj) = default;
  Focus& operator=(Focus&& obj) = default;

 private:
  core::Vector3 _point;
  double _amp;
};
}  // namespace autd3::gain
