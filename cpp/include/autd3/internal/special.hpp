// File: special.hpp
// Project: autd3
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/06/2023
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
  ModDelayConfig() = default;

  [[nodiscard]] native_methods::DatagramSpecialPtr ptr() const override { return native_methods::AUTDModDelayConfig(); }
};

class Clear final : public SpecialData {
 public:
  Clear() = default;

  [[nodiscard]] native_methods::DatagramSpecialPtr ptr() const override { return native_methods::AUTDClear(); }
};

class UpdateFlags final : public SpecialData {
 public:
  UpdateFlags() = default;

  [[nodiscard]] native_methods::DatagramSpecialPtr ptr() const override { return native_methods::AUTDUpdateFlags(); }
};

class Synchronize final : public SpecialData {
 public:
  Synchronize() = default;

  [[nodiscard]] native_methods::DatagramSpecialPtr ptr() const override { return native_methods::AUTDSynchronize(); }
};

class Stop final : public SpecialData {
 public:
  Stop() = default;

  [[nodiscard]] native_methods::DatagramSpecialPtr ptr() const override { return native_methods::AUTDStop(); }
};

}  // namespace autd3::internal
