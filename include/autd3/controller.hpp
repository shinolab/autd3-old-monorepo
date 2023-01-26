// File: controller.hpp
// Project: autd3
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 27/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
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
#include "autd3/core/geometry.hpp"
#include "autd3/core/link.hpp"
#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/firmware_version.hpp"
#include "autd3/driver/operation/force_fan.hpp"
#include "autd3/driver/operation/reads_fpga_info.hpp"
#include "autd3/special_data.hpp"

namespace autd3 {

/**
 * @brief AUTD Controller
 */
class Controller {
 public:
  Controller();
  Controller(const Controller& v) = delete;
  Controller& operator=(const Controller& obj) = delete;
  Controller(Controller&& obj) = delete;
  Controller& operator=(Controller&& obj) = delete;
  ~Controller() noexcept;

  /**
   * @brief Geometry of the devices
   */
  core::Geometry& geometry() noexcept;

  /**
   * @brief Geometry of the devices
   */
  [[nodiscard]] const core::Geometry& geometry() const noexcept;

  void open(core::LinkPtr link);

  /**
   * @brief Close the controller
   * \return if this function returns true and ack_check_timeout > 0, it guarantees that the devices have processed the data.
   */
  bool close();

  /**
   * @brief Verify the device is properly connected
   */
  [[nodiscard]] bool is_open() const noexcept;

  /**
   * @brief FPGA info
   *  \return vector of FPGAInfo. If failed, the vector is empty
   */
  std::vector<driver::FPGAInfo> fpga_info();

  /**
   * @brief Enumerate firmware information
   * \return vector of driver::FirmwareInfo
   */
  [[nodiscard]] std::vector<driver::FirmwareInfo> firmware_infos();

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
  bool send(core::DatagramHeader* header, core::DatagramBody* body, std::chrono::high_resolution_clock::duration timeout);

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
  bool send(SpecialData* s);

  /**
   * @brief Send header data to devices asynchronously
   */
  template <typename H>
  auto send_async(H header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>> {
    send_async(std::move(header), core::NullBody{});
  }

  /**
   * @brief Send body data to devices asynchronously
   */
  template <typename B>
  auto send_async(B body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>> {
    send_async(core::NullHeader{}, std::move(body));
  }

  /**
   * @brief Send header and body data to devices asynchronously
   */
  template <typename H, typename B>
  auto send_async(H header, B body) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>> {
    send_async(std::make_unique<H>(std::move(header)), std::make_unique<B>(std::move(body)));
  }

  /**
   * @brief Send special data to devices asynchronously
   */
  template <typename S>
  auto send_async(S s) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>> {
    send_async(&s);
  }

  /**
   * @brief Send special data to devices asynchronously
   */
  void send_async(SpecialData* s);

  /**
   * @brief Send header and body data to devices asynchronously
   */
  void send_async(std::unique_ptr<core::DatagramHeader> header, std::unique_ptr<core::DatagramBody> body);

  /**
   * @brief Send header and body data to devices asynchronously
   */
  void send_async(std::unique_ptr<core::DatagramHeader> header, std::unique_ptr<core::DatagramBody> body,
                  std::chrono::high_resolution_clock::duration timeout);

  /**
   * @brief Wait until all asynchronously sent data to complete the transmission
   */
  void wait() const;

  /**
   * @brief Flush all asynchronously sent data
   */
  void flush();

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
  [[nodiscard]] bool reads_fpga_info() const noexcept;

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
  [[nodiscard]] std::chrono::high_resolution_clock::duration get_send_interval() const noexcept;

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
  [[nodiscard]] std::chrono::high_resolution_clock::duration get_ack_check_timeout() const noexcept;

  /**
   * Set speed of sound from temperature
   * @param temp temperature in Celsius degree
   * @param k Heat capacity ratio
   * @param r Gas constant [J K^-1 mol^-1]
   * @param m Molar mass [kg mod^-1]
   * @return sound_speed
   */
  driver::autd3_float_t set_sound_speed_from_temp(driver::autd3_float_t temp, driver::autd3_float_t k = static_cast<driver::autd3_float_t>(1.4),
                                                  driver::autd3_float_t r = static_cast<driver::autd3_float_t>(8.31446261815324),
                                                  driver::autd3_float_t m = static_cast<driver::autd3_float_t>(28.9647e-3));

 private:
  static uint8_t get_id() noexcept;

  bool wait_msg_processed(std::chrono::high_resolution_clock::duration timeout);

  std::chrono::high_resolution_clock::duration _send_interval{std::chrono::milliseconds(1)};

  std::chrono::high_resolution_clock::duration _ack_check_timeout{std::chrono::high_resolution_clock::duration::zero()};

  struct AsyncData {
    std::unique_ptr<core::DatagramHeader> header{};
    std::unique_ptr<core::DatagramBody> body{};
    std::chrono::high_resolution_clock::duration timeout{};
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

  driver::ForceFan _force_fan;
  driver::ReadsFPGAInfo _reads_fpga_info;

 public:
  /**
   * @brief Controller wrapper for asynchronous send
   */
  class AsyncSender {
    friend class Controller;

   public:
    Controller& cnt;

    /**
     * @brief Buffer for stream operator
     * @tparam H Class inheriting from core::DatagramHeader
     */
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

      /**
       * @brief Send buffered core::DatagramHeader and core::DatagramBody
       * @tparam B Class inheriting from core::DatagramBody
       * @param body core::DatagramBody
       * @return AsyncSender&
       */
      template <typename B>
      auto operator,(B body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(_header), std::move(body));
        _sent = true;
        return _cnt;
      }

      /**
       * @brief Send buffered core::DatagramHeader and core::DatagramBody
       * @tparam B Class inheriting from core::DatagramBody
       * @param body core::DatagramBody
       * @return AsyncSender&
       */
      template <typename B>
      auto operator<<(B body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(_header), std::move(body));
        _sent = true;
        return _cnt;
      }

      /**
       * @brief Send buffered core::DatagramHeader and buffer core::DatagramHeader passed as argument
       * @tparam H2 Class inheriting from core::DatagramHeader
       * @param header core::DatagramHeader
       * @return StreamCommaInputHeaderAsync
       */
      template <typename H2>
      auto operator<<(H2 header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H2>, StreamCommaInputHeaderAsync<H2>> {
        _cnt.cnt.send_async(std::move(_header));
        _sent = true;
        return StreamCommaInputHeaderAsync<H2>(_cnt, std::move(header));
      }

      /**
       * @brief Send buffered core::DatagramHeader and SpecialData
       * @tparam S Class inheriting from SpecialData
       * @param special_f SpecialData function
       * @return AsyncSender&
       */
      template <typename S>
      auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(_header));
        _sent = true;
        _cnt.cnt.send_async(special_f());
        return _cnt;
      }

      /**
       * @brief Send buffered core::DatagramHeader and then send core::DatagramHeader and core::DatagramBody in DatagramPack
       * @tparam H2 Class inheriting from core::DatagramHeader
       * @tparam B2 Class inheriting from core::DatagramBody
       * @param pack DatagramPack
       * @return AsyncSender&
       */
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

    /**
     * @brief Buffer for stream operator
     * @tparam B Class inheriting from core::DatagramBody
     */
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

      /**
       * @brief Send buffered core::DatagramBody and core::DatagramHeader
       * @tparam H Class inheriting from core::DatagramHeader
       * @param header core::DatagramHeader
       * @return AsyncSender&
       */
      template <typename H>
      auto operator,(H header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(header), std::move(_body));
        _sent = true;
        return _cnt;
      }

