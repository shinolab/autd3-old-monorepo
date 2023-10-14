// File: fir.hpp
// Project: modulation
// Created Date: 12/10/2023
// Author: Shun Suzuki
// -----
// Last Modified: 12/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/modulation/cache.hpp"
#include "autd3/modulation/radiation_pressure.hpp"
#include "autd3/modulation/transform.hpp"

namespace autd3::modulation {

template <class M>
class LPF final : public internal::Modulation, public IntoCache<LPF<M>>, public IntoTransform<LPF<M>>, public IntoRadiationPressure<LPF<M>> {
 public:
  explicit LPF(M m, const uint32_t n_taps, const double cutoff) : _m(std::move(m)), _n_taps(n_taps), _cutoff(cutoff) {
    static_assert(std::is_base_of_v<Modulation, std::remove_reference_t<M>>, "This is not Modulation");
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return internal::native_methods::AUTDModulationWithLowPass(_m.modulation_ptr(), _n_taps, _cutoff);
  }

 private:
  M _m;
  uint32_t _n_taps;
  double _cutoff;
};

template <class M>
class HPF final : public internal::Modulation, public IntoCache<HPF<M>>, public IntoTransform<HPF<M>>, public IntoRadiationPressure<HPF<M>> {
 public:
  explicit HPF(M m, const uint32_t n_taps, const double cutoff) : _m(std::move(m)), _n_taps(n_taps), _cutoff(cutoff) {
    static_assert(std::is_base_of_v<Modulation, std::remove_reference_t<M>>, "This is not Modulation");
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return internal::native_methods::AUTDModulationWithHighPass(_m.modulation_ptr(), _n_taps, _cutoff);
  }

 private:
  M _m;
  uint32_t _n_taps;
  double _cutoff;
};

template <class M>
class BPF final : public internal::Modulation, public IntoCache<BPF<M>>, public IntoTransform<BPF<M>>, public IntoRadiationPressure<BPF<M>> {
 public:
  explicit BPF(M m, const uint32_t n_taps, const double f_low, const double f_high)
      : _m(std::move(m)), _n_taps(n_taps), _f_low(f_low), _f_high(f_high) {
    static_assert(std::is_base_of_v<Modulation, std::remove_reference_t<M>>, "This is not Modulation");
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return internal::native_methods::AUTDModulationWithBandPass(_m.modulation_ptr(), _n_taps, _f_low, _f_high);
  }

 private:
  M _m;
  uint32_t _n_taps;
  double _f_low;
  double _f_high;
};

template <class M>
class BSF final : public internal::Modulation, public IntoCache<BSF<M>>, public IntoTransform<BSF<M>>, public IntoRadiationPressure<BSF<M>> {
 public:
  explicit BSF(M m, const uint32_t n_taps, const double f_low, const double f_high)
      : _m(std::move(m)), _n_taps(n_taps), _f_low(f_low), _f_high(f_high) {
    static_assert(std::is_base_of_v<Modulation, std::remove_reference_t<M>>, "This is not Modulation");
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    return internal::native_methods::AUTDModulationWithBandStop(_m.modulation_ptr(), _n_taps, _f_low, _f_high);
  }

 private:
  M _m;
  uint32_t _n_taps;
  double _f_low;
  double _f_high;
};

template <typename M>
class IntoFIR {
 public:
  [[nodiscard]] LPF<M> with_low_pass(const uint32_t n_taps, const double cutoff) & { return LPF(*static_cast<M*>(this), n_taps, cutoff); }
  [[nodiscard]] LPF<M> with_low_pass(const uint32_t n_taps, const double cutoff) && { return LPF(std::move(*static_cast<M*>(this)), n_taps, cutoff); }
  [[nodiscard]] HPF<M> with_high_pass(const uint32_t n_taps, const double cutoff) & { return HPF(*static_cast<M*>(this), n_taps, cutoff); }
  [[nodiscard]] HPF<M> with_high_pass(const uint32_t n_taps, const double cutoff) && {
    return HPF(std::move(*static_cast<M*>(this)), n_taps, cutoff);
  }
  [[nodiscard]] BPF<M> with_band_pass(const uint32_t n_taps, const double f_low, const double f_high) & {
    return BPF(*static_cast<M*>(this), n_taps, f_low, f_high);
  }
  [[nodiscard]] BPF<M> with_band_pass(const uint32_t n_taps, const double f_low, const double f_high) && {
    return BPF(std::move(*static_cast<M*>(this)), n_taps, f_low, f_high);
  }
  [[nodiscard]] BSF<M> with_band_stop(const uint32_t n_taps, const double f_low, const double f_high) & {
    return BSF(*static_cast<M*>(this), n_taps, f_low, f_high);
  }
  [[nodiscard]] BSF<M> with_band_stop(const uint32_t n_taps, const double f_low, const double f_high) && {
    return BSF(std::move(*static_cast<M*>(this)), n_taps, f_low, f_high);
  }
};

}  // namespace autd3::modulation
