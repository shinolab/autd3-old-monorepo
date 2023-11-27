// File: trans_test.hpp
// Project: gain
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 24/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <numeric>
#include <vector>

#include "autd3/gain/cache.hpp"
#include "autd3/gain/transform.hpp"
#include "autd3/internal/emit_intensity.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

/**
 * @brief Gain to test
 */
class TransducerTest final : public internal::Gain, public IntoCache<TransducerTest>, public IntoTransform<TransducerTest> {
 public:
  TransducerTest() = default;

  void set(const internal::Transducer& tr, const double phase, const internal::EmitIntensity intensity) & {
    _props.emplace_back(Prop{tr.ptr(), phase, intensity});
  }
  TransducerTest&& set(const internal::Transducer& tr, const double phase, const internal::EmitIntensity intensity) && {
    _props.emplace_back(Prop{tr.ptr(), phase, intensity});
    return std::move(*this);
  }

  void set(const internal::Transducer& tr, const double phase, const uint8_t intensity) & {
    _props.emplace_back(Prop{tr.ptr(), phase, internal::EmitIntensity(intensity)});
  }
  TransducerTest&& set(const internal::Transducer& tr, const double phase, const uint8_t intensity) && {
    _props.emplace_back(Prop{tr.ptr(), phase, internal::EmitIntensity(intensity)});
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    return std::accumulate(_props.cbegin(), _props.cend(), internal::native_methods::AUTDGainTransducerTest(),
                           [](const internal::native_methods::GainPtr acc, const Prop& p) {
                             return AUTDGainTransducerTestSet(acc, p.tr, p.phase, p.intensity.value());
                           });
  }

 private:
  struct Prop {
    internal::native_methods::TransducerPtr tr;
    double phase;
    internal::EmitIntensity intensity;
  };

  std::vector<Prop> _props;
};

}  // namespace autd3::gain
