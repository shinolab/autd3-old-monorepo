// File: audio_file.hpp
// Project: modulation
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <filesystem>

#include "autd3/internal/exception.hpp"
#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::modulation {

class Wav final : public internal::Modulation {
 public:
  explicit Wav(std::filesystem::path path) : _path(std::move(path)) {}

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    char err[256]{};
    const auto ptr = internal::native_methods::AUTDModulationWav(_path.string().c_str(), err);
    if (ptr._0 == nullptr) throw internal::AUTDException(err);
    return ptr;
  }

 private:
  std::filesystem::path _path;
};

}  // namespace autd3::modulation
