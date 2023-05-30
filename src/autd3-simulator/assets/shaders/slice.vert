/*
 * File: slice.vert
 * Project: shaders
 * Created Date: 05/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/05/2023
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
    mat4 pvm;
} pc;

void main() {
    gl_Position = pc.pvm * position;
    o_tex_coords = tex_coords;
}
 