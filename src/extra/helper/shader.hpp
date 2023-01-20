// File: shader.hpp
// Project: helper
// Created Date: 03/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <fstream>
#include <string>
#include <vector>

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 28251 26451)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#include <vulkan/vulkan.hpp>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif

namespace autd3::extra::helper {

inline vk::UniqueShaderModule create_shader_module(const vk::Device& device, const std::vector<uint8_t>& code) {
  return device.createShaderModuleUnique(
      vk::ShaderModuleCreateInfo().setCodeSize(code.size()).setPCode(reinterpret_cast<const uint32_t*>(code.data())));
}

}  // namespace autd3::extra::helper
