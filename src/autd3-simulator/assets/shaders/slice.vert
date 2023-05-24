/*
 * File: slice.vert
 * Project: shaders
 * Created Date: 05/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

#version 450

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 tex_coords;

layout(location = 0) out vec2 o_tex_coords;

layout(push_constant) uniform PushConstsConfig {
    mat4 model;
    mat4 view;
    mat4 proj;
    uint _width;
    uint _height;
    uint _dummy0;
    uint _dummy1;
} pc;

void main() {
    gl_Position = pc.proj * pc.view * pc.model * position;
    o_tex_coords = tex_coords;
}
 