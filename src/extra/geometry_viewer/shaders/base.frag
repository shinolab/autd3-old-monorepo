#version 450

layout (binding = 1) uniform sampler2D samplerColorMap;

layout (location = 0) in vec3 inNormal;
layout (location = 1) in vec2 inUV;
layout (location = 2) in vec3 inViewVec;
layout (location = 3) in vec3 inLightVec;

layout (constant_id = 0) const bool hasTexture = true;
layout (constant_id = 1) const float baseColorR = 0.0f;
layout (constant_id = 2) const float baseColorG = 0.0f;
layout (constant_id = 3) const float baseColorB = 0.0f;

layout (location = 0) out vec4 outFragColor;

layout(push_constant) uniform PushConstsFragment {
	layout(offset = 64) float ambient;
	layout(offset = 68) float specular;
} pcf;

void main()
{
	vec4 color = hasTexture ? texture(samplerColorMap, inUV) : vec4(baseColorR, baseColorG, baseColorB, 1.0f);

	vec3 N = normalize(inNormal);

	vec3 L = normalize(inLightVec);
	vec3 V = normalize(inViewVec);
	vec3 R = reflect(-L, N);
	vec3 diffuse = max(dot(N, L), pcf.ambient).rrr;
	float specular = pow(max(dot(R, V), 0.0), pcf.specular);
	outFragColor = vec4(diffuse * color.rgb + specular, color.a);
}
