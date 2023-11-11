// File: Controller.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/11/2023
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
        _ptr = AUTDControllerBuilderAddDevice(_ptr, device.position().x(), device.position().y(), device.position().z(), euler.value().x(),
                                              euler.value().y(), euler.value().z());
      else if (const auto quat = device.quaternion(); quat.has_value())
        _ptr = AUTDControllerBuilderAddDeviceQuaternion(_ptr, device.position().x(), device.position().y(), device.position().z(), quat.value().w(),
                                                        quat.value().x(), quat.value().y(), quat.value().z());
      else
        throw std::runtime_error("unreachable!");
      return *this;
    }

    /**
     * @brief Open controller
     *
     * @tparam L LinkBuilder
     * @param link link builder
     * @return Controller
     */
    template <class L>
    [[nodiscard]] Controller open_with(L&& link) {
      static_assert(std::is_base_of_v<LinkBuilder, std::remove_reference_t<L>>, "This is not Link");
      const auto [result, err_len, err] = AUTDControllerOpenWith(_ptr, link.ptr());
      if (result._0 == nullptr) {
        const std::string err_str(err_len, ' ');
        native_methods::AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
        throw AUTDException(err_str);
      }
      auto ptr = result;
      Geometry geometry(AUTDGeometry(ptr));

      return {std::move(geometry), ptr, link.props()};
    }

    /**
     * @brief Open controller
     *
     * @tparam L LinkBuilder
     * @param link link builder
     * @return Controller
     */
    template <class L>
    [[nodiscard]] std::future<Controller> open_with_async(L&& link) {
      static_assert(std::is_base_of_v<LinkBuilder, std::remove_reference_t<L>>, "This is not Link");
      return std::async(std::launch::deferred, [this, link]() -> Controller { return open_with(link); });
    }

   private:
    explicit Builder() : _ptr(native_methods::AUTDControllerBuilder()) {}

    native_methods::ControllerBuilderPtr _ptr;
  };

  /**
   * @brief Create Controller builder
   *
   * @return Builder
   */
  static Builder builder() noexcept { return Builder{}; }

  Controller() = delete;
  Controller(const Controller& v) = delete;
  Controller& operator=(const Controller& obj) = delete;
  Controller(Controller&& obj) noexcept : _geometry(std::move(obj._geometry)), _ptr(obj._ptr), _link_props(std::move(obj._link_props)) {
    obj._ptr._0 = nullptr;
  }
  Controller& operator=(Controller&& obj) noexcept {
    if (this != &obj) {
      _geometry = std::move(obj._geometry);
      _ptr = obj._ptr;
      _link_props = std::move(obj._link_props);
      obj._ptr._0 = nullptr;
    }
    return *this;
  }

  ~Controller() noexcept {
    try {
      if (_ptr._0 != nullptr) {
        AUTDControllerDelete(_ptr);
        _ptr._0 = nullptr;
      }
    } catch (std::exception&) {
    }
  }

  [[nodiscard]] const Geometry& geometry() const { return _geometry; }
  [[nodiscard]] Geometry& geometry() { return _geometry; }

  template <typename L>
  [[nodiscard]] L link() const {
    return L(internal::native_methods::AUTDLinkGet(_ptr), _link_props);
  }

  /**
   * @brief Close connection
   */
  [[nodiscard]] bool close() const {
    const auto [result, err_len, err] = AUTDControllerClose(_ptr);
    if (result == native_methods::AUTD3_ERR) {
      const std::string err_str(err_len, ' ');
      native_methods::AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
      throw AUTDException(err_str);
    }
    return result == native_methods::AUTD3_TRUE;
  }

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
    if (const auto [result, err_len, err] = AUTDControllerFPGAInfo(_ptr, info.data()); result == native_methods::AUTD3_ERR) {
      const std::string err_str(err_len, ' ');
      native_methods::AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
      throw AUTDException(err_str);
    }
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
    const auto handle = AUTDControllerFirmwareInfoListPointer(_ptr);
    if (handle.result == nullptr) {
      const std::string err_str(handle.err_len, ' ');
      native_methods::AUTDGetErr(handle.err, const_cast<char*>(err_str.c_str()));
      throw AUTDException(err_str);
    }
    std::vector<FirmwareInfo> ret;
    for (uint32_t i = 0; i < static_cast<uint32_t>(geometry().num_devices()); i++) {
      char info[256]{};
      AUTDControllerFirmwareInfoGet(handle, i, info);
      ret.emplace_back(std::string(info));
    }
    AUTDControllerFirmwareInfoListPointerDelete(handle);
    return ret;
  }

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
  template <typename D, typename Rep, typename Period>
  auto send(D&& data, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_datagram_v<D>, bool> {
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
  template <typename D, typename Rep, typename Period>
  auto send_async(D&& data, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_datagram_v<D>, std::future<bool>> {
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
  template <typename D, typename Rep = uint64_t, typename Period = std::milli>
  auto send(D&& data, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<is_datagram_v<D>, bool> {
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
  template <typename D, typename Rep = uint64_t, typename Period = std::milli>
  auto send_async(D&& data, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
      -> std::enable_if_t<is_datagram_v<D>, std::future<bool>> {
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
  template <typename D1, typename D2, typename Rep, typename Period>
  auto send(D1&& data1, D2&& data2, const std::chrono::duration<Rep, Period> timeout)
      -> std::enable_if_t<is_datagram_v<D1> && is_datagram_v<D2>, bool> {
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
  template <typename D1, typename D2, typename Rep, typename Period>
  auto send_async(D1&& data1, D2&& data2, const std::chrono::duration<Rep, Period> timeout)
      -> std::enable_if_t<is_datagram_v<D1> && is_datagram_v<D2>, std::future<bool>> {
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
  template <typename D1, typename D2, typename Rep = uint64_t, typename Period = std::milli>
  auto send(D1&& data1, D2&& data2, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
      -> std::enable_if_t<is_datagram_v<D1> && is_datagram_v<D2>, bool> {
    const int64_t timeout_ns = timeout.has_value() ? std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count() : -1;
    const auto [result, err_len, err] = AUTDControllerSend(_ptr, data1.ptr(_geometry), data2.ptr(_geometry), timeout_ns);
    if (result == native_methods::AUTD3_ERR) {
      const std::string err_str(err_len, ' ');
      native_methods::AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
      throw AUTDException(err_str);
    }
    return result == native_methods::AUTD3_TRUE;
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
  template <typename D1, typename D2, typename Rep = uint64_t, typename Period = std::milli>
  auto send_async(D1&& data1, D2&& data2, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
      -> std::enable_if_t<is_datagram_v<D1> && is_datagram_v<D2>, std::future<bool>> {
    return std::async(std::launch::deferred, [this, d1 = std::forward<D1>(data1), d2 = std::forward<D2>(data2), timeout]() -> bool {
      const int64_t timeout_ns = timeout.has_value() ? std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count() : -1;
      const auto [result, err_len, err] = AUTDControllerSend(_ptr, d1.ptr(_geometry), d2.ptr(_geometry), timeout_ns);
      if (result == native_methods::AUTD3_ERR) {
        const std::string err_str(err_len, ' ');
        native_methods::AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
        throw AUTDException(err_str);
      }
      return result == native_methods::AUTD3_TRUE;
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
  template <typename S, typename Rep, typename Period>
  auto send(S&& s, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_special_v<S>, bool> {
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
  template <typename S, typename Rep, typename Period>
  auto send_async(S&& s, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_special_v<S>, std::future<bool>> {
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
  template <typename S, typename Rep = uint64_t, typename Period = std::milli>
  auto send(S&& s, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<is_special_v<S>, bool> {
    const int64_t timeout_ns =
        timeout.has_value() ? static_cast<int64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count()) : -1;
    const auto [result, err_len, err] = native_methods::AUTDControllerSendSpecial(_ptr, s.ptr(), timeout_ns);
    if (result == native_methods::AUTD3_ERR) {
      const std::string err_str(err_len, ' ');
      native_methods::AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
      throw AUTDException(err_str);
    }
    return result == native_methods::AUTD3_TRUE;
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
  template <typename S, typename Rep = uint64_t, typename Period = std::milli>
  auto send_async(S&& s, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
      -> std::enable_if_t<is_special_v<S>, std::future<bool>> {
    return std::async(std::launch::deferred, [this, s_ = std::forward<S>(s), timeout]() -> bool {
      const int64_t timeout_ns =
          timeout.has_value() ? static_cast<int64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count()) : -1;
      const auto [result, err_len, err] = native_methods::AUTDControllerSendSpecial(_ptr, s_.ptr(), timeout_ns);
      if (result == native_methods::AUTD3_ERR) {
        const std::string err_str(err_len, ' ');
        native_methods::AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
        throw AUTDException(err_str);
      }
      return result == native_methods::AUTD3_TRUE;
    });
  }

  template <typename F>
  class GroupGuard {
   public:
    using key_type = typename std::invoke_result_t<F, const Device&>::value_type;

    explicit GroupGuard(const F& map, Controller& controller)
        : _controller(controller), _map(map), _kv_map(native_methods::AUTDControllerGroupCreateKVMap()) {}

    template <typename D, typename Rep = uint64_t, typename Period = std::milli>
    auto set(const key_type key, D&& data, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
        -> std::enable_if_t<is_datagram_v<D>, GroupGuard> {
      if (_keymap.contains(key)) throw AUTDException("Key already exists");
      const int64_t timeout_ns = timeout.has_value() ? timeout.value().count() : -1;
      const auto ptr = data.ptr(_controller._geometry);
      _keymap[key] = _k++;
      _kv_map = native_methods::AUTDControllerGroupKVMapSet(_kv_map, _keymap[key], ptr, native_methods::DatagramPtr{nullptr}, timeout_ns);
      if (_kv_map.result == nullptr) {
        const std::string err_str(_kv_map.err_len, ' ');
        native_methods::AUTDGetErr(_kv_map.err, const_cast<char*>(err_str.c_str()));
        throw AUTDException(err_str);
      }
      return std::move(*this);
    }

    template <typename D, typename Rep, typename Period>
    auto set(const key_type key, D&& data, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_datagram_v<D>, GroupGuard> {
      return set(key, std::forward<D>(data), std::optional(timeout));
    }

    template <typename D1, typename D2, typename Rep = uint64_t, typename Period = std::milli>
    auto set(const key_type key, D1&& data1, D2&& data2, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
        -> std::enable_if_t<is_datagram_v<D1> && is_datagram_v<D2>, GroupGuard> {
      if (_keymap.contains(key)) throw AUTDException("Key already exists");
      const int64_t timeout_ns = timeout.has_value() ? timeout.value().count() : -1;
      const auto ptr1 = data1.ptr(_controller._geometry);
      const auto ptr2 = data2.ptr(_controller._geometry);
      _keymap[key] = _k++;
      _kv_map = native_methods::AUTDControllerGroupKVMapSet(_kv_map, _keymap[key], ptr1, ptr2, timeout_ns);
      if (_kv_map.result == nullptr) {
        const std::string err_str(_kv_map.err_len, ' ');
        native_methods::AUTDGetErr(_kv_map.err, const_cast<char*>(err_str.c_str()));
        throw AUTDException(err_str);
      }
      return std::move(*this);
    }

    template <typename D1, typename D2, typename Rep, typename Period>
    auto set(const key_type key, D1&& data1, D2&& data2, const std::chrono::duration<Rep, Period> timeout)
        -> std::enable_if_t<is_datagram_v<D1> && is_datagram_v<D2>, GroupGuard> {
      return set(key, std::forward<D1>(data1), std::forward<D2>(data2), std::optional(timeout));
    }

    template <typename D, typename Rep = uint64_t, typename Period = std::milli>
    auto set(const key_type key, D&& data, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
        -> std::enable_if_t<is_special_v<D>, GroupGuard> {
      if (_keymap.contains(key)) throw AUTDException("Key already exists");
      const int64_t timeout_ns = timeout.has_value() ? timeout.value().count() : -1;
      const auto ptr = data.ptr();
      _keymap[key] = _k++;
      _kv_map = native_methods::AUTDControllerGroupKVMapSetSpecial(_kv_map, _keymap[key], ptr, timeout_ns);
      if (_kv_map.result == nullptr) {
        const std::string err_str(_kv_map.err_len, ' ');
        native_methods::AUTDGetErr(_kv_map.err, const_cast<char*>(err_str.c_str()));
        throw AUTDException(err_str);
      }
      return std::move(*this);
    }

    template <typename D, typename Rep, typename Period>
    auto set(const key_type key, D&& data, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_special_v<D>, GroupGuard> {
      return set(key, std::forward<D>(data), std::optional(timeout));
    }

    bool send() {
      std::vector<int32_t> map;
      map.reserve(_controller.geometry().num_devices());
      std::transform(_controller.geometry().cbegin(), _controller.geometry().cend(), std::back_inserter(map), [this](const Device& d) {
        if (!d.enable()) return -1;
        const auto k = _map(d);
        return k.has_value() ? _keymap[k.value()] : -1;
      });
      const auto [result, err_len, err] = AUTDControllerGroup(_controller._ptr, map.data(), _kv_map);
      if (result == native_methods::AUTD3_ERR) {
        const std::string err_str(err_len, ' ');
        native_methods::AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
        throw AUTDException(err_str);
      }
      return result == native_methods::AUTD3_TRUE;
    }

    [[nodiscard]] std::future<bool> send_async() {
      return std::async(std::launch::deferred, [this]() { return send(); });
    }

   private:
    Controller& _controller;
    const F& _map;
    native_methods::ResultGroupKVMap _kv_map;
    std::unordered_map<key_type, int32_t> _keymap;
    int32_t _k{0};
  };

  template <typename F>
  GroupGuard<F> group(const F& map) {
    return GroupGuard<F>(map, *this);
  }

 private:
  Controller(Geometry geometry, const native_methods::ControllerPtr ptr, std::shared_ptr<void> link_props)
      : _geometry(std::move(geometry)), _ptr(ptr), _link_props(std::move(link_props)) {}

  Geometry _geometry;
  native_methods::ControllerPtr _ptr;
  std::shared_ptr<void> _link_props;
};

}  // namespace autd3::internal
