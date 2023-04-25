// File: remote_soem_local.hpp
// Project: remote_soem
// Created Date: 02/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <boost/interprocess/managed_shared_memory.hpp>
#include <boost/interprocess/sync/interprocess_mutex.hpp>
#include <mutex>
#include <string>

#include "autd3/core/link.hpp"
#include "autd3/driver/cpu/ec_config.hpp"

namespace autd3::link {

class RemoteSOEMLocal final : public core::Link {
  static constexpr std::string_view SHMEM_NAME{"autd3_soem_server_shmem"};
  static constexpr std::string_view SHMEM_MTX_NAME{"autd3_soem_server_shmem_mtx"};
  static constexpr std::string_view SHMEM_DATA_NAME{"autd3_soem_server_shmem_ptr"};

 public:
  explicit RemoteSOEMLocal(const core::Duration timeout) : Link(timeout), _output_size(0) {}
  ~RemoteSOEMLocal() override = default;
  RemoteSOEMLocal(const RemoteSOEMLocal& v) noexcept = delete;
  RemoteSOEMLocal& operator=(const RemoteSOEMLocal& obj) = delete;
  RemoteSOEMLocal(RemoteSOEMLocal&& obj) = delete;
  RemoteSOEMLocal& operator=(RemoteSOEMLocal&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    if (is_open()) return true;

    _output_size = driver::HEADER_SIZE + std::accumulate(geometry.device_map().begin(), geometry.device_map().end(), size_t{0}) * sizeof(uint16_t);

    _segment = boost::interprocess::managed_shared_memory(boost::interprocess::open_only, std::string(SHMEM_NAME).c_str());
    _ptr = _segment.find<uint8_t>(std::string(SHMEM_DATA_NAME).c_str()).first;
    _mtx = _segment.find<boost::interprocess::interprocess_mutex>(std::string(SHMEM_MTX_NAME).c_str()).first;

    _is_open = true;
    return true;
  }

  bool close() override {
    if (!is_open()) return true;

    {
      std::unique_lock lk(*_mtx);
      _ptr[0] = driver::MSG_SERVER_CLOSE;
    }

    _is_open = false;
    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    if (!is_open()) return false;

    {
      std::unique_lock lk(*_mtx);
      std::memcpy(_ptr, tx.data().data(), tx.transmitting_size_in_bytes());
    }

    return true;
  }

  bool receive(driver::RxDatagram& rx) override {
    if (!is_open()) return false;

    {
      std::unique_lock lk(*_mtx);
      rx.copy_from(reinterpret_cast<const driver::RxMessage*>(_ptr + _output_size));
    }

    return true;
  }

  bool is_open() override { return _is_open; }

 private:
  boost::interprocess::managed_shared_memory _segment{};
  boost::interprocess::interprocess_mutex* _mtx{nullptr};
  uint8_t* _ptr{nullptr};

  size_t _output_size;
  bool _is_open{false};
};

}  // namespace autd3::link
