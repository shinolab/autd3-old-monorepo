// File: group.cpp
// Project: gain
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/gain/group.hpp>
#include <autd3/gain/null.hpp>
#include <autd3/gain/uniform.hpp>

#include "utils.hpp"

TEST(Gain, Group) {
  auto autd = create_controller();

  const auto cx = autd.geometry().center().x();

  ASSERT_TRUE(autd.send_async(autd3::gain::Group([cx](const auto&, const auto& tr) -> std::optional<const char*> {
                                if (tr.position().x() < cx) return "uniform";
                                return "null";
                              })
                                  .set("uniform", autd3::gain::Uniform(0x80).with_phase(autd3::internal::Phase(0x90)))
                                  .set("null", autd3::gain::Null()))
                  .get());
  for (auto& dev : autd.geometry()) {
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    for (auto& tr : dev) {
      if (tr.position().x() < cx) {
        ASSERT_EQ(0x80, intensities[tr.idx()]);
        ASSERT_EQ(0x90, phases[tr.idx()]);
      } else {
        ASSERT_EQ(0, intensities[tr.idx()]);
        ASSERT_EQ(0, phases[tr.idx()]);
      }
    }
  }

  ASSERT_TRUE(autd.send_async(autd3::gain::Group([cx](const auto&, const auto& tr) -> std::optional<const char*> {
                                if (tr.position().x() < cx) return "uniform";
                                return std::nullopt;
                              }).set("uniform", autd3::gain::Uniform(0x81).with_phase(autd3::internal::Phase(0x91))))
                  .get());
  for (auto& dev : autd.geometry()) {
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    for (auto& tr : dev) {
      if (tr.position().x() < cx) {
        ASSERT_EQ(0x81, intensities[tr.idx()]);
        ASSERT_EQ(0x91, phases[tr.idx()]);
      } else {
        ASSERT_EQ(0, intensities[tr.idx()]);
        ASSERT_EQ(0, phases[tr.idx()]);
      }
    }
  }
}

TEST(Gain, GroupUnkownKey) {
  auto autd = create_controller();

  bool caught_err = false;
  try {
    autd.send_async(autd3::gain::Group([](const auto&, const auto&) -> std::optional<const char*> { return "null"; })
                        .set("uniform", autd3::gain::Uniform(0x80).with_phase(autd3::internal::Phase(0x90)))
                        .set("null", autd3::gain::Null()))
        .get();
  } catch (autd3::internal::AUTDException& e) {
    caught_err = true;
    ASSERT_STREQ("Unknown group key", e.what());
  }

  if (!caught_err) FAIL();
}

TEST(Gain, GroupUnspecifiedKey) {
  auto autd = create_controller();

  bool caught_err = false;
  try {
    autd.send_async(autd3::gain::Group([](const auto&, const auto&) -> std::optional<const char*> { return "null"; })).get();
  } catch (autd3::internal::AUTDException& e) {
    caught_err = true;
    ASSERT_STREQ("Unspecified group key", e.what());
  }

  if (!caught_err) FAIL();
}

TEST(Gain, GroupCheckOnlyForEnabled) {
  auto autd = create_controller();
  autd.geometry()[0].set_enable(false);

  std::vector check(autd.geometry().num_devices(), false);
  ASSERT_TRUE(autd.send_async(autd3::gain::Group([&check](const auto& dev, const auto& tr) -> std::optional<int> {
                                check[dev.idx()] = true;
                                return 0;
                              }).set(0, autd3::gain::Uniform(0x80).with_phase(autd3::internal::Phase(0x90))))
                  .get());

  ASSERT_FALSE(check[0]);
  ASSERT_TRUE(check[1]);

  {
    auto [intensities, phases] = autd.link().intensities_and_phases(0, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
  {
    auto [intensities, phases] = autd.link().intensities_and_phases(1, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0x90; }));
  }
}
