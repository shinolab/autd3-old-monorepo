// File: controller.hpp
// Project: autd3
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/02/2023
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

#include "autd3/core/geometry.hpp"
#include "autd3/core/link.hpp"
#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/firmware_version.hpp"
#include "autd3/driver/operation/force_fan.hpp"
#include "autd3/driver/operation/info.hpp"
#include "autd3/driver/operation/reads_fpga_info.hpp"
#include "autd3/special_data.hpp"

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

#ifdef AUTD3_CAPI
  static Controller* open(core::Geometry* geometry, core::LinkPtr link) {
    auto* cnt = new Controller(geometry, std::move(link));
    cnt->open();
    return cnt;
  }
  core::Geometry& geometry() noexcept { return *_geometry; }
  [[nodiscard]] core::Geometry* geometry_ptr() const noexcept { return _geometry; }
  [[nodiscard]] const core::Geometry& geometry() const noexcept { return *_geometry; }
#else
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
#endif

  /**
   * @brief Close the controller
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  bool close() {
    if (!is_open()) return true;
    auto res = send(stop());
    res &= send(clear());
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
      if (!_link->send_receive(_tx_buf, _rx_buf, _send_interval, std::chrono::nanoseconds(200 * 1000 * 1000))) return acks;
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
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename H>
  auto send(H& header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, bool> {
    core::NullBody b;
    return send(header, b);
  }

  /**
   * @brief Send header data to devices
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename H>
  auto send(H&& header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, bool> {
    return send(header);
  }

  /**
   * @brief Send body data to devices
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename B>
  auto send(B& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, bool> {
    core::NullHeader h;
    return send(h, body);
  }

  /**
   * @brief Send body data to devices
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename B>
  auto send(B&& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, bool> {
    return send(body);
  }

  /**
   * @brief Send header and body data to devices
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename H, typename B>
  auto send(H&& header, B&& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>, bool> {
    return send(header, body);
  }

  /**
   * @brief Send header and body data to devices
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename H, typename B>
  auto send(H& header, B& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>, bool> {
    return send(&header, &body, _ack_check_timeout);
  }

  /**
   * @brief Send header and body data to devices
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  bool send(core::DatagramHeader* header, core::DatagramBody* body, const std::chrono::high_resolution_clock::duration timeout) {
    const auto op_header = header->operation();
    const auto op_body = body->operation(geometry());

    op_header->init();
    op_body->init();

    _force_fan.pack(_tx_buf);
    _reads_fpga_info.pack(_tx_buf);

    const auto no_wait = timeout == std::chrono::high_resolution_clock::duration::zero();
    while (true) {
      const auto msg_id = get_id();
      _tx_buf.header().msg_id = msg_id;

      op_header->pack(_tx_buf);
      op_body->pack(_tx_buf);

      if (!_link->send_receive(_tx_buf, _rx_buf, _send_interval, timeout)) return false;

      if (op_header->is_finished() && op_body->is_finished()) break;
      if (no_wait) std::this_thread::sleep_for(_send_interval);
    }
    return true;
  }

  /**
   * @brief Send special data to devices
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  template <typename S>
  auto send(S s) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, bool> {
    const auto timeout = s.ack_check_timeout_override() ? s.ack_check_timeout() : _ack_check_timeout;
    auto h = s.header();
    auto b = s.body();
    const auto res = send(h.get(), b.get(), timeout);
    return res;
  }

  /**
   * @brief Send special data to devices
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  bool send(SpecialData* s) {
    const auto timeout = s->ack_check_timeout_override() ? s->ack_check_timeout() : _ack_check_timeout;
    const auto h = s->header();
    const auto b = s->body();
    return send(h.get(), b.get(), timeout);
  }

  /**
   * @brief If true, the fan will be forced to start.
   */
  bool& force_fan() noexcept { return _force_fan.value; }

  /**
   * @brief If true, the fan will be forced to start.
   */
  [[nodiscard]] bool force_fan() const noexcept { return _force_fan.value; }

  /**
   * @brief If true, the devices return FPGA info in all frames. The FPGA info can be read by fpga_info().
   */
  bool& reads_fpga_info() noexcept { return _reads_fpga_info.value; }

  /**
   * @brief If true, the devices return FPGA info in all frames. The FPGA info can be read by fpga_info().
   */
  [[nodiscard]] bool reads_fpga_info() const noexcept { return _reads_fpga_info.value; }

  /**
   * @brief Transmission interval between frames when sending multiple data.
   */
  template <typename Rep, typename Period>
  void set_send_interval(std::chrono::duration<Rep, Period> interval) {
    _send_interval = std::chrono::duration_cast<std::chrono::high_resolution_clock::duration>(interval);
  }

  /**
   * @brief Transmission interval between frames when sending multiple data.
   */
  [[nodiscard]] std::chrono::high_resolution_clock::duration get_send_interval() const noexcept { return _send_interval; }

  /**
   * @brief If > 0, this controller check ack from devices.
   */
  template <typename Rep, typename Period>
  void set_ack_check_timeout(std::chrono::duration<Rep, Period> timeout) {
    _ack_check_timeout = std::chrono::duration_cast<std::chrono::high_resolution_clock::duration>(timeout);
  }

  /**
   * @brief If > 0, this controller check ack from devices.
   */
  [[nodiscard]] std::chrono::high_resolution_clock::duration get_ack_check_timeout() const noexcept { return _ack_check_timeout; }

 private:
