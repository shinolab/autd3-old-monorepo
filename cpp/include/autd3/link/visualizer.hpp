// File: visualizer.hpp
// Project: link
// Created Date: 12/10/2023
// Author: Shun Suzuki
// -----
// Last Modified: 25/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <string>
#include <variant>

#include "autd3/internal/exception.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/native_methods/autd3capi-link-visualizer.h"

namespace autd3::link {

using internal::native_methods::CMap;

struct PlottersBackend {
  static constexpr internal::native_methods::Backend backend = internal::native_methods::Backend::Plotters;
};
struct PythonBackend {
  static constexpr internal::native_methods::Backend backend = internal::native_methods::Backend::Python;
};
struct NullBackend {
  static constexpr internal::native_methods::Backend backend = internal::native_methods::Backend::Null;
};

struct Sphere {
  static constexpr internal::native_methods::Directivity directivity = internal::native_methods::Directivity::Sphere;
};
struct T4010A1 {
  static constexpr internal::native_methods::Directivity directivity = internal::native_methods::Directivity::T4010A1;
};

struct PlotRange {
  double x_start;
  double x_end;
  double y_start;
  double y_end;
  double z_start;
  double z_end;
  double resolution;

  explicit PlotRange(const double x_start, const double x_end, const double y_start, const double y_end, const double z_start, const double z_end,
                     const double resolution = 1)
      : x_start(x_start), x_end(x_end), y_start(y_start), y_end(y_end), z_start(z_start), z_end(z_end), resolution(resolution) {}
};

struct PlotConfig {
  std::optional<std::pair<uint32_t, uint32_t>> figsize;
  std::optional<double> cbar_size;
  std::optional<uint32_t> font_size;
  std::optional<uint32_t> label_area_size;
  std::optional<uint32_t> margin;
  std::optional<double> ticks_step;
  std::optional<CMap> cmap;
  std::optional<std::string> fname;
};

struct PyPlotConfig {
  std::optional<std::pair<int32_t, int32_t>> figsize;
  std::optional<int32_t> dpi;
  std::optional<std::string> cbar_position;
  std::optional<std::string> cbar_size;
  std::optional<std::string> cbar_pad;
  std::optional<int32_t> fontsize;
  std::optional<double> ticks_step;
  std::optional<std::string> cmap;
  std::optional<bool> show;
  std::optional<std::string> fname;
};

struct NullPlotConfig {};

using Config = std::variant<PlotConfig, PyPlotConfig, NullPlotConfig>;

/**
 * @brief Link for visualizing
 *
 */
class Visualizer final {
  struct Props {
    internal::native_methods::Backend backend;
    internal::native_methods::Directivity directivity;
  };

  internal::native_methods::LinkPtr _ptr;
  internal::native_methods::Backend _backend;
  internal::native_methods::Directivity _directivity;

  [[nodiscard]] internal::native_methods::ConfigPtr get_plot_config(Config config) const {
    char err[256];
    if (_backend == internal::native_methods::Backend::Plotters && std::holds_alternative<PlotConfig>(config)) {
      const auto& [figsize, cbar_size, font_size, label_area_size, margin, ticks_step, cmap, fname] = std::get<PlotConfig>(config);
      auto ptr = internal::native_methods::AUTDLinkVisualizerPlotConfigDefault();
      if (figsize.has_value()) ptr = AUTDLinkVisualizerPlotConfigWithFigSize(ptr, figsize.value().first, figsize.value().second);
      if (cbar_size.has_value()) ptr = AUTDLinkVisualizerPlotConfigWithCBarSize(ptr, cbar_size.value());
      if (font_size.has_value()) ptr = AUTDLinkVisualizerPlotConfigWithFontSize(ptr, font_size.value());
      if (label_area_size.has_value()) ptr = AUTDLinkVisualizerPlotConfigWithLabelAreaSize(ptr, label_area_size.value());
      if (margin.has_value()) ptr = AUTDLinkVisualizerPlotConfigWithMargin(ptr, margin.value());
      if (ticks_step.has_value()) ptr = AUTDLinkVisualizerPlotConfigWithTicksStep(ptr, ticks_step.value());
      if (cmap.has_value()) ptr = AUTDLinkVisualizerPlotConfigWithCMap(ptr, cmap.value());
      if (fname.has_value()) ptr = AUTDLinkVisualizerPlotConfigWithFName(ptr, fname.value().c_str(), err);
      if (ptr._0 == nullptr) throw internal::AUTDException(err);
      return internal::native_methods::ConfigPtr{ptr._0};
    }

    if (_backend == internal::native_methods::Backend::Python && std::holds_alternative<PyPlotConfig>(config)) {
      const auto& [figsize, dpi, cbar_position, cbar_size, cbar_pad, fontsize, ticks_step, cmap, show, fname] = std::get<PyPlotConfig>(config);
      auto ptr = internal::native_methods::AUTDLinkVisualizerPyPlotConfigDefault();
      if (figsize.has_value()) ptr = AUTDLinkVisualizerPyPlotConfigWithFigSize(ptr, figsize.value().first, figsize.value().second);
      if (dpi.has_value()) ptr = AUTDLinkVisualizerPyPlotConfigWithDPI(ptr, dpi.value());
      if (cbar_position.has_value()) {
        ptr = AUTDLinkVisualizerPyPlotConfigWithCBarPosition(ptr, cbar_position.value().c_str(), err);
        if (ptr._0 == nullptr) throw internal::AUTDException(err);
      }
      if (cbar_size.has_value()) {
        ptr = AUTDLinkVisualizerPyPlotConfigWithCBarSize(ptr, cbar_size.value().c_str(), err);
        if (ptr._0 == nullptr) throw internal::AUTDException(err);
      }
      if (cbar_pad.has_value()) {
        ptr = AUTDLinkVisualizerPyPlotConfigWithCBarPad(ptr, cbar_pad.value().c_str(), err);
        if (ptr._0 == nullptr) throw internal::AUTDException(err);
      }
      if (fontsize.has_value()) ptr = AUTDLinkVisualizerPyPlotConfigWithFontSize(ptr, fontsize.value());
      if (ticks_step.has_value()) ptr = AUTDLinkVisualizerPyPlotConfigWithTicksStep(ptr, ticks_step.value());
      if (cmap.has_value()) {
        ptr = AUTDLinkVisualizerPyPlotConfigWithCMap(ptr, cmap.value().c_str(), err);
        if (ptr._0 == nullptr) throw internal::AUTDException(err);
      }
      if (show.has_value()) ptr = AUTDLinkVisualizerPyPlotConfigWithShow(ptr, show.value());
      if (fname.has_value()) {
        ptr = AUTDLinkVisualizerPyPlotConfigWithFName(ptr, fname.value().c_str(), err);
        if (ptr._0 == nullptr) throw internal::AUTDException(err);
      }
      return internal::native_methods::ConfigPtr{ptr._0};
    }

    if (_backend == internal::native_methods::Backend::Null && std::holds_alternative<NullPlotConfig>(config))
      return internal::native_methods::ConfigPtr{internal::native_methods::AUTDLinkVisualizerNullPlotConfigDefault()._0};

    throw internal::AUTDException("Invalid plot config type.");
  }

