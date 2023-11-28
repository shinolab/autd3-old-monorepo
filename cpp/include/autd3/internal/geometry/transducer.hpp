// File: transducer.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 28/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/def.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class Transducer {
 public:
  Transducer(const size_t dev_idx, const uint32_t tr_idx,
             const native_methods::DevicePtr ptr)
      : _ptr(AUTDTransducer(ptr, tr_idx)), _tr_idx(tr_idx), _dev_idx(dev_idx) {}

  /**
   * @brief Get the position of the transducer
   */
  [[nodiscard]] Vector3 position() const noexcept {
    Vector3 v;
    AUTDTransducerPosition(_ptr, v.data());
    return v;
  }

  /**
   * @brief Get the rotation quaternion of the transducer
   */
  [[nodiscard]] Quaternion rotation() const noexcept {
    double v[4];
    AUTDTransducerRotation(_ptr, v);
    return {v[0], v[1], v[2], v[3]};
  }

  /**
   * @brief Get the local index of the transducer
   */
  [[nodiscard]] size_t tr_idx() const noexcept { return _tr_idx; }

  /**
   * @brief Get the device index of the transducer
   */
  [[nodiscard]] size_t dev_idx() const noexcept { return _dev_idx; }

  /**
   * @brief Get the x direction of the transducer
   */
  [[nodiscard]] Vector3 x_direction() const {
    Vector3 v;
    AUTDTransducerDirectionX(_ptr, v.data());
    return v;
  }

  /**
   * @brief Get the y direction of the transducer
   */
  [[nodiscard]] Vector3 y_direction() const {
    Vector3 v;
    AUTDTransducerDirectionY(_ptr, v.data());
    return v;
  }

  /**
   * @brief Get the z direction of the transducer
   */
  [[nodiscard]] Vector3 z_direction() const {
    Vector3 v;
    AUTDTransducerDirectionZ(_ptr, v.data());
    return v;
  }

  /**
   * @brief Get wavelength of the transducer
   * @param sound_speed Speed of sound
   */
  [[nodiscard]] double wavelength(const double sound_speed) const {
    return AUTDTransducerWavelength(_ptr, sound_speed);
  }

  /**
   * @brief Get wavenumber of the transducer
   * @param sound_speed Speed of sound
   */
  [[nodiscard]] double wavenumber(const double sound_speed) const {
    return 2 * pi / wavelength(sound_speed);
  }

  /**
   * @brief Get modulation delay of the transducer
   */
  [[nodiscard]] uint16_t mod_delay() const {
    return AUTDTransducerModDelayGet(_ptr);
  }

  /**
   * @brief Set modulation delay of the transducer
   */
  void set_mod_delay(const uint16_t delay) const {
    AUTDTransducerModDelaySet(_ptr, delay);
  }

  [[nodiscard]] native_methods::TransducerPtr ptr() const { return _ptr; }

 private:
  native_methods::TransducerPtr _ptr;
  uint32_t _tr_idx;
  uint32_t _dev_idx;
};

}  // namespace autd3::internal
