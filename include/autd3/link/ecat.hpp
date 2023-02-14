// File: ecat.hpp
// Project: link
// Created Date: 07/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>
#include <utility>

namespace autd3::link {
/**
 * \brief EtherCAT adapter information for SOEM
 */
struct EtherCATAdapter final {
  EtherCATAdapter(std::string desc, std::string name) : desc(std::move(desc)), name(std::move(name)) {}

  std::string desc;
  std::string name;
};

enum class SyncMode { FreeRun, DC };

}  // namespace autd3::link
