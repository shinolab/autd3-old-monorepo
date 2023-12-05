// File: Controller.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <future>
#include <optional>
#include <vector>

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/exception.hpp"
#include "autd3/internal/firmware_info.hpp"
#include "autd3/internal/fpga_info.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::internal {

class ControllerBuilder;

template <class F>
concept group_f = requires(F f, const geometry::Device& dev) { typename std::invoke_result_t<F, const geometry::Device&>::value_type; };

/**
 * @brief Controller class for AUTD3
 */
template <typename L>
class Controller {
  friend class ControllerBuilder;

 public:
  Controller() = delete;
  Controller(const Controller& v) = delete;
  Controller& operator=(const Controller& obj) = delete;
  Controller(Controller&& obj) noexcept : _geometry(std::move(obj._geometry)), _ptr(obj._ptr), _link(std::move(obj._link)) { obj._ptr._0 = nullptr; }
  Controller& operator=(Controller&& obj) noexcept {
    if (this != &obj) {
      _geometry = std::move(obj._geometry);
      _ptr = obj._ptr;
      _link = std::move(obj._link);
      obj._ptr._0 = nullptr;
    }
    return *this;
  }

  ~Controller() noexcept {
    if (_ptr._0 != nullptr) {
      AUTDControllerDelete(_ptr);
      _ptr._0 = nullptr;
    }
  }

  [[nodiscard]] const geometry::Geometry& geometry() const { return _geometry; }
  [[nodiscard]] geometry::Geometry& geometry() { return _geometry; }

  [[nodiscard]] L& link() { return _link; }
  [[nodiscard]] const L& link() const { return _link; }

  /**
   * @brief Close connection
   */
  [[nodiscard]] bool close() const { return validate(AUTDControllerClose(_ptr)) == native_methods::AUTD3_TRUE; }

  /**
   * @brief Close connection
   */
  [[nodiscard]] std::future<bool> close_async() const {
    return std::async(std::launch::deferred, [this]() -> bool { return close(); });
  }

  /**
   * @brief Get FPGA information
   *
   * @return List of FPGA information
   */
  [[nodiscard]] std::vector<FPGAInfo> fpga_info() {
    const size_t num_devices = geometry().num_devices();
    std::vector<uint8_t> info(num_devices);
    validate(AUTDControllerFPGAInfo(_ptr, info.data()));
    std::vector<FPGAInfo> ret;
    ret.reserve(num_devices);
    std::ranges::transform(info, std::back_inserter(ret), [](const uint8_t i) { return FPGAInfo(i); });
    return ret;
  }

  /**
   * @brief Get FPGA information
   *
   * @return List of FPGA information
   */
  [[nodiscard]] std::future<std::vector<FPGAInfo>> fpga_info_async() {
    return std::async(std::launch::deferred, [this]() -> std::vector<FPGAInfo> { return fpga_info(); });
  }

  /**
   * @brief Get firmware information
   *
   * @return List of firmware information
   */
  [[nodiscard]] std::vector<FirmwareInfo> firmware_infos() {
    const auto handle = validate(AUTDControllerFirmwareInfoListPointer(_ptr));
    std::vector<FirmwareInfo> ret;
    for (uint32_t i = 0; i < static_cast<uint32_t>(geometry().num_devices()); i++) {
      char info[256]{};
      AUTDControllerFirmwareInfoGet(handle, i, info);
      ret.emplace_back(std::string(info));
    }
    AUTDControllerFirmwareInfoListPointerDelete(handle);
    return ret;
  }  // LCOV_EXCL_LINE

