/*
 * File: circle.vert
 * Project: shaders
 * Created Date: 26/11/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 * 
 */

#version 450 core

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 tex_coords;
layout(location = 2) in mat4 model;
layout(location = 6) in vec4 color;

layout(location = 0) out vec2 o_tex_coords;
layout(location = 1) out vec4 o_color;

layout(push_constant) uniform PushConsts {
    mat4 view;
    mat4 proj;
} primitive;

void main() {
    o_tex_coords = tex_coords;
    o_color = color;
    gl_Position = primitive.proj * primitive.view * model * position;
}
