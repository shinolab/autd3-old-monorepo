// File: local_interface.hpp
// Project: SOEMAUTDServer
// Created Date: 01/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26439 26451 26495)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#endif
#include <boost/interprocess/managed_shared_memory.hpp>
#include <boost/interprocess/sync/interprocess_mutex.hpp>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

#include <string>

#include "autd3/autd3_device.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "interface.hpp"

namespace autd3::publish {

class LocalInterface final : public Interface {
  static constexpr std::string_view SHMEM_NAME{"autd3_soem_server_shmem"};
  static constexpr std::string_view SHMEM_MTX_NAME{"autd3_soem_server_shmem_mtx"};
  static constexpr std::string_view SHMEM_DATA_NAME{"autd3_soem_server_shmem_ptr"};

 public:
  explicit LocalInterface(const size_t dev) noexcept : _dev(dev) {
    boost::interprocess::shared_memory_object::remove(std::string(SHMEM_NAME).c_str());
  }
  ~LocalInterface() override { boost::interprocess::shared_memory_object::remove(std::string(SHMEM_NAME).c_str()); };
  LocalInterface(const LocalInterface& v) noexcept = delete;
  LocalInterface& operator=(const LocalInterface& obj) = delete;
  LocalInterface(LocalInterface&& obj) = default;
  LocalInterface& operator=(LocalInterface&& obj) = default;

  void connect() override {
    const auto size = driver::HEADER_SIZE + _dev * (AUTD3::NUM_TRANS_IN_UNIT * sizeof(uint16_t) + driver::EC_INPUT_FRAME_SIZE);
    _segment = boost::interprocess::managed_shared_memory(boost::interprocess::create_only, std::string(SHMEM_NAME).c_str(),
                                                          size + sizeof(boost::interprocess::interprocess_mutex) + 1024);
    _mtx = _segment.construct<boost::interprocess::interprocess_mutex>(std::string(SHMEM_MTX_NAME).c_str())();
    _ptr = _segment.construct<uint8_t>(std::string(SHMEM_DATA_NAME).c_str())[size](0x00);
  }

  void close() override { boost::interprocess::shared_memory_object::remove(std::string(SHMEM_NAME).c_str()); }

  bool tx(driver::TxDatagram& tx) override {
    const auto msg_id = _ptr[0];
    if (_last_msg_id == msg_id) return false;
    _last_msg_id = msg_id;

    {
      std::unique_lock lk(*_mtx);
      std::memcpy(tx.data().data(), _ptr, tx.transmitting_size_in_bytes());
    }
    return true;
  }

  bool rx(driver::RxDatagram& rx) override {
    {
      std::unique_lock lk(*_mtx);
      std::memcpy(_ptr + driver::HEADER_SIZE + rx.messages().size() * AUTD3::NUM_TRANS_IN_UNIT * sizeof(uint16_t), rx.messages().data(),
                  rx.messages().size() * driver::EC_INPUT_FRAME_SIZE);
    }
    return true;
  }

 private:
  boost::interprocess::managed_shared_memory _segment{};
  boost::interprocess::interprocess_mutex* _mtx{nullptr};
  uint8_t* _ptr{nullptr};

  size_t _dev{0};
  uint8_t _last_msg_id{0};
};

}  // namespace autd3::publish