#ifdef AUTD3_CAPI
  explicit Controller(core::Geometry* geometry, core::LinkPtr link)
      : _geometry(geometry), _tx_buf({0}), _rx_buf(0), _link(std::move(link)), _last_send_res(false) {}
  core::Geometry* _geometry;
#else
  explicit Controller(core::Geometry geometry, core::LinkPtr link)
      : _geometry(std::move(geometry)), _tx_buf({0}), _rx_buf(0), _link(std::move(link)), _last_send_res(false) {}
  core::Geometry _geometry;
#endif

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

  std::chrono::high_resolution_clock::duration _send_interval{std::chrono::milliseconds(1)};

  std::chrono::high_resolution_clock::duration _ack_check_timeout{std::chrono::high_resolution_clock::duration::zero()};

  driver::TxDatagram _tx_buf;
  driver::RxDatagram _rx_buf;
  core::LinkPtr _link;

  bool _last_send_res;

  driver::ForceFan _force_fan;
  driver::ReadsFPGAInfo _reads_fpga_info;

 public:
  /**
   * @brief Buffer for stream operator
   * @tparam H Class inheriting from core::DatagramHeader
   */
  template <typename H>
  class StreamCommaInputHeader {
    friend class Controller;

   public:
    ~StreamCommaInputHeader() {
      if (!_sent) _cnt._last_send_res = _cnt.send(_header);
    }
    StreamCommaInputHeader(const StreamCommaInputHeader& v) noexcept = delete;
    StreamCommaInputHeader& operator=(const StreamCommaInputHeader& obj) = delete;
    StreamCommaInputHeader(StreamCommaInputHeader&& obj) = default;
    StreamCommaInputHeader& operator=(StreamCommaInputHeader&& obj) = delete;

    /**
     * @brief Send buffered core::DatagramHeader and core::DatagramBody
     * @tparam B Class inheriting from core::DatagramBody
     * @param body core::DatagramBody
     * @return Controller&
     */
    template <typename B>
    auto operator,(B&& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, std::remove_reference_t<B>>, Controller&> {
      _cnt._last_send_res = _cnt.send(_header, body);
      _sent = true;
      return _cnt;
    }

    /**
     * @brief Send buffered core::DatagramHeader and core::DatagramBody
     * @tparam B Class inheriting from core::DatagramBody
     * @param body core::DatagramBody
     * @return Controller&
     */
    template <typename B>
    auto operator<<(B&& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, std::remove_reference_t<B>>, Controller&> {
      _cnt._last_send_res = _cnt.send(_header, body);
      _sent = true;
      return _cnt;
    }

    /**
     * @brief Send buffered core::DatagramHeader and buffer core::DatagramHeader passed as argument
     * @tparam H2 Class inheriting from core::DatagramHeader
     * @param header core::DatagramHeader
     * @return StreamCommaInputHeader
     */
    template <typename H2>
    auto operator<<(H2&& header)
        -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, std::remove_reference_t<H2>>, StreamCommaInputHeader<H2>> {
      _cnt._last_send_res = _cnt.send(_header);
      _sent = true;
      return StreamCommaInputHeader<H2>(_cnt, header);
    }

    /**
     * @brief Send buffered core::DatagramHeader and SpecialData
     * @tparam S Class inheriting from SpecialData
     * @param special_f SpecialData function
     * @return Controller&
     */
    template <typename S>
    auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, Controller&> {
      _cnt._last_send_res = _cnt.send(_header);
      _sent = true;
      _cnt._last_send_res = _cnt.send(special_f());
      return _cnt;
    }

    auto operator<<(const core::DatagramPackRef pack) -> Controller& {
      _cnt._last_send_res = _cnt.send(_header);
      _sent = true;
      _cnt._last_send_res = _cnt.send(pack.header, pack.body);
      return _cnt;
    }

    /**
     * @brief Send buffered core::DatagramHeader and then send core::DatagramHeader and core::DatagramBody in DatagramPack
     * @tparam H2 Class inheriting from core::DatagramHeader
     * @tparam B2 Class inheriting from core::DatagramBody
     * @param pack core::DatagramPack
     * @return Controller&
     */
    template <typename H2, typename B2>
    auto operator<<(core::DatagramPack<H2, B2>&& pack)
        -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H2> && std::is_base_of_v<core::DatagramBody, B2>, Controller&> {
      _cnt._last_send_res = _cnt.send(_header);
      _sent = true;
      _cnt._last_send_res = _cnt.send(pack.header, pack.body);
      return _cnt;
    }

   private:
    explicit StreamCommaInputHeader(Controller& cnt, H& header) : _cnt(cnt), _header(header), _sent(false) {}

    Controller& _cnt;
    H& _header;
    bool _sent;
  };

  /**
   * @brief Buffer for stream operator
   * @tparam B Class inheriting from core::DatagramBody
   */
  template <typename B>
  class StreamCommaInputBody {
    friend class Controller;

   public:
    ~StreamCommaInputBody() {
      if (!_sent) _cnt._last_send_res = _cnt.send(_body);
    }
    StreamCommaInputBody(const StreamCommaInputBody& v) noexcept = delete;
    StreamCommaInputBody& operator=(const StreamCommaInputBody& obj) = delete;
    StreamCommaInputBody(StreamCommaInputBody&& obj) = default;
    StreamCommaInputBody& operator=(StreamCommaInputBody&& obj) = delete;

    /**
     * @brief Send buffered core::DatagramBody and core::DatagramHeader
     * @tparam H Class inheriting from core::DatagramHeader
     * @param header core::DatagramHeader
     * @return Controller&
     */
    template <typename H>
    auto operator,(H&& header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, std::remove_reference_t<H>>, Controller&> {
      _cnt._last_send_res = _cnt.send(header, _body);
      _sent = true;
      return _cnt;
    }

    /**
     * @brief Send buffered core::DatagramBody and core::DatagramHeader
     * @tparam H Class inheriting from core::DatagramHeader
     * @param header core::DatagramHeader
     * @return Controller&
     */
    template <typename H>
    auto operator<<(H&& header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, std::remove_reference_t<H>>, Controller&> {
      _cnt._last_send_res = _cnt.send(header, _body);
      _sent = true;
      return _cnt;
    }

    /**
     * @brief Send buffered core::DatagramBody and buffer core::DatagramBody passed as argument
     * @tparam B2 Class inheriting from core::DatagramBody
     * @param body core::DatagramBody
     * @return StreamCommaInputBody<B2>
     */
    template <typename B2>
    auto operator<<(B2&& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, std::remove_reference_t<B2>>, StreamCommaInputBody<B2>> {
      _cnt._last_send_res = _cnt.send(_body);
      _sent = true;
      return StreamCommaInputBody<B2>(_cnt, body);
    }

    /**
     * @brief Send buffered core::DatagramBody and SpecialData
     * @tparam S Class inheriting from SpecialData
     * @param special_f SpecialData function
     * @return Controller&
     */
    template <typename S>
    auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, Controller&> {
      _cnt._last_send_res = _cnt.send(_body);
      _sent = true;
      _cnt._last_send_res = _cnt.send(special_f());
      return _cnt;
    }

    /**
     * @brief Send buffered core::DatagramBody and then send core::DatagramHeader and core::DatagramBody in DatagramPackRef
     * @param pack core::DatagramPackRef
     * @return Controller&
     */
    Controller& operator<<(const core::DatagramPackRef pack) {
      _cnt._last_send_res = _cnt.send(_body);
      _sent = true;
      _cnt._last_send_res = _cnt.send(pack.header, pack.body);
      return _cnt;
    }

    /**
     * @brief Send buffered core::DatagramBody and then send core::DatagramHeader and core::DatagramBody in DatagramPack
     * @tparam H2 Class inheriting from core::DatagramHeader
     * @tparam B2 Class inheriting from core::DatagramBody
     * @param pack core::DatagramPack
     * @return Controller&
     */
    template <typename H2, typename B2>
    auto operator<<(core::DatagramPack<H2, B2>&& pack)
        -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H2> && std::is_base_of_v<core::DatagramBody, B2>, Controller&> {
      _cnt._last_send_res = _cnt.send(_body);
      _sent = true;
      _cnt._last_send_res = _cnt.send(pack.header, pack.body);
      return _cnt;
    }

   private:
    explicit StreamCommaInputBody(Controller& cnt, B& body) : _cnt(cnt), _body(body), _sent(false) {}

    Controller& _cnt;
    B& _body;
    bool _sent;
  };

  /**
   * @brief Buffer core::DatagramHeader
   * @tparam H Class inheriting from core::DatagramHeader
   * @param header core::DatagramHeader
   * @return StreamCommaInputHeader<H>
   */
  template <typename H>
  auto operator<<(H&& header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, std::remove_reference_t<H>>, StreamCommaInputHeader<H>> {
    return StreamCommaInputHeader<H>(*this, header);
  }

  /**
   * @brief Buffer core::DatagramBody
   * @tparam B Class inheriting from core::DatagramBody
   * @param body core::DatagramBody
   * @return StreamCommaInputBody
   */
  template <typename B>
  auto operator<<(B&& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, std::remove_reference_t<B>>, StreamCommaInputBody<B>> {
    return StreamCommaInputBody<B>(*this, body);
  }

  /**
   * @brief Send core::DatagramHeader and core::DatagramBody in core::DatagramPackRef
   * @param pack core::DatagramPackRef
   * @return Controller&
   */
  Controller& operator<<(const core::DatagramPackRef pack) {
    _last_send_res = send(pack.header, pack.body);
    return *this;
  }

  /**
   * @brief Send core::DatagramHeader and core::DatagramBody in core::DatagramPack
   * @tparam H Class inheriting from core::DatagramHeader
   * @tparam B Class inheriting from core::DatagramBody
   * @param pack core::DatagramPack
   * @return Controller&
   */
  template <typename H, typename B>
  auto operator<<(core::DatagramPack<H, B>&& pack)
      -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>, Controller&> {
    _last_send_res = send(pack.header, pack.body);
    return *this;
  }

  /**
   * @brief Send SpecialData
   * @tparam S Class inheriting from SpecialData
   * @param special_f SpecialData function
   * @return Controller&
   */
  template <typename S>
  auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, Controller&> {
    _last_send_res = send(special_f());
    return *this;
  }

  /**
   * @brief Set Mode
   * @param f mode function
   */
  void operator<<(core::Mode (*f)()) { geometry().mode = f(); }

  void operator>>(bool& res) const { res = _last_send_res; }
};

}  // namespace autd3
