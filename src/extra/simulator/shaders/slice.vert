/*
 * File: slice.vert
 * Project: shaders
 * Created Date: 05/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

#version 450

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 tex_coords;

layout(location = 0) out vec2 o_tex_coords;

layout(set = 0, binding = 0) uniform Data {
    mat4 model;
    mat4 view;
    mat4 proj;
} u_model_view_proj;

void main() {
    gl_Position = u_model_view_proj.proj * u_model_view_proj.view * u_model_view_proj.model * position;
    o_tex_coords = tex_coords;
}