      /**
       * @brief Send buffered core::DatagramBody and core::DatagramHeader
       * @tparam H Class inheriting from core::DatagramHeader
       * @param header core::DatagramHeader
       * @return AsyncSender&
       */
      template <typename H>
      auto operator<<(H header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(header), std::move(_body));
        _sent = true;
        return _cnt;
      }

      /**
       * @brief Send buffered core::DatagramBody and buffer core::DatagramBody passed as argument
       * @tparam B2 Class inheriting from core::DatagramBody
       * @param body core::DatagramBody
       * @return StreamCommaInputBodyAsync<B2>
       */
      template <typename B2>
      auto operator<<(B2 body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B2>, StreamCommaInputBodyAsync<B2>> {
        _cnt.cnt.send_async(std::move(_body));
        _sent = true;
        return StreamCommaInputBodyAsync<B2>(_cnt, std::move(body));
      }

      /**
       * @brief Send buffered core::DatagramBody and SpecialData
       * @tparam S Class inheriting from SpecialData
       * @param special_f SpecialData function
       * @return AsyncSender&
       */
      template <typename S>
      auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, AsyncSender&> {
        _cnt.cnt.send_async(std::move(_body));
        _sent = true;
        _cnt.cnt.send_async(special_f());
        return _cnt;
      }

      /**
       * @brief Send buffered core::DatagramBody and then send core::DatagramHeader and core::DatagramBody in DatagramPack
       * @tparam H2 Class inheriting from core::DatagramHeader
       * @tparam B2 Class inheriting from core::DatagramBody
       * @param pack core::DatagramPack
       * @return AsyncSender&
       */
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

    /**
     * @brief Buffer core::DatagramHeader
     * @tparam H Class inheriting from core::DatagramHeader
     * @param header core::DatagramHeader
     * @return StreamCommaInputHeaderAsync<H>
     */
    template <typename H>
    auto operator<<(H header) -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, StreamCommaInputHeaderAsync<H>> {
      return StreamCommaInputHeaderAsync<H>(*this, std::move(header));
    }

    /**
     * @brief Buffer core::DatagramBody
     * @tparam B Class inheriting from core::DatagramBody
     * @param body core::DatagramBody
     * @return StreamCommaInputBodyAsync<H>
     */
    template <typename B>
    auto operator<<(B body) -> std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, StreamCommaInputBodyAsync<B>> {
      return StreamCommaInputBodyAsync<B>(*this, std::move(body));
    }

    /**
     * @brief Send SpecialData
     * @tparam S Class inheriting from SpecialData
     * @param special_f SpecialData function
     * @return AsyncSender&
     */
    template <typename S>
    auto operator<<(S (*special_f)()) -> std::enable_if_t<std::is_base_of_v<SpecialData, S>, AsyncSender&> {
      cnt.send_async(special_f());
      return *this;
    }

    /**
     * @brief Send core::DatagramHeader and core::DatagramBody in core::DatagramPack
     * @tparam H Class inheriting from core::DatagramHeader
     * @tparam B Class inheriting from core::DatagramBody
     * @param pack core::DatagramPack
     * @return AsyncSender&
     */
    template <typename H, typename B>
    auto operator<<(core::DatagramPack<H, B>&& pack)
        -> std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>, AsyncSender&> {
      cnt.send_async(std::move(pack.header), std::move(pack.body));
      return *this;
    }

   private:
    explicit AsyncSender(Controller& cnt) : cnt(cnt) {}
  };

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
   * @brief Set asynchronous send mode
   * @return AsyncSender asynchronous sender
   */
  AsyncSender operator<<(Async (*)()) { return AsyncSender{*this}; }

  /**
   * @brief Set Mode
   * @param f mode function
   */
  void operator<<(core::Mode (*f)()) { _geometry.mode = f(); }

  void operator>>(bool& res) const { res = _last_send_res; }
};

}  // namespace autd3
