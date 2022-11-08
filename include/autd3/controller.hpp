// File: controller.hpp
// Project: autd3
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 09/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <atomic>
#include <chrono>
#include <condition_variable>
#include <memory>
#include <mutex>
#include <queue>
#include <thread>
#include <type_traits>
#include <utility>
#include <vector>

#include "autd3/async.hpp"
#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "autd3/gain/primitive.hpp"
#include "autd3/special_data.hpp"
#include "core/amplitudes.hpp"
#include "core/geometry.hpp"
#include "core/interface.hpp"
#include "core/link.hpp"
#include "core/silencer_config.hpp"
#include "driver/firmware_version.hpp"

namespace autd3 {

/**
 * @brief AUTD Controller
 */
class Controller {
 public:
  Controller()
      : force_fan(false),
        reads_fpga_info(false),
        check_trials(0),
        send_interval(1),
        _geometry(),
        _tx_buf(0),
        _rx_buf(0),
        _link(nullptr),
        _send_th_running(false),
        _last_send_res(false) {}
  ~Controller() noexcept {
    try {
      close();
    } catch (std::exception&) {
    }
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
   * @brief Mode
   */
  [[nodiscard]] std::unique_ptr<core::Mode>& mode() noexcept { return _geometry.mode(); }

  bool open(core::LinkPtr link) {
    _tx_buf = driver::TxDatagram(_geometry.num_devices());
    _rx_buf = driver::RxDatagram(_geometry.num_devices());

    _link = std::move(link);
    if (_link != nullptr) _link->open(_geometry);

    _send_th_running = true;
    _send_th = std::thread([this] {
      std::unique_ptr<core::DatagramHeader> header = nullptr;
      std::unique_ptr<core::DatagramBody> body = nullptr;
      std::function<void(void)> pre;
      std::function<void(void)> post;
      while (_send_th_running) {
        if (header == nullptr && body == nullptr) {
          std::unique_lock lk(_send_mtx);
          _send_cond.wait(lk, [&] { return !_send_queue.empty() || !this->_send_th_running; });
          if (!this->_send_th_running) break;
          AsyncData a = std::move(_send_queue.front());
          header = std::move(a.header);
          body = std::move(a.body);
          pre = std::move(a.pre);
          post = std::move(a.post);
          _send_queue.pop();
        }

        pre();

        header->init();
        body->init();

        driver::force_fan(_tx_buf, force_fan);
        driver::reads_fpga_info(_tx_buf, reads_fpga_info);

        while (true) {
          const auto msg_id = get_id();
          header->pack(msg_id, _tx_buf);
          body->pack(_geometry, _tx_buf);
          _link->send(_tx_buf);
          const auto trials = wait_msg_processed(check_trials);
          if ((check_trials != 0) && (trials == check_trials)) break;
          if (header->is_finished() && body->is_finished()) {
            header = nullptr;
            body = nullptr;
            break;
          }
          if (trials == 0) std::this_thread::sleep_for(std::chrono::microseconds(send_interval * driver::EC_CYCLE_TIME_BASE_MICRO_SEC));
        }

        post();
      }
    });

    return is_open();
  }

  /**
   * @brief Close the controller
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  bool close() {
    if (!is_open()) return true;

    _send_th_running = false;
    _send_cond.notify_all();
    if (_send_th.joinable()) _send_th.join();

    if (!send(autd3::Stop{})) return false;
    if (!send(autd3::Clear{})) return false;
    _link->close();
    return true;
  }

  /**
   * @brief Verify the device is properly connected
   */
  [[nodiscard]] bool is_open() const noexcept { return (_link != nullptr) && _link->is_open(); }

  /**
   * @brief FPGA info
   *  \return veetor of FPGAInfo. If failed, the vector is empty
   */
  std::vector<driver::FPGAInfo> read_fpga_info() {
    std::vector<driver::FPGAInfo> fpga_info;
    if (!_link->receive(_rx_buf)) return fpga_info;
    std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(fpga_info),
                   [](const driver::RxMessage& rx) { return driver::FPGAInfo(rx.ack); });
    return fpga_info;
  }

