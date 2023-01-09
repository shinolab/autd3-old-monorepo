﻿// File: remote_twincat.cpp
// Project: remote_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26495)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#endif
#include <AdsLib.h>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

#if WIN32
#include <Windows.h>
#endif

#include <string>
#include <vector>

#include "../../spdlog.hpp"
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

bool startup() {
#ifdef _WIN32
  WSADATA wsa_data;
  if (WSAStartup(MAKEWORD(2, 2), &wsa_data) != 0) throw std::runtime_error("WSAStartup failed: " + std::to_string(WSAGetLastError()));
#endif
  return true;
}

bool cleanup() {
#ifdef _WIN32
  if (WSACleanup() != 0) throw std::runtime_error("WSACleanup failed: " + std::to_string(WSAGetLastError()));
#endif
  return true;
}

}  // namespace

constexpr uint32_t INDEX_GROUP = 0x3040030;
constexpr uint32_t INDEX_OFFSET_BASE = 0x81000000;
constexpr uint32_t INDEX_OFFSET_BASE_READ = 0x80000000;
constexpr uint16_t PORT = 301;

class RemoteTwinCATImpl final : public core::Link {
 public:
  RemoteTwinCATImpl(std::string ipv4_addr, std::string remote_ams_net_id, std::string local_ams_net_id)
      : Link(), _client_ams_net_id(std::move(local_ams_net_id)), _server_ams_net_id(std::move(remote_ams_net_id)), _server_ip(std::move(ipv4_addr)) {}
  ~RemoteTwinCATImpl() override = default;
  RemoteTwinCATImpl(const RemoteTwinCATImpl& v) noexcept = delete;
  RemoteTwinCATImpl& operator=(const RemoteTwinCATImpl& obj) = delete;
  RemoteTwinCATImpl(RemoteTwinCATImpl&& obj) = delete;
  RemoteTwinCATImpl& operator=(RemoteTwinCATImpl&& obj) = delete;

  bool open(const core::Geometry&) override {
    const auto octets = split(_server_ams_net_id, '.');
    if (octets.size() != 6) throw std::runtime_error("Ams net id must have 6 octets");

    if (_server_ip.empty()) {
      for (auto i = 0; i < 3; i++) _server_ip += octets[i] + ".";
      _server_ip += octets[3];
    }

    if (!_client_ams_net_id.empty()) {
      const auto local_octets = split(_client_ams_net_id, '.');
      if (local_octets.size() != 6) throw std::runtime_error("Ams net id must have 6 octets");
      bhf::ads::SetLocalAddress({static_cast<uint8_t>(std::stoi(local_octets[0])), static_cast<uint8_t>(std::stoi(local_octets[1])),
                                 static_cast<uint8_t>(std::stoi(local_octets[2])), static_cast<uint8_t>(std::stoi(local_octets[3])),
                                 static_cast<uint8_t>(std::stoi(local_octets[4])), static_cast<uint8_t>(std::stoi(local_octets[5]))});
    }

    this->_net_id = {static_cast<uint8_t>(std::stoi(octets[0])), static_cast<uint8_t>(std::stoi(octets[1])),
                     static_cast<uint8_t>(std::stoi(octets[2])), static_cast<uint8_t>(std::stoi(octets[3])),
                     static_cast<uint8_t>(std::stoi(octets[4])), static_cast<uint8_t>(std::stoi(octets[5]))};

    startup();
    if (const auto res = AdsAddRoute(this->_net_id, _server_ip.c_str()); res != 0) {
      cleanup();
      throw std::runtime_error("Could not connect to remote: " + std::to_string(res));
    }

    this->_port = AdsPortOpenEx();

    if (this->_port == 0) {
      cleanup();
      throw std::runtime_error("Failed to open a new ADS port");
    }

    return true;
  }

  bool close() override {
    if (this->_port == 0) return true;
    if (AdsPortCloseEx(this->_port) != 0) throw std::runtime_error("Failed to close");
    this->_port = 0;
    return cleanup();
  }

  bool send(const driver::TxDatagram& tx) override {
    const AmsAddr p_addr = {this->_net_id, PORT};
    const auto ret = AdsSyncWriteReqEx(this->_port, &p_addr, INDEX_GROUP, INDEX_OFFSET_BASE, static_cast<uint32_t>(tx.transmitting_size_in_bytes()),
                                       tx.data().data());
    if (ret == 0) return true;
    if (ret == ADSERR_DEVICE_INVALIDSIZE) throw std::runtime_error("The number of devices is invalid.");
    throw std::runtime_error("Error on sending data: " + std::to_string(ret));
  }

  bool receive(driver::RxDatagram& rx) override {
    const AmsAddr p_addr = {this->_net_id, PORT};
    uint32_t receive_bytes;
    const auto ret = AdsSyncReadReqEx2(this->_port, &p_addr, INDEX_GROUP, INDEX_OFFSET_BASE_READ,
                                       static_cast<uint32_t>(rx.messages().size() * sizeof(driver::RxMessage)), rx.messages().data(), &receive_bytes);
    if (ret == 0) return true;
    throw std::runtime_error("Error on receiving data: " + std::to_string(ret));
  }

  bool is_open() override { return this->_port > 0; }

 private:
  std::string _client_ams_net_id;
  std::string _server_ams_net_id;
  std::string _server_ip;
  long _port = 0L;  // NOLINT
  AmsNetId _net_id;
};

core::LinkPtr RemoteTwinCAT::build() { return std::make_unique<RemoteTwinCATImpl>(_server_ip_address, _server_ams_net_id, _client_ams_net_id); }

}  // namespace autd3::link
