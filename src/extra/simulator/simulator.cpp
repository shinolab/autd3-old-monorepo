// File: simulator.cpp
// Project: simulator
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
// 

#include "autd3/extra/simulator/simulator.hpp"

namespace autd3::extra::simulator
{
Simulator Simulator::start() { return std::move(*this); }
void Simulator::exit() {}
}
