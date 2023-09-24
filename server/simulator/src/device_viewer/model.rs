/*
 * File: model.rs
 * Project: autd-server
 * Created Date: 23/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use bytemuck::{Pod, Zeroable};
use gltf::{buffer::Data, Document, Node, Semantic};
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod, Vertex)]
pub struct ModelVertex {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    norm: [f32; 3],
    #[format(R32G32_SFLOAT)]
    uv: [f32; 2],
}
pub struct Primitive {
    pub first_index: u32,
    pub index_count: u32,
    pub material_index: usize,
}

pub struct Material {
    pub base_color_factor: [f32; 4],
    pub base_color_texture_idx: Option<usize>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
}

pub struct Model {
    pub vertices: Vec<ModelVertex>,
    pub indices: Vec<u32>,
    pub primitives: Vec<Primitive>,
    pub image: gltf::image::Data,
    pub materials: Vec<Material>,
}

impl Model {
    fn load_autd3_model() -> anyhow::Result<Vec<u8>> {
        Ok(std::fs::read("assets/autd3.glb")?)
    }

    pub fn new() -> anyhow::Result<Self> {
        let glb = Self::load_autd3_model()?;
        let (document, buffers, images) = gltf::import_slice(glb).unwrap();
        let node = document.scenes().next().unwrap().nodes().next().unwrap();

        let materials = Self::load_materials(&document);

        let mut indices = Vec::new();
        let mut vertices = Vec::new();
        let mut primitives = Vec::new();
        Self::load_node(node, &buffers, &mut indices, &mut vertices, &mut primitives);

        Ok(Self {
            indices,
            vertices,
            primitives,
            image: images[0].clone(),
            materials,
        })
    }

    fn load_node(
        node: Node,
        buffers: &[Data],
        indices: &mut Vec<u32>,
        vertices: &mut Vec<ModelVertex>,
        primitives: &mut Vec<Primitive>,
    ) {
        node.children().for_each(|child| {
            Self::load_node(child, buffers, indices, vertices, primitives);
        });

        if let Some(mesh) = node.mesh() {
            primitives.extend(mesh.primitives().map(|primitive| {
                let first_index = indices.len() as _;
                let vertex_start = vertices.len() as _;
                Self::load_vertices(&primitive, buffers, vertices);
                let index_count =
                    Self::load_indices(&primitive, vertex_start, buffers, indices) as _;
                Primitive {
                    first_index,
                    index_count,
                    material_index: primitive.material().index().unwrap(),
                }
            }));
        }
    }

    fn load_vertices(
        primitive: &gltf::Primitive,
        buffers: &[Data],
        vertices: &mut Vec<ModelVertex>,
    ) {
        let vertex_count = primitive
            .attributes()
            .find(|attr| attr.0 == Semantic::Positions)
            .map(|attr| attr.1.count())
            .unwrap_or(0);

        unsafe {
            let load = |key: Semantic| -> Option<*const f32> {
                primitive
                    .attributes()
                    .find(|attr| attr.0 == key)
                    .map(|attr| {
                        let offset = attr.1.offset();
                        let buffer_view = attr.1.view().unwrap();
                        let buffer = buffer_view.buffer().index();
                        let offset = offset + buffer_view.offset();
                        buffers[buffer].as_ptr().add(offset) as _
                    })
            };

            let position_buffer = load(Semantic::Positions).unwrap();
            let normals_buffer = load(Semantic::Normals);
            let tex_coords_buffer = load(Semantic::TexCoords(0));

            vertices.extend((0..vertex_count).map(|v| {
                let position = [
                    std::ptr::read(position_buffer.add(v * 3)) / 100.,
                    std::ptr::read(position_buffer.add(v * 3 + 1)) / 100.,
                    std::ptr::read(position_buffer.add(v * 3 + 2)) / 100.,
                ];
                let position = [position[0], -position[2], position[1]];
                let norm = if let Some(buf) = normals_buffer {
                    [
                        std::ptr::read(buf.add(v * 3)),
                        std::ptr::read(buf.add(v * 3 + 1)),
                        std::ptr::read(buf.add(v * 3 + 2)),
                    ]
                } else {
                    [0., 0., 0.]
                };
                let norm = [norm[0], -norm[2], norm[1]];
                let uv = if let Some(buf) = tex_coords_buffer {
                    [
                        std::ptr::read(buf.add(v * 2)),
                        std::ptr::read(buf.add(v * 2 + 1)),
                    ]
                } else {
                    [0., 0.]
                };

                ModelVertex { position, norm, uv }
            }));
        }
    }

    fn load_indices(
        primitive: &gltf::Primitive,
        vertex_start: u32,
        buffers: &[Data],
        indices: &mut Vec<u32>,
    ) -> usize {
        let component_type = primitive.indices().unwrap().data_type();
        let buffer_view = primitive.indices().unwrap().view().unwrap();
        let offset = primitive.indices().unwrap().offset();
        let count = primitive.indices().unwrap().count();
        let buffer_idx = buffer_view.buffer().index();
        let offset = offset + buffer_view.offset();

        let data = &buffers[buffer_idx].0;
        unsafe {
            match component_type {
                gltf::accessor::DataType::U8 => {
                    indices.extend(
                        data[offset..]
                            .iter()
                            .take(count)
                            .map(|&b| b as u32 + vertex_start),
                    );
                }
                gltf::accessor::DataType::U16 => {
                    indices.extend(
                        std::slice::from_raw_parts(data[offset..].as_ptr() as *const u16, count)
                            .iter()
                            .map(|&b| b as u32 + vertex_start),
                    );
                }
                gltf::accessor::DataType::U32 => {
                    indices.extend(
                        std::slice::from_raw_parts(data[offset..].as_ptr() as *const u32, count)
                            .iter()
                            .map(|&b| b + vertex_start),
                    );
                }
                _ => unimplemented!(),
            }
        }
        count
    }

    fn load_materials(document: &Document) -> Vec<Material> {
        document
            .materials()
            .map(|m| Material {
                base_color_factor: m.pbr_metallic_roughness().base_color_factor(),
                base_color_texture_idx: m
                    .pbr_metallic_roughness()
                    .base_color_texture()
                    .map(|t| t.texture().index()),
                metallic_factor: m.pbr_metallic_roughness().metallic_factor(),
                roughness_factor: m.pbr_metallic_roughness().roughness_factor(),
            })
            .collect()
    }
}
