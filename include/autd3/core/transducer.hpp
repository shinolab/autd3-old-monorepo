// File: transducer.hpp
// Project: geometry
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/01/2023
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
 * \brief Transducer contains id, position, rotation, and frequency of a transducer
 */
struct Transducer {
  explicit Transducer(const size_t id, Vector3 pos, Quaternion rot) noexcept : Transducer(id, std::move(pos), std::move(rot), 0, 4096) {}
  explicit Transducer(const size_t id, Vector3 pos, Quaternion rot, const uint16_t mod_delay, const uint16_t cycle) noexcept
      : _id(id), _pos(std::move(pos)), _rot(std::move(rot)), _mod_delay(mod_delay), _cycle(cycle) {}
  ~Transducer() = default;
  Transducer(const Transducer& v) noexcept = default;
  Transducer& operator=(const Transducer& obj) = default;
  Transducer(Transducer&& obj) = default;
  Transducer& operator=(Transducer&& obj) = default;

  [[nodiscard]] driver::autd3_float_t align_phase_at(const driver::autd3_float_t dist, const driver::autd3_float_t sound_speed) const {
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
  [[nodiscard]] size_t id() const noexcept { return _id; }

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
  [[nodiscard]] uint16_t mod_delay() const noexcept { return _mod_delay; }

  /**
   * \brief modulation delay of the transducer
   */
  uint16_t& mod_delay() noexcept { return _mod_delay; }

  /**
   * \brief Frequency division ratio. The frequency will be autd3::driver::FPGA_CLK_FREQ/cycle.
   */
  [[nodiscard]] uint16_t cycle() const { return _cycle; }

  /**
   * \brief Frequency of the transducer
   */
  [[nodiscard]] driver::autd3_float_t frequency() const {
    return static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / static_cast<driver::autd3_float_t>(_cycle);
  }

  /**
   * \brief Set fFrequency division ratio. The frequency will be autd3::driver::FPGA_CLK_FREQ/cycle.
   */
  void set_cycle(const uint16_t cycle) noexcept { _cycle = cycle; }

  /**
   * \brief Set fFrequency of the transducer.
   */
  void set_frequency(const driver::autd3_float_t freq) noexcept {
    const auto cycle = static_cast<uint16_t>(std::round(static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / freq));
    set_cycle(cycle);
  }

  /**
   * \brief Wavelength of the ultrasound emitted from the transducer
   */
  [[nodiscard]] driver::autd3_float_t wavelength(const driver::autd3_float_t sound_speed) const { return sound_speed / frequency(); }

  /**
   * \brief Wavenumber of the ultrasound emitted from the transducer
   */
  [[nodiscard]] driver::autd3_float_t wavenumber(const driver::autd3_float_t sound_speed) const { return 2 * driver::pi * frequency() / sound_speed; }

 private:
  size_t _id;
  Vector3 _pos;
  Quaternion _rot;
  uint16_t _mod_delay;
  uint16_t _cycle;
};

}  // namespace autd3::core
