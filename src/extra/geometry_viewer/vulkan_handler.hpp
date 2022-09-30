// File: vulkan_handler.hpp
// Project: include
// Created Date: 23/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <iostream>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

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

namespace autd3::extra::geometry_viewer {

struct SwapChainSupportDetails {
  vk::SurfaceCapabilitiesKHR capabilities;
  std::vector<vk::SurfaceFormatKHR> formats;
  std::vector<vk::PresentModeKHR> present_modes;
};

struct QueueFamilyIndices {
  std::optional<uint32_t> graphics_family;
  std::optional<uint32_t> present_family;

  [[nodiscard]] bool is_complete() const { return graphics_family.has_value() && present_family.has_value(); }
};

class VulkanHandler {
 public:
  explicit VulkanHandler(const size_t gpu_idx = 0, const bool enable_validation_layers = true) noexcept
      : _gpu_idx(gpu_idx), _enable_validation_layers(enable_validation_layers) {}
  ~VulkanHandler() = default;
  VulkanHandler(const VulkanHandler& v) = delete;
  VulkanHandler& operator=(const VulkanHandler& obj) = delete;
  VulkanHandler(VulkanHandler&& obj) = default;
  VulkanHandler& operator=(VulkanHandler&& obj) = default;

  [[nodiscard]] bool is_device_suitable(const vk::PhysicalDevice& device) const {
    if (const QueueFamilyIndices family_indices = find_queue_families(device); !family_indices.is_complete()) return false;
    if (!check_device_extension_support(device, device_extensions)) return false;
    const auto [capabilities, formats, present_modes] = query_swap_chain_support(device);
    const auto supported_features = device.getFeatures();
    return !formats.empty() && !present_modes.empty() && supported_features.samplerAnisotropy;
  }

  [[nodiscard]] QueueFamilyIndices find_queue_families(const vk::PhysicalDevice& device) const {
    QueueFamilyIndices family_indices;
    uint32_t i = 0;
    for (const auto& queue_family : device.getQueueFamilyProperties()) {
      if (queue_family.queueFlags & vk::QueueFlagBits::eGraphics) family_indices.graphics_family = i;
      if (device.getSurfaceSupportKHR(i, _surface.get())) family_indices.present_family = i;
      if (family_indices.is_complete()) break;
      i++;
    }
    return family_indices;
  }

  [[nodiscard]] SwapChainSupportDetails query_swap_chain_support(const vk::PhysicalDevice& device) const {
    SwapChainSupportDetails details;
    details.capabilities = device.getSurfaceCapabilitiesKHR(_surface.get());
    details.formats = device.getSurfaceFormatsKHR(_surface.get());
    details.present_modes = device.getSurfacePresentModesKHR(_surface.get());
    return details;
  }

  [[nodiscard]] vk::Format find_depth_format() const {
    return find_supported_format({vk::Format::eD32Sfloat, vk::Format::eD32SfloatS8Uint, vk::Format::eD24UnormS8Uint}, vk::ImageTiling::eOptimal,
                                 vk::FormatFeatureFlagBits::eDepthStencilAttachment);
  }

  [[nodiscard]] vk::Format find_supported_format(const std::vector<vk::Format>& candidates, const vk::ImageTiling tiling,
                                                 const vk::FormatFeatureFlags features) const {
    const auto it = std::find_if(candidates.begin(), candidates.end(), [this, tiling, features](const auto format) {
      const auto props = _physical_device.getFormatProperties(format);
      if (tiling == vk::ImageTiling::eLinear && (props.linearTilingFeatures & features) == features) return true;
      if (tiling == vk::ImageTiling::eOptimal && (props.optimalTilingFeatures & features) == features) return true;
      return false;
    });
    if (it == candidates.end()) throw std::runtime_error("failed to find supported format!");
    return *it;
  }

  [[nodiscard]] uint32_t find_memory_type(const uint32_t type_filter, const vk::MemoryPropertyFlags properties) const {
    const auto mem_properties = _physical_device.getMemoryProperties();
    for (uint32_t i = 0; i < mem_properties.memoryTypeCount; i++)
      if (type_filter & 1 << i)
        if ((mem_properties.memoryTypes[i].propertyFlags & properties) == properties) return i;
    throw std::runtime_error("failed to find suitable memory type!");
  }

