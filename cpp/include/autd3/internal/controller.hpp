// File: controller.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2023
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
  Controller() = delete;
  Controller(const Controller& v) = default;
  Controller& operator=(const Controller& obj) = default;
  Controller(Controller&& obj) = default;
  Controller& operator=(Controller&& obj) = default;

  ~Controller() noexcept {
    try {
      if (_ptr != nullptr) {
        native_methods::AUTDFreeController(_ptr);
        _ptr = nullptr;
      }
    } catch (std::exception&) {
    }
  }

  static Controller open(const Geometry& geometry, const Link link) {
    const auto mode = geometry.mode();

    char err[256]{};
    void* ptr = native_methods::AUTDOpenController(geometry.ptr(), link.ptr(), err);
    if (ptr == nullptr) throw AUTDException(err);
    auto geometry_ = Geometry(ptr, mode);
    auto cnt = Controller(std::move(geometry_), ptr, geometry.mode());
    cnt.geometry().configure_transducers();
    return cnt;
  }

  [[nodiscard]] const Geometry& geometry() const { return _geometry; }
  [[nodiscard]] Geometry& geometry() { return _geometry; }

  void close() const {
    if (char err[256]{}; !native_methods::AUTDClose(_ptr, err)) throw AUTDException(err);
  }

  std::vector<FPGAInfo> fpga_info() {
    char err[256]{};
    const size_t num_devices = geometry().num_devices();
    std::vector<uint8_t> info(num_devices);
    if (!native_methods::AUTDGetFPGAInfo(_ptr, info.data(), err)) throw AUTDException(err);
    std::vector<FPGAInfo> ret;
    ret.reserve(num_devices);
    std::transform(info.begin(), info.end(), std::back_inserter(ret), [](const uint8_t i) { return FPGAInfo(i); });
    return ret;
  }

  [[nodiscard]] std::vector<FirmwareInfo> firmware_infos() {
    char err[256]{};
    auto* handle = native_methods::AUTDGetFirmwareInfoListPointer(_ptr, err);
    if (handle == nullptr) throw AUTDException(err);
    std::vector<FirmwareInfo> ret;
    for (uint32_t i = 0; i < static_cast<uint32_t>(geometry().num_devices()); i++) {
      char info[256]{};
      bool is_valid, is_supported;
      native_methods::AUTDGetFirmwareInfo(handle, i, info, &is_valid, &is_supported);
      ret.emplace_back(FirmwareInfo(std::string(info), is_valid, is_supported));
    }
    native_methods::AUTDFreeFirmwareInfoListPointer(handle);
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
    if (res == native_methods::ERR) throw AUTDException(err);
    return res == native_methods::TRUE;
  }

  void force_fan(const bool value) const { native_methods::AUTDSetForceFan(_ptr, value); }
  void reads_fpga_info(const bool value) const { native_methods::AUTDSetReadsFPGAInfo(_ptr, value); }

 private:
  Controller(Geometry geometry, void* ptr, const native_methods::TransMode mode) : _geometry(std::move(geometry)), _ptr(ptr), _mode(mode) {}

  bool send(Header* header, Body* body, const std::optional<std::chrono::nanoseconds> timeout) {
    char err[256]{};
    const int64_t timeout_ns = timeout.has_value() ? static_cast<int64_t>(timeout.value().count()) : -1;
    const auto res = AUTDSend(_ptr, _mode, header->ptr(), body->calc_ptr(geometry()), timeout_ns, err);
    if (res == native_methods::ERR) throw AUTDException(err);
    return res == native_methods::TRUE;
  }

  Geometry _geometry;
  void* _ptr;
  native_methods::TransMode _mode;
};

}  // namespace autd3::internal
