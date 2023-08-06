// File: soem.hpp
// Project: link
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <string>

#include "autd3/internal/exception.hpp"
#include "autd3/internal/link.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::link {

using internal::native_methods::SyncMode;

/**
 * @brief Link using [SOEM](https://github.com/OpenEtherCATsociety/SOEM)
 *
 */
class SOEM : public internal::Link {
 public:
  SOEM() : Link(internal::native_methods::AUTDLinkSOEM()) {}

  /**
   * @brief Set network interface name
   * @details If empty, this link will automatically find the network interface that is connected to AUTD3 devices.
   *
   * @param ifname Network interface name
   * @return SOEM
   */
  SOEM with_ifname(const std::string& ifname) {
    _ptr = AUTDLinkSOEMIfname(_ptr, ifname.c_str());
    return *this;
  }

  /**
   * @brief Set buffer size
   *
   * @param value
   * @return SOEM
   */
  SOEM with_buf_size(const size_t value) {
    _ptr = AUTDLinkSOEMBufSize(_ptr, static_cast<uint32_t>(value));
    return *this;
  }

  /**
   * @brief Set send cycle (the unit is 500us)
   *
   * @param value
   * @return SOEM
   */
  SOEM with_send_cycle(const uint16_t value) {
    _ptr = AUTDLinkSOEMSendCycle(_ptr, value);
    return *this;
  }

  /**
   * @brief Set sync0 cycle (the unit is 500us)
   *
   * @param value
   * @return SOEM
   */
  SOEM with_sync0_cycle(const uint16_t value) {
    _ptr = AUTDLinkSOEMSync0Cycle(_ptr, value);
    return *this;
  }

  /**
   * @brief Set callback function when the link is lost
   *
   * @param value
   * @return SOEM
   */
  SOEM with_on_lost(const internal::LogOutCallback value) {
    _ptr = AUTDLinkSOEMOnLost(_ptr, reinterpret_cast<void*>(value));
    return *this;
  }

  /**
   * @brief Set timer strategy
   *
   * @param value
   * @return SOEM
   */
  SOEM with_timer_strategy(const internal::native_methods::TimerStrategy value) {
    _ptr = AUTDLinkSOEMTimerStrategy(_ptr, value);
    return *this;
  }

  /**
   * @brief Set sync mode
   * @details See [Beckhoff's site](https://infosys.beckhoff.com/content/1033/ethercatsystem/2469122443.html) for more details.
   *
   * @param value
   * @return SOEM
   */
  SOEM with_sync_mode(const internal::native_methods::SyncMode value) {
    _ptr = AUTDLinkSOEMSyncMode(_ptr, value);
    return *this;
  }

  /**
   * @brief Set state check interval
   *
   * @param value
   * @return SOEM
   */
  template <typename Rep, typename Period>
  SOEM with_state_check_interval(const std::chrono::duration<Rep, Period> value) {
    const auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(value).count();
    _ptr = AUTDLinkSOEMStateCheckInterval(_ptr, static_cast<uint64_t>(ms));
    return std::move(*this);
  }

  /**
   * @brief Set log level
   *
   * @param value
   * @return SOEM
   */
  SOEM with_log_level(const internal::native_methods::Level value) {
    _ptr = AUTDLinkSOEMLogLevel(_ptr, value);
    return *this;
  }

  /**
   * @brief Set log function
   *
   * @param out Output callback function
   * @param flush Flush callback function
   * @return SOEM
   */
  SOEM with_log_func(const internal::LogOutCallback out, const internal::LogFlushCallback flush) {
    _ptr = AUTDLinkSOEMLogFunc(_ptr, reinterpret_cast<void*>(out), reinterpret_cast<void*>(flush));
    return *this;
  }

  template <typename Rep, typename Period>
  SOEM with_timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _ptr = AUTDLinkSOEMTimeout(_ptr, static_cast<uint64_t>(ns));
    return std::move(*this);
  }
};

/**
 * @brief Link to connect to remote SOEMServer
 */
class RemoteSOEM : public internal::Link {
 public:
  /**
   * @brief Constructor
   *
   * @param addr IP address and port of SOEMServer (e.g., "127.0.0.1:8080")
   */
  explicit RemoteSOEM(const std::string& addr) : Link(internal::native_methods::LinkPtr{nullptr}) {
    char err[256];
    _ptr = internal::native_methods::AUTDLinkRemoteSOEM(addr.c_str(), err);
    if (_ptr._0 == nullptr) throw internal::AUTDException(err);
  }

  template <typename Rep, typename Period>
  RemoteSOEM with_timeout(const std::chrono::duration<Rep, Period> timeout) {
    const auto ns = std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count();
    _ptr = AUTDLinkRemoteSOEMTimeout(_ptr, static_cast<uint64_t>(ns));
    return std::move(*this);
  }
};

}  // namespace autd3::link
