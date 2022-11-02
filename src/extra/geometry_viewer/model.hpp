// File: Model.hpp
// Project: include
// Created Date: 26/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <spdlog/fmt/fmt.h>

#include <memory>
#include <string>
#include <utility>
#include <vector>

#if _MSC_VER
#pragma warning(push)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#endif
#define GLM_FORCE_DEPTH_ZERO_TO_ONE
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <glm/gtx/string_cast.hpp>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 6262 26495 26800 26819 28020 26819)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#include <fx/gltf.h>
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

namespace autd3::extra::geometry_viewer::gltf {

struct Primitive {
  uint32_t first_index;
  uint32_t index_count;
  int32_t material_index;
};

struct Material {
  glm::vec4 base_color_factor;
  int32_t base_color_texture_idx;
  float metallic_factor;
  float roughness_factor;
};

struct Vertex {
  glm::vec4 pos;
  glm::vec3 normal;
  glm::vec2 uv;

  static vk::VertexInputBindingDescription get_binding_description() {
    constexpr vk::VertexInputBindingDescription binding_description(0, sizeof(Vertex), vk::VertexInputRate::eVertex);
    return binding_description;
  }

  static auto get_attribute_descriptions() {
    return std::array{
        vk::VertexInputAttributeDescription(0, 0, vk::Format::eR32G32B32A32Sfloat, offsetof(Vertex, pos)),
        vk::VertexInputAttributeDescription(1, 0, vk::Format::eR32G32B32Sfloat, offsetof(Vertex, normal)),
        vk::VertexInputAttributeDescription(2, 0, vk::Format::eR32G32Sfloat, offsetof(Vertex, uv)),
    };
  }
};

struct Lighting {
  float ambient;
  float specular;
};

struct Geometry {
  glm::vec3 pos;
  glm::quat rot;
};

class Model {
 public:
  explicit Model(const std::string& glb_path, std::vector<Geometry> geometries) : _geometries(std::move(geometries)) {
    _doc = fx::gltf::LoadFromBinary(glb_path);

    load_images(_doc);
    load_materials(_doc);

    const auto& parent_gltf_node = _doc.nodes[_doc.scenes[0].nodes[0]];
    std::vector<uint32_t> indices;
    std::vector<Vertex> vertices;
    load_node(parent_gltf_node, _doc, indices, vertices);
    _indices = std::move(indices);
    _vertices = std::move(vertices);
  }

  ~Model() = default;
  Model(const Model& v) = delete;
  Model& operator=(const Model& obj) = delete;
  Model(Model&& obj) = default;
  Model& operator=(Model&& obj) = default;

  [[nodiscard]] const uint8_t* image_data() const { return _image_data; }
  [[nodiscard]] uint32_t image_size() const { return _image_size; }

  [[nodiscard]] std::vector<uint32_t> indices() const { return _indices; }
  [[nodiscard]] std::vector<Vertex> vertices() const { return _vertices; }
  [[nodiscard]] std::vector<Material> materials() const { return _materials; }
  [[nodiscard]] std::vector<Primitive> primitives() const { return _primitives; }
  [[nodiscard]] std::vector<Geometry> geometries() const { return _geometries; }

  void load_node(const fx::gltf::Node& gltf_node, const fx::gltf::Document& doc, std::vector<uint32_t>& indices, std::vector<Vertex>& vertices) {
    for (const int i : gltf_node.children) load_node(doc.nodes[i], doc, indices, vertices);
    if (gltf_node.mesh > -1)
      for (const auto& gltf_primitive : doc.meshes[gltf_node.mesh].primitives) {
        const auto first_index = static_cast<uint32_t>(indices.size());
        const auto vertex_start = static_cast<uint32_t>(vertices.size());
        load_vertices(gltf_primitive, doc, vertices);
        const auto index_count = load_indices(gltf_primitive, vertex_start, doc, indices);
        Primitive primitive{};
        primitive.first_index = first_index;
        primitive.index_count = index_count;
        primitive.material_index = gltf_primitive.material;
        _primitives.emplace_back(primitive);
      }
  }

