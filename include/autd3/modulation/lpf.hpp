// File: lpf.hpp
// Project: modulation
// Created Date: 08/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/core/modulation.hpp"

namespace autd3::modulation {

namespace lpf {
constexpr size_t COEFFICIENT_SIZE = 199;
#ifdef AUTD3_USE_SINGLE_FLOAT
static driver::autd3_float_t coefficient[COEFFICIENT_SIZE] = {
    0.0000103f,  0.0000071f,  0.0000034f,  -0.0000008f, -0.0000055f, -0.0000108f, -0.0000165f, -0.0000227f, -0.0000294f, -0.0000366f, -0.0000441f,
    -0.0000520f, -0.0000601f, -0.0000684f, -0.0000767f, -0.0000850f, -0.0000930f, -0.0001007f, -0.0001078f, -0.0001142f, -0.0001195f, -0.0001236f,
    -0.0001261f, -0.0001268f, -0.0001254f, -0.0001215f, -0.0001148f, -0.0001048f, -0.0000913f, -0.0000737f, -0.0000516f, -0.0000247f, 0.0000076f,
    0.0000458f,  0.0000903f,  0.0001416f,  0.0002002f,  0.0002665f,  0.0003411f,  0.0004245f,  0.0005170f,  0.0006192f,  0.0007315f,  0.0008544f,
    0.0009881f,  0.0011331f,  0.0012897f,  0.0014582f,  0.0016389f,  0.0018320f,  0.0020376f,  0.0022559f,  0.0024869f,  0.0027308f,  0.0029873f,
    0.0032564f,  0.0035381f,  0.0038319f,  0.0041377f,  0.0044550f,  0.0047835f,  0.0051226f,  0.0054719f,  0.0058306f,  0.0061981f,  0.0065736f,
    0.0069563f,  0.0073453f,  0.0077398f,  0.0081386f,  0.0085408f,  0.0089452f,  0.0093508f,  0.0097564f,  0.0101608f,  0.0105628f,  0.0109611f,
    0.0113544f,  0.0117415f,  0.0121212f,  0.0124921f,  0.0128529f,  0.0132025f,  0.0135397f,  0.0138631f,  0.0141717f,  0.0144644f,  0.0147401f,
    0.0149977f,  0.0152363f,  0.0154550f,  0.0156530f,  0.0158295f,  0.0159839f,  0.0161156f,  0.0162240f,  0.0163087f,  0.0163695f,  0.0164061f,
    0.0164183f,  0.0164061f,  0.0163695f,  0.0163087f,  0.0162240f,  0.0161156f,  0.0159839f,  0.0158295f,  0.0156530f,  0.0154550f,  0.0152363f,
    0.0149977f,  0.0147401f,  0.0144644f,  0.0141717f,  0.0138631f,  0.0135397f,  0.0132025f,  0.0128529f,  0.0124921f,  0.0121212f,  0.0117415f,
    0.0113544f,  0.0109611f,  0.0105628f,  0.0101608f,  0.0097564f,  0.0093508f,  0.0089452f,  0.0085408f,  0.0081386f,  0.0077398f,  0.0073453f,
    0.0069563f,  0.0065736f,  0.0061981f,  0.0058306f,  0.0054719f,  0.0051226f,  0.0047835f,  0.0044550f,  0.0041377f,  0.0038319f,  0.0035381f,
    0.0032564f,  0.0029873f,  0.0027308f,  0.0024869f,  0.0022559f,  0.0020376f,  0.0018320f,  0.0016389f,  0.0014582f,  0.0012897f,  0.0011331f,
    0.0009881f,  0.0008544f,  0.0007315f,  0.0006192f,  0.0005170f,  0.0004245f,  0.0003411f,  0.0002665f,  0.0002002f,  0.0001416f,  0.0000903f,
    0.0000458f,  0.0000076f,  -0.0000247f, -0.0000516f, -0.0000737f, -0.0000913f, -0.0001048f, -0.0001148f, -0.0001215f, -0.0001254f, -0.0001268f,
    -0.0001261f, -0.0001236f, -0.0001195f, -0.0001142f, -0.0001078f, -0.0001007f, -0.0000930f, -0.0000850f, -0.0000767f, -0.0000684f, -0.0000601f,
    -0.0000520f, -0.0000441f, -0.0000366f, -0.0000294f, -0.0000227f, -0.0000165f, -0.0000108f, -0.0000055f, -0.0000008f, 0.0000034f,  0.0000071f,
    0.0000103f};
#else
static driver::autd3_float_t coefficient[COEFFICIENT_SIZE] = {
    0.0000103,  0.0000071,  0.0000034,  -0.0000008, -0.0000055, -0.0000108, -0.0000165, -0.0000227, -0.0000294, -0.0000366, -0.0000441, -0.0000520,
    -0.0000601, -0.0000684, -0.0000767, -0.0000850, -0.0000930, -0.0001007, -0.0001078, -0.0001142, -0.0001195, -0.0001236, -0.0001261, -0.0001268,
    -0.0001254, -0.0001215, -0.0001148, -0.0001048, -0.0000913, -0.0000737, -0.0000516, -0.0000247, 0.0000076,  0.0000458,  0.0000903,  0.0001416,
    0.0002002,  0.0002665,  0.0003411,  0.0004245,  0.0005170,  0.0006192,  0.0007315,  0.0008544,  0.0009881,  0.0011331,  0.0012897,  0.0014582,
    0.0016389,  0.0018320,  0.0020376,  0.0022559,  0.0024869,  0.0027308,  0.0029873,  0.0032564,  0.0035381,  0.0038319,  0.0041377,  0.0044550,
    0.0047835,  0.0051226,  0.0054719,  0.0058306,  0.0061981,  0.0065736,  0.0069563,  0.0073453,  0.0077398,  0.0081386,  0.0085408,  0.0089452,
    0.0093508,  0.0097564,  0.0101608,  0.0105628,  0.0109611,  0.0113544,  0.0117415,  0.0121212,  0.0124921,  0.0128529,  0.0132025,  0.0135397,
    0.0138631,  0.0141717,  0.0144644,  0.0147401,  0.0149977,  0.0152363,  0.0154550,  0.0156530,  0.0158295,  0.0159839,  0.0161156,  0.0162240,
    0.0163087,  0.0163695,  0.0164061,  0.0164183,  0.0164061,  0.0163695,  0.0163087,  0.0162240,  0.0161156,  0.0159839,  0.0158295,  0.0156530,
    0.0154550,  0.0152363,  0.0149977,  0.0147401,  0.0144644,  0.0141717,  0.0138631,  0.0135397,  0.0132025,  0.0128529,  0.0124921,  0.0121212,
    0.0117415,  0.0113544,  0.0109611,  0.0105628,  0.0101608,  0.0097564,  0.0093508,  0.0089452,  0.0085408,  0.0081386,  0.0077398,  0.0073453,
    0.0069563,  0.0065736,  0.0061981,  0.0058306,  0.0054719,  0.0051226,  0.0047835,  0.0044550,  0.0041377,  0.0038319,  0.0035381,  0.0032564,
    0.0029873,  0.0027308,  0.0024869,  0.0022559,  0.0020376,  0.0018320,  0.0016389,  0.0014582,  0.0012897,  0.0011331,  0.0009881,  0.0008544,
    0.0007315,  0.0006192,  0.0005170,  0.0004245,  0.0003411,  0.0002665,  0.0002002,  0.0001416,  0.0000903,  0.0000458,  0.0000076,  -0.0000247,
    -0.0000516, -0.0000737, -0.0000913, -0.0001048, -0.0001148, -0.0001215, -0.0001254, -0.0001268, -0.0001261, -0.0001236, -0.0001195, -0.0001142,
    -0.0001078, -0.0001007, -0.0000930, -0.0000850, -0.0000767, -0.0000684, -0.0000601, -0.0000520, -0.0000441, -0.0000366, -0.0000294, -0.0000227,
    -0.0000165, -0.0000108, -0.0000055, -0.0000008, 0.0000034,  0.0000071,  0.0000103};
#endif
}  // namespace lpf

/**
 * @brief LPF Modulation to reduce noise
 */
template <typename T>
class LPF final : public core::Modulation {
 public:
#ifdef AUTD3_CAPI
  explicit LPF(std::shared_ptr<core::Modulation> modulation) : Modulation(8192), _modulation(std::move(modulation)) {}
  core::Modulation& modulation() noexcept { return *_modulation; }
#else
  /**
   * \param args constructor parameter of T
   */
  template <typename... Args>
  explicit LPF(Args&&... args) : Modulation(8192), _modulation(std::forward<Args>(args)...) {}
  T& modulation() noexcept { return _modulation; }
#endif