 public:
  class Builder final : public internal::LinkBuilder {
    friend class Visualizer;

    Props _props;
    std::optional<int32_t> _gpu_idx;

    explicit Builder(const internal::native_methods::Backend backend = internal::native_methods::Backend::Plotters,
                     const internal::native_methods::Directivity directivity = internal::native_methods::Directivity::Sphere)
        : LinkBuilder(), _props{backend, directivity} {}

   public:
    [[nodiscard]] internal::native_methods::LinkBuilderPtr ptr() const override {
      switch (_props.backend) {
        case internal::native_methods::Backend::Plotters:
          switch (_props.directivity) {
            case internal::native_methods::Directivity::Sphere:
              return internal::native_methods::AUTDLinkVisualizerSpherePlotters(_gpu_idx.has_value(), _gpu_idx.value_or(0));
            case internal::native_methods::Directivity::T4010A1:
              return internal::native_methods::AUTDLinkVisualizerT4010A1Plotters(_gpu_idx.has_value(), _gpu_idx.value_or(0));
            default:
              throw std::runtime_error("unreachable");
          }
        case internal::native_methods::Backend::Python:
          switch (_props.directivity) {
            case internal::native_methods::Directivity::Sphere:
              return internal::native_methods::AUTDLinkVisualizerSpherePython(_gpu_idx.has_value(), _gpu_idx.value_or(0));
            case internal::native_methods::Directivity::T4010A1:
              return internal::native_methods::AUTDLinkVisualizerT4010A1Python(_gpu_idx.has_value(), _gpu_idx.value_or(0));
            default:
              throw std::runtime_error("unreachable");
          }
        case internal::native_methods::Backend::Null:
          switch (_props.directivity) {
            case internal::native_methods::Directivity::Sphere:
              return internal::native_methods::AUTDLinkVisualizerSphereNull(_gpu_idx.has_value(), _gpu_idx.value_or(0));
            case internal::native_methods::Directivity::T4010A1:
              return internal::native_methods::AUTDLinkVisualizerT4010A1Null(_gpu_idx.has_value(), _gpu_idx.value_or(0));
            default:
              throw std::runtime_error("unreachable");
          }
        default:
          throw std::runtime_error("unreachable");
      }
    }

    [[nodiscard]] std::shared_ptr<void> props() const override { return std::shared_ptr<void>(new Props{_props.backend, _props.directivity}); }

    [[nodiscard]] Builder with_gpu(const int32_t gpu_idx) {
      _gpu_idx = gpu_idx;
      return *this;
    }

    template <typename B>
    [[nodiscard]] Builder with_backend() {
      _props.backend = B::backend;
      return *this;
    }

    template <typename D>
    [[nodiscard]] Builder with_directivity() {
      _props.directivity = D::directivity;
      return *this;
    }
  };

  static Builder builder() { return Builder(); }
  static Builder plotters() { return Builder().with_backend<PlottersBackend>(); }
  static Builder python() { return Builder().with_backend<PythonBackend>(); }
  static Builder null() { return Builder().with_backend<NullBackend>(); }

