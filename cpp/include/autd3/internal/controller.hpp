// File: controller.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/06/2023
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
class Controller {
 public:
  class Builder {
    friend class Controller;

   public:
    Builder add_device(const AUTD3& device) {
      if (const auto euler = device.euler(); euler.has_value()) {
        _ptr = AUTDAddDevice(_ptr, device.position().x(), device.position().y(), device.position().z(), euler.value().x(), euler.value().y(),
                             euler.value().z());
      } else {
        if (const auto quat = device.quaternion(); quat.has_value())
          _ptr = AUTDAddDeviceQuaternion(_ptr, device.position().x(), device.position().y(), device.position().z(), quat.value().w(),
                                         quat.value().x(), quat.value().y(), quat.value().z());
        else {
          throw std::runtime_error("unreachable!");
        }
      }
      return *this;
    }

    Builder legacy_mode() {
      _mode = native_methods::TransMode::Legacy;
      return *this;
    }

    Builder advanced_mode() {
      _mode = native_methods::TransMode::Advanced;
      return *this;
    }

    Builder advanced_phase_mode() {
      _mode = native_methods::TransMode::AdvancedPhase;
      return *this;
    }

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

  static Builder builder() noexcept { return {}; }

  Controller() = delete;
  Controller(const Controller& v) = default;
  Controller& operator=(const Controller& obj) = default;
  Controller(Controller&& obj) = default;
  Controller& operator=(Controller&& obj) = default;

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

  void close() const {
    if (char err[256]{}; !AUTDClose(_ptr, err)) throw AUTDException(err);
  }

  std::vector<FPGAInfo> fpga_info() {
    char err[256]{};
    const size_t num_devices = geometry().num_devices();
    std::vector<uint8_t> info(num_devices);
    if (!AUTDGetFPGAInfo(_ptr, info.data(), err)) throw AUTDException(err);
    std::vector<FPGAInfo> ret;
    ret.reserve(num_devices);
    std::transform(info.begin(), info.end(), std::back_inserter(ret), [](const uint8_t i) { return FPGAInfo(i); });
    return ret;
  }

  [[nodiscard]] std::vector<FirmwareInfo> firmware_infos() {
    char err[256]{};
    const auto handle = AUTDGetFirmwareInfoListPointer(_ptr, err);
    if (handle._0 == nullptr) throw AUTDException(err);
    std::vector<FirmwareInfo> ret;
    for (uint32_t i = 0; i < static_cast<uint32_t>(geometry().num_devices()); i++) {
      char info[256]{};
      bool is_valid, is_supported;
      AUTDGetFirmwareInfo(handle, i, info, &is_valid, &is_supported);
      ret.emplace_back(std::string(info), is_valid, is_supported);
    }
    AUTDFreeFirmwareInfoListPointer(handle);
    return ret;
  }

  template <typename H, typename Rep, typename Period>
  auto send(H&& header, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_header_v<H>, bool> {
    return send(std::forward<H>(header), std::optional(timeout));
  }

  template <typename H, typename Rep = uint64_t, typename Period = std::milli>
  auto send(H&& header, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<is_header_v<H>, bool> {
    return send(std::forward<H>(header), NullBody(), timeout);
  }

  template <typename B, typename Rep, typename Period>
  auto send(B&& body, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_body_v<B>, bool> {
    return send(std::forward<B>(body), std::optional(timeout));
  }

  template <typename B, typename Rep = uint64_t, typename Period = std::milli>
  auto send(B&& body, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<is_body_v<B>, bool> {
    return send(NullHeader(), std::forward<B>(body), timeout);
  }

  template <typename H, typename B, typename Rep, typename Period>
  auto send(H&& header, B&& body, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_header_v<H> && is_body_v<B>, bool> {
    return send(std::forward<H>(header), std::forward<B>(body), std::optional(timeout));
  }

  template <typename H, typename B, typename Rep = uint64_t, typename Period = std::milli>
  auto send(H&& header, B&& body, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
      -> std::enable_if_t<is_header_v<H> && is_body_v<B>, bool> {
    return send(&header, &body, timeout);
  }

  template <typename S, typename Rep, typename Period>
  auto send(S&& s, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<is_special_v<S>, bool> {
    return send(std::forward<S>(s), std::optional(timeout));
  }

  template <typename S, typename Rep = uint64_t, typename Period = std::milli>
  auto send(S&& s, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<is_special_v<S>, bool> {
    char err[256]{};
    const int64_t timeout_ns =
        timeout.has_value() ? static_cast<int64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(timeout.value()).count()) : -1;
    const auto res = native_methods::AUTDSendSpecial(_ptr, _mode, s.ptr(), timeout_ns, err);
    if (res == native_methods::AUTD3_ERR) throw AUTDException(err);
    return res == native_methods::AUTD3_TRUE;
  }

  void force_fan(const bool value) const { AUTDSetForceFan(_ptr, value); }
  void reads_fpga_info(const bool value) const { AUTDSetReadsFPGAInfo(_ptr, value); }

 private:
  static Controller open_impl(const native_methods::ControllerBuilderPtr builder, const native_methods::TransMode mode,
                              const native_methods::LinkPtr link) {
    char err[256]{};

    const auto ptr = AUTDControllerOpenWith(builder, link, err);
    if (ptr._0 == nullptr) throw AUTDException(err);

    auto geometry = Geometry(AUTDGetGeometry(ptr), mode);

    auto cnt = Controller(std::move(geometry), ptr, mode);

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
