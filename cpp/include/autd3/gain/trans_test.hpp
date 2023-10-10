// File: trans_test.hpp
// Project: gain
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2023
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

  void set(const size_t dev_idx, const size_t tr_idx, const double phase, const double amp) & {
    _props.emplace_back(Prop{dev_idx, tr_idx, phase, amp});
  }
  TransducerTest&& set(const size_t dev_idx, const size_t tr_idx, const double phase, const double amp) && {
    _props.emplace_back(Prop{dev_idx, tr_idx, phase, amp});
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    return std::accumulate(_props.cbegin(), _props.cend(), internal::native_methods::AUTDGainTransducerTest(),
                           [](const internal::native_methods::GainPtr acc, const Prop& p) {
                             return AUTDGainTransducerTestSet(acc, static_cast<uint32_t>(p.dev_idx), static_cast<uint32_t>(p.tr_idx), p.phase, p.amp);
                           });
  }

 private:
  struct Prop {
    size_t dev_idx;
    size_t tr_idx;
    double phase;
    double amp;
  };

  std::vector<Prop> _props;
};

}  // namespace autd3::gain
