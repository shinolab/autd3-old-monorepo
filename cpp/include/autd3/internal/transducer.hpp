// File: transducer.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/def.hpp"
#include "autd3/internal/exception.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class Transducer {
 public:
  Transducer(const uint32_t idx, const native_methods::GeometryPtr ptr) : _ptr(ptr), _idx(idx) {}

  /**
   * @brief Get the position of the transducer
   */
  [[nodiscard]] Vector3 position() const noexcept {
    Vector3 v;
    AUTDTransPosition(_ptr, _idx, v.data());
    return v;
  }

  /**
   * @brief Get the rotation quaternion of the transducer
   */
  [[nodiscard]] Quaternion rotation() const noexcept {
    Quaternion v;
    AUTDTransRotation(_ptr, _idx, v.vec().data());
    return v;
  }

  /**
   * @brief Get the index of the transducer
   */
  [[nodiscard]] size_t idx() const noexcept { return static_cast<size_t>(_idx); }

  /**
   * @brief Get the x direction of the transducer
   */
  [[nodiscard]] Vector3 x_direction() const {
    Vector3 v;
    AUTDTransXDirection(_ptr, _idx, v.data());
    return v;
  }

  /**
   * @brief Get the y direction of the transducer
   */
  [[nodiscard]] Vector3 y_direction() const {
    Vector3 v;
    AUTDTransYDirection(_ptr, _idx, v.data());
    return v;
  }

  /**
   * @brief Get the z direction of the transducer
   */
  [[nodiscard]] Vector3 z_direction() const {
    Vector3 v;
    AUTDTransZDirection(_ptr, _idx, v.data());
    return v;
  }

  /**
   * @brief Get frequency of the transducer
   */
  [[nodiscard]] double frequency() const { return AUTDGetTransFrequency(_ptr, _idx); }

  /**
   * @brief Set frequency of the transducer
   */
  void set_frequency(const double freq) const {
    if (char err[256]{}; !AUTDSetTransFrequency(_ptr, _idx, freq, err)) throw AUTDException(err);
  }

  /**
   * @brief Get wavelength of the transducer
   * @param sound_speed Speed of sound
   */
  [[nodiscard]] double wavelength(const double sound_speed) const { return AUTDGetWavelength(_ptr, _idx, sound_speed); }

  /**
   * @brief Get wavenumber of the transducer
   * @param sound_speed Speed of sound
   */
  [[nodiscard]] double wavenumber(const double sound_speed) const { return 2 * pi / wavelength(sound_speed); }

  /**
   * @brief Get modulation delay of the transducer
   */
  [[nodiscard]] uint16_t mod_delay() const { return AUTDGetTransModDelay(_ptr, _idx); }

  /**
   * @brief Set modulation delay of the transducer
   */
  void set_mod_delay(const uint16_t delay) const { AUTDSetTransModDelay(_ptr, _idx, delay); }

  /**
   * @brief Get cycle of the transducer
   */
  [[nodiscard]] uint16_t cycle() const { return AUTDGetTransCycle(_ptr, _idx); }

  /**
   * @brief Set cycle of the transducer
   */
  void set_cycle(const uint16_t cycle) const {
    if (char err[256]; !AUTDSetTransCycle(_ptr, _idx, cycle, err)) throw AUTDException(err);
  }

 private:
  native_methods::GeometryPtr _ptr;
  uint32_t _idx;
};

}  // namespace autd3::internal
