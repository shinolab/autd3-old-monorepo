/*
 * File: base.frag
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

layout(set = 0, binding = 0) uniform sampler2D samplerColorMap;

layout(location = 0) in vec3 inNormal;
layout(location = 1) in vec2 inUV;
layout(location = 2) in vec3 inViewVec;
layout(location = 3) in vec3 inLightVec;

layout(location = 0) out vec4 outFragColor;

layout(push_constant) uniform PushConsts {
  mat4 proj_view;
  mat4 model;
  vec4 lightPos;
  vec4 viewPos;
  float ambient;
  float specular;
  float lightPower;
  float roughness;
  float metallic;
  float baseColorR;
  float baseColorG;
  float baseColorB;
  bool hasTexture;
}
pcf;

const float PI = 3.1415926535897932384626433832795;

float diffuse(float ndotv, float ndotl, float ldoth, float roughness) {
  float fd90 = 0.5 + 2 * ldoth * ldoth * roughness;
  float lightScatter = (1 + (fd90 - 1) * pow(1 - ndotl, 5));
  float viewScatter = (1 + (fd90 - 1) * pow(1 - ndotv, 5));
  return lightScatter * viewScatter / PI;
}

float smith(float ndotl, float ndotv, float alpha) {
  float lambdaV = ndotl * (ndotv * (1 - alpha) + alpha);
  float lambdaL = ndotv * (ndotl * (1 - alpha) + alpha);
  return 0.5f / (lambdaV + lambdaL + 0.0001);
}

float ggx(float perceptualRoughness, float ndoth) {
  float a = ndoth * perceptualRoughness;
  float k = perceptualRoughness / (1.0 - ndoth * ndoth + a * a);
  return k * k / PI;
}

vec3 fresnel(vec3 f0, float cos) { return f0 + (1 - f0) * pow(1 - cos, 5); }

void main() {
  vec4 color = pcf.hasTexture
                   ? texture(samplerColorMap, inUV)
                   : vec4(pcf.baseColorR, pcf.baseColorG, pcf.baseColorB, 1.0f);

  vec3 lightColor = vec3(pcf.lightPower, pcf.lightPower, pcf.lightPower);

  vec3 N = normalize(inNormal);
  vec3 L = normalize(inLightVec);
  vec3 V = normalize(inViewVec);
  vec3 R = reflect(-L, N);
  vec3 indirectDiffuse = max(dot(N, L), pcf.ambient).rrr;
  indirectDiffuse =
      indirectDiffuse * color.rgb + pow(max(dot(R, V), 0.0), pcf.specular),
  color.a;

  vec3 halfDir = normalize(inLightVec + inViewVec);
  float ndotv = abs(dot(inNormal, inViewVec));
  float ndotl = max(0, dot(inNormal, inLightVec));
  float ndoth = max(0, dot(inNormal, halfDir));
  float ldoth = max(0, dot(inLightVec, halfDir));
  float reflectivity = mix(0.04, 1.0, pcf.metallic);
  vec3 f0 = mix(vec3(0.04, 0.04, 0.04), color.xyz, pcf.metallic);

  vec3 diffusev = color.xyz * (1 - reflectivity) * lightColor *
                      diffuse(ndotv, ndotl, ldoth, pcf.roughness) * ndotl +
                  color.xyz * (1 - reflectivity) * indirectDiffuse;

  float alpha = pcf.roughness * pcf.roughness;
  float D = ggx(pcf.roughness, ndotv);
  vec3 F = fresnel(f0, ldoth);
  vec3 specular = smith(ndotl, ndotv, alpha) * D * F * ndotl * lightColor;
  specular = max(vec3(0, 0, 0), specular);

  outFragColor = vec4(diffusev + specular, color.a);
}
