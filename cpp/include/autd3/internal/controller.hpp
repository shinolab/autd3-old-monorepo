// File: controller.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <optional>
#include <vector>

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/exception.hpp"
#include "autd3/internal/firmware_info.hpp"
#include "autd3/internal/fpga_info.hpp"
#include "autd3/internal/geometry.hpp"
#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

/**
 * @brief Controller class for AUTD3
 */
class Controller {
 public:
  /**
   * @brief Builder for Controller
   */
  class Builder {
    friend class Controller;

   public:
    /**
     * @brief Add device
     *
     * @param device AUTD3 device
     * @return Builder
     */
    Builder add_device(const AUTD3& device) {
      if (const auto euler = device.euler(); euler.has_value())
        _ptr = AUTDAddDevice(_ptr, device.position().x(), device.position().y(), device.position().z(), euler.value().x(), euler.value().y(),
                             euler.value().z());
      else if (const auto quat = device.quaternion(); quat.has_value())
        _ptr = AUTDAddDeviceQuaternion(_ptr, device.position().x(), device.position().y(), device.position().z(), quat.value().w(), quat.value().x(),
                                       quat.value().y(), quat.value().z());
      else
        throw std::runtime_error("unreachable!");
      return *this;
    }

    /**
     * @brief Set legacy mode
     *
     * @return Builder
     */
    Builder legacy_mode() {
      _mode = native_methods::TransMode::Legacy;
      return *this;
    }

    /**
     * @brief Set advanced mode
     *
     * @return Builder
     */
    Builder advanced_mode() {
      _mode = native_methods::TransMode::Advanced;
      return *this;
    }

    /**
     * @brief Set advanced phase mode
     *
     * @return Builder
     */
    Builder advanced_phase_mode() {
      _mode = native_methods::TransMode::AdvancedPhase;
      return *this;
    }

    /**
     * @brief Open controller
     *
     * @tparam L Link
     * @param link link
     * @return Controller
     */
    template <class L>
    Controller open_with(L&& link) {
      static_assert(std::is_base_of_v<Link, std::remove_reference_t<L>>, "This is not Link");
      return Controller::open_impl(_ptr, _mode, link.ptr());
    }

   private:
    Builder() : _ptr(native_methods::AUTDCreateControllerBuilder()), _mode(native_methods::TransMode::Legacy) {}

    native_methods::ControllerBuilderPtr _ptr;
    native_methods::TransMode _mode;
  };

  /**
   * @brief Create Controller builder
   *
   * @return Builder
   */
  static Builder builder() noexcept { return {}; }

  Controller() = delete;
  Controller(const Controller& v) = delete;
  Controller& operator=(const Controller& obj) = delete;
  Controller(Controller&& obj) noexcept : _geometry(std::move(obj._geometry)), _ptr(std::move(obj._ptr)), _mode(std::move(obj._mode)) {
    obj._ptr._0 = nullptr;
  }
  Controller& operator=(Controller&& obj) noexcept {
    if (this != &obj) {
      if (_ptr._0 != nullptr) AUTDFreeController(_ptr);

      _geometry = std::move(obj._geometry);
      _ptr = std::move(obj._ptr);
      _mode = std::move(obj._mode);
      obj._ptr._0 = nullptr;
    }
    return *this;
  }

  ~Controller() noexcept {
    try {
      if (_ptr._0 != nullptr) {
        AUTDFreeController(_ptr);
        _ptr._0 = nullptr;
      }
    } catch (std::exception&) {
    }
  }

  [[nodiscard]] const Geometry& geometry() const { return _geometry; }
  [[nodiscard]] Geometry& geometry() { return _geometry; }

  /**
   * @brief Close connection
   */
  void close() const {
    if (char err[256]{}; !AUTDClose(_ptr, err)) throw AUTDException(err);
  }

  /**
   * @brief Get FPGA information
   *
   * @return List of FPGA information
   */
  [[nodiscard]] std::vector<FPGAInfo> fpga_info() {
    char err[256]{};
    const size_t num_devices = geometry().num_devices();
    std::vector<uint8_t> info(num_devices);
    if (!AUTDGetFPGAInfo(_ptr, info.data(), err)) throw AUTDException(err);
    std::vector<FPGAInfo> ret;
    ret.reserve(num_devices);
    std::transform(info.begin(), info.end(), std::back_inserter(ret), [](const uint8_t i) { return FPGAInfo(i); });
    return ret;
  }

