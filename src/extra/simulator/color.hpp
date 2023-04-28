// File: color.hpp
// Project: simulator
// Created Date: 05/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#define GLM_FORCE_DEPTH_ZERO_TO_ONE
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <glm/gtx/string_cast.hpp>

namespace autd3::extra::simulator {

class Color {
 public:
  [[nodiscard]] virtual glm::vec4 rgba() const = 0;
  [[nodiscard]] virtual glm::vec4 hsva() const = 0;

  Color() noexcept = default;
  Color(const Color& v) = delete;
  Color& operator=(const Color& obj) = delete;
  Color(Color&& obj) = default;
  Color& operator=(Color&& obj) = default;
  virtual ~Color() = default;
};

class Hsv final : Color {
 public:
  Hsv(const float h, const float s, const float v, const float a) noexcept : h(h), s(s), v(v), a(a) {}
  ~Hsv() override = default;
  Hsv(const Hsv& v) = delete;
  Hsv& operator=(const Hsv& obj) = delete;
  Hsv(Hsv&& obj) = default;
  Hsv& operator=(Hsv&& obj) = default;

  [[nodiscard]] glm::vec4 rgba() const override {
    const auto hue = std::fmod(h, 1.0f);

    if (s == 0.0f) return {v, v, v, a};

    const auto i = std::floor(hue * 6.0f);
    const auto f = hue * 6.0f - i;
    const auto p = v * (1.0f - s);
    const auto q = v * (1.0f - s * f);
    const auto t = v * (1.0f - s * (1.0f - f));

    switch (static_cast<int32_t>(i)) {
      case 0:
        return {v, t, p, a};
      case 1:
        return {q, v, p, a};
      case 2:
        return {p, v, t, a};
      case 3:
        return {p, q, v, a};
      case 4:
        return {t, p, v, a};
      case 5:
        return {v, p, q, a};
      default:
        return {0, 0, 0, 0};
    }
  }

  [[nodiscard]] glm::vec4 hsva() const override { return glm::vec4{h, s, v, a}; }

  float h;
  float s;
  float v;
  float a;
};

}  // namespace autd3::extra::simulator
