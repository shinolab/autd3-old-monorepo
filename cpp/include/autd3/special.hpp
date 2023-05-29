// File: special.hpp
// Project: autd3
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3 {

class ModDelayConfig final : public internal::SpecialData {
 public:
  ModDelayConfig() : internal::SpecialData(internal::native_methods::AUTDModDelayConfig()) {}
};

class Clear final : public internal::SpecialData {
 public:
  Clear() : internal::SpecialData(internal::native_methods::AUTDClear()) {}
};

class UpdateFlag final : public internal::SpecialData {
 public:
  UpdateFlag() : internal::SpecialData(internal::native_methods::AUTDUpdateFlags()) {}
};

class Synchronize final : public internal::SpecialData {
 public:
  Synchronize() : internal::SpecialData(internal::native_methods::AUTDSynchronize()) {}
};

class Stop final : public internal::SpecialData {
 public:
  Stop() : internal::SpecialData(internal::native_methods::AUTDStop()) {}
};

}  // namespace autd3
