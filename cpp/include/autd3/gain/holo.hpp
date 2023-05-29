// File: holo.hpp
// Project: gain
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <variant>

#include "autd3/internal/gain.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain::holo {

class Backend {
 public:
  Backend(void *ptr) noexcept : _ptr(ptr) {}

  [[nodiscard]] void *ptr() const { return _ptr; }

 protected:
  void *_ptr;
};

[[nodiscard]] inline Backend default_backend() { return Backend(internal::native_methods::AUTDDefaultBackend()); }

class DontCare {
 public:
  DontCare() = default;
};

class Normalize {
 public:
  Normalize() = default;
};

class Uniform {
 public:
  Uniform(const double value) : _value(value) {}

  [[nodiscard]] double value() const { return _value; }

 private:
  double _value;
};

class Clamp {
 public:
  Clamp(const double min, const double max) : _min(min), _max(max) {}

  [[nodiscard]] double min() const { return _min; }
  [[nodiscard]] double max() const { return _max; }

 private:
  double _min;
  double _max;
};

using AmplitudeConstraint = std::variant<DontCare, Normalize, Uniform, Clamp>;

class Holo : public internal::Gain {
 public:
  Holo(void *ptr) : internal::Gain(ptr) {}

  virtual void add_focus(const internal::Vector3 &focus, double amp) = 0;
  virtual void set_constraint(AmplitudeConstraint constraint) = 0;
};

class SDP : public Holo {
 public:
  SDP(Backend backend) : Holo(internal::native_methods::AUTDGainHoloSDP(backend.ptr())) {}

  void alpha(const double value) { internal::native_methods::AUTDGainHoloSDPAlpha(_ptr, value); }
  void repeat(const size_t value) { internal::native_methods::AUTDGainHoloSDPRepeat(_ptr, static_cast<uint32_t>(value)); }
  void lambda(const double value) { internal::native_methods::AUTDGainHoloSDPLambda(_ptr, value); }

  void add_focus(const internal::Vector3 &focus, double amp) override {
    internal::native_methods::AUTDGainHoloSDPAdd(_ptr, focus.x(), focus.y(), focus.z(), amp);
  }

  void set_constraint(AmplitudeConstraint constraint) override {
    if (std::holds_alternative<DontCare>(constraint)) {
      internal::native_methods::AUTDGainHoloSDPSetDotCareConstraint(_ptr);
    } else if (std::holds_alternative<Normalize>(constraint)) {
      internal::native_methods::AUTDGainHoloSDPSetNormalizeConstraint(_ptr);
    } else if (std::holds_alternative<Uniform>(constraint)) {
      Uniform &c = std::get<Uniform>(constraint);
      internal::native_methods::AUTDGainHoloSDPSetUniformConstraint(_ptr, c.value());
    } else if (std::holds_alternative<Clamp>(constraint)) {
      Clamp &c = std::get<Clamp>(constraint);
      internal::native_methods::AUTDGainHoloSDPSetClampConstraint(_ptr, c.min(), c.max());
    }
  }
};

class EVP : public Holo {
 public:
  EVP(Backend backend) : Holo(internal::native_methods::AUTDGainHoloEVP(backend.ptr())) {}

  void gamma(const double value) { internal::native_methods::AUTDGainHoloEVPGamma(_ptr, value); }

  void add_focus(const internal::Vector3 &focus, double amp) override {
    internal::native_methods::AUTDGainHoloEVPAdd(_ptr, focus.x(), focus.y(), focus.z(), amp);
  }

