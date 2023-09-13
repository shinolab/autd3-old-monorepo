// File: firmware_info.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>

#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class FirmwareInfo {
 public:
  explicit FirmwareInfo(std::string info) noexcept : _info(std::move(info)) {}

  [[nodiscard]] std::string info() const { return _info; }

  [[nodiscard]] static std::string latest_version() {
    char info[256];
    native_methods::AUTDGetLatestFirmware(info);
    return {info};
  }

 private:
  std::string _info;
};

inline std::ostream& operator<<(std::ostream& os, const FirmwareInfo& obj) {
  os << obj.info();
  return os;
}

}  // namespace autd3::internal
