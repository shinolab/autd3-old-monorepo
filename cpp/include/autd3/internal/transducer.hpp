// File: transducer.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/exception.hpp"
#include "autd3/internal/def.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class Transducer {
 public:
  Transducer(const uint32_t idx,  void*  ptr) : _ptr(ptr), _idx(idx) {}

  [[nodiscard]] Vector3 position() const noexcept {
    double x, y, z;
    native_methods::AUTDTransPosition(_ptr, _idx, &x, &y, &z);
    return Vector3(x, y, z);
  }

  [[nodiscard]] size_t idx() const noexcept { return static_cast<size_t>(_idx); }

  [[nodiscard]] Vector3 x_direction() const {
    double x, y, z;
    native_methods::AUTDTransXDirection(_ptr, _idx, &x, &y, &z);
    return Vector3(x, y, z);
  }

  [[nodiscard]] Vector3 y_direction() const {
    double x, y, z;
    native_methods::AUTDTransYDirection(_ptr, _idx, &x, &y, &z);
    return Vector3(x, y, z);
  }

  [[nodiscard]] Vector3 z_direction() const {
    double x, y, z;
    native_methods::AUTDTransZDirection(_ptr, _idx, &x, &y, &z);
    return Vector3(x, y, z);
  }

  [[nodiscard]] double frequency() const { return native_methods::AUTDGetTransFrequency(_ptr, _idx); }

  void set_frequency(const double freq) {
      char err[256]{};
    if (!native_methods::AUTDSetTransFrequency(_ptr, _idx, freq, err)) throw AUTDException(err);
  }

  [[nodiscard]] double wavelength() const { return native_methods::AUTDGetWavelength(_ptr, _idx); }

  [[nodiscard]] double wavenumber() const { return 2 * pi / wavelength(); }

  [[nodiscard]] uint16_t mod_delay() const { return native_methods::AUTDGetTransModDelay(_ptr, _idx); }

  void set_mod_delay(const uint16_t delay) { native_methods::AUTDSetTransModDelay(_ptr, _idx, delay); }

  [[nodiscard]] uint16_t cycle() const { return native_methods::AUTDGetTransCycle(_ptr, _idx); }

  void set_cycle(const uint16_t cycle) {
    char err[256];
    if (!native_methods::AUTDSetTransCycle(_ptr, _idx, cycle, err)) throw AUTDException(err);
  }

 private:
  void* _ptr;
  uint32_t _idx;
};

}  // namespace autd3::internal