  void set_constraint(AmplitudeConstraint constraint) override {
    if (std::holds_alternative<DontCare>(constraint)) {
      internal::native_methods::AUTDGainHoloEVPSetDotCareConstraint(_ptr);
    } else if (std::holds_alternative<Normalize>(constraint)) {
      internal::native_methods::AUTDGainHoloEVPSetNormalizeConstraint(_ptr);
    } else if (std::holds_alternative<Uniform>(constraint)) {
      Uniform &c = std::get<Uniform>(constraint);
      internal::native_methods::AUTDGainHoloEVPSetUniformConstraint(_ptr, c.value());
    } else if (std::holds_alternative<Clamp>(constraint)) {
      Clamp &c = std::get<Clamp>(constraint);
      internal::native_methods::AUTDGainHoloEVPSetClampConstraint(_ptr, c.min(), c.max());
    }
  }
};

class GS : public Holo {
 public:
  GS(Backend backend) : Holo(internal::native_methods::AUTDGainHoloGS(backend.ptr())) {}

  void repeat(const size_t value) { internal::native_methods::AUTDGainHoloGSRepeat(_ptr, static_cast<uint32_t>(value)); }

  void add_focus(const internal::Vector3 &focus, double amp) override {
    internal::native_methods::AUTDGainHoloGSAdd(_ptr, focus.x(), focus.y(), focus.z(), amp);
  }

  void set_constraint(AmplitudeConstraint constraint) override {
    if (std::holds_alternative<DontCare>(constraint)) {
      internal::native_methods::AUTDGainHoloGSSetDotCareConstraint(_ptr);
    } else if (std::holds_alternative<Normalize>(constraint)) {
      internal::native_methods::AUTDGainHoloGSSetNormalizeConstraint(_ptr);
    } else if (std::holds_alternative<Uniform>(constraint)) {
      Uniform &c = std::get<Uniform>(constraint);
      internal::native_methods::AUTDGainHoloGSSetUniformConstraint(_ptr, c.value());
    } else if (std::holds_alternative<Clamp>(constraint)) {
      Clamp &c = std::get<Clamp>(constraint);
      internal::native_methods::AUTDGainHoloGSSetClampConstraint(_ptr, c.min(), c.max());
    }
  }
};

class GSPAT : public Holo {
 public:
  GSPAT(Backend backend) : Holo(internal::native_methods::AUTDGainHoloGSPAT(backend.ptr())) {}

  void repeat(const size_t value) { internal::native_methods::AUTDGainHoloGSPATRepeat(_ptr, static_cast<uint32_t>(value)); }

  void add_focus(const internal::Vector3 &focus, double amp) override {
    internal::native_methods::AUTDGainHoloGSPATAdd(_ptr, focus.x(), focus.y(), focus.z(), amp);
  }

  void set_constraint(AmplitudeConstraint constraint) override {
    if (std::holds_alternative<DontCare>(constraint)) {
      internal::native_methods::AUTDGainHoloGSPATSetDotCareConstraint(_ptr);
    } else if (std::holds_alternative<Normalize>(constraint)) {
      internal::native_methods::AUTDGainHoloGSPATSetNormalizeConstraint(_ptr);
    } else if (std::holds_alternative<Uniform>(constraint)) {
      Uniform &c = std::get<Uniform>(constraint);
      internal::native_methods::AUTDGainHoloGSPATSetUniformConstraint(_ptr, c.value());
    } else if (std::holds_alternative<Clamp>(constraint)) {
      Clamp &c = std::get<Clamp>(constraint);
      internal::native_methods::AUTDGainHoloGSPATSetClampConstraint(_ptr, c.min(), c.max());
    }
  }
};

class Naive : public Holo {
 public:
  Naive(Backend backend) : Holo(internal::native_methods::AUTDGainHoloNaive(backend.ptr())) {}

  void add_focus(const internal::Vector3 &focus, double amp) override {
    internal::native_methods::AUTDGainHoloNaiveAdd(_ptr, focus.x(), focus.y(), focus.z(), amp);
  }