  /**
   * @brief Enumerate firmware information
   * \return vector of driver::FirmwareInfo. If failed, the vector is empty.
   */
  [[nodiscard]] std::vector<driver::FirmwareInfo> firmware_infos() {
    std::vector<driver::FirmwareInfo> firmware_infos;

    const auto pack_ack = [&]() -> std::vector<uint8_t> {
      std::vector<uint8_t> acks;
      if (!_link->send(_tx_buf)) return acks;
      if (wait_msg_processed(200) == 200) return acks;
      std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(acks), [](driver::RxMessage msg) noexcept { return msg.ack; });
      return acks;
    };

    cpu_version(_tx_buf);
    const auto cpu_versions = pack_ack();
    if (cpu_versions.empty()) return firmware_infos;

    fpga_version(_tx_buf);
    const auto fpga_versions = pack_ack();
    if (fpga_versions.empty()) return firmware_infos;

    fpga_functions(_tx_buf);
    const auto fpga_functions = pack_ack();
    if (fpga_functions.empty()) return firmware_infos;

    for (size_t i = 0; i < _geometry.num_devices(); i++)
      firmware_infos.emplace_back(i, cpu_versions.at(i), fpga_versions.at(i), fpga_functions.at(i));

    return firmware_infos;
  }

  /**
   * @brief Synchronize devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */

  [[deprecated("please send autd3::synchronize instead")]] bool synchronize() { return send(Synchronize{}); }

  /**
   * @brief Update flags (force fan and reads_fpga_info)
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  [[deprecated("please send autd3::update_flag instead")]] bool update_flag() { return send(UpdateFlag{}); }

  /**
   * @brief Clear all data in devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  [[deprecated("please send autd3::clear instead")]] bool clear() { return send(autd3::Clear{}); }

  /**
   * @brief Stop outputting
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  [[deprecated("please send autd3::stop instead")]] bool stop() { return send(autd3::Stop{}); }

  /**
   * @brief Send header data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename H>
  auto send(H& header) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, bool> {
    core::NullBody b;
    return send(header, b);
  }

  /**
   * @brief Send header data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename H>
  auto send(H&& header) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, bool> {
    return send(header);
  }

  /**
   * @brief Send body data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename B>
  auto send(B& body) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, bool> {
    core::NullHeader h;
    return send(h, body);
  }

  /**
   * @brief Send body data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename B>
  auto send(B&& body) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, bool> {
    return send(body);
  }

  /**
   * @brief Send header and body data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename H, typename B>
  auto send(H&& header, B&& body) ->
      typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>, bool> {
    return send(header, body);
  }

  /**
   * @brief Send header and body data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename H, typename B>
  auto send(H& header, B& body) ->
      typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>, bool> {
    return send(&header, &body);
  }

  /**
   * @brief Send header and body data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  auto send(core::DatagramHeader* header, core::DatagramBody* body) -> bool {
    header->init();
    body->init();

    driver::force_fan(_tx_buf, force_fan);
    driver::reads_fpga_info(_tx_buf, reads_fpga_info);

    while (true) {
      const auto msg_id = get_id();
      header->pack(msg_id, _tx_buf);
      body->pack(_geometry, _tx_buf);
      _link->send(_tx_buf);
      const auto trials = wait_msg_processed(check_trials);
      if ((check_trials != 0) && (trials == check_trials)) return false;
      if (header->is_finished() && body->is_finished()) break;
      if (trials == 0) std::this_thread::sleep_for(std::chrono::microseconds(send_interval * driver::EC_CYCLE_TIME_BASE_MICRO_SEC));
    }
    return true;
  }

  /**
   * @brief Send seprcial data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename S>
  auto send(S s) -> typename std::enable_if_t<std::is_base_of_v<SpecialData, S>, bool> {
    push_check_trial();
    if (s.check_trials_override()) check_trials = s.check_trials();
    auto h = s.header();
    auto b = s.body();
    const auto res = send(h.get(), b.get());
    pop_check_trial();
    return res;
  }

  /**
   * @brief Send seprcial data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  auto send(SpecialData* s) -> bool {
    push_check_trial();
    if (s->check_trials_override()) check_trials = s->check_trials();
    auto h = s->header();
    auto b = s->body();
    const auto res = send(h.get(), b.get());
    pop_check_trial();
    return res;
  }

  /**
   * @brief Send header data to devices asynchronously
   */
  template <typename H>
  auto send_async(H header) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>> {
    send_async(std::move(header), core::NullBody{});
  }

