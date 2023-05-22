// File: transform.hpp
// Project: helper
// Created Date: 02/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::extra::helper {
inline glm::vec3 to_gl_pos(glm::vec3 v) {
#ifdef AUTD3_USE_LEFT_HANDED
  return glm::vec3(v.x, v.y, -v.z);
#else
  return glm::vec3(v.x, v.y, v.z);
#endif
}

inline glm::quat to_gl_rot(glm::quat rot) {
#ifdef AUTD3_USE_LEFT_HANDED
  return glm::quat(rot.w, -rot.x, -rot.y, rot.z);
#else
  return rot;
#endif
}

inline glm::mat4 orthogonal(glm::vec3 pos, glm::quat rot) {
  const auto model = mat4_cast(rot);
  const auto r = make_vec3(model[0]);
  const auto u = make_vec3(model[1]);
  const auto f = make_vec3(model[2]);
  return glm::mat4({r[0], u[0], f[0], 0.0f, r[1], u[1], f[1], 0.0f, r[2], u[2], f[2], 0.f, -dot(r, pos), -dot(u, pos), -dot(f, pos), 1.0f});
}
inline glm::quat quaternion_to(glm::vec3 v, glm::vec3 to) {
  const auto a = normalize(v);
  const auto b = normalize(to);
  const auto c = normalize(cross(b, a));
  if (std::isnan(c.x) || std::isnan(c.y) || std::isnan(c.z)) return {1, 0, 0, 0};
  const auto ip = dot(a, b);
  if (constexpr float eps = 1e-4f; length(c) < eps || 1.0f < ip) {
    if (ip < eps - 1.0f) {
      const auto a2 = glm::vec3(-a.y, a.z, a.x);
      const auto c2 = normalize(cross(a2, a));
      return {0.0, c2};
    }
    return {1, 0, 0, 0};
  }
  const auto e = c * std::sqrt(0.5f * (1.0f - ip));
  return {std::sqrt(0.5f * (1.0f + ip)), e};
}

}  // namespace autd3::extra::helper
