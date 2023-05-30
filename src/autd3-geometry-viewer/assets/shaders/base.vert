/*
 * File: base.vert
 * Project: shaders
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 * 
 */

#version 450

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 norm;
layout (location = 2) in vec2 uv;

layout(push_constant) uniform PushConsts {
	mat4 proj_view;
	mat4 model;
	vec4 lightPos;
	vec4 viewPos;
} primitive;

layout (location = 0) out vec3 outNormal;
layout (location = 1) out vec2 outUV;
layout (location = 2) out vec3 outViewVec;
layout (location = 3) out vec3 outLightVec;

void main() 
{
	vec4 pos = primitive.model * vec4(position, 1.0);

	outUV = uv;
	outNormal = mat3(primitive.model) * norm;
	outLightVec = primitive.lightPos.xyz - pos.xyz;
	outViewVec = primitive.viewPos.xyz - pos.xyz;

	gl_Position = primitive.proj_view * pos;	
}
