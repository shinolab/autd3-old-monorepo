// File: modulation.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/sampling_config.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::internal {

class Modulation {
 public:
  Modulation() = default;
  Modulation(const Modulation& obj) = default;
  Modulation& operator=(const Modulation& obj) = default;
  Modulation(Modulation&& obj) = default;
  Modulation& operator=(Modulation&& obj) = default;
  virtual ~Modulation() = default;  // LCOV_EXCL_LINE

  /**
   * @brief Get sampling frequency division
   * @details The sampling frequency is
   * [autd3::internal::native_methods::FPGA_CLK_FREQ] / (sampling frequency
   * division).
   */
  [[nodiscard]] SamplingConfiguration sampling_config() const { return SamplingConfiguration(AUTDModulationSamplingConfig(modulation_ptr())); }

  [[nodiscard]] native_methods::DatagramPtr ptr(const geometry::Geometry&) const { return AUTDModulationIntoDatagram(modulation_ptr()); }

  [[nodiscard]] virtual native_methods::ModulationPtr modulation_ptr() const = 0;

  [[nodiscard]] size_t size() const { return native_methods::validate<size_t>(AUTDModulationSize(modulation_ptr())); }
};

template <class M>
class ModulationWithSamplingConfig : public Modulation {
 protected:
  std::optional<SamplingConfiguration> _config;

 public:
  void with_sampling_config(const SamplingConfiguration config) & { _config = config; }
  [[nodiscard]] M&& with_sampling_config(const SamplingConfiguration config) && {
    _config = config;
    return std::move(*static_cast<M*>(this));
  }
};

}  // namespace autd3::internal
