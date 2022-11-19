// File: shader.hpp
// Project: helper
// Created Date: 03/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 19/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <fstream>
#include <string>
#include <vector>

namespace autd3::extra::helper {

inline vk::UniqueShaderModule create_shader_module(const vk::Device& device, const std::vector<uint8_t>& code) {
  return device.createShaderModuleUnique(
      vk::ShaderModuleCreateInfo().setCodeSize(code.size()).setPCode(reinterpret_cast<const uint32_t*>(code.data())));
}

}  // namespace autd3::extra::helper
