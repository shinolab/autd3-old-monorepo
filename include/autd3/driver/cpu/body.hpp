// File: body.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cmath>
#include <cstdint>
#include <cstring>

#include "autd3/driver/fpga/defined.hpp"

namespace autd3::driver {

/**
* \brief Focus data structure for FocusSTM
* \details The focal point data consists of the three-dimensional focus position from the local coordinates of a device and duty shift data that
control amplitude control. The focus position is represented in 18-bit signed fixed-point for each axis, where the unit is
autd3::driver::FOCUS_STM_FIXED_NUM_UNIT. The duty ratio is cycle >> (duty_shift+1): When duty_shift=0, the duty ratio is cycle/2, which means maximum
amplitude.
*/
#pragma pack(push)
#pragma pack(1)
struct STMFocus {
  /**
   * \brief Constructor
   * \details The x-axis data is stored in the lowest 18 bits, followed by the y-axis and z-axis data.
   * The duty shift data is stored next to the z-axis data. The highest 2 bits are not used.
   */
  explicit STMFocus(const autd3_float_t x, const autd3_float_t y, const autd3_float_t z, const uint8_t duty_shift) noexcept {
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
#pragma pack(pop)

/**
* \brief Initial Body data for FocusSTM
* \details The number of STMFocus data is stored in the first 16 bits, the frequency division data in the next 32 bits, and the sound speed data in
the next 32 bits. The STMFocus data is stored after them.
*/
#pragma pack(push)
#pragma pack(1)
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

  void set_point(const STMFocus* const points, const size_t n) noexcept {
    std::memcpy(reinterpret_cast<uint8_t*>(this) + sizeof(FocusSTMBodyInitial), points, sizeof(STMFocus) * n);
  }

  uint16_t size;
  uint32_t freq_div;
  uint32_t sound_speed;
  uint16_t stm_start_idx;
  uint16_t stm_finish_idx;
};
#pragma pack(pop)

/**
 * \brief Subsequent Body data for FocusSTM
 * \details The number of STMFocus data is stored in the first 16 bits, followed by the STMFocus data.
 */
#pragma pack(push)
#pragma pack(1)
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

  void set_point(const STMFocus* const points, const size_t n) noexcept {
    std::memcpy(reinterpret_cast<uint8_t*>(this) + sizeof(FocusSTMBodySubsequent), points, sizeof(STMFocus) * n);
  }

  uint16_t size;
};
#pragma pack(pop)

/**
 * @brief Data transmission mode for GainSTM
 */
enum class GainSTMMode : uint16_t {
  /**
   * @brief Both phase and amplitude data are transmitted
   */
  PhaseDutyFull = 0x0001,
  /**
   * @brief Only phase data is transmitted (x2 faster than PhaseDutyFull)
   */
  PhaseFull = 0x0002,
  /**
   * @brief Only phase data is transmitted and phase data is compressed by half (x4 faster than PhaseDutyFull)
   */
  PhaseHalf = 0x0004,
};

/**
 * @brief Transmission data when using GainSTMMode::PhaseFull in Legacy mode (for the low 8-bit part)
 */
#pragma pack(push)
#pragma pack(1)
struct LegacyPhaseFull0 {
  uint8_t phase_0;
  uint8_t phase_1;

  void set(const Drive d) {
    const auto phase = LegacyDrive::to_phase(d);
    phase_0 = phase;
  }

  LegacyPhaseFull0& operator=(const Drive& d) {
    set(d);
    return *this;
  }
  LegacyPhaseFull0& operator=(Drive&& d) {
    set(d);
    return *this;
  }
};
#pragma pack(pop)

/**
 * @brief Transmission data when using GainSTMMode::PhaseFull in Legacy mode (for the high 8-bit part)
 */
#pragma pack(push)
#pragma pack(1)
struct LegacyPhaseFull1 {
  uint8_t phase_0;
  uint8_t phase_1;

  void set(const Drive d) {
    const auto phase = LegacyDrive::to_phase(d);
    phase_1 = phase;
  }

