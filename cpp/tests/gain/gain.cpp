// File: gain.cpp
// Project: gain
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <autd3/gain/gain.hpp>
#include <gtest/gtest.h>

#include "utils.hpp"

class Uniform final : public autd3::gain::Gain {
 public:
  explicit Uniform(const double amp, const double phase) : _amp(amp), _phase(phase) {}

  [[nodiscard]] std::unordered_map<size_t, std::vector<autd3::internal::native_methods::Drive>> calc(const autd3::internal::Geometry& geometry) const override {
    return transform(geometry, [&](const auto& dev, const auto& tr) { return autd3::internal::native_methods::Drive{_phase, _amp}; });
  }

 private:
  double _amp;
  double _phase;
};

TEST(Gain, Gain) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(Uniform(0.5, autd3::internal::pi)));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 2048; }));
  }
}
