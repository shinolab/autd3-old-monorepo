// File: body.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <cmath>
#include <cstdint>
#include <gsl/gsl>

#include "autd3/driver/fpga/defined.hpp"
#include "autd3/driver/hardware.hpp"

namespace autd3::driver {

struct STMFocus {
  STMFocus(const double x, const double y, const double z, const uint8_t duty_shift) noexcept {
    const auto ix = static_cast<int32_t>(std::round(x / POINT_STM_FIXED_NUM_UNIT));
    const auto iy = static_cast<int32_t>(std::round(y / POINT_STM_FIXED_NUM_UNIT));
    const auto iz = static_cast<int32_t>(std::round(z / POINT_STM_FIXED_NUM_UNIT));
    _data[0] = gsl::narrow_cast<uint16_t>(ix & 0xFFFF);
    _data[1] =
        gsl::narrow_cast<uint16_t>(iy << 2 & 0xFFFC) | gsl::narrow_cast<uint16_t>(ix >> 30 & 0x0002) | gsl::narrow_cast<uint16_t>(ix >> 16 & 0x0001);
    _data[2] =
        gsl::narrow_cast<uint16_t>(iz << 4 & 0xFFF0) | gsl::narrow_cast<uint16_t>(iy >> 28 & 0x0008) | gsl::narrow_cast<uint16_t>(iy >> 14 & 0x0007);
    _data[3] = gsl::narrow_cast<uint16_t>(duty_shift << 6 & 0x3FC0) | gsl::narrow_cast<uint16_t>(iz >> 26 & 0x0020) |
               gsl::narrow_cast<uint16_t>(iz >> 12 & 0x001F);
  }

 private:
  uint16_t _data[4]{};
};

struct PointSTMBodyHead {
  gsl::span<uint16_t> data() noexcept { return gsl::span{_data}; }

  void set_size(const uint16_t size) noexcept { _data[0] = size; }

  void set_freq_div(uint32_t freq_div) noexcept {
    _data[1] = gsl::narrow_cast<uint16_t>(freq_div & 0xFFFF);
    _data[2] = gsl::narrow_cast<uint16_t>(freq_div >> 16 & 0xFFFF);
  }

  void set_sound_speed(uint32_t sound_speed) noexcept {
    _data[3] = gsl::narrow_cast<uint16_t>(sound_speed & 0xFFFF);
    _data[4] = gsl::narrow_cast<uint16_t>(sound_speed >> 16 & 0xFFFF);
  }

  void set_point(const gsl::span<const STMFocus> points) noexcept { std::memcpy(&_data[5], points.data(), points.size_bytes()); }

 private:
  uint16_t _data[NUM_TRANS_IN_UNIT]{};
};

struct PointSTMBodyBody {
  gsl::span<uint16_t> data() noexcept { return gsl::span{_data}; }

  void set_size(const uint16_t size) noexcept { _data[0] = size; }

  void set_point(const gsl::span<const STMFocus> points) noexcept { std::memcpy(&_data[1], points.data(), points.size_bytes()); }

 private:
  uint16_t _data[NUM_TRANS_IN_UNIT]{};
};

struct GainSTMBodyHead {
  gsl::span<uint16_t> data() noexcept { return gsl::span{_data}; }

  void set_freq_div(uint32_t freq_div) noexcept {
    _data[0] = gsl::narrow_cast<uint16_t>(freq_div & 0xFFFF);
    _data[1] = gsl::narrow_cast<uint16_t>(freq_div >> 16 & 0xFFFF);
  }

 private:
  uint16_t _data[NUM_TRANS_IN_UNIT]{};
};

struct GainSTMBodyBody {
  gsl::span<uint16_t> data() noexcept { return gsl::span{_data}; }

 private:
  uint16_t _data[NUM_TRANS_IN_UNIT]{};
};

struct Body {
  uint16_t data[NUM_TRANS_IN_UNIT]{};

  Body() noexcept = default;

  [[nodiscard]] const PointSTMBodyHead& point_stm_head() const noexcept { return *std::bit_cast<const PointSTMBodyHead* const>(&data[0]); }
  PointSTMBodyHead& point_stm_head() noexcept { return *std::bit_cast<PointSTMBodyHead*>(&data[0]); }
  [[nodiscard]] const PointSTMBodyBody& point_stm_body() const noexcept { return *std::bit_cast<const PointSTMBodyBody* const>(&data[0]); }
  PointSTMBodyBody& point_stm_body() noexcept { return *std::bit_cast<PointSTMBodyBody*>(&data[0]); }

  [[nodiscard]] const GainSTMBodyHead& gain_stm_head() const noexcept { return *std::bit_cast<const GainSTMBodyHead* const>(&data[0]); }
  GainSTMBodyHead& gain_stm_head() noexcept { return *std::bit_cast<GainSTMBodyHead*>(&data[0]); }
  [[nodiscard]] const GainSTMBodyBody& gain_stm_body() const noexcept { return *std::bit_cast<const GainSTMBodyBody* const>(&data[0]); }
  GainSTMBodyBody& gain_stm_body() noexcept { return *std::bit_cast<GainSTMBodyBody*>(&data[0]); }
};

}  // namespace autd3::driver
