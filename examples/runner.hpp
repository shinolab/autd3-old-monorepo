// File: runner.hpp
// Project: examples
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <functional>
#include <iostream>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "autd3.hpp"
#include "tests/advanced.hpp"
#include "tests/bessel.hpp"
#include "tests/flag.hpp"
#include "tests/focus.hpp"
#include "tests/gain_stm.hpp"
#include "tests/group.hpp"
#include "tests/plane.hpp"
#include "tests/soft_stm.hpp"
#ifdef BUILD_GAIN_HOLO
#include "tests/holo.hpp"
#endif
#ifdef BUILD_MODULATION_AUDIO_FILE
#include "tests/mod_audio_file.hpp"
#endif
#include "tests/focus_stm.hpp"

inline int run(autd3::Controller& autd) {
  using F = std::function<void(autd3::Controller&)>;
  std::vector<std::pair<F, std::string>> tests = {
      std::pair(F{focus_test}, "Single focus Test"),
      std::pair(F{bessel_test}, "Bessel beam Test"),
      std::pair(F{plane_test}, "Plane wave Test"),
#ifdef BUILD_MODULATION_AUDIO_FILE
      std::pair(F{mod_audio_file_test}, "Wav and RawPCM modulation Test"),
#endif
      std::pair(F{focus_stm}, "FocusSTM Test"),
      std::pair(F{gain_stm}, "GainSTM Test"),
      std::pair(F{soft_stm}, "Software STM Test"),
#ifdef BUILD_GAIN_HOLO
      std::pair(F{holo_test}, "Holo Test"),
#endif
      std::pair(F{advanced_test}, "Custom Gain & Modulation Test"),
      std::pair(F{flag_test}, "Flag Test"),
  };
  if (autd.geometry().num_devices() == 2) tests.emplace_back(std::pair(F{group_test}, "Grouped Gain Test"));

  const auto firm_infos = autd.firmware_infos();

  if (!std::all_of(firm_infos.begin(), firm_infos.end(), autd3::FirmwareInfo::matches_version))
    std::cerr << "\033[93mWARN: FPGA and CPU firmware version do not match.\033[0m" << std::endl;
  if (!std::all_of(firm_infos.begin(), firm_infos.end(), autd3::FirmwareInfo::is_supported))
    std::cerr << "\033[93mWARN: You are using old firmware. Please consider updating to " << autd3::FirmwareInfo::latest_version() << ".\033[0m"
              << std::endl;

  std::cout << "================================== AUTD3 firmware information ==================================" << std::endl;
  std::copy(firm_infos.begin(), firm_infos.end(), std::ostream_iterator<autd3::FirmwareInfo>(std::cout, "\n"));
  std::cout << "================================================================================================" << std::endl;

  autd << autd3::clear << autd3::synchronize;

  while (true) {
    size_t i = 0;
    std::transform(tests.begin(), tests.end(), std::ostream_iterator<std::string>(std::cout, "\n"),
                   [&i](const auto& test) { return "[" + std::to_string(i++) + "]: " + test.second; });
    std::cout << "[Others]: finish." << std::endl;

    std::cout << "Choose number: ";
    std::string in;
    size_t idx;
    getline(std::cin, in);
    std::stringstream s(in);
    if (const auto empty = in == "\n"; !(s >> idx) || idx >= tests.size() || empty) break;

    tests[idx].first(autd);

    std::cout << "press any key to finish..." << std::endl;
    std::cin.ignore();

    std::cout << "finish." << std::endl;
    autd << autd3::stop;
  }

  autd.close();

  return 0;
}