  [[nodiscard]] vk::SampleCountFlagBits get_max_usable_sample_count() const {
    const auto physical_device_properties = _physical_device.getProperties();

    const auto counts =
        physical_device_properties.limits.framebufferColorSampleCounts & physical_device_properties.limits.framebufferDepthSampleCounts;
    if (counts & vk::SampleCountFlagBits::e64) {
      return vk::SampleCountFlagBits::e64;
    }
    if (counts & vk::SampleCountFlagBits::e32) {
      return vk::SampleCountFlagBits::e32;
    }
    if (counts & vk::SampleCountFlagBits::e16) {
      return vk::SampleCountFlagBits::e16;
    }
    if (counts & vk::SampleCountFlagBits::e8) {
      return vk::SampleCountFlagBits::e8;
    }
    if (counts & vk::SampleCountFlagBits::e4) {
      return vk::SampleCountFlagBits::e4;
    }
    if (counts & vk::SampleCountFlagBits::e2) {
      return vk::SampleCountFlagBits::e2;
    }

    return vk::SampleCountFlagBits::e1;
  }

  void init_vulkan(const WindowHandler& window) {
    create_instance();
    create_surface(window);
    pick_physical_device();
    create_logical_device();
  }

  void create_instance() {
    const auto app_info = vk::ApplicationInfo()
                              .setPApplicationName("Geometry Viewer")
                              .setApplicationVersion(VK_MAKE_VERSION(1, 0, 0))
                              .setApiVersion(VK_API_VERSION_1_2);

    if (_enable_validation_layers && !check_validation_layer_support(validation_layers))
      throw std::runtime_error("validation layers requested, but not available!");

    uint32_t glfw_extension_count = 0;
    const char** glfw_extensions = glfwGetRequiredInstanceExtensions(&glfw_extension_count);

    vk::InstanceCreateInfo create_info = vk::InstanceCreateInfo()
                                             .setPApplicationInfo(&app_info)
                                             .setEnabledExtensionCount(glfw_extension_count)
                                             .setPpEnabledExtensionNames(glfw_extensions);
    if (_enable_validation_layers) create_info.setPEnabledLayerNames(validation_layers);

    _instance = createInstanceUnique(create_info, nullptr);
  }

  void create_surface(const WindowHandler& window) {
    VkSurfaceKHR surface{};
    if (glfwCreateWindowSurface(_instance.get(), window.window(), nullptr, &surface) != VK_SUCCESS)
      throw std::runtime_error("failed to create window surface!");
    const vk::ObjectDestroy<vk::Instance, VULKAN_HPP_DEFAULT_DISPATCHER_TYPE> deleter(_instance.get());
    _surface = vk::UniqueSurfaceKHR(vk::SurfaceKHR(surface), deleter);
  }

  void pick_physical_device() {
    const auto devices = _instance->enumeratePhysicalDevices();
    std::vector<vk::PhysicalDevice> suitable_devices;
    std::copy_if(devices.begin(), devices.end(), std::back_inserter(suitable_devices),
                 [this](const auto& device) { return is_device_suitable(device); });
    if (suitable_devices.empty()) throw std::runtime_error("failed to find a suitable GPU!");
    if (_gpu_idx < suitable_devices.size())
      _physical_device = suitable_devices[_gpu_idx];
    else {
      _physical_device = suitable_devices[0];
      const auto props = _physical_device.getProperties();
      std::cerr << "WARN: Cannot use selected GPU (" << _gpu_idx << "), " << props.deviceName << " is used instead." << std::endl;
    }
    _msaa_samples = get_max_usable_sample_count();
  }

