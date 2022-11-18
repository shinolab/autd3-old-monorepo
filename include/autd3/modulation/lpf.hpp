// File: lpf.hpp
// Project: modulation
// Created Date: 08/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/core/modulation.hpp"
#include "autd3/driver/utils.hpp"

namespace autd3::modulation {

/**
 * @brief LPF Modulation to reduce noise
 */
class LPF final : public core::Modulation {
 public:
  /**
   * \param modulation Modulation which passes through LPF to reduce noise
   */
  explicit LPF(Modulation& modulation) {
    _coef = {0.0000103,  0.0000071,  0.0000034,  -0.0000008, -0.0000055, -0.0000108, -0.0000165, -0.0000227, -0.0000294, -0.0000366, -0.0000441,
             -0.0000520, -0.0000601, -0.0000684, -0.0000767, -0.0000850, -0.0000930, -0.0001007, -0.0001078, -0.0001142, -0.0001195, -0.0001236,
             -0.0001261, -0.0001268, -0.0001254, -0.0001215, -0.0001148, -0.0001048, -0.0000913, -0.0000737, -0.0000516, -0.0000247, 0.0000076,
             0.0000458,  0.0000903,  0.0001416,  0.0002002,  0.0002665,  0.0003411,  0.0004245,  0.0005170,  0.0006192,  0.0007315,  0.0008544,
             0.0009881,  0.0011331,  0.0012897,  0.0014582,  0.0016389,  0.0018320,  0.0020376,  0.0022559,  0.0024869,  0.0027308,  0.0029873,
             0.0032564,  0.0035381,  0.0038319,  0.0041377,  0.0044550,  0.0047835,  0.0051226,  0.0054719,  0.0058306,  0.0061981,  0.0065736,
             0.0069563,  0.0073453,  0.0077398,  0.0081386,  0.0085408,  0.0089452,  0.0093508,  0.0097564,  0.0101608,  0.0105628,  0.0109611,
             0.0113544,  0.0117415,  0.0121212,  0.0124921,  0.0128529,  0.0132025,  0.0135397,  0.0138631,  0.0141717,  0.0144644,  0.0147401,
             0.0149977,  0.0152363,  0.0154550,  0.0156530,  0.0158295,  0.0159839,  0.0161156,  0.0162240,  0.0163087,  0.0163695,  0.0164061,
             0.0164183,  0.0164061,  0.0163695,  0.0163087,  0.0162240,  0.0161156,  0.0159839,  0.0158295,  0.0156530,  0.0154550,  0.0152363,
             0.0149977,  0.0147401,  0.0144644,  0.0141717,  0.0138631,  0.0135397,  0.0132025,  0.0128529,  0.0124921,  0.0121212,  0.0117415,
             0.0113544,  0.0109611,  0.0105628,  0.0101608,  0.0097564,  0.0093508,  0.0089452,  0.0085408,  0.0081386,  0.0077398,  0.0073453,
             0.0069563,  0.0065736,  0.0061981,  0.0058306,  0.0054719,  0.0051226,  0.0047835,  0.0044550,  0.0041377,  0.0038319,  0.0035381,
             0.0032564,  0.0029873,  0.0027308,  0.0024869,  0.0022559,  0.0020376,  0.0018320,  0.0016389,  0.0014582,  0.0012897,  0.0011331,
             0.0009881,  0.0008544,  0.0007315,  0.0006192,  0.0005170,  0.0004245,  0.0003411,  0.0002665,  0.0002002,  0.0001416,  0.0000903,
             0.0000458,  0.0000076,  -0.0000247, -0.0000516, -0.0000737, -0.0000913, -0.0001048, -0.0001148, -0.0001215, -0.0001254, -0.0001268,
             -0.0001261, -0.0001236, -0.0001195, -0.0001142, -0.0001078, -0.0001007, -0.0000930, -0.0000850, -0.0000767, -0.0000684, -0.0000601,
             -0.0000520, -0.0000441, -0.0000366, -0.0000294, -0.0000227, -0.0000165, -0.0000108, -0.0000055, -0.0000008, 0.0000034,  0.0000071,
             0.0000103};

    modulation.build();
    _resampled.reserve(modulation.buffer().size() * modulation.sampling_frequency_division());
    for (const auto d : modulation.buffer())
      for (size_t i = 0; i < modulation.sampling_frequency_division(); i++) _resampled.emplace_back(d);
  }

  bool calc() override {
    std::vector<uint8_t> mf;
    if (_resampled.size() % 2 == 0) {
      mf.reserve(_resampled.size() / 2);
      for (size_t i = 0; i < _resampled.size(); i += 2)
        mf.emplace_back(static_cast<uint8_t>((static_cast<uint16_t>(_resampled[i]) + static_cast<uint16_t>(_resampled[i + 1])) >> 1));
    } else {
      mf.reserve(_resampled.size());
      size_t i;
      for (i = 0; i < _resampled.size() - 1; i += 2)
        mf.emplace_back(static_cast<uint8_t>((static_cast<uint16_t>(_resampled[i]) + static_cast<uint16_t>(_resampled[i + 1])) >> 1));
      mf.emplace_back(static_cast<uint8_t>((static_cast<uint16_t>(_resampled[i]) + static_cast<uint16_t>(_resampled[0])) >> 1));
      for (i = 1; i < _resampled.size(); i += 2)
        mf.emplace_back(static_cast<uint8_t>((static_cast<uint16_t>(_resampled[i]) + static_cast<uint16_t>(_resampled[i + 1])) >> 1));
    }

    this->_props.buffer.reserve(mf.size());
    for (int32_t i = 0; i < static_cast<int32_t>(mf.size()); i++) {
      double r = 0.0;
      for (int32_t j = 0; j < static_cast<int32_t>(_coef.size()); j++)
        r += _coef[j] * static_cast<double>(mf[static_cast<size_t>(autd3::driver::rem_euclid(i - j, static_cast<int32_t>(mf.size())))]);
      this->_props.buffer.emplace_back(static_cast<uint8_t>(std::round(r)));
    }
    return true;
  }

  ~LPF() override = default;
  LPF(const LPF& v) noexcept = default;
  LPF& operator=(const LPF& obj) = default;
  LPF(LPF&& obj) = default;
  LPF& operator=(LPF&& obj) = default;

 private:
  std::vector<uint8_t> _resampled;
  std::vector<double> _coef;
};

}  // namespace autd3::modulation
