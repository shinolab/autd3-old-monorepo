// File: transducer.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/def.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal::geometry {

class Transducer {
 public:
  Transducer(const uint32_t idx, const native_methods::DevicePtr ptr) : _ptr(AUTDTransducer(ptr, idx)), _idx(idx) {}

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
  [[nodiscard]] size_t idx() const noexcept { return _idx; }

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
  [[nodiscard]] double wavelength(const double sound_speed) const { return AUTDTransducerWavelength(_ptr, sound_speed); }

  /**
   * @brief Get wavenumber of the transducer
   * @param sound_speed Speed of sound
   */
  [[nodiscard]] double wavenumber(const double sound_speed) const { return 2 * pi / wavelength(sound_speed); }

  [[nodiscard]] native_methods::TransducerPtr ptr() const { return _ptr; }

 private:
  native_methods::TransducerPtr _ptr;
  uint32_t _idx;
};

}  // namespace autd3::internal::geometry
