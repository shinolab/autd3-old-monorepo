/*
 * File: circle.frag
 * Project: shaders
 * Created Date: 03/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 * 
 */

#version 450 core

layout(location = 0) in vec2 v_tex_coords;
layout(location = 1) in vec4 i_color;

layout(location = 0) out vec4 o_color;

layout(set = 0, binding = 0) uniform sampler2D t_color;

void main() {
    o_color = i_color * texture(t_color, v_tex_coords);
}
