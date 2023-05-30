// File: special.hpp
// Project: autd3
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class ModDelayConfig final : public SpecialData {
 public:
  ModDelayConfig() : SpecialData(native_methods::AUTDModDelayConfig()) {}
};

class Clear final : public SpecialData {
 public:
  Clear() : SpecialData(native_methods::AUTDClear()) {}
};

class UpdateFlag final : public SpecialData {
 public:
  UpdateFlag() : SpecialData(native_methods::AUTDUpdateFlags()) {}
};

class Synchronize final : public SpecialData {
 public:
  Synchronize() : SpecialData(native_methods::AUTDSynchronize()) {}
};

class Stop final : public SpecialData {
 public:
  Stop() : SpecialData(native_methods::AUTDStop()) {}
};

}  // namespace autd3::internal
