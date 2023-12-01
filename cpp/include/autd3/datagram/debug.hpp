// File: debug.hpp
// Project: datagram
// Created Date: 01/12/2023
// Author: Shun Suzuki
// -----
// Last Modified: 01/12/2023
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
concept configure_debug_output_idx_f = requires(F f, const internal::Device& d) {
  { f(d) } -> std::same_as<const internal::Transducer*>;
};

/**
 * @brief Datagram to configure debug output
 */
template <configure_debug_output_idx_f F>
class ConfigureDebugOutputIdx final {
  using native_f = uint8_t (*)(const void*, internal::native_methods::GeometryPtr, uint32_t);

 public:
  ConfigureDebugOutputIdx(const F& f) : _f(f) {
    _f_native = +[](const void* context, const internal::native_methods::GeometryPtr geometry_ptr, const uint32_t dev_idx) -> uint8_t {
      const internal::Device dev(static_cast<size_t>(dev_idx), AUTDDevice(geometry_ptr, dev_idx));
      const auto* tr = static_cast<const ConfigureDebugOutputIdx*>(context)->_f(dev);
      return tr != nullptr ? static_cast<uint8_t>(tr->tr_idx()) : 0xFF;
    };
  }

  [[nodiscard]] internal::native_methods::DatagramPtr ptr(const internal::Geometry& geometry) const {
    return internal::native_methods::AUTDDatagramConfigureDebugOutputIdx(const_cast<void*>(reinterpret_cast<const void*>(_f_native)),
                                                                         const_cast<void*>(static_cast<const void*>(this)), geometry.ptr());
  }

 private:
  const F& _f;
  native_f _f_native;
};

}  // namespace autd3::datagram
