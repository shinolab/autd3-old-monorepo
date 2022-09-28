#version 450

layout (location = 0) in vec3 inPos;
layout (location = 1) in vec3 inNormal;
layout (location = 2) in vec2 inUV;
layout (location = 3) in vec3 inColor;
layout (location = 4) in vec4 inTangent;

layout (set = 0, binding = 0) uniform UBO 
{
	mat4 view;
	mat4 projection;
	vec4 lightPos;
	vec4 viewPos;
} ubo;

layout(push_constant) uniform PushConstsVertex {
	mat4 model;
} pcv;

layout (location = 0) out vec3 outNormal;
layout (location = 1) out vec3 outColor;
layout (location = 2) out vec2 outUV;
layout (location = 3) out vec3 outViewVec;
layout (location = 4) out vec3 outLightVec;
layout (location = 5) out vec4 outTangent;

void main() 
{
	outColor = inColor;
	outUV = inUV;
	outTangent = inTangent;
	gl_Position = ubo.projection * ubo.view * pcv.model * vec4(inPos.xyz, 1.0);
	
	outNormal = mat3(pcv.model) * inNormal;
	vec4 pos = pcv.model * vec4(inPos, 1.0);
	outLightVec = ubo.lightPos.xyz - pos.xyz;
	outViewVec = ubo.viewPos.xyz - pos.xyz;
}
