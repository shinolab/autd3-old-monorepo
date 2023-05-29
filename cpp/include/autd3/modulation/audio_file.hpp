// File: audio_file.hpp
// Project: modulation
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/exception.hpp"
#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::modulation {

class Wav : public internal::Modulation {
 public:
  Wav(const std::filesystem::path& path) : internal::Modulation(nullptr) {
    char err[256]{};
    _ptr = internal::native_methods::AUTDModulationWav(path.string().c_str(), err);
    if (_ptr == nullptr) {
      throw internal::AUTDException(err);
    }
  }
};

}  // namespace autd3::modulation