  /**
   * @brief Send body data to devices asynchronously
   */
  template <typename B>
  auto send_async(B body) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>> {
    send_async(core::NullHeader{}, std::move(body));
  }

  /**
   * @brief Send header and body data to devices asynchronously
   */
  template <typename H, typename B>
  auto send_async(H header, B body) ->
      typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>> {
    send_async(std::make_unique<H>(std::move(header)), std::make_unique<B>(std::move(body)));
  }

  /**
   * @brief Send special data to devices asynchronously
   */
  template <typename S>
  auto send_async(S s) -> typename std::enable_if_t<std::is_base_of_v<SpecialData, S>> {
    send_async(&s);
  }

  /**
   * @brief Send special data to devices asynchronously
   */
  void send_async(SpecialData* s) {
    auto check_trials_override = s->check_trials_override();
    auto trials = s->check_trials();
    send_async(
        s->header(), s->body(),
        [this, check_trials_override, trials] {
          push_check_trial();
          if (check_trials_override) check_trials = trials;
        },
        [this] { pop_check_trial(); });
  }

  /**
   * @brief Send header and body data to devices asynchronously
   */
  void send_async(
      std::unique_ptr<core::DatagramHeader> header, std::unique_ptr<core::DatagramBody> body, std::function<void()> pre = [] {},
      std::function<void()> post = [] {}) {
    {
      std::unique_lock lk(_send_mtx);
      AsyncData data;
      data.header = std::move(header);
      data.body = std::move(body);
      data.pre = std::move(pre);
      data.post = std::move(post);
      _send_queue.emplace(std::move(data));
    }
    _send_cond.notify_all();
  }

  /**
   * @brief Wait until all asynchronously sent data to complete the transmission
   */
  void wait() {
    if (!is_open()) return;
    while (!_send_queue.empty()) std::this_thread::sleep_for(std::chrono::milliseconds(100));
  }

  /**
   * @brief Flush all asynchronously sent data
   */
  void flush() {
    std::unique_lock lk(_send_mtx);
    std::queue<AsyncData>().swap(_send_queue);
  }

  /**
   * @brief If true, the fan will be forced to start.
   */
  bool force_fan;

  /**
   * @brief If true, the devices return FPGA info in all frames. The FPGA info can be read by fpga_info().
   */
  bool reads_fpga_info;

  /**
   * @brief If > 0, this controller check ack from devices. This value represents the maximum number of trials for the check.
   */
  size_t check_trials;

  /**
   * @brief Transmission interval between frames when sending multiple data. The interval will be send_interval *
   * driver::EC_SYNC0_CYCLE_TIME_MICRO_SEC.
   */
  size_t send_interval;

 private:
  static uint8_t get_id() noexcept {
    static std::atomic id_body{driver::MSG_BEGIN};
    if (uint8_t expected = driver::MSG_END; !id_body.compare_exchange_weak(expected, driver::MSG_BEGIN)) id_body.fetch_add(0x01);
    return id_body.load();
  }

  size_t wait_msg_processed(const size_t max_trial) {
    size_t i;
    const auto msg_id = _tx_buf.header().msg_id;
    for (i = 0; i < max_trial; i++) {
      if (_link->receive(_rx_buf) && _rx_buf.is_msg_processed(msg_id)) break;
      std::this_thread::sleep_for(std::chrono::microseconds(send_interval * driver::EC_CYCLE_TIME_BASE_MICRO_SEC));
    }
    return i;
  }

  size_t _check_trials_{};

  void push_check_trial() { _check_trials_ = check_trials; }

  void pop_check_trial() { check_trials = _check_trials_; }

  struct AsyncData {
    std::unique_ptr<core::DatagramHeader> header;
    std::unique_ptr<core::DatagramBody> body;
    std::function<void()> pre = [] {};
    std::function<void()> post = [] {};
  };

  core::Geometry _geometry;
  driver::TxDatagram _tx_buf;
  driver::RxDatagram _rx_buf;
  core::LinkPtr _link;

  bool _send_th_running;
  std::thread _send_th;
  std::queue<AsyncData> _send_queue;
  std::condition_variable _send_cond;
  std::mutex _send_mtx;

