// File: cache.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/internal/emit_intensity.hpp"
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
    auto cache = validate(internal::native_methods::AUTDModulationWithCache(m.modulation_ptr()));
    _buffer.resize(internal::native_methods::AUTDModulationCacheGetBufferLen(cache), internal::EmitIntensity::minimum());
    AUTDModulationCacheGetBuffer(cache, reinterpret_cast<uint8_t*>(_buffer.data()));
    _cache = std::shared_ptr<internal::native_methods::CachePtr>(
        new internal::native_methods::CachePtr(cache), [](const internal::native_methods::CachePtr* ptr) { AUTDModulationCacheDelete(*ptr); });
  }
  Cache(const Cache& v) = default;
  Cache& operator=(const Cache& obj) = delete;
  Cache(Cache&& obj) noexcept = default;
  Cache& operator=(Cache&& obj) noexcept = delete;
  ~Cache() noexcept override = default;  // LCOV_EXCL_LINE

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override { return AUTDModulationCacheIntoModulation(*_cache); }

  [[nodiscard]] const std::vector<internal::EmitIntensity>& buffer() const { return _buffer; }

  [[nodiscard]] std::vector<internal::EmitIntensity>::const_iterator cbegin() const noexcept { return _buffer.cbegin(); }
  [[nodiscard]] std::vector<internal::EmitIntensity>::const_iterator cend() const noexcept { return _buffer.cend(); }
  [[nodiscard]] std::vector<internal::EmitIntensity>::const_iterator begin() const noexcept { return _buffer.begin(); }
  [[nodiscard]] std::vector<internal::EmitIntensity>::const_iterator end() const noexcept { return _buffer.end(); }

  [[nodiscard]] const internal::EmitIntensity& operator[](const size_t i) const { return _buffer[i]; }

 private:
  std::shared_ptr<internal::native_methods::CachePtr> _cache;
  std::vector<internal::EmitIntensity> _buffer;
};

template <class M>
class IntoCache {
 public:
  [[nodiscard]] Cache with_cache() & { return Cache(*static_cast<M*>(this)); }
  [[nodiscard]] Cache with_cache() && { return Cache(std::move(*static_cast<M*>(this))); }
};

}  // namespace autd3::modulation