  void create_logical_device() {
    const auto [graphics_family, present_family] = find_queue_families(_physical_device);

    const std::array queue_priority = {1.0f};
    const std::array create_infos = {vk::DeviceQueueCreateInfo().setQueueFamilyIndex(graphics_family.value()).setQueuePriorities(queue_priority)};
    constexpr vk::PhysicalDeviceFeatures features = vk::PhysicalDeviceFeatures().setSamplerAnisotropy(VK_TRUE);
    vk::DeviceCreateInfo create_info =
        vk::DeviceCreateInfo().setQueueCreateInfos(create_infos).setPEnabledFeatures(&features).setPEnabledExtensionNames(device_extensions);
    if (_enable_validation_layers) create_info.setPEnabledLayerNames(validation_layers);

    _device = _physical_device.createDeviceUnique(create_info);
    _graphics_queue = _device->getQueue(graphics_family.value(), 0);
    _present_queue = _device->getQueue(present_family.value(), 0);
  }

  void cleanup() { _device->waitIdle(); }

  [[nodiscard]] vk::UniqueImageView create_image_view(const vk::Image image, const vk::Format format, const vk::ImageAspectFlagBits aspect_flags,
                                                      const uint32_t mip_levels) const {
    return _device->createImageViewUnique(vk::ImageViewCreateInfo()
                                              .setImage(image)
                                              .setViewType(vk::ImageViewType::e2D)
                                              .setFormat(format)
                                              .setSubresourceRange(vk::ImageSubresourceRange()
                                                                       .setAspectMask(aspect_flags)
                                                                       .setBaseMipLevel(0)
                                                                       .setLevelCount(mip_levels)
                                                                       .setBaseArrayLayer(0)
                                                                       .setLayerCount(1)));
  }

  [[nodiscard]] std::pair<vk::UniqueImage, vk::UniqueDeviceMemory> create_image(const uint32_t width, const uint32_t height,
                                                                                const uint32_t mip_levels, const vk::SampleCountFlagBits num_samples,
                                                                                const vk::Format format, const vk::ImageTiling tiling,
                                                                                const vk::ImageUsageFlags usage,
                                                                                const vk::MemoryPropertyFlags properties) const {
    const vk::ImageCreateInfo image_info = vk::ImageCreateInfo()
                                               .setImageType(vk::ImageType::e2D)
                                               .setFormat(format)
                                               .setExtent(vk::Extent3D(width, height, 1))
                                               .setMipLevels(mip_levels)
                                               .setArrayLayers(1)
                                               .setSamples(num_samples)
                                               .setTiling(tiling)
                                               .setUsage(usage)
                                               .setSharingMode(vk::SharingMode::eExclusive);

    auto image = _device->createImageUnique(image_info);

    const vk::MemoryRequirements mem_requirements = _device->getImageMemoryRequirements(image.get());

    const vk::MemoryAllocateInfo alloc_info = vk::MemoryAllocateInfo()
                                                  .setAllocationSize(mem_requirements.size)
                                                  .setMemoryTypeIndex(find_memory_type(mem_requirements.memoryTypeBits, properties));
    auto image_memory = _device->allocateMemoryUnique(alloc_info);

    _device->bindImageMemory(image.get(), image_memory.get(), 0);

    return std::make_pair(std::move(image), std::move(image_memory));
  }

  [[nodiscard]] std::pair<vk::UniqueBuffer, vk::UniqueDeviceMemory> create_buffer(const vk::DeviceSize size, const vk::BufferUsageFlags usage,
                                                                                  const vk::MemoryPropertyFlags properties) const {
    const vk::BufferCreateInfo buffer_info = vk::BufferCreateInfo().setSize(size).setUsage(usage).setSharingMode(vk::SharingMode::eExclusive);
    auto buffer = _device->createBufferUnique(buffer_info);

    const auto mem_requirements = _device->getBufferMemoryRequirements(buffer.get());
    const vk::MemoryAllocateInfo alloc_info = vk::MemoryAllocateInfo()
                                                  .setAllocationSize(mem_requirements.size)
                                                  .setMemoryTypeIndex(find_memory_type(mem_requirements.memoryTypeBits, properties));
    auto buffer_memory = _device->allocateMemoryUnique(alloc_info);

    _device->bindBufferMemory(buffer.get(), buffer_memory.get(), 0);

    return std::make_pair(std::move(buffer), std::move(buffer_memory));
  }

