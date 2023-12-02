// File: mod_delay.hpp
// Project: datagram
// Created Date: 01/12/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <concepts>

#include "autd3/internal/geometry/device.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/geometry/transducer.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::datagram {

template <class F>
concept configure_mod_delay_f = requires(F f, const internal::geometry::Device& d, const internal::geometry::Transducer& tr) {
  { f(d, tr) } -> std::same_as<uint16_t>;
};

/**
 * @brief Datagram to set modulation delay
 */
template <configure_mod_delay_f F>
class ConfigureModDelay final {
  using native_f = uint16_t (*)(const void*, internal::native_methods::GeometryPtr, uint32_t, uint8_t);

 public:
  explicit ConfigureModDelay(const F& f) : _f(f) {
    _f_native =
        +[](const void* context, const internal::native_methods::GeometryPtr geometry_ptr, const uint32_t dev_idx, const uint8_t tr_idx) -> uint16_t {
      const internal::geometry::Device dev(dev_idx, AUTDDevice(geometry_ptr, dev_idx));
      const internal::geometry::Transducer tr(static_cast<size_t>(tr_idx), dev.ptr());
      return static_cast<const ConfigureModDelay*>(context)->_f(dev, tr);
    };
  }

  [[nodiscard]] internal::native_methods::DatagramPtr ptr(const internal::geometry::Geometry& geometry) const {
    return AUTDDatagramConfigureModDelay(const_cast<void*>(reinterpret_cast<const void*>(_f_native)),
                                         const_cast<void*>(static_cast<const void*>(this)), geometry.ptr());
  }

 private:
  const F& _f;
  native_f _f_native;
};

}  // namespace autd3::datagram
