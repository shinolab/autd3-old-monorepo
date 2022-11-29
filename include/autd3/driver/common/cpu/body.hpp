// File: body.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cmath>
#include <cstdint>
#include <cstring>
#include <vector>

#include "autd3/driver/common/fpga/defined.hpp"

namespace autd3::driver {

/**
 * \brief Focus data structure for FocusSTM
 * \details The focal point data consists of the three-dimensional focus position from the local coordinates of a device and duty shift data that
control amplitude control. The focus position is represented in 18-bit signed fixed-point for each axis, where the unit is
autd3::driver::FOCUS_STM_FIXED_NUM_UNIT. The duty ratio is cycle >> (duty_shift+1): When duty_shift=0, the duty ratio is cycle/2, which means maximum
amplitude.
 */
struct STMFocus {
  /**
   * \brief Constructor
   * \details The x-axis data is stored in the lowest 18 bits, followed by the y-axis and z-axis data.
   * The duty shift data is stored next to the z-axis data. The highest 2 bits are not used.
   */
  explicit STMFocus(const double x, const double y, const double z, const uint8_t duty_shift) noexcept {
    const auto ix = static_cast<int32_t>(std::round(x / FOCUS_STM_FIXED_NUM_UNIT));
    const auto iy = static_cast<int32_t>(std::round(y / FOCUS_STM_FIXED_NUM_UNIT));
    const auto iz = static_cast<int32_t>(std::round(z / FOCUS_STM_FIXED_NUM_UNIT));
    _data[0] = static_cast<uint16_t>(ix & 0xFFFF);
    _data[1] = static_cast<uint16_t>(iy << 2 & 0xFFFC) | static_cast<uint16_t>(ix >> 30 & 0x0002) | static_cast<uint16_t>(ix >> 16 & 0x0001);
    _data[2] = static_cast<uint16_t>(iz << 4 & 0xFFF0) | static_cast<uint16_t>(iy >> 28 & 0x0008) | static_cast<uint16_t>(iy >> 14 & 0x0007);
    _data[3] = static_cast<uint16_t>(duty_shift << 6 & 0x3FC0) | static_cast<uint16_t>(iz >> 26 & 0x0020) | static_cast<uint16_t>(iz >> 12 & 0x001F);
  }

 private:
  uint16_t _data[4]{};
};

/**
 * \brief Initial Body data for FocusSTM
 * \details The number of STMFocus data is stored in the first 16 bits, the frequency division data in the next 32 bits, and the sound speed data in
 the next 32 bits. The STMFocus data is stored after them.
 */
struct FocusSTMBodyInitial {
  /**
   * \brief This data is cast from the Body data. Never construct directly.
   */
  FocusSTMBodyInitial() = delete;
  ~FocusSTMBodyInitial() = delete;
  FocusSTMBodyInitial(const FocusSTMBodyInitial& v) = delete;
  FocusSTMBodyInitial& operator=(const FocusSTMBodyInitial& obj) = delete;
  FocusSTMBodyInitial(FocusSTMBodyInitial&& obj) = delete;
  FocusSTMBodyInitial& operator=(FocusSTMBodyInitial&& obj) = delete;

  [[nodiscard]] const uint16_t* data() const noexcept { return _data; }

  void set_size(const uint16_t size) noexcept { _data[0] = size; }

  void set_freq_div(const uint32_t freq_div) noexcept {
    _data[1] = static_cast<uint16_t>(freq_div & 0xFFFF);
    _data[2] = static_cast<uint16_t>(freq_div >> 16 & 0xFFFF);
  }

  void set_sound_speed(const uint32_t sound_speed) noexcept {
    _data[3] = static_cast<uint16_t>(sound_speed & 0xFFFF);
    _data[4] = static_cast<uint16_t>(sound_speed >> 16 & 0xFFFF);
  }

  void set_point(const std::vector<STMFocus>& points) noexcept { std::memcpy(&_data[5], points.data(), sizeof(STMFocus) * points.size()); }

 private:
  uint16_t _data[5]{};  // Data size has no meaning.
};

/**
 * \brief Subsequent Body data for FocusSTM
 * \details The number of STMFocus data is stored in the first 16 bits, followed by the STMFocus data.
 */
struct FocusSTMBodySubsequent {
  /**
   * \brief This data is cast from the Body data. Never construct directly.
   */
  FocusSTMBodySubsequent() = delete;
  ~FocusSTMBodySubsequent() = delete;
  FocusSTMBodySubsequent(const FocusSTMBodySubsequent& v) = delete;
  FocusSTMBodySubsequent& operator=(const FocusSTMBodySubsequent& obj) = delete;
  FocusSTMBodySubsequent(FocusSTMBodySubsequent&& obj) = delete;
  FocusSTMBodySubsequent& operator=(FocusSTMBodySubsequent&& obj) = delete;

  [[nodiscard]] const uint16_t* data() const noexcept { return _data; }

  void set_size(const uint16_t size) noexcept { _data[0] = size; }

  void set_point(const std::vector<STMFocus>& points) noexcept { std::memcpy(&_data[1], points.data(), sizeof(STMFocus) * points.size()); }

