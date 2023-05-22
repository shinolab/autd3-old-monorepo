/*
 * File: slice.frag
 * Project: shaders
 * Created Date: 05/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

#version 450

layout(location = 0) in vec2 v_tex_coords;

layout(location = 0) out vec4 f_color;

layout(push_constant) uniform PushConstsConfig {
    layout(offset = 192) uint width;
    layout(offset = 196) uint height;
    layout(offset = 200) uint dummy0;
    layout(offset = 204) uint dummy1;
} config;

layout(set = 0, binding = 0) buffer Data {
    vec4 data[];
} data;

void main() {
  uint w = uint(floor(v_tex_coords.x * config.width));
  uint h = uint(floor(v_tex_coords.y * config.height));
  uint idx = w + config.width * h;
  f_color = data.data[idx];
}
