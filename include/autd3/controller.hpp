// File: controller.hpp
// Project: autd3
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <atomic>
#include <chrono>
#include <memory>
#include <type_traits>
#include <vector>

#include "autd3/core/clear.hpp"
#include "autd3/core/datagram.hpp"
#include "autd3/core/geometry.hpp"
#include "autd3/core/link.hpp"
#include "autd3/core/stop.hpp"
#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/firmware_version.hpp"
#include "autd3/driver/operation/force_fan.hpp"
#include "autd3/driver/operation/info.hpp"
#include "autd3/driver/operation/reads_fpga_info.hpp"

namespace autd3 {

/**
 * @brief AUTD Controller
 */
class Controller {
 public:
  Controller(const Controller& v) = delete;
  Controller& operator=(const Controller& obj) = delete;
  Controller(Controller&& obj) = default;
  Controller& operator=(Controller&& obj) = default;
  ~Controller() noexcept {
    try {
      close();
    } catch (std::exception&) {
    }
  }

  static Controller open(core::Geometry geometry, core::LinkPtr link) {
    Controller cnt(std::move(geometry), std::move(link));
    cnt.open();
    return cnt;
  }

  /**
   * @brief Geometry of the devices
   */
  core::Geometry& geometry() noexcept { return _geometry; }

  /**
   * @brief Geometry of the devices
   */
  [[nodiscard]] const core::Geometry& geometry() const noexcept { return _geometry; }

  /**
   * @brief Close the controller
   * \return if true, it guarantees that the devices have processed the data.
   */
  bool close() {
    if (!is_open()) return true;
    auto res = send(core::Stop());
    res &= send(core::Clear());
    res &= _link->close();
    return res;
  }

  /**
   * @brief Verify the device is properly connected
   */
  [[nodiscard]] bool is_open() const noexcept { return _link != nullptr && _link->is_open(); }

  /**
   * @brief FPGA info
   *  \return vector of FPGAInfo. If failed, the vector is empty
   */
  std::vector<driver::FPGAInfo> fpga_info() {
    std::vector<driver::FPGAInfo> fpga_info;
    if (!_link->receive(_rx_buf)) return fpga_info;
    std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(fpga_info),
                   [](const driver::RxMessage& rx) { return driver::FPGAInfo(rx.ack); });
    return fpga_info;
  }

  /**
   * @brief Enumerate firmware information
   * \return vector of driver::FirmwareInfo
   */
  [[nodiscard]] std::vector<driver::FirmwareInfo> firmware_infos() {
    std::vector<driver::FirmwareInfo> firmware_infos;

    const auto pack_ack = [&]() -> std::vector<uint8_t> {
      std::vector<uint8_t> acks;
      if (!_link->send_receive(_tx_buf, _rx_buf, std::chrono::duration_cast<core::Duration>(core::Milliseconds(200)))) return acks;
      std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(acks), [](const driver::RxMessage msg) noexcept { return msg.ack; });
      return acks;
    };

    driver::CPUVersionMajor::pack(_tx_buf);
    const auto cpu_versions = pack_ack();
    if (cpu_versions.empty()) throw std::runtime_error("Failed to get firmware information.");

    driver::FPGAVersionMajor::pack(_tx_buf);
    const auto fpga_versions = pack_ack();
    if (fpga_versions.empty()) throw std::runtime_error("Failed to get firmware information.");

    driver::FPGAFunctions::pack(_tx_buf);
    const auto fpga_functions = pack_ack();
    if (fpga_functions.empty()) throw std::runtime_error("Failed to get firmware information.");

    driver::CPUVersionMinor::pack(_tx_buf);
    auto cpu_versions_minor = pack_ack();
    if (cpu_versions_minor.empty()) cpu_versions_minor.resize(cpu_versions.size(), 0);

    driver::FPGAVersionMinor::pack(_tx_buf);
    auto fpga_versions_minor = pack_ack();
    if (fpga_versions_minor.empty()) fpga_versions_minor.resize(fpga_versions.size(), 0);

    for (size_t i = 0; i < cpu_versions.size(); i++)
      firmware_infos.emplace_back(i, cpu_versions[i], cpu_versions_minor[i], fpga_versions[i], fpga_versions_minor[i], fpga_functions[i]);