 private:
  uint16_t _data[2]{};  // Data size has no meaning.
};

enum class GainSTMMode : uint16_t {
  PhaseDutyFull = 0x0001,
  PhaseFull = 0x0002,
  PhaseHalf = 0x0004,
};

struct LegacyPhaseFull {
  uint8_t phase_0;
  uint8_t phase_1;
  void set(const size_t idx, const Drive d) {
    const auto phase = LegacyDrive::to_phase(d);
    switch (idx) {
      case 0:
        phase_0 = phase;
        break;
      case 1:
        phase_1 = phase;
        break;
      default:
        throw std::runtime_error("Unreachable!");
    }
  }
};

struct LegacyPhaseHalf {
  uint8_t phase_01;
  uint8_t phase_23;

  void set(const size_t idx, const Drive d) {
    const auto phase = LegacyDrive::to_phase(d);
    switch (idx) {
      case 0:
        phase_01 = (phase_01 & 0xF0) | ((phase >> 4) & 0x0F);
        break;
      case 1:
        phase_01 = (phase_01 & 0x0F) | (phase & 0xF0);
        break;
      case 2:
        phase_23 = (phase_23 & 0xF0) | ((phase >> 4) & 0x0F);
        break;
      case 3:
        phase_23 = (phase_23 & 0x0F) | (phase & 0xF0);
        break;
      default:
        throw std::runtime_error("Unreachable!");
    }
  }
};

/**
 * \brief Initial Body data for GainSTM
 * \details The frequency division data in the first 32 bits, and the GainSTMMode data in the next 16 bits, and the total pattern size in the next 16
 * bits. Amplitude and phase data are not stored.
 */
struct GainSTMBodyInitial {
  /**
   * \brief This data is cast from the Body data. Never construct directly.
   */
  GainSTMBodyInitial() = delete;
  ~GainSTMBodyInitial() = delete;
  GainSTMBodyInitial(const GainSTMBodyInitial& v) = delete;
  GainSTMBodyInitial& operator=(const GainSTMBodyInitial& obj) = delete;
  GainSTMBodyInitial(GainSTMBodyInitial&& obj) = delete;
  GainSTMBodyInitial& operator=(GainSTMBodyInitial&& obj) = delete;

  [[nodiscard]] const uint16_t* data() const noexcept { return _data; }

  void set_freq_div(const uint32_t freq_div) noexcept {
    _data[0] = static_cast<uint16_t>(freq_div & 0xFFFF);
    _data[1] = static_cast<uint16_t>(freq_div >> 16 & 0xFFFF);
  }

  void set_mode(const GainSTMMode mode) noexcept { _data[2] = static_cast<uint16_t>(mode); }

  void set_cycle(const size_t size) noexcept { _data[3] = static_cast<uint16_t>(size); }

 private:
  uint16_t _data[4]{};
};

/**
 * \brief Subsequent Body data for GainSTM
 * \details Amplitude/phase data is stored.
 */
struct GainSTMBodySubsequent {
  /**
   * \brief This data is cast from the Body data. Never construct directly.
   */
  GainSTMBodySubsequent() = delete;
  ~GainSTMBodySubsequent() = delete;
  GainSTMBodySubsequent(const GainSTMBodySubsequent& v) = delete;
  GainSTMBodySubsequent& operator=(const GainSTMBodySubsequent& obj) = delete;
  GainSTMBodySubsequent(GainSTMBodySubsequent&& obj) = delete;
  GainSTMBodySubsequent& operator=(GainSTMBodySubsequent&& obj) = delete;

  [[nodiscard]] const uint16_t* data() const noexcept { return reinterpret_cast<const uint16_t*>(this); }
};

/**
 * \brief Body data for each device
 */
struct Body {
  Body() noexcept = delete;
  ~Body() = delete;
  Body(const Body& v) = delete;
  Body& operator=(const Body& obj) = delete;
  Body(Body&& obj) = delete;
  Body& operator=(Body&& obj) = delete;

  [[nodiscard]] const FocusSTMBodyInitial& focus_stm_initial() const noexcept { return *reinterpret_cast<const FocusSTMBodyInitial* const>(this); }
  FocusSTMBodyInitial& focus_stm_initial() noexcept { return *reinterpret_cast<FocusSTMBodyInitial*>(this); }
  [[nodiscard]] const FocusSTMBodySubsequent& focus_stm_subsequent() const noexcept {
    return *reinterpret_cast<const FocusSTMBodySubsequent* const>(this);
  }
  FocusSTMBodySubsequent& focus_stm_subsequent() noexcept { return *reinterpret_cast<FocusSTMBodySubsequent*>(this); }

  [[nodiscard]] const GainSTMBodyInitial& gain_stm_initial() const noexcept { return *reinterpret_cast<const GainSTMBodyInitial* const>(this); }
  GainSTMBodyInitial& gain_stm_initial() noexcept { return *reinterpret_cast<GainSTMBodyInitial*>(this); }
  [[nodiscard]] const GainSTMBodySubsequent& gain_stm_subsequent() const noexcept {
    return *reinterpret_cast<const GainSTMBodySubsequent* const>(this);
  }
  GainSTMBodySubsequent& gain_stm_subsequent() noexcept { return *reinterpret_cast<GainSTMBodySubsequent*>(this); }
};

}  // namespace autd3::driver
