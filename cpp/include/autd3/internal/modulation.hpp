// File: modulation.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>

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

  [[nodiscard]] native_methods::DatagramPtr ptr(const Geometry&) const override { return AUTDModulationIntoDatagram(modulation_ptr()); }

  [[nodiscard]] virtual native_methods::ModulationPtr modulation_ptr() const = 0;

  [[nodiscard]] size_t size() const {
    char err[256]{};
    const int32_t n = AUTDModulationSize(modulation_ptr(), err);
    if (n < 0) throw AUTDException(err);
    return static_cast<size_t>(n);
  }
};

template <class M>
class ModulationWithFreqDiv : public Modulation {
 protected:
  std::optional<uint32_t> _freq_div;

 public:
  void with_sampling_frequency_division(const uint32_t div) & { _freq_div = div; }
  [[nodiscard]] M&& with_sampling_frequency_division(const uint32_t div) && {
    _freq_div = div;
    return std::move(*static_cast<M*>(this));
  }
  void with_sampling_frequency(const double freq) & {
    with_sampling_frequency_division(static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq));
  }
  [[nodiscard]] M&& with_sampling_frequency(const double freq) && {
    return std::move(*this).with_sampling_frequency_division(
        static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq));
  }
  template <typename Rep, typename Period>
  void with_sampling_period(const std::chrono::duration<Rep, Period> period) & {
    with_sampling_frequency_division(
        static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / 1000000000.0 *
                              static_cast<double>(std::chrono::duration_cast<std::chrono::nanoseconds>(period).count())));
  }
  template <typename Rep, typename Period>
  [[nodiscard]] M&& with_sampling_period(const std::chrono::duration<Rep, Period> period) && {
    return std::move(*this).with_sampling_frequency_division(
        static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / 1000000000.0 *
                              static_cast<double>(std::chrono::duration_cast<std::chrono::nanoseconds>(period).count())));
  }
};

}  // namespace autd3::internal
