// File: body.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <cmath>
#include <cstdint>
#include <cstring>
#include <vector>

#include "autd3/driver/fpga/defined.hpp"
#include "autd3/driver/hardware.hpp"

namespace autd3::driver {

struct STMFocus {
  STMFocus(const double x, const double y, const double z, const uint8_t duty_shift) noexcept {
    const auto ix = static_cast<int32_t>(std::round(x / POINT_STM_FIXED_NUM_UNIT));
    const auto iy = static_cast<int32_t>(std::round(y / POINT_STM_FIXED_NUM_UNIT));
    const auto iz = static_cast<int32_t>(std::round(z / POINT_STM_FIXED_NUM_UNIT));
    _data[0] = static_cast<uint16_t>(ix & 0xFFFF);
    _data[1] = static_cast<uint16_t>(iy << 2 & 0xFFFC) | static_cast<uint16_t>(ix >> 30 & 0x0002) | static_cast<uint16_t>(ix >> 16 & 0x0001);
    _data[2] = static_cast<uint16_t>(iz << 4 & 0xFFF0) | static_cast<uint16_t>(iy >> 28 & 0x0008) | static_cast<uint16_t>(iy >> 14 & 0x0007);
    _data[3] = static_cast<uint16_t>(duty_shift << 6 & 0x3FC0) | static_cast<uint16_t>(iz >> 26 & 0x0020) | static_cast<uint16_t>(iz >> 12 & 0x001F);
  }

 private:
  uint16_t _data[4]{};
};

struct PointSTMBodyHead {
  [[nodiscard]] const uint16_t* data() const noexcept { return _data; }

  void set_size(const uint16_t size) noexcept { _data[0] = size; }

  void set_freq_div(uint32_t freq_div) noexcept {
    _data[1] = static_cast<uint16_t>(freq_div & 0xFFFF);
    _data[2] = static_cast<uint16_t>(freq_div >> 16 & 0xFFFF);
  }

  void set_sound_speed(uint32_t sound_speed) noexcept {
    _data[3] = static_cast<uint16_t>(sound_speed & 0xFFFF);
    _data[4] = static_cast<uint16_t>(sound_speed >> 16 & 0xFFFF);
  }

  void set_point(const std::vector<STMFocus>& points) noexcept { std::memcpy(&_data[5], points.data(), sizeof(STMFocus) * points.size()); }

 private:
  uint16_t _data[NUM_TRANS_IN_UNIT]{};
};

struct PointSTMBodyBody {
  [[nodiscard]] const uint16_t* data() const noexcept { return _data; }

  void set_size(const uint16_t size) noexcept { _data[0] = size; }

  void set_point(const std::vector<STMFocus>& points) noexcept { std::memcpy(&_data[1], points.data(), sizeof(STMFocus) * points.size()); }

 private:
  uint16_t _data[NUM_TRANS_IN_UNIT]{};
};

struct GainSTMBodyHead {
  uint16_t* data() noexcept { return _data; }

  void set_freq_div(uint32_t freq_div) noexcept {
    _data[0] = static_cast<uint16_t>(freq_div & 0xFFFF);
    _data[1] = static_cast<uint16_t>(freq_div >> 16 & 0xFFFF);
  }

 private:
  uint16_t _data[NUM_TRANS_IN_UNIT]{};
};

struct GainSTMBodyBody {
  [[nodiscard]] const uint16_t* data() const noexcept { return _data; }

 private:
  uint16_t _data[NUM_TRANS_IN_UNIT]{};
};

struct Body {
  uint16_t data[NUM_TRANS_IN_UNIT]{};

  Body() noexcept = default;

  [[nodiscard]] const PointSTMBodyHead& point_stm_head() const noexcept { return *reinterpret_cast<const PointSTMBodyHead* const>(&data[0]); }
  PointSTMBodyHead& point_stm_head() noexcept { return *reinterpret_cast<PointSTMBodyHead*>(&data[0]); }
  [[nodiscard]] const PointSTMBodyBody& point_stm_body() const noexcept { return *reinterpret_cast<const PointSTMBodyBody* const>(&data[0]); }
  PointSTMBodyBody& point_stm_body() noexcept { return *reinterpret_cast<PointSTMBodyBody*>(&data[0]); }

  [[nodiscard]] const GainSTMBodyHead& gain_stm_head() const noexcept { return *reinterpret_cast<const GainSTMBodyHead* const>(&data[0]); }
  GainSTMBodyHead& gain_stm_head() noexcept { return *reinterpret_cast<GainSTMBodyHead*>(&data[0]); }
  [[nodiscard]] const GainSTMBodyBody& gain_stm_body() const noexcept { return *reinterpret_cast<const GainSTMBodyBody* const>(&data[0]); }
  GainSTMBodyBody& gain_stm_body() noexcept { return *reinterpret_cast<GainSTMBodyBody*>(&data[0]); }
};

}  // namespace autd3::driver
