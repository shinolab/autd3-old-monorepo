// File: cache.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::modulation {

/**
 * @brief Modulation to cache the result of calculation
 */
class Cache final : public internal::Modulation {
 public:
  template <class M>
  explicit Cache(M&& m) {
    static_assert(std::is_base_of_v<Modulation, std::remove_reference_t<M>>, "This is not Modulation");
    char err[256]{};
    auto cache = internal::native_methods::AUTDModulationWithCache(m.modulation_ptr(), err);
    if (cache._0 == nullptr) throw internal::AUTDException(err);
    const auto size = AUTDModulationCacheGetBufferSize(cache);
    _buffer.resize(size);
    AUTDModulationCacheGetBuffer(cache, _buffer.data());
    _cache = std::shared_ptr<internal::native_methods::ModulationCachePtr>(
        new internal::native_methods::ModulationCachePtr(cache),
        [](const internal::native_methods::ModulationCachePtr* ptr) { AUTDModulationCacheDelete(*ptr); });
  }
  Cache(const Cache& v) = default;
  Cache& operator=(const Cache& obj) = default;
  Cache(Cache&& obj) noexcept = default;
  Cache& operator=(Cache&& obj) noexcept = default;
  ~Cache() noexcept override = default;

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override { return AUTDModulationCacheIntoModulation(*_cache); }

  [[nodiscard]] const std::vector<double>& buffer() const { return _buffer; }

  [[nodiscard]] std::vector<double>::const_iterator cbegin() const noexcept { return _buffer.cbegin(); }
  [[nodiscard]] std::vector<double>::const_iterator cend() const noexcept { return _buffer.cend(); }
  [[nodiscard]] std::vector<double>::const_iterator begin() const noexcept { return _buffer.begin(); }
  [[nodiscard]] std::vector<double>::const_iterator end() const noexcept { return _buffer.end(); }
  [[nodiscard]] const double& operator[](const size_t i) const { return _buffer[i]; }

 private:
  std::shared_ptr<internal::native_methods::ModulationCachePtr> _cache;
  std::vector<double> _buffer;
};

}  // namespace autd3::modulation