  bool _last_send_res;

 public:
  class AsyncSender {
    friend class Controller;

   public:
    Controller& cnt;

    template <typename H>
    class StreamCommaInputHeaderAsync {
      friend class AsyncSender;

     public:
      ~StreamCommaInputHeaderAsync() {
        if (!_sent) _cnt.cnt.send_async(std::move(_header));
      }
      StreamCommaInputHeaderAsync(const StreamCommaInputHeaderAsync& v) noexcept = delete;
      StreamCommaInputHeaderAsync& operator=(const StreamCommaInputHeaderAsync& obj) = delete;
      StreamCommaInputHeaderAsync(StreamCommaInputHeaderAsync&& obj) = default;
      StreamCommaInputHeaderAsync& operator=(StreamCommaInputHeaderAsync&& obj) = delete;

      template <typename B>
      auto operator,(B body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(_header), std::move(body));
        _sent = true;
        return _cnt;
      }

      template <typename B>
      auto operator<<(B body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(_header), std::move(body));
        _sent = true;
        return _cnt;
      }

      template <typename H2>
      auto operator<<(H2 header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H2>, StreamCommaInputHeaderAsync<H2>> {
        _cnt.cnt.send_async(std::move(_header));
        _sent = true;
        return StreamCommaInputHeaderAsync<H2>(_cnt, std::move(header));
      }

      template <typename S>
      auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(_header));
        _sent = true;
        _cnt.cnt.send_async(special_f());
        return _cnt;
      }

      template <typename H2, typename B2>
      auto operator<<(core::DatagramPack<H2, B2>&& pack)
          -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H2> && std::is_base_of_v<core::DatagramBody, B2>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(_header));
        _sent = true;
        _cnt.cnt.send_async(std::move(pack.header), std::move(pack.body));
        return _cnt;
      }

     private:
      explicit StreamCommaInputHeaderAsync(AsyncSender& cnt, H header) : _cnt(cnt), _header(std::move(header)), _sent(false) {}

      AsyncSender& _cnt;
      H _header;
      bool _sent;
    };

    template <typename B>
    class StreamCommaInputBodyAsync {
      friend class AsyncSender;

     public:
      ~StreamCommaInputBodyAsync() {
        if (!_sent) _cnt.cnt.send_async(std::move(_body));
      }
      StreamCommaInputBodyAsync(const StreamCommaInputBodyAsync& v) noexcept = delete;
      StreamCommaInputBodyAsync& operator=(const StreamCommaInputBodyAsync& obj) = delete;
      StreamCommaInputBodyAsync(StreamCommaInputBodyAsync&& obj) = default;
      StreamCommaInputBodyAsync& operator=(StreamCommaInputBodyAsync&& obj) = delete;

      template <typename H>
      auto operator,(H header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(header), std::move(_body));
        _sent = true;
        return _cnt;
      }

      template <typename H>
      auto operator<<(H header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(header), std::move(_body));
        _sent = true;
        return _cnt;
      }

      template <typename B2>
      auto operator<<(B2 body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B2>, StreamCommaInputBodyAsync<B2>> {
        _cnt.cnt.send_async(std::move(_body));
        _sent = true;
        return StreamCommaInputBodyAsync<B2>(_cnt, std::move(body));
      }

      template <typename S>
      auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(_body));
        _sent = true;
        _cnt.cnt.send_async(special_f());
        return _cnt;
      }

      template <typename H2, typename B2>
      auto operator<<(core::DatagramPack<H2, B2>&& pack)
          -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H2> && std::is_base_of_v<core::DatagramBody, B2>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(_body));
        _sent = true;
        _cnt.cnt.send_async(std::move(pack.header), std::move(pack.body));
        return _cnt;
      }

     private:
      explicit StreamCommaInputBodyAsync(AsyncSender& cnt, B body) : _cnt(cnt), _body(std::move(body)), _sent(false) {}

      AsyncSender& _cnt;
      B _body;
      bool _sent;
    };

    template <typename H>
    auto operator<<(H header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, StreamCommaInputHeaderAsync<H>> {
      return StreamCommaInputHeaderAsync<H>(*this, std::move(header));
    }

    template <typename B>
    auto operator<<(B body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, StreamCommaInputBodyAsync<B>> {
      return StreamCommaInputBodyAsync<B>(*this, std::move(body));
    }

    template <typename S>
    auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, AsyncSender&> {
      cnt.send_async(special_f());
      return *this;
    }

    template <typename H2, typename B2>
    auto operator<<(core::DatagramPack<H2, B2>&& pack)
        -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H2> && std::is_base_of_v<core::DatagramBody, B2>, AsyncSender&> {
      cnt.send_async(std::move(pack.header), std::move(pack.body));
      return *this;
    }

   private:
    explicit AsyncSender(Controller& cnt) : cnt(cnt) {}
  };

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

    template <typename B>
    auto operator,(B&& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, std::remove_reference_t<B>>, Controller&> {
      _cnt._last_send_res = _cnt.send(_header, body);
      _sent = true;
      return _cnt;
    }

    template <typename B>
    auto operator<<(B&& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, std::remove_reference_t<B>>, Controller&> {
      _cnt._last_send_res = _cnt.send(_header, body);
      _sent = true;
      return _cnt;
    }

    template <typename H2>
    auto operator<<(H2&& header)
        -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, std::remove_reference_t<H2>>, StreamCommaInputHeader<H2>> {
      _cnt._last_send_res = _cnt.send(_header);
      _sent = true;
      return StreamCommaInputHeader<H2>(_cnt, header);
    }

    template <typename S>
    auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, Controller&> {
      _cnt._last_send_res = _cnt.send(_header);
      _sent = true;
      _cnt._last_send_res = _cnt.send(special_f());
      return _cnt;
    }

    auto operator<<(core::DatagramPackRef pack) -> Controller& {
      _cnt._last_send_res = _cnt.send(_header);
      _sent = true;
      _cnt._last_send_res = _cnt.send(pack.header, pack.body);
      return _cnt;
    }

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

    template <typename H>
    auto operator,(H&& header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, std::remove_reference_t<H>>, Controller&> {
      _cnt._last_send_res = _cnt.send(header, _body);
      _sent = true;
      return _cnt;
    }

    template <typename H>
    auto operator<<(H&& header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, std::remove_reference_t<H>>, Controller&> {
      _cnt._last_send_res = _cnt.send(header, _body);
      _sent = true;
      return _cnt;
    }

    template <typename B2>
    auto operator<<(B2&& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, std::remove_reference_t<B2>>, StreamCommaInputBody<B2>> {
      _cnt._last_send_res = _cnt.send(_body);
      _sent = true;
      return StreamCommaInputBody<B2>(_cnt, body);
    }

    template <typename S>
    auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, Controller&> {
      _cnt._last_send_res = _cnt.send(_body);
      _sent = true;
      _cnt._last_send_res = _cnt.send(special_f());
      return _cnt;
    }

    auto operator<<(core::DatagramPackRef pack) -> Controller& {
      _cnt._last_send_res = _cnt.send(_body);
      _sent = true;
      _cnt._last_send_res = _cnt.send(pack.header, pack.body);
      return _cnt;
    }

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

  template <typename H>
  auto operator<<(H&& header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, std::remove_reference_t<H>>, StreamCommaInputHeader<H>> {
    return StreamCommaInputHeader<H>(*this, header);
  }

  template <typename B>
  auto operator<<(B&& body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, std::remove_reference_t<B>>, StreamCommaInputBody<B>> {
    return StreamCommaInputBody<B>(*this, body);
  }

  auto operator<<(core::DatagramPackRef pack) -> Controller& {
    _last_send_res = send(pack.header, pack.body);
    return *this;
  }

  template <typename H, typename B>
  auto operator<<(core::DatagramPack<H, B>&& pack)
      -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>, Controller&> {
    _last_send_res = send(pack.header, pack.body);
    return *this;
  }

  template <typename S>
  auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, Controller&> {
    _last_send_res = send(special_f());
    return *this;
  }

  template <typename A>
  auto operator<<(A (*)()) -> std::enable_if_t<std::is_same_v<autd3::Async, A>, AsyncSender> {
    return AsyncSender{*this};
  }

  void operator>>(bool& res) { res = _last_send_res; }
};

}  // namespace autd3
