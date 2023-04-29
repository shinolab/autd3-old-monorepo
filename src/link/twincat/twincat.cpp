// File: twincat.cpp
// Project: twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#if WIN32
#include <Windows.h>
#endif

#include "autd3/core/link.hpp"
#include "autd3/link/twincat.hpp"

namespace autd3::link {

struct AmsNetId {
  uint8_t b[6];
};

struct AmsAddr {
  AmsNetId net_id;
  uint16_t port;
};

#ifdef _WIN32
constexpr uint32_t INDEX_GROUP = 0x3040030;
constexpr uint32_t INDEX_OFFSET_BASE = 0x81000000;
constexpr uint32_t INDEX_OFFSET_BASE_READ = 0x80000000;
constexpr uint16_t PORT = 301;

typedef long(_stdcall* TcAdsPortOpenEx)();                       // NOLINT
typedef long(_stdcall* TcAdsPortCloseEx)(long);                  // NOLINT
typedef long(_stdcall* TcAdsGetLocalAddressEx)(long, AmsAddr*);  // NOLINT
typedef long(_stdcall* TcAdsSyncWriteReqEx)(long, AmsAddr*,      // NOLINT
                                            unsigned long,       // NOLINT
                                            unsigned long,       // NOLINT
                                            unsigned long,       // NOLINT
                                            void*);              // NOLINT
typedef long(_stdcall* TcAdsSyncReadReqEx)(long, AmsAddr*,       // NOLINT
                                           unsigned long,        // NOLINT
                                           unsigned long,        // NOLINT
                                           unsigned long,        // NOLINT
                                           void*,                // NOLINT
                                           unsigned long*);      // NOLINT

constexpr auto TCADS_ADS_PORT_OPEN_EX = "AdsPortOpenEx";
constexpr auto TCADS_ADS_GET_LOCAL_ADDRESS_EX = "AdsGetLocalAddressEx";
constexpr auto TCADS_ADS_PORT_CLOSE_EX = "AdsPortCloseEx";
constexpr auto TCADS_ADS_SYNC_WRITE_REQ_EX = "AdsSyncWriteReqEx";
constexpr auto TCADS_ADS_SYNC_READ_REQ_EX = "AdsSyncReadReqEx2";
#endif

class TwinCATImpl final : public core::Link {
 public:
  explicit TwinCATImpl(const core::Duration timeout) : Link(timeout), _port(0) {}
  ~TwinCATImpl() override = default;
  TwinCATImpl(const TwinCATImpl& v) noexcept = delete;
  TwinCATImpl& operator=(const TwinCATImpl& obj) = delete;
  TwinCATImpl(TwinCATImpl&& obj) = delete;
  TwinCATImpl& operator=(TwinCATImpl&& obj) = delete;

  bool open(const core::Geometry& geometry) override;
  bool close() override;
  bool send(const driver::TxDatagram& tx) override;
  bool receive(driver::RxDatagram& rx) override;
  bool is_open() override;

 private:
  long _port;  // NOLINT
#ifdef _WIN32
  AmsAddr _net_addr{};
  HMODULE _lib = nullptr;

  TcAdsSyncWriteReqEx _write = nullptr;
  TcAdsSyncReadReqEx _read = nullptr;
#endif
};

core::LinkPtr TwinCAT::build_() { return std::make_unique<TwinCATImpl>(_timeout); }

bool TwinCATImpl::is_open() { return this->_port > 0; }

#ifdef _WIN32

bool TwinCATImpl::open(const core::Geometry&) {
  this->_lib = LoadLibrary("TcAdsDll.dll");
  if (_lib == nullptr) throw std::runtime_error("couldn't find TcADS-DLL");

  const auto port_open = reinterpret_cast<TcAdsPortOpenEx>(GetProcAddress(this->_lib, TCADS_ADS_PORT_OPEN_EX));  // NOLINT
  this->_port = (*port_open)();
  if (this->_port == 0) throw std::runtime_error("Failed to open a new ADS port");

  AmsAddr addr{};
  const auto get_addr = reinterpret_cast<TcAdsGetLocalAddressEx>(GetProcAddress(this->_lib, TCADS_ADS_GET_LOCAL_ADDRESS_EX));  // NOLINT
  if (const auto ret = get_addr(this->_port, &addr); ret) throw std::runtime_error("AdsGetLocalAddress: " + std::to_string(ret));

  _write = reinterpret_cast<TcAdsSyncWriteReqEx>(GetProcAddress(this->_lib, TCADS_ADS_SYNC_WRITE_REQ_EX));  // NOLINT
  _read = reinterpret_cast<TcAdsSyncReadReqEx>(GetProcAddress(this->_lib, TCADS_ADS_SYNC_READ_REQ_EX));     // NOLINT

  this->_net_addr = {addr.net_id, PORT};

  return true;
}

bool TwinCATImpl::close() {
  if (!this->is_open()) return true;

  const auto port_close = reinterpret_cast<TcAdsPortCloseEx>(GetProcAddress(this->_lib, TCADS_ADS_PORT_CLOSE_EX));  // NOLINT
  if (const auto res = (*port_close)(this->_port); res != 0) throw std::runtime_error("Error on closing (local): " + std::to_string(res));

  this->_port = 0;
  return true;
}

bool TwinCATImpl::send(const driver::TxDatagram& tx) {
  if (!this->is_open()) throw std::runtime_error("Link is closed");

  const auto ret = this->_write(this->_port,  // NOLINT
                                &this->_net_addr, INDEX_GROUP, INDEX_OFFSET_BASE,
                                static_cast<unsigned long>(tx.transmitting_size_in_bytes()),  // NOLINT
                                const_cast<void*>(static_cast<const void*>(tx.data().data())));
  if (ret == 0) return true;

  throw std::runtime_error("Error on sending data (local): {:#x}" + std::to_string(ret));  // 6 : target port not found
}

bool TwinCATImpl::receive(driver::RxDatagram& rx) {
  if (!this->is_open()) throw std::runtime_error("Link is closed");

  unsigned long read_bytes = 0;              // NOLINT
  const auto ret = this->_read(this->_port,  // NOLINT
                               &this->_net_addr, INDEX_GROUP, INDEX_OFFSET_BASE_READ,
                               static_cast<uint32_t>(rx.messages().size() * sizeof(driver::RxMessage)), rx.messages().data(), &read_bytes);
  if (ret == 0) return true;

  throw std::runtime_error("Error on receiving data (local): " + std::to_string(ret));
}

#else
bool TwinCATImpl::open(const core::Geometry&) { throw std::runtime_error("TwinCAT link is only supported in Windows."); }
bool TwinCATImpl::close() { return false; }
bool TwinCATImpl::send(const driver::TxDatagram&) { return false; }
bool TwinCATImpl::receive(driver::RxDatagram&) { return false; }
#endif  // TC_ADS

}  // namespace autd3::link
