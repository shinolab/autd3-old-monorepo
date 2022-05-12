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

#include <AdsLib.h>

#if _WINDOWS
#include <Windows.h>
#endif

#include <sstream>
#include <string>
#include <vector>

#include "autd3/core/link.hpp"
#include "autd3/link/remote_twincat.hpp"

namespace autd3::link {

namespace {

std::vector<std::string> split(const std::string& s, const char deliminator) {
  std::vector<std::string> tokens;
  std::string token;
  for (const auto& ch : s) {
    if (ch == deliminator) {
      if (!token.empty()) tokens.emplace_back(token);
      token.clear();
    } else {
      token += ch;
    }
  }
  if (!token.empty()) tokens.emplace_back(token);
  return tokens;
}
}  // namespace

constexpr uint32_t INDEX_GROUP = 0x3040030;
constexpr uint32_t INDEX_OFFSET_BASE = 0x81000000;
constexpr uint32_t INDEX_OFFSET_BASE_READ = 0x80000000;
constexpr uint16_t PORT = 301;

class RemoteTwinCATImpl final : public core::Link {
 public:
  RemoteTwinCATImpl(const uint16_t cycle_ticks, std::string ipv4_addr, std::string remote_ams_net_id, std::string local_ams_net_id)
      : Link(cycle_ticks),
        _local_ams_net_id(std::move(local_ams_net_id)),
        _remote_ams_net_id(std::move(remote_ams_net_id)),
        _ipv4_addr(std::move(ipv4_addr)) {}
  ~RemoteTwinCATImpl() override = default;
  RemoteTwinCATImpl(const RemoteTwinCATImpl& v) noexcept = delete;
  RemoteTwinCATImpl& operator=(const RemoteTwinCATImpl& obj) = delete;
  RemoteTwinCATImpl(RemoteTwinCATImpl&& obj) = delete;
  RemoteTwinCATImpl& operator=(RemoteTwinCATImpl&& obj) = delete;

  void open() override;
  void close() override;
  bool send(const driver::TxDatagram& tx) override;
  bool receive(driver::RxDatagram& rx) override;
  bool is_open() override;

 private:
  std::string _local_ams_net_id;
  std::string _remote_ams_net_id;
  std::string _ipv4_addr;
  long _port = 0L;  // NOLINT
  AmsNetId _net_id;
};

core::LinkPtr RemoteTwinCAT::build() { return std::make_unique<RemoteTwinCATImpl>(_cycle_ticks, _ipv4_addr, _remote_ams_net_id, _local_ams_net_id); }

void RemoteTwinCATImpl::open() {
  const auto octets = split(_remote_ams_net_id, '.');
  if (octets.size() != 6) throw std::runtime_error("Ams net id must have 6 octets");

  if (_ipv4_addr.empty()) {
    for (auto i = 0; i < 3; i++) _ipv4_addr += octets[i] + ".";
    _ipv4_addr += octets[3];
  }

  if (!_local_ams_net_id.empty()) {
    const auto local_octets = split(_local_ams_net_id, '.');
    if (local_octets.size() != 6) throw std::runtime_error("Ams net id must have 6 octets");
    bhf::ads::SetLocalAddress({static_cast<uint8_t>(std::stoi(local_octets[0])), static_cast<uint8_t>(std::stoi(local_octets[1])),
                               static_cast<uint8_t>(std::stoi(local_octets[2])), static_cast<uint8_t>(std::stoi(local_octets[3])),
                               static_cast<uint8_t>(std::stoi(local_octets[4])), static_cast<uint8_t>(std::stoi(local_octets[5]))});
  }

  this->_net_id = {static_cast<uint8_t>(std::stoi(octets[0])), static_cast<uint8_t>(std::stoi(octets[1])),
                   static_cast<uint8_t>(std::stoi(octets[2])), static_cast<uint8_t>(std::stoi(octets[3])),
                   static_cast<uint8_t>(std::stoi(octets[4])), static_cast<uint8_t>(std::stoi(octets[5]))};

  if (AdsAddRoute(this->_net_id, _ipv4_addr.c_str()) != 0) throw std::runtime_error("Could not connect to remote");

  this->_port = AdsPortOpenEx();

  if (this->_port == 0) throw std::runtime_error("Failed to open a new ADS port");
}

void RemoteTwinCATImpl::close() {
  if (AdsPortCloseEx(this->_port) != 0) throw std::runtime_error("Failed to close");
  this->_port = 0;
}

bool RemoteTwinCATImpl::send(const driver::TxDatagram& tx) {
  const AmsAddr p_addr = {this->_net_id, PORT};
  const auto ret = AdsSyncWriteReqEx(this->_port, &p_addr, INDEX_GROUP, INDEX_OFFSET_BASE, static_cast<uint32_t>(tx.size()), tx.data().data());
  if (ret == 0) return true;

  std::stringstream ss;
  if (ret == ADSERR_DEVICE_INVALIDSIZE)
    ss << "The number of devices is invalid.";
  else
    ss << "Error on sending data: " << std::hex << ret;
  throw std::runtime_error(ss.str());
}

bool RemoteTwinCATImpl::receive(driver::RxDatagram& rx) {
  const AmsAddr p_addr = {this->_net_id, PORT};
  uint32_t receive_bytes;
  const auto ret = AdsSyncReadReqEx2(this->_port, &p_addr, INDEX_GROUP, INDEX_OFFSET_BASE_READ,
                                     static_cast<uint32_t>(rx.messages().size() * sizeof(driver::RxMessage)), rx.messages().data(), &receive_bytes);
  if (ret == 0) return true;

  std::stringstream ss;
  ss << "Error on receiving data: " << std::hex << ret;
  throw std::runtime_error(ss.str());
}

bool RemoteTwinCATImpl::is_open() { return this->_port > 0; }

}  // namespace autd3::link
