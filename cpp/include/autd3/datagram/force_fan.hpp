// File: force_fan.hpp
// Project: datagram
// Created Date: 06/12/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <concepts>

#include "autd3/internal/geometry/device.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::datagram {

template <class F>
concept configure_force_fan_f = requires(F f, const internal::geometry::Device& d) {
  { f(d) } -> std::same_as<bool>;
};

/**
 * @brief Datagram to configure force fan
 */
template <configure_force_fan_f F>
class ConfigureForceFan final {
  using native_f = bool (*)(const void*, internal::native_methods::GeometryPtr, uint32_t);

 public:
  explicit ConfigureForceFan(const F& f) : _f(f) {
    _f_native = +[](const void* context, const internal::native_methods::GeometryPtr geometry_ptr, const uint32_t dev_idx) -> bool {
      const internal::geometry::Device dev(dev_idx, AUTDDevice(geometry_ptr, dev_idx));
      return static_cast<const ConfigureForceFan*>(context)->_f(dev);
    };
  }

  [[nodiscard]] internal::native_methods::DatagramPtr ptr(const internal::geometry::Geometry& geometry) const {
    return AUTDDatagramConfigureForceFan(const_cast<void*>(reinterpret_cast<const void*>(_f_native)),
                                         const_cast<void*>(static_cast<const void*>(this)), geometry.ptr());
  }

 private:
  const F& _f;
  native_f _f_native;
};

}  // namespace autd3::datagram