  /**
   * @brief Get firmware information
   *
   * @return List of firmware information
   */
  [[nodiscard]] std::vector<FirmwareInfo> firmware_infos() {
    char err[256]{};
    const auto handle = AUTDGetFirmwareInfoListPointer(_ptr, err);
    if (handle._0 == nullptr) throw AUTDException(err);
    std::vector<FirmwareInfo> ret;
    for (uint32_t i = 0; i < static_cast<uint32_t>(geometry().num_devices()); i++) {
      char info[256]{};
      bool props[2];
      AUTDGetFirmwareInfo(handle, i, info, props);
      ret.emplace_back(std::string(info), props[0], props[1]);
    }
    AUTDFreeFirmwareInfoListPointer(handle);
    return ret;
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam H Header type (SilencerConfig or Modulation)
   * @tparam Rep
   * @tparam Period
   * @param header header
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <typename H, typename Rep, typename Period>
  auto send(H&& header, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_header_v<H>, bool> {
    return send(std::forward<H>(header), std::optional(timeout));
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam H Header type (SilencerConfig or Modulation)
   * @tparam Rep
   * @tparam Period
   * @param header header
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <typename H, typename Rep = uint64_t, typename Period = std::milli>
  auto send(H&& header, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<is_header_v<H>, bool> {
    return send(std::forward<H>(header), NullBody(), timeout);
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam B body type (Gain, STM, or Amplitudes)
   * @tparam Rep
   * @tparam Period
   * @param body body
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <typename B, typename Rep, typename Period>
  auto send(B&& body, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_body_v<B>, bool> {
    return send(std::forward<B>(body), std::optional(timeout));
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam B body type (Gain, STM, or Amplitudes)
   * @tparam Rep
   * @tparam Period
   * @param body body
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <typename B, typename Rep = uint64_t, typename Period = std::milli>
  auto send(B&& body, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<is_body_v<B>, bool> {
    return send(NullHeader(), std::forward<B>(body), timeout);
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam H Header type (SilencerConfig or Modulation)
   * @tparam B body type (Gain, STM, or Amplitudes)
   * @tparam Rep
   * @tparam Period
   * @param header header
   * @param body body
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <typename H, typename B, typename Rep, typename Period>
  auto send(H&& header, B&& body, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_header_v<H> && is_body_v<B>, bool> {
    return send(std::forward<H>(header), std::forward<B>(body), std::optional(timeout));
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam H Header type (SilencerConfig or Modulation)
   * @tparam B body type (Gain, STM, or Amplitudes)
   * @tparam Rep
   * @tparam Period
   * @param header header
   * @param body body
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <typename H, typename B, typename Rep = uint64_t, typename Period = std::milli>
  auto send(H&& header, B&& body, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
      -> std::enable_if_t<is_header_v<H> && is_body_v<B>, bool> {
    return send(&header, &body, timeout);
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam S Special type (Clear, Synchronize, Stop, ModDelay, or UpdateFlag)
   * @tparam Rep
   * @tparam Period
   * @param s special data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <typename S, typename Rep, typename Period>
  auto send(S&& s, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_special_v<S>, bool> {
    return send(std::forward<S>(s), std::optional(timeout));
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam S Special type (Clear, Synchronize, Stop, ModDelay, or UpdateFlag)
   * @tparam Rep
   * @tparam Period
   * @param s special data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <typename S, typename Rep = uint64_t, typename Period = std::milli>
  auto send(S&& s, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<is_special_v<S>, bool> {
    char err[256]{};
    const int64_t timeout_ns =
        timeout.has_value() ? static_cast<int64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count()) : -1;
    const auto res = native_methods::AUTDSendSpecial(_ptr, _mode, s.ptr(), timeout_ns, err);
    if (res == native_methods::AUTD3_ERR) throw AUTDException(err);
    return res == native_methods::AUTD3_TRUE;
  }

  /**
   * @brief set force fan flag
   *
   * @param value
   */
  void force_fan(const bool value) const { AUTDSetForceFan(_ptr, value); }

  /**
   * @brief set reads fpga info flag
   *
   * @param value
   */
  void reads_fpga_info(const bool value) const { AUTDSetReadsFPGAInfo(_ptr, value); }

 private:
  static Controller open_impl(const native_methods::ControllerBuilderPtr builder, const native_methods::TransMode mode,
                              const native_methods::LinkPtr link) {
    char err[256]{};

    const auto ptr = AUTDControllerOpenWith(builder, link, err);
    if (ptr._0 == nullptr) throw AUTDException(err);

    Geometry geometry(AUTDGetGeometry(ptr), mode);

    Controller cnt(std::move(geometry), ptr, mode);

    cnt.geometry().configure_transducers();

    return cnt;
  }

  Controller(Geometry geometry, const native_methods::ControllerPtr ptr, const native_methods::TransMode mode)
      : _geometry(std::move(geometry)), _ptr(ptr), _mode(mode) {}

  bool send(const Header* header, const Body* body, const std::optional<std::chrono::nanoseconds> timeout) {
    char err[256]{};
    const int64_t timeout_ns = timeout.has_value() ? static_cast<int64_t>(timeout.value().count()) : -1;
    const auto res = AUTDSend(_ptr, _mode, header->ptr(), body->ptr(geometry()), timeout_ns, err);
    if (res == native_methods::AUTD3_ERR) throw AUTDException(err);
    return res == native_methods::AUTD3_TRUE;
  }

  Geometry _geometry;
  native_methods::ControllerPtr _ptr;
  native_methods::TransMode _mode;
};

}  // namespace autd3::internal
