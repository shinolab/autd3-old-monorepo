// File: twincat_link.cpp
// Project: twincat
// Created Date: 08/03/2021
// Author: Shun Suzuki
// -----
// Last Modified: 12/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#if _WINDOWS
#include <Windows.h>
#endif

#include <sstream>
#include <string>

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

class TwinCATImpl final : public autd3::core::Link {
 public:
  TwinCATImpl(const uint16_t cycle_ticks) : Link(cycle_ticks), _port(0) {}
  ~TwinCATImpl() override = default;
  TwinCATImpl(const TwinCATImpl& v) noexcept = delete;
  TwinCATImpl& operator=(const TwinCATImpl& obj) = delete;
  TwinCATImpl(TwinCATImpl&& obj) = delete;
  TwinCATImpl& operator=(TwinCATImpl&& obj) = delete;

  void open() override;
  void close() override;
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

core::LinkPtr TwinCAT::build() { return std::make_unique<TwinCATImpl>(_cycle_ticks); }

bool TwinCATImpl::is_open() { return this->_port > 0; }

#ifdef _WIN32

void TwinCATImpl::open() {
  this->_lib = LoadLibrary("TcAdsDll.dll");
  if (_lib == nullptr) throw std::runtime_error("couldn't find TcADS-DLL");

  const auto port_open = reinterpret_cast<TcAdsPortOpenEx>(GetProcAddress(this->_lib, TCADS_ADS_PORT_OPEN_EX));  // NOLINT
  this->_port = (*port_open)();
  if (!this->_port) throw std::runtime_error("failed to open a new ADS port");

  AmsAddr addr{};
  const auto get_addr = reinterpret_cast<TcAdsGetLocalAddressEx>(GetProcAddress(this->_lib, TCADS_ADS_GET_LOCAL_ADDRESS_EX));  // NOLINT
  if (const auto ret = get_addr(this->_port, &addr); ret) {
    std::stringstream ss;
    ss << "AdsGetLocalAddress: " << std::hex << ret;
    throw std::runtime_error(ss.str());
  }

  _write = reinterpret_cast<TcAdsSyncWriteReqEx>(GetProcAddress(this->_lib, TCADS_ADS_SYNC_WRITE_REQ_EX));  // NOLINT
  _read = reinterpret_cast<TcAdsSyncReadReqEx>(GetProcAddress(this->_lib, TCADS_ADS_SYNC_READ_REQ_EX));     // NOLINT

  this->_net_addr = {addr.net_id, PORT};
}

void TwinCATImpl::close() {
  if (!this->is_open()) return;

  const auto port_close = reinterpret_cast<TcAdsPortCloseEx>(GetProcAddress(this->_lib, TCADS_ADS_PORT_CLOSE_EX));  // NOLINT
  if (const auto res = (*port_close)(this->_port); res != 0) {
    std::stringstream ss;
    ss << "Error on closing (local): " << std::hex << res;
    throw std::runtime_error(ss.str());
  }

  this->_port = 0;
}

bool TwinCATImpl::send(const driver::TxDatagram& tx) {
  if (!this->is_open()) throw std::runtime_error("Link is closed");

  if (const auto ret = this->_write(this->_port,  // NOLINT
                                    &this->_net_addr, INDEX_GROUP, INDEX_OFFSET_BASE,
                                    static_cast<unsigned long>(tx.effective_size()),  // NOLINT
                                    const_cast<void*>(static_cast<const void*>(tx.data().data())));
      ret != 0) {
    // https://infosys.beckhoff.com/english.php?content=../content/1033/tcadscommon/html/tcadscommon_intro.htm&id=
    // 6 : target port not found
    std::stringstream ss;
    ss << "Error on sending data (local): " << std::hex << ret;
    throw std::runtime_error(ss.str());
  }

  return true;
}

bool TwinCATImpl::receive(driver::RxDatagram& rx) {
  if (!this->is_open()) throw std::runtime_error("Link is closed");

  unsigned long read_bytes = 0;                  // NOLINT
  if (const auto ret = this->_read(this->_port,  // NOLINT
                                   &this->_net_addr, INDEX_GROUP, INDEX_OFFSET_BASE_READ,
                                   static_cast<uint32_t>(rx.messages().size() * sizeof(driver::RxMessage)), rx.messages().data(), &read_bytes);
      ret != 0) {
    std::stringstream ss;
    ss << "Error on receiving data: " << std::hex << ret;
    throw std::runtime_error(ss.str());
  }

  return true;
}

#else
void TwinCATImpl::open() {
  throw std::runtime_error("Link to localhost has not been compiled. Rebuild this library on a Twincat3 host machine with TcADS-DLL.");
}
void TwinCATImpl::close() { return; }
bool TwinCATImpl::send(const driver::TxDatagram& tx) { return false; }
bool TwinCATImpl::receive(driver::RxDatagram& rx) { return false; }
#endif  // TC_ADS

}  // namespace autd3::link