  void create_command_pool() {
    const auto [graphics_family, present_family] = find_queue_families(_physical_device);
    const vk::CommandPoolCreateInfo pool_info =
        vk::CommandPoolCreateInfo().setFlags(vk::CommandPoolCreateFlagBits::eResetCommandBuffer).setQueueFamilyIndex(graphics_family.value());
    _command_pool = _device->createCommandPoolUnique(pool_info);
  }

  [[nodiscard]] vk::UniqueCommandBuffer begin_single_time_commands() const {
    const vk::CommandBufferAllocateInfo alloc_info(_command_pool.get(), vk::CommandBufferLevel::ePrimary, 1);

    auto command_buffers = _device->allocateCommandBuffersUnique(alloc_info);
    vk::UniqueCommandBuffer command_buffer = std::move(command_buffers[0]);

    const vk::CommandBufferBeginInfo begin_info = vk::CommandBufferBeginInfo().setFlags(vk::CommandBufferUsageFlagBits::eOneTimeSubmit);

    command_buffer->begin(begin_info);

    return command_buffer;
  }

  void end_single_time_commands(vk::UniqueCommandBuffer& command_buffer) const {
    command_buffer->end();
    const vk::SubmitInfo submit_info = vk::SubmitInfo().setCommandBuffers(command_buffer.get());
    _graphics_queue.submit(submit_info);
    _graphics_queue.waitIdle();
  }

  void generate_mipmaps(vk::UniqueImage& image, const vk::Format format, const int32_t tex_width, const int32_t tex_height,
                        const uint32_t mip_levels) const {
    if (const auto format_properties = _physical_device.getFormatProperties(format);
        !(format_properties.optimalTilingFeatures & vk::FormatFeatureFlagBits::eSampledImageFilterLinear))
      throw std::runtime_error("texture image format does not support linear blitting!");

    auto command_buffer = begin_single_time_commands();

    vk::ImageMemoryBarrier barrier =
        vk::ImageMemoryBarrier()
            .setImage(image.get())
            .setSrcQueueFamilyIndex(VK_QUEUE_FAMILY_IGNORED)
            .setDstQueueFamilyIndex(VK_QUEUE_FAMILY_IGNORED)
            .setSubresourceRange(
                vk::ImageSubresourceRange().setAspectMask(vk::ImageAspectFlagBits::eColor).setBaseArrayLayer(0).setLayerCount(1).setLevelCount(1));

    int32_t mip_width = tex_width;
    int32_t mip_height = tex_height;

    for (uint32_t i = 1; i < mip_levels; i++) {
      barrier.subresourceRange.setBaseMipLevel(i - 1);
      barrier.setOldLayout(vk::ImageLayout::eTransferDstOptimal);
      barrier.setNewLayout(vk::ImageLayout::eTransferSrcOptimal);
      barrier.setSrcAccessMask(vk::AccessFlagBits::eTransferWrite);
      barrier.setDstAccessMask(vk::AccessFlagBits::eTransferRead);

      command_buffer->pipelineBarrier(vk::PipelineStageFlagBits::eTransfer, vk::PipelineStageFlagBits::eTransfer, vk::DependencyFlagBits{}, {}, {},
                                      barrier);

      vk::ImageBlit blit =
          vk::ImageBlit()
              .setSrcOffsets({vk::Offset3D{0, 0, 0}, vk::Offset3D{mip_width, mip_height, 1}})
              .setSrcSubresource(vk::ImageSubresourceLayers()
                                     .setAspectMask(vk::ImageAspectFlagBits::eColor)
                                     .setMipLevel(i - 1)
                                     .setBaseArrayLayer(0)
                                     .setLayerCount(1))
              .setDstOffsets({vk::Offset3D{0, 0, 0}, vk::Offset3D{mip_width > 1 ? mip_width / 2 : 1, mip_height > 1 ? mip_height / 2 : 1, 1}})
              .setDstSubresource(
                  vk::ImageSubresourceLayers().setAspectMask(vk::ImageAspectFlagBits::eColor).setMipLevel(i).setBaseArrayLayer(0).setLayerCount(1));

      command_buffer->blitImage(image.get(), vk::ImageLayout::eTransferSrcOptimal, image.get(), vk::ImageLayout::eTransferDstOptimal, blit,
                                vk::Filter::eLinear);

      barrier.setOldLayout(vk::ImageLayout::eTransferSrcOptimal);
      barrier.setNewLayout(vk::ImageLayout::eShaderReadOnlyOptimal);
      barrier.setSrcAccessMask(vk::AccessFlagBits::eTransferRead);
      barrier.setDstAccessMask(vk::AccessFlagBits::eShaderRead);

      command_buffer->pipelineBarrier(vk::PipelineStageFlagBits::eTransfer, vk::PipelineStageFlagBits::eFragmentShader, vk::DependencyFlagBits{}, {},
                                      {}, barrier);

      if (mip_width > 1) mip_width /= 2;
      if (mip_height > 1) mip_height /= 2;
    }

    barrier.subresourceRange.setBaseMipLevel(mip_levels - 1);
    barrier.setOldLayout(vk::ImageLayout::eTransferDstOptimal);
    barrier.setNewLayout(vk::ImageLayout::eShaderReadOnlyOptimal);
    barrier.setSrcAccessMask(vk::AccessFlagBits::eTransferWrite);
    barrier.setDstAccessMask(vk::AccessFlagBits::eShaderRead);

    command_buffer->pipelineBarrier(vk::PipelineStageFlagBits::eTransfer, vk::PipelineStageFlagBits::eFragmentShader, vk::DependencyFlagBits{}, {},
                                    {}, barrier);

    end_single_time_commands(command_buffer);
  }

