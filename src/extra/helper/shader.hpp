// File: shader.hpp
// Project: helper
// Created Date: 03/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <fstream>
#include <string>
#include <vector>

namespace autd3::extra::helper {

inline std::vector<char> read_file(const std::string& filename) {
  std::ifstream file(filename, std::ios::ate | std::ios::binary);
  if (!file.is_open()) throw std::runtime_error("failed to open file!");

  const size_t file_size = file.tellg();
  std::vector<char> buffer(file_size);

  file.seekg(0);
  file.read(buffer.data(), static_cast<std::streamsize>(file_size));

  file.close();

  return buffer;
}

inline vk::UniqueShaderModule create_shader_module(const vk::Device& device, const std::vector<char>& code) {
  return device.createShaderModuleUnique(
      vk::ShaderModuleCreateInfo().setCodeSize(code.size()).setPCode(reinterpret_cast<const uint32_t*>(code.data())));
}

}  // namespace autd3::extra::helper