  std::vector<driver::autd3_float_t> calc() override {
    const auto original_buffer = modulation().calc();

    std::vector<driver::autd3_float_t> resampled;
    resampled.reserve(original_buffer.size() * modulation().sampling_frequency_division / 4096);
    for (const auto d : original_buffer)
      std::generate_n(std::back_inserter(resampled), modulation().sampling_frequency_division / 4096, [d] { return d; });

    std::vector<driver::autd3_float_t> mf;
    if (resampled.size() % 2 == 0) {
      mf.reserve(resampled.size() / 2);
      for (size_t i = 0; i < resampled.size(); i += 2) mf.emplace_back((resampled[i] + resampled[i + 1]) / 2);
    } else {
      mf.reserve(resampled.size());
      size_t i;
      for (i = 0; i < resampled.size() - 1; i += 2) mf.emplace_back((resampled[i] + resampled[i + 1]) / 2);
      mf.emplace_back((resampled[i] + resampled[0]) / 2);
      for (i = 1; i < resampled.size(); i += 2) mf.emplace_back((resampled[i] + resampled[i + 1]) / 2);
    }

    return generate_iota(mf.size(), [this, &mf](const size_t i) {
      driver::autd3_float_t r = 0;
      for (int32_t j = 0; j < static_cast<int32_t>(lpf::COEFFICIENT_SIZE); j++) {
        const auto duty =
            driver::Modulation::to_duty(mf[static_cast<size_t>(driver::rem_euclid(static_cast<int32_t>(i) - j, static_cast<int32_t>(mf.size())))]);
        r += lpf::coefficient[j] * static_cast<driver::autd3_float_t>(duty);
      }
      const auto duty = std::clamp<driver::autd3_float_t>(std::round(r) / 255, 0, 1) / 2;
      return std::sin(duty * driver::pi);
    });
  }

  ~LPF() override = default;
  LPF(const LPF& v) noexcept = default;
  LPF& operator=(const LPF& obj) = delete;
  LPF(LPF&& obj) = default;
  LPF& operator=(LPF&& obj) = delete;

 private:
  T _modulation;
};

}  // namespace autd3::modulation