  LegacyPhaseFull1& operator=(const Drive& d) {
    set(d);
    return *this;
  }
  LegacyPhaseFull1& operator=(Drive&& d) {
    set(d);
    return *this;
  }
};
#pragma pack(pop)

/**
 * @brief Transmission data when using GainSTMMode::PhaseHalf in Legacy mode (for the [3:0] bits)
 */
#pragma pack(push)
#pragma pack(1)
struct LegacyPhaseHalf0 {
  uint8_t phase_01;
  uint8_t phase_23;

  void set(const Drive d) {
    const auto phase = LegacyDrive::to_phase(d);
    phase_01 &= 0xF0;
    phase_01 |= phase >> 4 & 0x0F;
  }

  LegacyPhaseHalf0& operator=(const Drive& d) {
    set(d);
    return *this;
  }
  LegacyPhaseHalf0& operator=(Drive&& d) {
    set(d);
    return *this;
  }
};
#pragma pack(pop)

/**
 * @brief Transmission data when using GainSTMMode::PhaseHalf in Legacy mode (for the [7:4] bits)
 */
#pragma pack(push)
#pragma pack(1)
struct LegacyPhaseHalf1 {
  uint8_t phase_01;
  uint8_t phase_23;

  void set(const Drive d) {
    const auto phase = LegacyDrive::to_phase(d);
    phase_01 &= 0x0F;
    phase_01 |= phase & 0xF0;
  }

  LegacyPhaseHalf1& operator=(const Drive& d) {
    set(d);
    return *this;
  }
  LegacyPhaseHalf1& operator=(Drive&& d) {
    set(d);
    return *this;
  }
};
#pragma pack(pop)

/**
 * @brief Transmission data when using GainSTMMode::PhaseHalf in Legacy mode (for the [11:8] bits)
 */
#pragma pack(push)
#pragma pack(1)
struct LegacyPhaseHalf2 {
  uint8_t phase_01;
  uint8_t phase_23;

  void set(const Drive d) {
    const auto phase = LegacyDrive::to_phase(d);
    phase_23 &= 0xF0;
    phase_23 |= phase >> 4 & 0x0F;
  }

  LegacyPhaseHalf2& operator=(const Drive& d) {
    set(d);
    return *this;
  }
  LegacyPhaseHalf2& operator=(Drive&& d) {
    set(d);
    return *this;
  }
};
#pragma pack(pop)

/**
 * @brief Transmission data when using GainSTMMode::PhaseHalf in Legacy mode (for the [15:12] bits)
 */
#pragma pack(push)
#pragma pack(1)
struct LegacyPhaseHalf3 {
  uint8_t phase_01;
  uint8_t phase_23;

  void set(const Drive d) {
    const auto phase = LegacyDrive::to_phase(d);
    phase_23 &= 0x0F;
    phase_23 |= phase & 0xF0;
  }

  LegacyPhaseHalf3& operator=(const Drive& d) {
    set(d);
    return *this;
  }
  LegacyPhaseHalf3& operator=(Drive&& d) {
    set(d);
    return *this;
  }
};
#pragma pack(pop)

/**
 * \brief Initial Body data for GainSTM
 * \details The frequency division data in the first 32 bits, and the GainSTMMode data in the next 16 bits, and the total pattern size in the next 16
 * bits. Amplitude and phase data are not stored.
 */
#pragma pack(push)
#pragma pack(1)
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

  uint32_t freq_div;
  GainSTMMode mode;
  uint16_t cycle;
  uint16_t stm_start_idx;
  uint16_t stm_finish_idx;
};
#pragma pack(pop)

/**
 * \brief Subsequent Body data for GainSTM
 * \details Amplitude/phase data is stored.
 */
#pragma pack(push)
#pragma pack(1)
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
#pragma pack(pop)

/**
 * \brief Body data for each device
 */
struct Body {
  Body() noexcept = delete;
  ~Body() = delete;
  Body(const Body& v) = delete;
  Body& operator=(const Body& obj) = default;
  Body(Body&& obj) = delete;
  Body& operator=(Body&& obj) = delete;

  uint16_t* data() { return reinterpret_cast<uint16_t*>(this); }

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