    return firmware_infos;
  }

  /**
   * @brief Send header data to devices
   * @param[in] header header data
   * @param[in] timeout Timeout per frame
   * \return if this function returns true and timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename H, typename Rep, typename Period>
  auto send(H&& header, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<core::is_header_v<H>, bool> {
    return send(std::forward<H>(header), std::optional(timeout));
  }

  /**
   * @brief Send header data to devices
   * @param[in] header header data
   * @param[in] timeout Timeout per frame (default: none)
   * \return if this function returns true and timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename H, typename Rep = uint64_t, typename Period = std::milli>
  auto send(H&& header, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
      -> std::enable_if_t<core::is_header_v<H>, bool> {
    return send(std::forward<H>(header), core::NullBody(), timeout);
  }

  /**
   * @brief Send body data to devices
   * @param[in] body body data
   * @param[in] timeout Timeout per frame
   * \return if this function returns true and timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename B, typename Rep, typename Period>
  auto send(B&& body, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<core::is_body_v<B>, bool> {
    return send(std::forward<B>(body), std::optional(timeout));
  }

  /**
   * @brief Send body data to devices
   * @param[in] body body data
   * @param[in] timeout Timeout per frame (default: none)
   * \return if this function returns true and timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename B, typename Rep = uint64_t, typename Period = std::milli>
  auto send(B&& body, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<core::is_body_v<B>, bool> {
    return send(core::NullHeader(), std::forward<B>(body), timeout);
  }

  /**
   * @brief Send header and body data to devices
   * @param[in] header header data
   * @param[in] body body data
   * @param[in] timeout Timeout per frame
   * \return if this function returns true and timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename H, typename B, typename Rep, typename Period>
  auto send(H&& header, B&& body, const std::chrono::duration<Rep, Period> timeout)
      -> std::enable_if_t<core::is_header_v<H> && core::is_body_v<B>, bool> {
    return send(std::forward<H>(header), std::forward<B>(body), std::optional(timeout));
  }

  /**
   * @brief Send header and body data to devices
   * @param[in] header header data
   * @param[in] body body data
   * @param[in] timeout Timeout per frame (default: none)
   * \return if this function returns true and timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename H, typename B, typename Rep = uint64_t, typename Period = std::milli>
  auto send(H&& header, B&& body, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt)
      -> std::enable_if_t<core::is_header_v<H> && core::is_body_v<B>, bool> {
    return send(&header, &body, timeout);
  }

  /**
   * @brief Send special data to devices
   * @param[in] s special data
   * @param[in] timeout Timeout per frame
   * \return if this function returns true timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename S, typename Rep, typename Period>
  auto send(S&& s, const std::chrono::duration<Rep, Period> timeout) -> std::enable_if_t<core::is_special_v<S>, bool> {
    return send(std::forward<S>(s), std::optional(timeout));
  }

  /**
   * @brief Send special data to devices
   * @param[in] s special data
   * @param[in] timeout Timeout per frame (default: none)
   * \return if this function returns true timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename S, typename Rep = uint64_t, typename Period = std::milli>
  auto send(S&& s, const std::optional<std::chrono::duration<Rep, Period>> timeout = std::nullopt) -> std::enable_if_t<core::is_special_v<S>, bool> {
    return send(&s, timeout);
  }

  /**
   * @brief Send special data to devices
   * @param[in] s special data
   * @param[in] timeout Timeout per frame
   * \return if this function returns true timeout > 0, it guarantees that the devices have processed the data.
   */
  auto send(core::SpecialData* s, const std::optional<core::Duration> timeout) -> bool {
    const auto h = s->header();
    const auto b = s->body();
    const auto t = timeout ? timeout : s->min_timeout();
    return send(h.get(), b.get(), t);
  }

  /**
   * @brief Send header and body data to devices
   * @param[in] header header data
   * @param[in] body body data
   * @param[in] timeout Timeout per frame
   * \return if this function returns true and timeout > 0, it guarantees that the devices have processed the data.
   */
  bool send(core::DatagramHeader* header, core::DatagramBody* body, const std::optional<core::Duration> timeout) {
    const auto op_header = header->operation();
    const auto op_body = body->operation(geometry());

    op_header->init();
    op_body->init();

    _force_fan.pack(_tx_buf);
    _reads_fpga_info.pack(_tx_buf);

    const auto no_wait = timeout.value_or(_link->timeout()) == core::Duration::zero();
    while (true) {
      const auto msg_id = get_id();
      _tx_buf.header().msg_id = msg_id;

      op_header->pack(_tx_buf);
      op_body->pack(_tx_buf);

      if (!_link->send_receive(_tx_buf, _rx_buf, timeout)) return false;

      if (op_header->is_finished() && op_body->is_finished()) break;
      if (no_wait) std::this_thread::sleep_for(core::Milliseconds(1));
    }
    return true;
  }

  /**
   * @brief If true, the fan will be forced to start.
   */
  void force_fan(const bool value) noexcept { _force_fan.value = value; }

  /**
   * @brief If true, the devices return FPGA info in all frames. The FPGA info can be read by fpga_info().
   */
  void reads_fpga_info(const bool value) noexcept { _reads_fpga_info.value = value; }

 private:
  explicit Controller(core::Geometry geometry, core::LinkPtr link)
      : _geometry(std::move(geometry)), _tx_buf({0}), _rx_buf(0), _link(std::move(link)) {}
  core::Geometry _geometry;

  void open() {
    if (geometry().num_transducers() == 0) throw std::runtime_error("Please add devices before opening.");
    if (_link == nullptr) throw std::runtime_error("link is null");
    if (!_link->open(geometry())) throw std::runtime_error("Failed to open link.");
    _tx_buf = driver::TxDatagram(geometry().device_map());
    _rx_buf = driver::RxDatagram(geometry().num_devices());
  }

  static uint8_t get_id() noexcept {
    static std::atomic id_body{driver::MSG_BEGIN};
    if (uint8_t expected = driver::MSG_END; !id_body.compare_exchange_weak(expected, driver::MSG_BEGIN)) id_body.fetch_add(0x01);
    return id_body.load();
  }

  driver::TxDatagram _tx_buf;
  driver::RxDatagram _rx_buf;
  core::LinkPtr _link;

  driver::ForceFan _force_fan;
  driver::ReadsFPGAInfo _reads_fpga_info;
};

}  // namespace autd3
