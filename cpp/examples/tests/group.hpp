// File: group.hpp
// Project: tests
// Created Date: 15/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3.hpp"

template <typename T>
inline void group_by_device_test(autd3::Controller<T>& autd) {
  autd3::Silencer silencer;
  autd.send_async(silencer).get();

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);

  autd.group([](const autd3::Device& dev) -> std::optional<const char*> {
        if (dev.idx() == 0) {
          return "null";
        } else if (dev.idx() == 1) {
          return "focus";
        } else {
          return std::nullopt;
        }
      })
      .set("null", autd3::modulation::Static(), autd3::gain::Null())
      .set("focus", autd3::modulation::Sine(150), autd3::gain::Focus(center))
      .send_async()
      .get();
}

template <typename T>
inline void group_by_transducer_test(autd3::Controller<T>& autd) {
  autd3::Silencer silencer;
  autd.send_async(silencer).get();

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);

  const auto cx = autd.geometry().center().x();
  autd3::gain::Focus g1(autd.geometry().center() + autd3::Vector3(0, 0, 150));
  autd3::gain::Null g2;

  const auto g = autd3::gain::Group([&cx](const autd3::Device&, const autd3::Transducer& tr) -> std::optional<const char*> {
                   if (tr.position().x() < cx) return "focus";
                   return "null";
                 })
                     .set("focus", g1)
                     .set("null", g2);

  autd3::modulation::Sine m(150);
  autd.send_async(m, g).get();
}
