// File: trans_test.hpp
// Project: gain
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <optional>

#include "autd3/gain/cache.hpp"
#include "autd3/gain/transform.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

template <class F>
concept transducer_test_f = requires(F f, const internal::geometry::Device& d, const internal::geometry::Transducer& tr) {
  { f(d, tr) } -> std::same_as<std::optional<internal::Drive>>;
};

/**
 * @brief Gain to test
 */
template <transducer_test_f F>
class TransducerTest final : public internal::Gain, public IntoCache<TransducerTest<F>>, public IntoTransform<TransducerTest<F>> {
  using native_f = void (*)(const void*, internal::native_methods::GeometryPtr, uint32_t, uint8_t, internal::native_methods::Drive*);

 public:
  explicit TransducerTest(const F& f) : _f(f) {
    _f_native = +[](const void* context, const internal::native_methods::GeometryPtr geometry_ptr, const uint32_t dev_idx, const uint8_t tr_idx,
                    internal::native_methods::Drive* raw) {
      const internal::geometry::Device dev(dev_idx, AUTDDevice(geometry_ptr, dev_idx));
      const internal::geometry::Transducer tr(static_cast<size_t>(tr_idx), dev.ptr());
      if (const auto d = static_cast<const TransducerTest*>(context)->_f(dev, tr); d.has_value()) {
        raw->phase = d.value().phase.value();
        raw->intensity = d.value().intensity.value();
      }
    };
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::geometry::Geometry& geometry) const override {
    return AUTDGainTransducerTest(const_cast<void*>(reinterpret_cast<const void*>(_f_native)),
                                  internal::native_methods::ContextPtr{const_cast<void*>(static_cast<const void*>(this))}, geometry.ptr());
  }

 private:
  const F& _f;
  native_f _f_native;
};

}  // namespace autd3::gain
