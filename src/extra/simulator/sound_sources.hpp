// File: sound_sources.hpp
// Project: simulator
// Created Date: 02/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 23/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "glm.hpp"

namespace autd3::extra::simulator {
struct Drive {
  float amp;
  float phase;
  float enable;
  float wave_num;

  explicit Drive(const float amp, const float phase, const float enable, const float frequency, const float sound_speed)
      : amp(amp), phase(phase), enable(enable), wave_num(to_wave_num(frequency, sound_speed)) {}

  void set_wave_num(const float frequency, const float sound_speed) { wave_num = to_wave_num(frequency, sound_speed); }

  static float to_wave_num(const float frequency, const float sound_speed) { return 2.0f * glm::pi<float>() * frequency / sound_speed; }
};

class SoundSources {
 public:
  SoundSources() noexcept = default;
  ~SoundSources() = default;
  SoundSources(const SoundSources& v) = delete;
  SoundSources& operator=(const SoundSources& obj) = delete;
  SoundSources(SoundSources&& obj) = default;
  SoundSources& operator=(SoundSources&& obj) = default;

  void add(const glm::vec3 pos, glm::quat rot, Drive drive, float visibility) {
    _pos.emplace_back(pos, 0.0f);
    _rot.emplace_back(rot);
    _drive.emplace_back(drive);
    _visibilities.emplace_back(visibility);
  }

  void clear() {
    _pos.clear();
    _rot.clear();
    _drive.clear();
    _visibilities.clear();
  }

  [[nodiscard]] size_t size() const { return _pos.size(); }

  [[nodiscard]] bool is_empty() const { return size() == 0; }

  [[nodiscard]] const std::vector<glm::vec4>& positions() const { return _pos; }
  [[nodiscard]] const std::vector<glm::quat>& rotations() const { return _rot; }
  [[nodiscard]] const std::vector<Drive>& drives() const { return _drive; }
  std::vector<Drive>& drives() { return _drive; }
  [[nodiscard]] const std::vector<float>& visibilities() const { return _visibilities; }
  std::vector<float>& visibilities() { return _visibilities; }

 private:
  std::vector<glm::vec4> _pos;
  std::vector<glm::quat> _rot;
  std::vector<Drive> _drive;
  std::vector<float> _visibilities;
};

}  // namespace autd3::extra::simulator