  void set_constraint(AmplitudeConstraint constraint) override {
    if (std::holds_alternative<DontCare>(constraint)) {
      internal::native_methods::AUTDGainHoloNaiveSetDotCareConstraint(_ptr);
    } else if (std::holds_alternative<Normalize>(constraint)) {
      internal::native_methods::AUTDGainHoloNaiveSetNormalizeConstraint(_ptr);
    } else if (std::holds_alternative<Uniform>(constraint)) {
      Uniform &c = std::get<Uniform>(constraint);
      internal::native_methods::AUTDGainHoloNaiveSetUniformConstraint(_ptr, c.value());
    } else if (std::holds_alternative<Clamp>(constraint)) {
      Clamp &c = std::get<Clamp>(constraint);
      internal::native_methods::AUTDGainHoloNaiveSetClampConstraint(_ptr, c.min(), c.max());
    }
  }
};

class LM : public Holo {
 public:
  LM(Backend backend) : Holo(internal::native_methods::AUTDGainHoloLM(backend.ptr())) {}

  void eps1(const double value) { internal::native_methods::AUTDGainHoloLMEps1(_ptr, value); }
  void eps2(const double value) { internal::native_methods::AUTDGainHoloLMEps2(_ptr, value); }
  void tau(const double value) { internal::native_methods::AUTDGainHoloLMTau(_ptr, value); }
  void k_max(const size_t value) { internal::native_methods::AUTDGainHoloLMKMax(_ptr, static_cast<uint32_t>(value)); }
  void initial(const std::vector<double> &value) {
    const auto size = static_cast<uint64_t>(value.size());
    internal::native_methods::AUTDGainHoloLMInitial(_ptr, value.data(), size);
  }

  void add_focus(const internal::Vector3 &focus, double amp) override {
    internal::native_methods::AUTDGainHoloLMAdd(_ptr, focus.x(), focus.y(), focus.z(), amp);
  }

  void set_constraint(AmplitudeConstraint constraint) override {
    if (std::holds_alternative<DontCare>(constraint)) {
      internal::native_methods::AUTDGainHoloLMSetDotCareConstraint(_ptr);
    } else if (std::holds_alternative<Normalize>(constraint)) {
      internal::native_methods::AUTDGainHoloLMSetNormalizeConstraint(_ptr);
    } else if (std::holds_alternative<Uniform>(constraint)) {
      Uniform &c = std::get<Uniform>(constraint);
      internal::native_methods::AUTDGainHoloLMSetUniformConstraint(_ptr, c.value());
    } else if (std::holds_alternative<Clamp>(constraint)) {
      Clamp &c = std::get<Clamp>(constraint);
      internal::native_methods::AUTDGainHoloLMSetClampConstraint(_ptr, c.min(), c.max());
    }
  }
};

class Greedy : public Holo {
 public:
  Greedy() : Holo(internal::native_methods::AUTDGainHoloGreedy()) {}

  void phase_div(const size_t value) { internal::native_methods::AUTDGainHoloGreedyPhaseDiv(_ptr, static_cast<uint32_t>(value)); }

  void add_focus(const internal::Vector3 &focus, double amp) override {
    internal::native_methods::AUTDGainHoloGreedyAdd(_ptr, focus.x(), focus.y(), focus.z(), amp);
  }

  void set_constraint(AmplitudeConstraint constraint) override {
    if (std::holds_alternative<DontCare>(constraint)) {
      internal::native_methods::AUTDGainHoloGreedySetDotCareConstraint(_ptr);
    } else if (std::holds_alternative<Normalize>(constraint)) {
      internal::native_methods::AUTDGainHoloGreedySetNormalizeConstraint(_ptr);
    } else if (std::holds_alternative<Uniform>(constraint)) {
      Uniform &c = std::get<Uniform>(constraint);
      internal::native_methods::AUTDGainHoloGreedySetUniformConstraint(_ptr, c.value());
    } else if (std::holds_alternative<Clamp>(constraint)) {
      Clamp &c = std::get<Clamp>(constraint);
      internal::native_methods::AUTDGainHoloGreedySetClampConstraint(_ptr, c.min(), c.max());
    }
  }
};

}  // namespace autd3::gain::holo
