// File: cache.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/internal/exception.hpp"
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
    auto cache = internal::native_methods::AUTDModulationWithCache(m.modulation_ptr());
    if (cache.result == nullptr) {
      const std::string err_str(cache.err_len, ' ');
      internal::native_methods::AUTDGetErr(cache.err, const_cast<char*>(err_str.c_str()));
      throw internal::AUTDException(err_str);
    }
    _buffer.resize(cache.buffer_len);
    AUTDModulationCacheGetBuffer(cache, _buffer.data());
    _cache = std::shared_ptr<internal::native_methods::ResultCache>(
        new internal::native_methods::ResultCache(cache), [](const internal::native_methods::ResultCache* ptr) { AUTDModulationCacheDelete(*ptr); });
  }
  Cache(const Cache& v) = default;
  Cache& operator=(const Cache& obj) = default;
  Cache(Cache&& obj) noexcept = default;
  Cache& operator=(Cache&& obj) noexcept = default;
  ~Cache() noexcept override = default;

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return internal::native_methods::AUTDModulationCacheIntoModulation(*_cache);
  }

  [[nodiscard]] const std::vector<double>& buffer() const { return _buffer; }

  [[nodiscard]] std::vector<double>::const_iterator cbegin() const noexcept { return _buffer.cbegin(); }
  [[nodiscard]] std::vector<double>::const_iterator cend() const noexcept { return _buffer.cend(); }
  [[nodiscard]] std::vector<double>::const_iterator begin() const noexcept { return _buffer.begin(); }
  [[nodiscard]] std::vector<double>::const_iterator end() const noexcept { return _buffer.end(); }
  [[nodiscard]] const double& operator[](const size_t i) const { return _buffer[i]; }

 private:
  std::shared_ptr<internal::native_methods::ResultCache> _cache;
  std::vector<double> _buffer;
};

template <typename M>
class IntoCache {
 public:
  [[nodiscard]] Cache with_cache() & { return Cache(*static_cast<M*>(this)); }
  [[nodiscard]] Cache with_cache() && { return Cache(std::move(*static_cast<M*>(this))); }
};

}  // namespace autd3::modulation
