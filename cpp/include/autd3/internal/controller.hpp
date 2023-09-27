// File: Controller.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
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
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {
class Audit;
}

namespace autd3::internal {

/**
 * @brief Controller class for AUTD3
 */
class Controller {
 public:
  friend class link::Audit;

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

    Builder legacy() {
      _mode = native_methods::TransMode::Legacy;
      return *this;
    }

    Builder advanced() {
      _mode = native_methods::TransMode::Advanced;
      return *this;
    }

    Builder advanced_phase() {
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
    explicit Builder() : _ptr(native_methods::AUTDControllerBuilder()), _mode(native_methods::TransMode::Legacy) {}

    native_methods::ControllerBuilderPtr _ptr;
    native_methods::TransMode _mode;
  };

  /**
   * @brief Create Controller builder (legacy mode)
   *
   * @return Builder
   */
  static Builder builder() noexcept { return Builder{}; }

  Controller() = delete;
  Controller(const Controller& v) = delete;
  Controller& operator=(const Controller& obj) = delete;
  Controller(Controller&& obj) noexcept : _geometry(std::move(obj._geometry)), _ptr(obj._ptr), _mode(obj._mode) { obj._ptr._0 = nullptr; }
  Controller& operator=(Controller&& obj) noexcept {
    if (this != &obj) {
      if (_ptr._0 != nullptr) AUTDControllerDelete(_ptr);

      _geometry = std::move(obj._geometry);
      _ptr = obj._ptr;
      _mode = obj._mode;
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

  /**
   * @brief Close connection
   */
  void close() const {
    if (char err[256]{}; !AUTDControllerClose(_ptr, err)) throw AUTDException(err);
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
    if (!AUTDControllerFPGAInfo(_ptr, info.data(), err)) throw AUTDException(err);
    std::vector<FPGAInfo> ret;
    ret.reserve(num_devices);
    std::ranges::transform(info, std::back_inserter(ret), [](const uint8_t i) { return FPGAInfo(i); });
    return ret;
  }

  /**
   * @brief Get firmware information
   *
   * @return List of firmware information
   */
  [[nodiscard]] std::vector<FirmwareInfo> firmware_infos() {
    char err[256]{};
    const auto handle = AUTDControllerFirmwareInfoListPointer(_ptr, err);
    if (handle._0 == nullptr) throw AUTDException(err);
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
  template <typename D, typename Rep = uint64_t, typename Period = std::milli>
  auto send(D&& data, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<is_datagram_v<D>, bool> {
    return send(std::forward<D>(data), NullDatagram(), timeout);
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
  template <typename D1, typename D2, typename Rep = uint64_t, typename Period = std::milli>
  auto send(D1&& data1, D2&& data2, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
      -> std::enable_if_t<is_datagram_v<D1> && is_datagram_v<D2>, bool> {
    return send(&data1, &data2, timeout);
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
  template <typename S, typename Rep = uint64_t, typename Period = std::milli>
  auto send(S&& s, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<is_special_v<S>, bool> {
    char err[256]{};
    const int64_t timeout_ns =
        timeout.has_value() ? static_cast<int64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count()) : -1;
    const auto res = native_methods::AUTDControllerSendSpecial(_ptr, _mode, s.ptr(), timeout_ns, err);
    if (res == native_methods::AUTD3_ERR) throw AUTDException(err);
    return res == native_methods::AUTD3_TRUE;
  }

  template <typename F>
  class SoftwareSTM {
    friend class Controller;
    using software_stm_callback = bool (*)(void*, uint64_t, uint64_t);

    struct Context {
      Context(Controller& controller, const F& callback) : controller(controller), callback(callback) {}

      Controller& controller;
      const F& callback;
    };

   public:
    template <typename Rep, typename Period>
    void start(const std::chrono::duration<Rep, Period> interval) {
      const auto interval_ns = static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(interval).count());
      const software_stm_callback callback_native = +[](void* context, const uint64_t idx, const uint64_t time) -> bool {
        auto* c = static_cast<Context*>(context);
        return c->callback(c->controller, static_cast<size_t>(idx), std::chrono::nanoseconds(time));
      };
      if (char err[256]{}; native_methods::AUTDControllerSoftwareSTM(_ptr, reinterpret_cast<void*>(callback_native), _context.get(), _strategy,
                                                                     interval_ns, err) == native_methods::AUTD3_ERR)
        throw AUTDException(err);
    }

    SoftwareSTM with_timer_strategy(const native_methods::TimerStrategy strategy) && {
      _strategy = strategy;
      return std::move(*this);
    }

   private:
    explicit SoftwareSTM(const native_methods::ControllerPtr ptr, std::unique_ptr<Context> context)
        : _ptr(ptr), _context(std::move(context)), _strategy(native_methods::TimerStrategy::Sleep) {}

    native_methods::ControllerPtr _ptr;
    std::unique_ptr<Context> _context;
    native_methods::TimerStrategy _strategy;
  };

  template <typename F>
  SoftwareSTM<F> software_stm(const F& callback) {
    return SoftwareSTM<F>(_ptr, std::make_unique<typename SoftwareSTM<F>::Context>(*this, callback));
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
      char err[256]{};
      _kv_map = native_methods::AUTDControllerGroupKVMapSet(_kv_map, _keymap[key], ptr, native_methods::DatagramPtr{nullptr}, _controller._mode,
                                                            timeout_ns, err);
      if (_kv_map._0 == nullptr) throw AUTDException(err);
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
      char err[256]{};
      _kv_map = native_methods::AUTDControllerGroupKVMapSet(_kv_map, _keymap[key], ptr1, ptr2, _controller._mode, timeout_ns, err);
      if (_kv_map._0 == nullptr) throw AUTDException(err);
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
      char err[256]{};
      _kv_map = native_methods::AUTDControllerGroupKVMapSetSpecial(_kv_map, _keymap[key], ptr, _controller._mode, timeout_ns, err);
      if (_kv_map._0 == nullptr) throw AUTDException(err);
      return std::move(*this);
    }

    template <typename D, typename Rep, typename Period>
    auto set(const key_type key, D&& data, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_special_v<D>, GroupGuard> {
      return set(key, std::forward<D>(data), std::optional(timeout));
    }

    void send() {
      std::vector<int32_t> map;
      map.reserve(_controller.geometry().num_devices());
      std::transform(_controller.geometry().cbegin(), _controller.geometry().cend(), std::back_inserter(map), [this](const Device& d) {
        const auto k = _map(d);
        return k.has_value() ? _keymap[k.value()] : -1;
      });
      if (char err[256]{}; AUTDControllerGroup(_controller._ptr, map.data(), _kv_map, err) == native_methods::AUTD3_ERR) throw AUTDException(err);
    }

   private:
    Controller& _controller;
    const F& _map;
    native_methods::GroupKVMapPtr _kv_map;
    std::unordered_map<key_type, int32_t> _keymap;
    int32_t _k{0};
  };

  template <typename F>
  GroupGuard<F> group(const F& map) {
    return GroupGuard<F>(map, *this);
  }

 private:
  static Controller open_impl(const native_methods::ControllerBuilderPtr builder, const native_methods::TransMode mode,
                              const native_methods::LinkPtr link) {
    char err[256]{};

    const auto ptr = AUTDControllerOpenWith(builder, link, err);
    if (ptr._0 == nullptr) throw AUTDException(err);

    Geometry geometry(AUTDGeometry(ptr), mode);

    return {std::move(geometry), ptr, mode};
  }

  Controller(Geometry geometry, const native_methods::ControllerPtr ptr, const native_methods::TransMode mode)
      : _geometry(std::move(geometry)), _ptr(ptr), _mode(mode) {}

  bool send(const Datagram* d1, const Datagram* d2, const std::optional<std::chrono::nanoseconds> timeout) const {
    char err[256]{};
    const int64_t timeout_ns = timeout.has_value() ? timeout.value().count() : -1;
    const auto res = AUTDControllerSend(_ptr, _mode, d1->ptr(_geometry), d2->ptr(_geometry), timeout_ns, err);
    if (res == native_methods::AUTD3_ERR) throw AUTDException(err);
    return res == native_methods::AUTD3_TRUE;
  }

  Geometry _geometry;
  native_methods::ControllerPtr _ptr;
  native_methods::TransMode _mode;
};

}  // namespace autd3::internal
