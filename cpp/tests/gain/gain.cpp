// File: gain.cpp
// Project: gain
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/gain/gain.hpp>

#include "utils.hpp"

class Uniform final : public autd3::gain::Gain {
 public:
  explicit Uniform(const double amp, const double phase, std::vector<bool>* cnt) : _amp(amp), _phase(phase), _cnt(cnt) {}

  [[nodiscard]] std::unordered_map<size_t, std::vector<autd3::internal::native_methods::Drive>> calc(
      const autd3::internal::Geometry& geometry) const override {
    return transform(geometry, [&](const auto& dev, const auto&) {
      _cnt->operator[](dev.idx()) = true;
      return autd3::internal::native_methods::Drive{_phase, _amp};
    });
  }

 private:
  double _amp;
  double _phase;
  std::vector<bool>* _cnt;
};

TEST(Gain, Gain) {
  auto autd = create_controller();

  std::vector cnt(autd.geometry().num_devices(), false);
  ASSERT_TRUE(autd.send(Uniform(0.5, autd3::internal::pi, &cnt)));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 2048; }));
  }
}

TEST(Gain, GainCheckOnlyForEnabled) {
  auto autd = create_controller();
  autd.geometry()[0].set_enable(false);

  std::vector check(autd.geometry().num_devices(), false);
  ASSERT_TRUE(autd.send(Uniform(0.5, autd3::internal::pi, &check)));

  ASSERT_FALSE(check[0]);
  ASSERT_TRUE(check[1]);

  {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(0, 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
  {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(1, 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 2048; }));
  }
}
