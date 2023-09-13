// File: cache.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::modulation {

/**
 * @brief Modulation to cache the result of calculation
 */
class Cache final : public internal::Modulation {
 public:
  template <class M>
  explicit Cache(M&& m) : _freq_div(m.sampling_frequency_division()) {
    static_assert(std::is_base_of_v<Modulation, std::remove_reference_t<M>>, "This is not Modulation");
    char err[256]{};
    const auto size = internal::native_methods::AUTDModulationSize(m.modulation_ptr(), err);
    if (size == internal::native_methods::AUTD3_ERR) throw internal::AUTDException(err);
    _buffer.resize(static_cast<size_t>(size));
    if (internal::native_methods::AUTDModulationCalc(m.modulation_ptr(), _buffer.data(), err) == internal::native_methods::AUTD3_ERR)
      throw internal::AUTDException(err);
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return internal::native_methods::AUTDModulationCustom(_freq_div, _buffer.data(), _buffer.size());
  }

  [[nodiscard]] const std::vector<double>& buffer() const { return _buffer; }
  std::vector<double>& buffer() { return _buffer; }

  [[nodiscard]] std::vector<double>::const_iterator begin() const noexcept { return _buffer.cbegin(); }
  [[nodiscard]] std::vector<double>::const_iterator end() const noexcept { return _buffer.cend(); }
  [[nodiscard]] std::vector<double>::iterator begin() noexcept { return _buffer.begin(); }
  [[nodiscard]] std::vector<double>::iterator end() noexcept { return _buffer.end(); }
  [[nodiscard]] const double& operator[](const size_t i) const { return _buffer[i]; }
  [[nodiscard]] double& operator[](const size_t i) { return _buffer[i]; }

 private:
  std::vector<double> _buffer;
  uint32_t _freq_div;
};

}  // namespace autd3::modulation
