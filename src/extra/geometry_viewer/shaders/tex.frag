#version 450

layout (set = 0, binding = 1) uniform sampler2D samplerColorMap;

layout (location = 0) in vec2 inUV;
layout (location = 1) in vec3 inNormal;
layout (location = 2) in vec3 inViewVec;
layout (location = 3) in vec3 inLightVec;

layout (location = 0) out vec4 outFragColor;

layout(push_constant) uniform PushConstsFragment {
	layout(offset = 64) float ambient;
	layout(offset = 68) float specular;
} pcf;

void main()
{
	vec4 color = texture(samplerColorMap, inUV);

	vec3 n = normalize(inNormal);
	vec3 l = normalize(inLightVec);
	vec3 v = normalize(inViewVec);
	vec3 r = reflect(-l, n);
	vec3 diffuse = max(dot(n, l), pcf.ambient).rrr;
	float specular = pow(max(dot(r, v), 0.0), pcf.specular);
	outFragColor = vec4(diffuse * color.rgb + specular, color.a);
}
