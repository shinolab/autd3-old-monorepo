// File: pcap_interface.hpp
// Project: Lib
// Created Date: 07/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <stdexcept>
#include <string>

#include "../result.hpp"
#include "pcap.h"

#ifdef _WIN32
#include <timeapi.h>
#endif

namespace autd3::link::pcap {

class PcapInterface final {
 public:
  explicit PcapInterface() : _pcap(nullptr), _closed(true) {}

  void open(const std::string& ifname) {
    char errbuf[256] = {};
    _pcap = pcap_open(ifname.c_str(), 65536, PCAP_OPENFLAG_PROMISCUOUS | PCAP_OPENFLAG_MAX_RESPONSIVENESS | PCAP_OPENFLAG_NOCAPTURE_LOCAL, -1,
                      nullptr, errbuf);
    if (_pcap == nullptr) throw std::runtime_error(errbuf);

#ifdef _WIN32
    timeBeginPeriod(1);
#endif

    _closed = false;
  }

  EmemResult send(const uint8_t* data, const size_t size) const {
    if (pcap_sendpacket(_pcap, data, static_cast<int32_t>(size)) == PCAP_ERROR) return EmemResult::SendFrame;
    return EmemResult::Ok;
  }

  EmemResult read(uint8_t* data, const size_t size) const {
    pcap_pkthdr* header = nullptr;
    const u_char* data_recv = nullptr;
    if (pcap_next_ex(_pcap, &header, &data_recv) <= 0) return EmemResult::ReceiveFrame;

    std::memcpy(data, data_recv, (std::min)(static_cast<size_t>(header->len), size));
    return EmemResult::Ok;
  }

  void close() {
    if (_closed) return;
#ifdef _WIN32
    timeEndPeriod(1);
#endif
    _closed = true;

    if (_pcap) pcap_close(_pcap);
    _pcap = nullptr;
  }

  ~PcapInterface() = default;
  PcapInterface(const PcapInterface& v) = default;
  PcapInterface& operator=(const PcapInterface& obj) = default;
  PcapInterface(PcapInterface&& obj) = default;
  PcapInterface& operator=(PcapInterface&& obj) = default;

 private:
  pcap_t* _pcap;
  bool _closed;
};

}  // namespace autd3::link::pcap
