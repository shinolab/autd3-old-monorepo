// File: transducer.hpp
// Project: geometry
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <utility>

#include "autd3/driver/defined.hpp"
#include "autd3/driver/fpga/defined.hpp"

namespace autd3::core {

using driver::Affine3;
using driver::Matrix3X3;
using driver::Matrix4X4;
using driver::Quaternion;
using driver::Vector3;
using driver::Vector4;

/**
 * \brief Transducer contains idx, position, rotation, and frequency of a transducer
 */
struct Transducer {
  explicit Transducer(const size_t idx, Vector3 pos, Quaternion rot) noexcept : Transducer(idx, std::move(pos), std::move(rot), 0, 4096) {}
  explicit Transducer(const size_t idx, Vector3 pos, Quaternion rot, const uint16_t mod_delay, const uint16_t cycle) noexcept
      : mod_delay(mod_delay), cycle(cycle), _idx(idx), _pos(std::move(pos)), _rot(std::move(rot)) {}
  ~Transducer() = default;
  Transducer(const Transducer& v) noexcept = default;
  Transducer& operator=(const Transducer& obj) = default;
  Transducer(Transducer&& obj) = default;
  Transducer& operator=(Transducer&& obj) = default;

  [[nodiscard]] driver::float_t align_phase_at(const Vector3& p, const driver::float_t sound_speed) const {
    const auto dist = (p - _pos).norm();
    return dist * wavenumber(sound_speed);
  }

  /**
   * \brief Position of the transducer
   */
  [[nodiscard]] const Vector3& position() const noexcept { return _pos; }

  /**
   * \brief Rotation of the transducer
   */
  [[nodiscard]] const Quaternion& rotation() const noexcept { return _rot; }

  /**
   * \brief ID of the transducer
   */
  [[nodiscard]] size_t idx() const noexcept { return _idx; }

  /**
   * \brief x direction of the transducer
   */
  [[nodiscard]] Vector3 x_direction() const { return _rot * Vector3(1, 0, 0); }

  /**
   * \brief y direction of the transducer
   */
  [[nodiscard]] Vector3 y_direction() const { return _rot * Vector3(0, 1, 0); }

  /**
   * \brief z direction of the transducer
   */
  [[nodiscard]] Vector3 z_direction() const { return _rot * Vector3(0, 0, 1); }

  /**
   * \brief modulation delay of the transducer
   */
  uint16_t mod_delay;

  /**
   * \brief Frequency division ratio. The frequency will be autd3::driver::FPGA_CLK_FREQ/cycle.
   */
  uint16_t cycle;

  /**
   * \brief Frequency of the transducer
   */
  [[nodiscard]] driver::float_t frequency() const {
    return static_cast<driver::float_t>(driver::FPGA_CLK_FREQ) / static_cast<driver::float_t>(cycle);
  }

  /**
   * \brief Set fFrequency of the transducer.
   */
  void set_frequency(const driver::float_t freq) noexcept {
    cycle = static_cast<uint16_t>(std::round(static_cast<driver::float_t>(driver::FPGA_CLK_FREQ) / freq));
  }

  /**
   * \brief Wavelength of the ultrasound emitted from the transducer
   */
  [[nodiscard]] driver::float_t wavelength(const driver::float_t sound_speed) const { return sound_speed / frequency(); }

  /**
   * \brief Wavenumber of the ultrasound emitted from the transducer
   */
  [[nodiscard]] driver::float_t wavenumber(const driver::float_t sound_speed) const { return 2 * driver::pi * frequency() / sound_speed; }

 private:
  size_t _idx;
  Vector3 _pos;
  Quaternion _rot;
};

}  // namespace autd3::core