  void transition_image_layout(vk::UniqueImage& image, const vk::Format format, const vk::ImageLayout old_layout, const vk::ImageLayout new_layout,
                               const uint32_t mip_levels) const {
    auto command_buffer = begin_single_time_commands();

    vk::ImageMemoryBarrier barrier = vk::ImageMemoryBarrier()
                                         .setOldLayout(old_layout)
                                         .setNewLayout(new_layout)
                                         .setImage(image.get())
                                         .setSubresourceRange(vk::ImageSubresourceRange()
                                                                  .setAspectMask(vk::ImageAspectFlagBits::eColor)
                                                                  .setBaseMipLevel(0)
                                                                  .setLevelCount(mip_levels)
                                                                  .setBaseArrayLayer(0)
                                                                  .setLayerCount(1));
    if (new_layout == vk::ImageLayout::eDepthStencilAttachmentOptimal) {
      barrier.subresourceRange.aspectMask = vk::ImageAspectFlagBits::eDepth;
      if (has_stencil_component(format)) barrier.subresourceRange.aspectMask |= vk::ImageAspectFlagBits::eStencil;
    }

    vk::PipelineStageFlags source_stage;
    vk::PipelineStageFlags destination_stage;

    if (old_layout == vk::ImageLayout::eUndefined && new_layout == vk::ImageLayout::eTransferDstOptimal) {
      barrier.srcAccessMask = vk::AccessFlags(0);
      barrier.dstAccessMask = vk::AccessFlagBits::eTransferWrite;
      source_stage = vk::PipelineStageFlagBits::eTopOfPipe;
      destination_stage = vk::PipelineStageFlagBits::eTransfer;
    } else if (old_layout == vk::ImageLayout::eTransferDstOptimal && new_layout == vk::ImageLayout::eShaderReadOnlyOptimal) {
      barrier.srcAccessMask = vk::AccessFlagBits::eTransferWrite;
      barrier.dstAccessMask = vk::AccessFlagBits::eShaderRead;
      source_stage = vk::PipelineStageFlagBits::eTransfer;
      destination_stage = vk::PipelineStageFlagBits::eFragmentShader;
    } else if (old_layout == vk::ImageLayout::eUndefined && new_layout == vk::ImageLayout::eDepthStencilAttachmentOptimal) {
      barrier.srcAccessMask = vk::AccessFlags(0);
      barrier.dstAccessMask = vk::AccessFlagBits::eDepthStencilAttachmentRead | vk::AccessFlagBits::eDepthStencilAttachmentWrite;
      source_stage = vk::PipelineStageFlagBits::eTopOfPipe;
      destination_stage = vk::PipelineStageFlagBits::eEarlyFragmentTests;
    } else
      throw std::invalid_argument("unsupported layout transition!");

    command_buffer->pipelineBarrier(source_stage, destination_stage, {}, {}, {}, barrier);
    end_single_time_commands(command_buffer);
  }

