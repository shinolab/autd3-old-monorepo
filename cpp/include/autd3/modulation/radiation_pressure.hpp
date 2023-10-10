// File: radiation_pressure.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/modulation/cache.hpp"

namespace autd3::modulation {

/**
 * @brief Modulation for modulating radiation pressure
 */
template <class M>
class RadiationPressure final : public internal::Modulation, public IntoCache<RadiationPressure<M>> {
 public:
  explicit RadiationPressure(M m) : _m(std::move(m)) {
    static_assert(std::is_base_of_v<Modulation, std::remove_reference_t<M>>, "This is not Modulation");
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return internal::native_methods::AUTDModulationWithRadiationPressure(_m.modulation_ptr());
  }

 private:
  M _m;
};

template <typename M>
class IntoRadiationPressure {
 public:
  [[nodiscard]] RadiationPressure<M> with_radiation_pressure() & { return RadiationPressure(*static_cast<M*>(this)); }
  [[nodiscard]] RadiationPressure<M> with_radiation_pressure() && { return RadiationPressure(std::move(*static_cast<M*>(this))); }
};

}  // namespace autd3::modulation
