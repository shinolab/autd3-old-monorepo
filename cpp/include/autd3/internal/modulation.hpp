// File: modulation.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class Modulation : public Datagram {
 public:
  explicit Modulation() : Datagram() {}
  Modulation(const Modulation& obj) = default;
  Modulation& operator=(const Modulation& obj) = default;
  Modulation(Modulation&& obj) = default;
  Modulation& operator=(Modulation&& obj) = default;
  ~Modulation() override = default;

  /**
   * @brief Get sampling frequency division
   * @details The sampling frequency is [autd3::internal::native_methods::FPGA_SUB_CLK_FREQ] / (sampling frequency division).
   */
  [[nodiscard]] uint32_t sampling_frequency_division() const { return AUTDModulationSamplingFrequencyDivision(modulation_ptr()); }

  /**
   * @brief Get sampling frequency
   */
  [[nodiscard]] double sampling_frequency() const { return native_methods::FPGA_SUB_CLK_FREQ / static_cast<double>(sampling_frequency_division()); }

  [[nodiscard]] native_methods::DatagramPtr ptr(const std::vector<const Device*>&) const override {
    return AUTDModulationIntoDatagram(modulation_ptr());
  }

  [[nodiscard]] virtual native_methods::ModulationPtr modulation_ptr() const = 0;
};

}  // namespace autd3::internal