  void copy_buffer(const vk::Buffer src_buffer, const vk::Buffer dst_buffer, const vk::DeviceSize size) const {
    auto command_buffer = begin_single_time_commands();

    const vk::BufferCopy copy_region = vk::BufferCopy().setSrcOffset(0).setDstOffset(0).setSize(size);
    command_buffer->copyBuffer(src_buffer, dst_buffer, copy_region);

    end_single_time_commands(command_buffer);
  }

  void copy_buffer_to_image(vk::UniqueBuffer& buffer, vk::UniqueImage& image, const uint32_t width, const uint32_t height) const {
    auto command_buffer = begin_single_time_commands();
    vk::BufferImageCopy region =
        vk::BufferImageCopy()
            .setBufferOffset(0)
            .setBufferRowLength(0)
            .setBufferImageHeight(0)
            .setImageSubresource(
                vk::ImageSubresourceLayers().setAspectMask(vk::ImageAspectFlagBits::eColor).setMipLevel(0).setBaseArrayLayer(0).setLayerCount(1))
            .setImageOffset(vk::Offset3D(0, 0, 0))
            .setImageExtent(vk::Extent3D(width, height, 1));
    command_buffer->copyBufferToImage(buffer.get(), image.get(), vk::ImageLayout::eTransferDstOptimal, region);

    end_single_time_commands(command_buffer);
  }

  void create_descriptor_pool() {
    std::array pool_size = {
        vk::DescriptorPoolSize(vk::DescriptorType::eCombinedImageSampler, 100),
        vk::DescriptorPoolSize(vk::DescriptorType::eSampledImage, 100),
        vk::DescriptorPoolSize(vk::DescriptorType::eUniformBuffer, 100),
    };
    const vk::DescriptorPoolCreateInfo pool_info = vk::DescriptorPoolCreateInfo()
                                                       .setFlags(vk::DescriptorPoolCreateFlagBits::eFreeDescriptorSet)
                                                       .setMaxSets(100 * pool_size.size())
                                                       .setPoolSizes(pool_size);
    _descriptor_pool = _device->createDescriptorPoolUnique(pool_info);
  }

