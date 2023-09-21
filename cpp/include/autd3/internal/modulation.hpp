// File: modulation.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 21/09/2023
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

  [[nodiscard]] native_methods::DatagramPtr ptr(const Geometry&) const override { return AUTDModulationIntoDatagram(modulation_ptr()); }

  [[nodiscard]] virtual native_methods::ModulationPtr modulation_ptr() const = 0;

  [[nodiscard]] size_t size() const {
    char err[256]{};
    const int32_t n = native_methods::AUTDModulationSize(modulation_ptr(), err);
    if (n < 0) throw AUTDException(err);
    return static_cast<size_t>(n);
  }
};

#define AUTD3_IMPL_WITH_CACHE_MODULATION                                 \
  [[nodiscard]] Cache with_cache()&& { return Cache(std::move(*this)); } \
  [[nodiscard]] Cache with_cache()& { return Cache(*this); }

#define AUTD3_IMPL_WITH_TRANSFORM_MODULATION             \
  template <typename F>                                  \
  [[nodiscard]] Transform with_transform(const F& f)&& { \
    return Transform(std::move(*this), f);               \
  }                                                      \
  template <typename F>                                  \
  [[nodiscard]] Transform with_transform(const F& f)& {  \
    return Transform(*this, f);                          \
  }

#define AUTD3_IMPL_WITH_RADIATION_PRESSURE(TYPE)                                                                    \
  [[nodiscard]] RadiationPressure<TYPE> with_radiation_pressure()&& { return RadiationPressure(std::move(*this)); } \
  [[nodiscard]] RadiationPressure<TYPE> with_radiation_pressure()& { return RadiationPressure(*this); }

#define AUTD3_IMPL_MOD_PROP(TYPE)                                                                                                     \
  void with_sampling_frequency_division(const uint32_t div)& { _freq_div = div; }                                                     \
  [[nodiscard]] TYPE&& with_sampling_frequency_division(const uint32_t div)&& {                                                       \
    _freq_div = div;                                                                                                                  \
    return std::move(*this);                                                                                                          \
  }                                                                                                                                   \
  void with_sampling_frequency(const double freq)& {                                                                                  \
    with_sampling_frequency_division(static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq)); \
  }                                                                                                                                   \
  [[nodiscard]] TYPE&& with_sampling_frequency(const double freq)&& {                                                                 \
    return std::move(*this).with_sampling_frequency_division(                                                                         \
        static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq));                              \
  }                                                                                                                                   \
  template <typename Rep, typename Period>                                                                                            \
  void with_sampling_period(const std::chrono::duration<Rep, Period> period)& {                                                       \
    with_sampling_frequency_division(                                                                                                 \
        static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / 1000000000.0 *                       \
                              static_cast<double>(std::chrono::duration_cast<std::chrono::nanoseconds>(period).count())));            \
  }                                                                                                                                   \
  template <typename Rep, typename Period>                                                                                            \
  [[nodiscard]] TYPE&& with_sampling_period(const std::chrono::duration<Rep, Period> period)&& {                                      \
    return std::move(*this).with_sampling_frequency_division(                                                                         \
        static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / 1000000000.0 *                       \
                              static_cast<double>(std::chrono::duration_cast<std::chrono::nanoseconds>(period).count())));            \
  }

}  // namespace autd3::internal