  void load_vertices(const fx::gltf::Primitive& gltf_primitive, const fx::gltf::Document& doc, std::vector<Vertex>& vertices) const {
    const size_t vertex_count = gltf_primitive.attributes.find("POSITION") != gltf_primitive.attributes.end()
                                    ? doc.accessors[gltf_primitive.attributes.find("POSITION")->second].count
                                    : 0;

    const auto load = [gltf_primitive, doc](const char* key) -> const float* {
      if (gltf_primitive.attributes.find(key) != gltf_primitive.attributes.end()) {
        const auto accessor_byte_offset = static_cast<size_t>(doc.accessors[gltf_primitive.attributes.find(key)->second].byteOffset);
        const auto buffer_view = doc.accessors[gltf_primitive.attributes.find(key)->second].bufferView;
        const auto buffer = doc.bufferViews[buffer_view].buffer;
        const auto byte_offset = static_cast<size_t>(doc.bufferViews[buffer_view].byteOffset);
        return reinterpret_cast<const float*>(&doc.buffers[buffer].data[accessor_byte_offset + byte_offset]);
      }
      return nullptr;
    };

    const auto* position_buffer = load("POSITION");
    const auto* normals_buffer = load("NORMAL");
    const auto* tex_coords_buffer = load("TEXCOORD_0");

    for (size_t v = 0; v < vertex_count; v++) {
      Vertex vert{};
      const auto p_gl = glm::make_vec3(&position_buffer[v * 3]);
      const auto n_gl = normals_buffer ? glm::make_vec3(&normals_buffer[v * 3]) : glm::vec3(0.0f);
      vert.pos = glm::vec4(p_gl.x, -p_gl.z, p_gl.y, 1.0f) / 100.0f;  // into AUTD3 coordinate
      vert.normal = normalize(glm::vec3(n_gl.x, -n_gl.z, n_gl.y));
      vert.uv = tex_coords_buffer ? glm::make_vec2(&tex_coords_buffer[v * 2]) : glm::vec3(0.0f);
      vertices.emplace_back(vert);
    }
  }

  uint32_t load_indices(const fx::gltf::Primitive& gltf_primitive, const uint32_t vertex_start, const fx::gltf::Document& doc,
                        std::vector<uint32_t>& indices) const {
    const auto component_type = doc.accessors[gltf_primitive.indices].componentType;
    const auto buffer_view = doc.accessors[gltf_primitive.indices].bufferView;
    const auto accessor_byte_offset = static_cast<size_t>(doc.accessors[gltf_primitive.indices].byteOffset);
    const auto count = doc.accessors[gltf_primitive.indices].count;
    const auto buffer_idx = doc.bufferViews[buffer_view].buffer;
    const auto byte_offset = static_cast<size_t>(doc.bufferViews[buffer_view].byteOffset);
    const auto& data = doc.buffers[buffer_idx].data;

    switch (component_type) {
      case fx::gltf::Accessor::ComponentType::UnsignedInt: {
        const auto buf = std::make_unique<uint32_t[]>(count);
        memcpy(buf.get(), &data[accessor_byte_offset + byte_offset], count * sizeof(uint32_t));
        for (size_t index = 0; index < count; index++) indices.emplace_back(buf[index] + vertex_start);
        break;
      }
      case fx::gltf::Accessor::ComponentType::UnsignedShort: {
        const auto buf = std::make_unique<uint16_t[]>(count);
        memcpy(buf.get(), &data[accessor_byte_offset + byte_offset], count * sizeof(uint16_t));
        for (size_t index = 0; index < count; index++) indices.emplace_back(buf[index] + vertex_start);
        break;
      }
      case fx::gltf::Accessor::ComponentType::UnsignedByte: {
        const auto buf = std::make_unique<uint8_t[]>(count);
        memcpy(buf.get(), &data[accessor_byte_offset + byte_offset], count * sizeof(uint8_t));
        for (size_t index = 0; index < count; index++) indices.emplace_back(buf[index] + vertex_start);
        break;
      }
      case fx::gltf::Accessor::ComponentType::None:
      case fx::gltf::Accessor::ComponentType::Byte:
      case fx::gltf::Accessor::ComponentType::Short:
      case fx::gltf::Accessor::ComponentType::Float:
        throw std::runtime_error(fmt::format("Not supported component type: {}", static_cast<uint16_t>(component_type)));
    }

    return count;
  }

  void load_images(const fx::gltf::Document& doc) {
    const auto& [name, buffer, byteOffset, byteLength, byteStride, target, extensionsAndExtras] = doc.bufferViews[doc.images[0].bufferView];
    _image_data = &doc.buffers[buffer].data[byteOffset];
    _image_size = byteLength;
  }

  void load_materials(const fx::gltf::Document& doc) {
    _materials.reserve(doc.materials.size());
    for (const auto& [alphaCutoff, alphaMode, doubleSided, normalTexture, occlusionTexture, pbrMetallicRoughness, emissiveTexture, emissiveFactor,
                      name, extensionsAndExtras] : doc.materials) {
      _materials.emplace_back(Material{glm::make_vec4(pbrMetallicRoughness.baseColorFactor.data()), pbrMetallicRoughness.baseColorTexture.index,
                                       pbrMetallicRoughness.metallicFactor, pbrMetallicRoughness.roughnessFactor});
    }
  }

 private:
  std::vector<Geometry> _geometries;

  fx::gltf::Document _doc;

  const uint8_t* _image_data = nullptr;
  uint32_t _image_size{};

  std::vector<Material> _materials;
  std::vector<uint32_t> _indices;
  std::vector<Vertex> _vertices;
  std::vector<Primitive> _primitives;
};

}  // namespace autd3::extra::geometry_viewer::gltf
