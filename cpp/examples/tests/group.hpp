// File: group.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 19/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3.hpp"

inline void group_test(autd3::Controller& autd) {
  autd3::SilencerConfig silencer;
  autd.send(silencer);

  autd3::modulation::Sine m(150);  // 150Hz AM

  if (autd.geometry().num_devices() > 1) {
    const autd3::Vector3 center = autd.geometry().center_of(0) + autd3::Vector3(0.0, 0.0, 150.0);
    autd3::gain::Focus g1(center);

    const autd3::Vector3 apex = autd.geometry().center_of(1);
    autd3::gain::Bessel g2(apex, autd3::Vector3::UnitZ(), 13.0 / 180.0 * autd3::pi);

    auto g = autd3::gain::Group::by_device([](size_t dev) {
               if (dev == 0)
                 return std::optional("focus");
               else
                 return std::optional("bessel");
             })
                 .set("focus", g1)
                 .set("bessel", g2);

    autd.send(m, g);
  } else {
    const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
    autd3::gain::Focus g1(center);
    autd3::gain::Null g2;

    const auto cx = center.x();
    auto g = autd3::gain::Group::by_transducer([cx](const autd3::Transducer& tr) {
               if (tr.position().x() < cx)
                 return std::optional("focus");
               else
                 return std::optional("null");
             })
                 .set("focus", g1)
                 .set("null", g2);
    autd.send(m, g);
  }
}
