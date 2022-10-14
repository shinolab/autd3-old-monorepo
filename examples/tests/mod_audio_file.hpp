// File: mod_audio_file.hpp
// Project: tests
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <filesystem>
#include <string>

#include "autd3.hpp"
#include "autd3/modulation/audio_file.hpp"

namespace fs = std::filesystem;

inline void mod_audio_file_test(autd3::Controller& autd) {
  autd3::SilencerConfig config;
  autd.send(config);

  const fs::path path = fs::path(AUTD3_RESOURCE_PATH).append("sin150.wav");
  autd3::modulation::Wav m(path.string());
     //const fs::path path = fs::path(AUTD3_RESOURCE_PATH).append("sin150.dat");
     //autd3::modulation::RawPCM m(path.string(), 4e3);

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);

  autd3::gain::Focus g(center);

  autd.send(m, g);
}