  void create_texture_image(const uint8_t* image_buffer, const uint32_t image_len) {
    int tex_width, tex_height, tex_channels;
    stbi_uc* pixels = stbi_load_from_memory(image_buffer, static_cast<int>(image_len), &tex_width, &tex_height, &tex_channels, STBI_rgb_alpha);
    if (!pixels) throw std::runtime_error("failed to load texture image!");
    _mip_levels = static_cast<uint32_t>(std::floor(std::log2(std::max(tex_width, tex_height)))) + 1;
    const auto image_size = tex_width * tex_height * 4;

    auto [staging_buffer, staging_buffer_memory] = create_buffer(
        image_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

    void* data;
    if (_device->mapMemory(staging_buffer_memory.get(), 0, image_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("failed to map texture buffer.");
    std::memcpy(data, pixels, static_cast<size_t>(image_size));
    _device->unmapMemory(staging_buffer_memory.get());
    stbi_image_free(pixels);

    const auto flag = vk::ImageUsageFlagBits::eTransferDst | vk::ImageUsageFlagBits::eSampled | vk::ImageUsageFlagBits::eTransferSrc;
    auto [texture_image, texture_image_memory] =
        create_image(static_cast<uint32_t>(tex_width), static_cast<uint32_t>(tex_height), _mip_levels, vk::SampleCountFlagBits::e1,
                     vk::Format::eR8G8B8A8Srgb, vk::ImageTiling::eOptimal, flag, vk::MemoryPropertyFlagBits::eDeviceLocal);

    _texture_image = std::move(texture_image);
    _texture_image_memory = std::move(texture_image_memory);

    transition_image_layout(_texture_image, vk::Format::eR8G8B8A8Srgb, vk::ImageLayout::eUndefined, vk::ImageLayout::eTransferDstOptimal,
                            _mip_levels);
    copy_buffer_to_image(staging_buffer, _texture_image, static_cast<uint32_t>(tex_width), static_cast<uint32_t>(tex_height));
    generate_mipmaps(_texture_image, vk::Format::eR8G8B8A8Srgb, tex_width, tex_height, _mip_levels);
  }

  void create_texture_image_view() {
    _texture_image_view = create_image_view(_texture_image.get(), vk::Format::eR8G8B8A8Srgb, vk::ImageAspectFlagBits::eColor, _mip_levels);
  }

  void create_texture_sampler() {
    const auto properties = _physical_device.getProperties();

    _texture_sampler = _device->createSamplerUnique(vk::SamplerCreateInfo()
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
  [[nodiscard]] vk::SurfaceKHR surface() const { return _surface.get(); }
  [[nodiscard]] vk::Instance instance() const { return _instance.get(); }
  [[nodiscard]] vk::Device device() const { return _device.get(); }
  [[nodiscard]] vk::PhysicalDevice physical_device() const { return _physical_device; }
  [[nodiscard]] vk::Queue graphics_queue() const { return _graphics_queue; }
  [[nodiscard]] vk::Queue present_queue() const { return _present_queue; }
  [[nodiscard]] vk::SampleCountFlagBits msaa_samples() const { return _msaa_samples; }
  [[nodiscard]] vk::CommandPool command_pool() const { return _command_pool.get(); }
  [[nodiscard]] vk::DescriptorPool descriptor_pool() const { return _descriptor_pool.get(); }

  [[nodiscard]] vk::ImageView image_view() const { return _texture_image_view.get(); }
  [[nodiscard]] vk::Sampler sampler() const { return _texture_sampler.get(); }

 private:
  size_t _gpu_idx;
  bool _enable_validation_layers;

  vk::SampleCountFlagBits _msaa_samples = vk::SampleCountFlagBits::e1;

  vk::UniqueInstance _instance;
  vk::UniqueSurfaceKHR _surface;

  vk::PhysicalDevice _physical_device;
  vk::UniqueDevice _device;

  vk::Queue _graphics_queue;
  vk::Queue _present_queue;

  vk::UniqueDescriptorPool _descriptor_pool;

  vk::UniqueCommandPool _command_pool;
  std::vector<vk::UniqueCommandBuffer> _command_buffers;

  uint32_t _mip_levels = 1;
  vk::UniqueImage _texture_image;
  vk::UniqueDeviceMemory _texture_image_memory;
  vk::UniqueImageView _texture_image_view;
  vk::UniqueSampler _texture_sampler;

  static inline const std::vector validation_layers = {"VK_LAYER_KHRONOS_validation"};
  static inline const std::vector device_extensions = {VK_KHR_SWAPCHAIN_EXTENSION_NAME
#ifdef __APPLE__
                                                       ,
                                                       "VK_KHR_portability_subset"
#endif
  };

  static bool has_stencil_component(const vk::Format format) {
    return format == vk::Format::eD32SfloatS8Uint || format == vk::Format::eD24UnormS8Uint;
  }
  static bool check_validation_layer_support(const std::vector<const char*>& layers) {
    const std::vector<vk::LayerProperties> available_layers = vk::enumerateInstanceLayerProperties();
    return std::all_of(layers.begin(), layers.end(), [available_layers](const char* layer_name) {
      return std::any_of(available_layers.begin(), available_layers.end(),
                         [layer_name](const auto& layer_properties) { return strcmp(layer_name, layer_properties.layerName) == 0; });
    });
  }

  static bool check_device_extension_support(const vk::PhysicalDevice& device, const std::vector<const char*>& extensions) {
    const std::vector<vk::ExtensionProperties> available_extensions = device.enumerateDeviceExtensionProperties();
    std::set<std::string> required_extensions(extensions.begin(), extensions.end());
    for (const auto& extension : available_extensions) required_extensions.erase(extension.extensionName);
    return required_extensions.empty();
  }
};

}  // namespace autd3::extra::geometry_viewer
