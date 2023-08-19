// File: simulator.hpp
// Project: extra
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 18/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <filesystem>

#include "autd3/internal/exception.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::extra {

/**
 * @brief AUTD Simulator
 */
class Simulator {
 public:
  Simulator() : _ptr(internal::native_methods::AUTDSimulator()) {}

  /**
   * @brief Set simulator port
   *
   * @param port Port
   * @return Simulator&
   */
  Simulator& with_port(const uint16_t port) {
    _ptr = internal::native_methods::AUTDSimulatorPort(_ptr, port);
    return *this;
  }

  /**
   * @brief Set window size
   *
   * @param width Width of window
   * @param height Height of window
   * @return Simulator&
   */
  Simulator& with_window_size(const uint32_t width, const uint32_t height) {
    _ptr = internal::native_methods::AUTDSimulatorWindowSize(_ptr, width, height);
    return *this;
  }

  /**
   * @brief Set vsync
   *
   * @param vsync vsync
   * @return Simulator&
   */
  Simulator& with_vsync(const uint32_t vsync) {
    _ptr = internal::native_methods::AUTDSimulatorVsync(_ptr, vsync);
    return *this;
  }

  /**
   * @brief Set GPU index
   *
   * @param idx GPU index. If -1, use the most suitable GPU.
   * @return Simulator&
   */
  Simulator& with_gpu_idx(const int32_t idx) {
    _ptr = internal::native_methods::AUTDSimulatorGpuIdx(_ptr, idx);
    return *this;
  }

  /**
   * @brief Set settings path
   *
   * @param path Settings path
   * @return Simulator&
   */
  Simulator& with_settings_path(const std::filesystem::path& path) {
    char err[256]{};
    if (auto* ptr = internal::native_methods::AUTDSimulatorSettingsPath(_ptr, path.string().c_str(), err); ptr != nullptr) _ptr = ptr;
    return *this;
  }

  /**
   * @brief Run simulator
   *
   * @return 0 if success, otherwise error code
   */
  [[nodiscard]] int32_t run() const { return internal::native_methods::AUTDSimulatorRun(_ptr); }

  /**
   * @brief Save current simulator settings to file
   *
   * @param path File path
   */
  void save_settings(const std::filesystem::path& path) const {
    if (char err[256]{}; !internal::native_methods::AUTDSimulatorSaveSettings(_ptr, path.string().c_str(), err)) throw internal::AUTDException(err);
  }

 private:
  void* _ptr;
};
}  // namespace autd3::extra
