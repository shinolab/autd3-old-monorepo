// File: holo.cpp
// Project: holo
// Created Date: 09/12/2021
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#include "autd3/gain/holo.hpp"

#include <autd3/core/geometry/legacy_transducer.hpp>
#include <autd3/core/geometry/normal_transducer.hpp>
#include <iostream>

namespace autd3::gain::holo {

namespace {
template <typename T>
void sdp_calc_impl(const BackendPtr backend, const core::Geometry<T>& geometry) {
  VectorXd r(3);
  VectorXd i(3);
  VectorXc c(3);
  r << 0.0, 1.0, 2.0;
  i << 3.0, 4.0, 5.0;
  backend->make_complex(r, i, c);
  std::cout << c << std::endl;
  r << 6.0, 7.0, 8.0;
  i << 9.0, 10.0, 11.0;
  backend->make_complex(r, i, c);
  std::cout << c << std::endl;
}
}  // namespace

void SDP<core::LegacyTransducer>::calc(const core::Geometry<core::LegacyTransducer>& geometry) { sdp_calc_impl(_backend, geometry); }
void SDP<core::NormalTransducer>::calc(const core::Geometry<core::NormalTransducer>& geometry) { sdp_calc_impl(_backend, geometry); }

}  // namespace autd3::gain::holo
