// File: fourier.hpp
// Project: modulation
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <numeric>

#include "autd3/internal/native_methods.hpp"
#include "autd3/modulation/cache.hpp"
#include "autd3/modulation/radiation_pressure.hpp"
#include "autd3/modulation/sine.hpp"
#include "autd3/modulation/transform.hpp"

namespace autd3::modulation {

/**
 * @brief Multi-frequency sine wave modulation
 */
class Fourier final : public internal::Modulation {
 public:
  Fourier(Sine component) { _components.emplace_back(std::move(component)); }

  AUTD3_IMPL_WITH_CACHE_MODULATION
  AUTD3_IMPL_WITH_RADIATION_PRESSURE(Fourier)
  AUTD3_IMPL_WITH_TRANSFORM_MODULATION(Fourier)

  void add_component(Sine component) & { _components.emplace_back(std::move(component)); }

  [[nodiscard]] Fourier&& add_component(Sine component) && {
    _components.emplace_back(std::move(component));
    return std::move(*this);
  }

  /**
   * @brief Add components from iterator
   *
   * @tparam R
   * @param iter iterator of focus points
   */
  template <std::ranges::viewable_range R>
  auto add_components_from_iter(R&& iter) -> std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, Sine>>& {
    for (Sine e : iter) _components.emplace_back(std::move(e));
  }
  /**
   * @brief Add components from iterator
   *
   * @tparam R
   * @param iter iterator of focus points
   */
  template <std::ranges::viewable_range R>
  auto add_components_from_iter(R&& iter) -> std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, Sine>, Fourier&&>&& {
    for (Sine e : iter) _components.emplace_back(std::move(e));
    return std::move(*this);
  }

  Fourier& operator+=(const Sine& rhs) {
    _components.emplace_back(rhs);
    return *this;
  }

  friend Fourier&& operator+(Fourier&& lhs, const Sine& rhs) {
    lhs._components.emplace_back(rhs);
    return std::move(lhs);
  }

  friend Fourier operator+(Sine&& lhs, const Sine& rhs) {
    Fourier m(lhs);
    m._components.emplace_back(rhs);
    return m;
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return std::accumulate(_components.begin() + 1, _components.end(),
                           internal::native_methods::AUTDModulationFourier(_components[0].modulation_ptr()),
                           [](const internal::native_methods::ModulationPtr ptr, const Sine& sine) {
                             return AUTDModulationFourierAddComponent(ptr, sine.modulation_ptr());
                           });
  }

 private:
  std::vector<Sine> _components;
};

}  // namespace autd3::modulation