  /**
   * @brief Get firmware information
   *
   * @return List of firmware information
   */
  [[nodiscard]] std::future<std::vector<FirmwareInfo>> firmware_infos_async() {
    return std::async(std::launch::deferred, [this]() -> std::vector<FirmwareInfo> { return firmware_infos(); });
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam D Datagram
   * @tparam Rep
   * @tparam Period
   * @param data data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <datagram D, typename Rep, typename Period>
  bool send(D&& data, const std::chrono::duration<Rep, Period> timeout) {
    return send(std::forward<D>(data), std::optional(timeout));
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam D Datagram
   * @tparam Rep
   * @tparam Period
   * @param data data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <datagram D, typename Rep, typename Period>
  std::future<bool> send_async(D&& data, const std::chrono::duration<Rep, Period> timeout) {
    return send_async(std::forward<D>(data), std::optional(timeout));
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam D Datagram
   * @tparam Rep
   * @tparam Period
   * @param data data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <datagram D, typename Rep = uint64_t, typename Period = std::milli>
  bool send(D&& data, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) {
    return send(std::forward<D>(data), NullDatagram(), timeout);
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam D Datagram
   * @tparam Rep
   * @tparam Period
   * @param data data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <datagram D, typename Rep = uint64_t, typename Period = std::milli>
  std::future<bool> send_async(D&& data, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) {
    return send_async(std::forward<D>(data), NullDatagram(), timeout);
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam D1 Datagram
   * @tparam D2 Datagram
   * @tparam Rep
   * @tparam Period
   * @param data1 first data
   * @param data2 second data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <datagram D1, datagram D2, typename Rep, typename Period>
  bool send(D1&& data1, D2&& data2, const std::chrono::duration<Rep, Period> timeout) {
    return send(std::forward<D1>(data1), std::forward<D2>(data2), std::optional(timeout));
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam D1 Datagram
   * @tparam D2 Datagram
   * @tparam Rep
   * @tparam Period
   * @param data1 first data
   * @param data2 second data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <datagram D1, datagram D2, typename Rep, typename Period>
  std::future<bool> send_async(D1&& data1, D2&& data2, const std::chrono::duration<Rep, Period> timeout) {
    return send_async(std::forward<D1>(data1), std::forward<D2>(data2), std::optional(timeout));
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam D1 Datagram
   * @tparam D2 Datagram
   * @tparam Rep
   * @tparam Period
   * @param data1 first data
   * @param data2 second data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <datagram D1, datagram D2, typename Rep = uint64_t, typename Period = std::milli>
  bool send(D1&& data1, D2&& data2, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) {
    const int64_t timeout_ns = timeout.has_value() ? std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count() : -1;
    return validate(AUTDControllerSend(_ptr, data1.ptr(_geometry), data2.ptr(_geometry), timeout_ns)) == native_methods::AUTD3_TRUE;
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam D1 Datagram
   * @tparam D2 Datagram
   * @tparam Rep
   * @tparam Period
   * @param data1 first data
   * @param data2 second data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <datagram D1, datagram D2, typename Rep = uint64_t, typename Period = std::milli>
  std::future<bool> send_async(D1&& data1, D2&& data2, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) {
    return std::async(std::launch::deferred, [this, d1 = std::forward<D1>(data1), d2 = std::forward<D2>(data2), timeout]() -> bool {
      const int64_t timeout_ns = timeout.has_value() ? std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count() : -1;
      return validate(AUTDControllerSend(_ptr, d1.ptr(_geometry), d2.ptr(_geometry), timeout_ns)) == native_methods::AUTD3_TRUE;
    });
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam S Special type (Stop)
   * @tparam Rep
   * @tparam Period
   * @param s special data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <special_datagram S, typename Rep, typename Period>
  bool send(S&& s, const std::chrono::duration<Rep, Period> timeout) {
    return send(std::forward<S>(s), std::optional(timeout));
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam S Special type (Stop)
   * @tparam Rep
   * @tparam Period
   * @param s special data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <special_datagram S, typename Rep, typename Period>
  std::future<bool> send_async(S&& s, const std::chrono::duration<Rep, Period> timeout) {
    return send_async(std::forward<S>(s), std::optional(timeout));
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam S Special type (Stop)
   * @tparam Rep
   * @tparam Period
   * @param s special data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <special_datagram S, typename Rep = uint64_t, typename Period = std::milli>
  bool send(S&& s, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) {
    const int64_t timeout_ns =
        timeout.has_value() ? static_cast<int64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count()) : -1;
    return validate(native_methods::AUTDControllerSendSpecial(_ptr, s.ptr(), timeout_ns)) == native_methods::AUTD3_TRUE;
  }

  /**
   * @brief Send data to the devices
   *
   * @tparam S Special type (Stop)
   * @tparam Rep
   * @tparam Period
   * @param s special data
   * @param timeout timeout
   * @return If true, it is confirmed that the data has been successfully transmitted. Otherwise, there are no errors, but it is unclear whether the
   * data has been sent reliably or not.
   */
  template <special_datagram S, typename Rep = uint64_t, typename Period = std::milli>
  std::future<bool> send_async(S&& s, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) {
    return std::async(std::launch::deferred, [this, s_ = std::forward<S>(s), timeout]() -> bool {
      const int64_t timeout_ns =
          timeout.has_value() ? static_cast<int64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count()) : -1;
      return validate(native_methods::AUTDControllerSendSpecial(_ptr, s_.ptr(), timeout_ns)) == native_methods::AUTD3_TRUE;
    });
  }

  template <group_f F>
  class GroupGuard {
   public:
    using key_type = typename std::invoke_result_t<F, const geometry::Device&>::value_type;

    explicit GroupGuard(const F& map, Controller& controller)
        : _controller(controller), _map(map), _kv_map(native_methods::AUTDControllerGroupCreateKVMap()) {}

    template <datagram D, typename Rep = uint64_t, typename Period = std::milli>
    GroupGuard set(const key_type key, D&& data, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) {
      if (_keymap.contains(key)) throw AUTDException("Key already exists");
      const int64_t timeout_ns = timeout.has_value() ? timeout.value().count() : -1;
      const auto ptr = data.ptr(_controller._geometry);
      _keymap[key] = _k++;
      _kv_map = validate(native_methods::AUTDControllerGroupKVMapSet(_kv_map, _keymap[key], ptr, native_methods::DatagramPtr{nullptr}, timeout_ns));
      return std::move(*this);
    }

    template <datagram D, typename Rep, typename Period>
    GroupGuard set(const key_type key, D&& data, const std::chrono::duration<Rep, Period> timeout) {
      return set(key, std::forward<D>(data), std::optional(timeout));
    }

    template <datagram D1, datagram D2, typename Rep = uint64_t, typename Period = std::milli>
    GroupGuard set(const key_type key, D1&& data1, D2&& data2, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) {
      if (_keymap.contains(key)) throw AUTDException("Key already exists");
      const int64_t timeout_ns = timeout.has_value() ? timeout.value().count() : -1;
      const auto ptr1 = data1.ptr(_controller._geometry);
      const auto ptr2 = data2.ptr(_controller._geometry);
      _keymap[key] = _k++;
      _kv_map = validate(native_methods::AUTDControllerGroupKVMapSet(_kv_map, _keymap[key], ptr1, ptr2, timeout_ns));
      return std::move(*this);
    }

    template <datagram D1, datagram D2, typename Rep, typename Period>
    GroupGuard set(const key_type key, D1&& data1, D2&& data2, const std::chrono::duration<Rep, Period> timeout) {
      return set(key, std::forward<D1>(data1), std::forward<D2>(data2), std::optional(timeout));
    }

    template <special_datagram D, typename Rep = uint64_t, typename Period = std::milli>
    GroupGuard set(const key_type key, D&& data, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) {
      if (_keymap.contains(key)) throw AUTDException("Key already exists");
      const int64_t timeout_ns = timeout.has_value() ? timeout.value().count() : -1;
      const auto ptr = data.ptr();
      _keymap[key] = _k++;
      _kv_map = validate(native_methods::AUTDControllerGroupKVMapSetSpecial(_kv_map, _keymap[key], ptr, timeout_ns));
      return std::move(*this);
    }

    template <special_datagram D, typename Rep, typename Period>
    GroupGuard set(const key_type key, D&& data, const std::chrono::duration<Rep, Period> timeout) {
      return set(key, std::forward<D>(data), std::optional(timeout));
    }

    bool send() {
      std::vector<int32_t> map;
      map.reserve(_controller.geometry().num_devices());
      std::transform(_controller.geometry().cbegin(), _controller.geometry().cend(), std::back_inserter(map), [this](const geometry::Device& d) {
        if (!d.enable()) return -1;
        const auto k = _map(d);
        return k.has_value() ? _keymap[k.value()] : -1;
      });
      return validate(AUTDControllerGroup(_controller._ptr, map.data(), _kv_map)) == native_methods::AUTD3_TRUE;
    }

    [[nodiscard]] std::future<bool> send_async() {
      return std::async(std::launch::deferred, [this] { return send(); });
    }

   private:
    Controller& _controller;
    const F& _map;
    native_methods::GroupKVMapPtr _kv_map;
    std::unordered_map<key_type, int32_t> _keymap;
    int32_t _k{0};
  };

  template <group_f F>
  GroupGuard<F> group(const F& map) {
    return GroupGuard<F>(map, *this);
  }

 private:
  Controller(geometry::Geometry geometry, const native_methods::ControllerPtr ptr, L link)
      : _geometry(std::move(geometry)), _ptr(ptr), _link(std::move(link)) {}

  geometry::Geometry _geometry;
  native_methods::ControllerPtr _ptr;
  L _link;
};

/**
 * @brief Builder for Controller
 */
class ControllerBuilder {
 public:
  /**
   * @brief Add device
   *
   * @param device AUTD3 device
   * @return Builder
   */
  ControllerBuilder add_device(const geometry::AUTD3& device) {
    const auto rot = device.rotation().has_value() ? device.rotation().value() : Quaternion::Identity();
    _ptr =
        AUTDControllerBuilderAddDevice(_ptr, device.position().x(), device.position().y(), device.position().z(), rot.w(), rot.x(), rot.y(), rot.z());
    return *this;
  }

  /**
   * @brief Open controller
   *
   * @tparam B LinkBuilder
   * @param link_builder link builder
   * @return Controller
   */
  template <link_builder B>
  [[nodiscard]] Controller<typename B::Link> open_with(B&& link_builder) {
    auto ptr = validate(AUTDControllerOpenWith(_ptr, link_builder.ptr()));
    geometry::Geometry geometry(AUTDGeometry(ptr));
    return Controller<typename B::Link>{std::move(geometry), ptr, link_builder.resolve_link(native_methods::AUTDLinkGet(ptr))};
  }

  /**
   * @brief Open controller
   *
   * @tparam B LinkBuilder
   * @param link_builder link builder
   * @return Controller
   */
  template <link_builder B>
  [[nodiscard]] std::future<Controller<typename B::Link>> open_with_async(B&& link_builder) {
    return std::async(std::launch::deferred, [this, builder = std::forward<B>(link_builder)]() -> Controller<typename B::Link> {
      auto ptr = validate(AUTDControllerOpenWith(_ptr, builder.ptr()));
      geometry::Geometry geometry(AUTDGeometry(ptr));
      return Controller<typename B::Link>{std::move(geometry), ptr, builder.resolve_link(native_methods::AUTDLinkGet(ptr))};
    });
  }

  ControllerBuilder() : _ptr(native_methods::AUTDControllerBuilder()) {}

 private:
  native_methods::ControllerBuilderPtr _ptr;
};

}  // namespace autd3::internal
