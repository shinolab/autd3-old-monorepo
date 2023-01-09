// File: vulkan_handler.hpp
// Project: include
// Created Date: 23/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <utility>
#include <vulkan_context.hpp>

#include "window_handler.hpp"

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 6262 26451 26819)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#define STB_IMAGE_IMPLEMENTATION
#include <stb_image.h>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif

namespace autd3::extra::geometry_viewer {

class VulkanHandler {
 public:
  explicit VulkanHandler(const helper::VulkanContext* const context) noexcept : _context(context) {}
  ~VulkanHandler() = default;
  VulkanHandler(const VulkanHandler& v) = delete;
  VulkanHandler& operator=(const VulkanHandler& obj) = delete;
  VulkanHandler(VulkanHandler&& obj) = default;
  VulkanHandler& operator=(VulkanHandler&& obj) = default;

  void create_texture_image(const uint8_t* image_buffer, const uint32_t image_len) {
    int tex_width, tex_height, tex_channels;
    stbi_uc* pixels = stbi_load_from_memory(image_buffer, static_cast<int>(image_len), &tex_width, &tex_height, &tex_channels, STBI_rgb_alpha);
    if (!pixels) throw std::runtime_error("Failed to load texture image!");

    _mip_levels = static_cast<uint32_t>(std::floor(std::log2(std::max(tex_width, tex_height)))) + 1;
    const auto image_size = tex_width * tex_height * 4;

    auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
        image_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

    void* data;
    if (_context->device().mapMemory(staging_buffer_memory.get(), 0, image_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("Failed to map texture buffer.");

    std::memcpy(data, pixels, static_cast<size_t>(image_size));
    _context->device().unmapMemory(staging_buffer_memory.get());
    stbi_image_free(pixels);

    const auto flag = vk::ImageUsageFlagBits::eTransferDst | vk::ImageUsageFlagBits::eSampled | vk::ImageUsageFlagBits::eTransferSrc;
    auto [texture_image, texture_image_memory] =
        _context->create_image(static_cast<uint32_t>(tex_width), static_cast<uint32_t>(tex_height), _mip_levels, vk::SampleCountFlagBits::e1,
                               vk::Format::eR8G8B8A8Srgb, vk::ImageTiling::eOptimal, flag, vk::MemoryPropertyFlagBits::eDeviceLocal);
    _texture_image = std::move(texture_image);
    _texture_image_memory = std::move(texture_image_memory);

    _context->transition_image_layout(_texture_image, vk::Format::eR8G8B8A8Srgb, vk::ImageLayout::eUndefined, vk::ImageLayout::eTransferDstOptimal,
                                      _mip_levels);
    _context->copy_buffer_to_image(staging_buffer, _texture_image, static_cast<uint32_t>(tex_width), static_cast<uint32_t>(tex_height));
    return _context->generate_mipmaps(_texture_image, vk::Format::eR8G8B8A8Srgb, tex_width, tex_height, _mip_levels);
  }

  void create_texture_image_view() {
    _texture_image_view = _context->create_image_view(_texture_image.get(), vk::Format::eR8G8B8A8Srgb, vk::ImageAspectFlagBits::eColor, _mip_levels);
  }

  void create_texture_sampler() {
    const auto properties = _context->physical_device().getProperties();

    _texture_sampler = _context->device().createSamplerUnique(vk::SamplerCreateInfo()
                                                                  .setMagFilter(vk::Filter::eLinear)
                                                                  .setMinFilter(vk::Filter::eLinear)
                                                                  .setMipmapMode(vk::SamplerMipmapMode::eLinear)
                                                                  .setAddressModeU(vk::SamplerAddressMode::eRepeat)
                                                                  .setAddressModeV(vk::SamplerAddressMode::eRepeat)
                                                                  .setAddressModeW(vk::SamplerAddressMode::eRepeat)
                                                                  .setMipLodBias(0.0f)
                                                                  .setAnisotropyEnable(true)
                                                                  .setMaxAnisotropy(properties.limits.maxSamplerAnisotropy)
                                                                  .setCompareEnable(false)
                                                                  .setCompareOp(vk::CompareOp::eAlways)
                                                                  .setMinLod(0.0f)
                                                                  .setMaxLod(static_cast<float>(_mip_levels))
                                                                  .setBorderColor(vk::BorderColor::eIntOpaqueBlack)
                                                                  .setUnnormalizedCoordinates(false));
  }

  [[nodiscard]] vk::ImageView image_view() const { return _texture_image_view.get(); }
  [[nodiscard]] vk::Sampler sampler() const { return _texture_sampler.get(); }

 private:
  const helper::VulkanContext* _context{nullptr};

  uint32_t _mip_levels = 1;
  vk::UniqueImage _texture_image;
  vk::UniqueDeviceMemory _texture_image_memory;
  vk::UniqueImageView _texture_image_view;
  vk::UniqueSampler _texture_sampler;
};

}  // namespace autd3::extra::geometry_viewer