  Visualizer() = delete;

  explicit Visualizer(const internal::native_methods::LinkPtr ptr, const internal::native_methods::RuntimePtr, const std::shared_ptr<void>& props)
      : _ptr(ptr) {
    const auto* p = static_cast<const Props*>(props.get());
    _backend = p->backend;
    _directivity = p->directivity;
  }

  [[nodiscard]] std::vector<double> phases_of(const size_t idx) const {
    const auto size = AUTDLinkVisualizerPhasesOf(_ptr, _backend, _directivity, static_cast<uint32_t>(idx), nullptr);
    std::vector<double> buf;
    buf.resize(size);
    AUTDLinkVisualizerPhasesOf(_ptr, _backend, _directivity, static_cast<uint32_t>(idx), buf.data());
    return buf;
  }

  [[nodiscard]] std::vector<double> phases() const { return phases_of(0); }

  [[nodiscard]] std::vector<double> duties_of(const size_t idx) const {
    const auto size = AUTDLinkVisualizerDutiesOf(_ptr, _backend, _directivity, static_cast<uint32_t>(idx), nullptr);
    std::vector<double> buf;
    buf.resize(size);
    AUTDLinkVisualizerDutiesOf(_ptr, _backend, _directivity, static_cast<uint32_t>(idx), buf.data());
    return buf;
  }

  [[nodiscard]] std::vector<double> duties() const { return duties_of(0); }

  [[nodiscard]] std::vector<double> modulation_raw() const {
    const auto size = AUTDLinkVisualizerModulationRaw(_ptr, _backend, _directivity, nullptr);
    std::vector<double> buf;
    buf.resize(size);
    AUTDLinkVisualizerModulationRaw(_ptr, _backend, _directivity, buf.data());
    return buf;
  }

  [[nodiscard]] std::vector<double> modulation() const {
    const auto size = AUTDLinkVisualizerModulation(_ptr, _backend, _directivity, nullptr);
    std::vector<double> buf;
    buf.resize(size);
    AUTDLinkVisualizerModulation(_ptr, _backend, _directivity, buf.data());
    return buf;
  }

  [[nodiscard]] std::vector<std::complex<double>> calc_field_of(std::vector<internal::Vector3>& points, const internal::Geometry& geometry,
                                                                const size_t idx) const {
    const auto points_len = static_cast<uint32_t>(points.size());
    const auto points_ptr = reinterpret_cast<double*>(points.data());
    std::vector<std::complex<double>> buf;
    buf.resize(points_len);
    AUTDLinkVisualizerCalcFieldOf(_ptr, _backend, _directivity, points_ptr, points_len, geometry.ptr(), static_cast<uint32_t>(idx),
                                  reinterpret_cast<double*>(buf.data()));
    return buf;
  }

  [[nodiscard]] std::vector<std::complex<double>> calc_field(std::vector<internal::Vector3>& points, const internal::Geometry& geometry) const {
    return calc_field_of(points, geometry, 0);
  }

  void plot_field_of(const Config& config, const PlotRange& range, const internal::Geometry& geometry, const size_t idx) const {
    const auto config_ptr = get_plot_config(config);
    const auto range_ptr = internal::native_methods::AUTDLinkVisualizerPlotRange(range.x_start, range.x_end, range.y_start, range.y_end,
                                                                                 range.z_start, range.z_end, range.resolution);
    if (char err[256]; AUTDLinkVisualizerPlotFieldOf(_ptr, _backend, _directivity, config_ptr, range_ptr, geometry.ptr(), static_cast<uint32_t>(idx),
                                                     err) == internal::native_methods::AUTD3_ERR)
      throw internal::AUTDException(err);
  }

  void plot_field(const Config& config, const PlotRange& range, const internal::Geometry& geometry) const {
    plot_field_of(config, range, geometry, 0);
  }

  void plot_phase_of(const Config& config, const internal::Geometry& geometry, const size_t idx) const {
    const auto config_ptr = get_plot_config(config);
    if (char err[256]; AUTDLinkVisualizerPlotPhaseOf(_ptr, _backend, _directivity, config_ptr, geometry.ptr(), static_cast<uint32_t>(idx), err) ==
                       internal::native_methods::AUTD3_ERR)
      throw internal::AUTDException(err);
  }

  void plot_phase(const Config& config, const internal::Geometry& geometry) const { plot_phase_of(config, geometry, 0); }

  void plot_modulation_raw(const Config& config) const {
    const auto config_ptr = get_plot_config(config);
    if (char err[256]; AUTDLinkVisualizerPlotModulationRaw(_ptr, _backend, _directivity, config_ptr, err) == internal::native_methods::AUTD3_ERR)
      throw internal::AUTDException(err);
  }

  void plot_modulation(const Config& config) const {
    const auto config_ptr = get_plot_config(config);
    if (char err[256]; AUTDLinkVisualizerPlotModulation(_ptr, _backend, _directivity, config_ptr, err) == internal::native_methods::AUTD3_ERR)
      throw internal::AUTDException(err);
  }
};

}  // namespace autd3::link
