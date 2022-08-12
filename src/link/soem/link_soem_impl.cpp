// File: autdsoem.cpp
// Project: autdsoem
// Created Date: 23/08/2019
// Author: Shun Suzuki
// -----
// Last Modified: 12/08/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2019-2020 Shun Suzuki. All rights reserved.
//

#include "link_soem_impl.hpp"

#include <algorithm>
#include <cstdint>
#include <iostream>
#include <limits>
#include <memory>
#include <queue>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "./ethercat.h"
#include "autd3/link/soem.hpp"
#include "ecat_thread/ecat_thread.hpp"

namespace {
std::string lookup_autd() {
  pcap_if_t* alldevs = nullptr;
  static char errbuf[PCAP_ERRBUF_SIZE] = {0};
  if (pcap_findalldevs(&alldevs, errbuf) == -1) {
    std::stringstream ss;
    ss << "failed to find devices: " << errbuf;
    throw std::runtime_error(ss.str());
  }

  for (const pcap_if_t* cursor = alldevs; cursor != nullptr; cursor = cursor->next) {
    if (ec_init(cursor->name) <= 0) continue;
    const auto wc = ec_config_init(0);
    if (wc <= 0) continue;
    if (std::strcmp(ec_slave[1].name, "AUTD") == 0) return std::string(cursor->name);
  }

  std::stringstream ss;
  ss << "No AUTD device is found.";
  throw std::runtime_error(ss.str());
}
}  // namespace

namespace autd3::link {

bool SOEMLink::is_open() { return _is_open.load(); }

bool SOEMLink::send(const driver::TxDatagram& tx) {
  if (!_is_open.load()) throw std::runtime_error("link is closed");

  std::lock_guard lock(_send_mtx);
  _send_buf.push(tx.clone());
  return true;
}

bool SOEMLink::receive(driver::RxDatagram& rx) {
  if (!_is_open.load()) throw std::runtime_error("link is closed");
  rx.copy_from(_io_map.input());
  return true;
}

void SOEMLink::open(const core::Geometry& geometry) {
  const auto dev_num = geometry.num_devices();

  if (_ifname.empty()) _ifname = lookup_autd();

  if (ec_init(_ifname.c_str()) <= 0) {
    std::stringstream ss;
    ss << "No socket connection on " << _ifname;
    throw std::runtime_error(ss.str());
  }

  const auto wc = ec_config_init(0);
  if (wc <= 0) throw std::runtime_error("No slaves found!");

  for (auto i = 1; i <= wc; i++)
    if (std::strcmp(ec_slave[i].name, "AUTD") != 0) {
      std::stringstream ss;
      ss << "Slave[" << i << "] is not AUTD.";
      throw std::runtime_error(ss.str());
    }

  if (static_cast<size_t>(wc) != dev_num) {
    std::stringstream ss;
    ss << "The number of slaves you configured: " << dev_num << ", but found: " << wc;
    throw std::runtime_error(ss.str());
  }

  _user_data = std::make_unique<uint32_t[]>(1);
  _user_data[0] = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle;
  ecx_context.userdata = _user_data.get();
  auto dc_config = [](ecx_contextt* const context, const uint16_t slave) -> int {
    const auto cyc_time = static_cast<uint32_t*>(context->userdata)[0];
    ec_dcsync0(slave, true, cyc_time, 0U);
    return 0;
  };

  if (_sync_mode == SYNC_MODE::DC)
    for (int cnt = 1; cnt <= ec_slavecount; cnt++) ec_slave[cnt].PO2SOconfigx = dc_config;

  ec_configdc();

  _io_map.resize(dev_num);
  ec_config_map(_io_map.get());

  ec_statecheck(0, EC_STATE_SAFE_OP, EC_TIMEOUTSTATE * 4);
  ec_readstate();
  ec_slave[0].state = EC_STATE_OPERATIONAL;
  ec_writestate(0);

  const auto expected_wkc = ec_group[0].outputsWKC * 2 + ec_group[0].inputsWKC;
  const auto cycle_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _send_cycle;
  _is_running = true;
  std::queue<driver::TxDatagram>().swap(_send_buf);
  ecat_init();
  _ecat_thread = std::thread([this, expected_wkc, cycle_time] {
    ecat_run(this->_high_precision, &this->_is_open, &this->_is_running, expected_wkc, cycle_time, this->_send_mtx, this->_send_buf, this->_io_map,
             std::move(this->_on_lost));
  });

  std::this_thread::sleep_for(std::chrono::milliseconds(100));

  ec_statecheck(0, EC_STATE_OPERATIONAL, EC_TIMEOUTSTATE * 5);

  if (ec_slave[0].state != EC_STATE_OPERATIONAL) {
    _is_running = false;
    if (this->_ecat_thread.joinable()) this->_ecat_thread.join();
    throw std::runtime_error("One ore more slaves are not responding");
  }

  if (_sync_mode == SYNC_MODE::FREE_RUN)
    for (int cnt = 1; cnt <= ec_slavecount; cnt++) dc_config(&ecx_context, static_cast<uint16_t>(cnt));

  _is_open.store(true);
}

void SOEMLink::close() {
  if (!is_open()) return;

  while (!_send_buf.empty()) std::this_thread::sleep_for(std::chrono::milliseconds(1));

  _is_open.store(false);

  _is_running = false;
  if (this->_ecat_thread.joinable()) this->_ecat_thread.join();

  const auto cyc_time = static_cast<uint32_t*>(ecx_context.userdata)[0];
  for (uint16_t slave = 1; slave <= static_cast<uint16_t>(ec_slavecount); slave++) ec_dcsync0(slave, false, cyc_time, 0U);

  ec_slave[0].state = EC_STATE_SAFE_OP;
  ec_writestate(0);
  ec_statecheck(0, EC_STATE_SAFE_OP, EC_TIMEOUTSTATE);

  ec_slave[0].state = EC_STATE_PRE_OP;
  ec_writestate(0);
  ec_statecheck(0, EC_STATE_PRE_OP, EC_TIMEOUTSTATE);

  ec_close();
}

SOEMLink::~SOEMLink() {
  try {
    this->close();
  } catch (std::exception&) {
  }
}

core::LinkPtr SOEM::build() {
  return std::make_unique<SOEMLink>(_high_precision, _ifname, _device_num, _sync0_cycle, _send_cycle, std::move(_callback), _sync_mode);
}

std::vector<EtherCATAdapter> SOEM::enumerate_adapters() {
  auto* adapter = ec_find_adapters();
  std::vector<EtherCATAdapter> adapters;
  while (adapter != nullptr) {
    EtherCATAdapter info(std::string(adapter->desc), std::string(adapter->name));
    adapters.emplace_back(info);
    adapter = adapter->next;
  }
  return adapters;
}

void SOEM::diagnose(const std::string& ifname) {
  std::cout << "================================ pcap diagnotics ================================" << std::endl;
  char errbuf[PCAP_ERRBUF_SIZE] = {0};

  const auto ifname_ = ifname.empty() ? lookup_autd() : ifname;
  const auto* ifnamec = ifname_.c_str();
  pcap_t* psock =
      pcap_open(ifnamec, 65536, PCAP_OPENFLAG_PROMISCUOUS | PCAP_OPENFLAG_MAX_RESPONSIVENESS | PCAP_OPENFLAG_NOCAPTURE_LOCAL, -1, nullptr, errbuf);
  if (psock == nullptr) {
    std::cerr << "cannot open " << ifname << " with pcap\n";
    return;
  }

  pcap_addr_t* addr = nullptr;
  {
    pcap_if_t* alldevs = nullptr;
    if (0 != pcap_findalldevs(&alldevs, errbuf)) {
      std::cerr << "failed to find devices: " << errbuf << std::endl;
      return;
    }
    const pcap_if_t* dev;
    for (dev = alldevs; dev != nullptr; dev = dev->next)
      if (std::strcmp(dev->name, ifnamec) == 0) {
        std::cout << "Interface name       \t: " << dev->name << std::endl;
        std::cout << "Interface description\t: " << dev->description << std::endl;
        break;
      }
    if (dev == nullptr) {
      std::cerr << "failed to find " << ifname_ << std::endl;
      return;
    }
    for (addr = dev->addresses; addr != nullptr; addr = addr->next)
      if (addr->addr->sa_family == AF_INET) break;
    pcap_freealldevs(alldevs);
  }

  const auto* libv = pcap_lib_version();
  std::cout << "pcap lib version\t: " << libv << std::endl;

  {
    int packet_len = 64;
    uint8_t packet[64] =
        "\xff\xff\xff\xff\xff\xff\x02\x02\x02\x02\x02\x02\x08\x00\x45\x00\x00\x00\x12\x34\x00\x00\x10\x11\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\x00"
        "\x07\x00\x07\x00\x00\x00\x00";
    uint8_t* send_data;
    {
      packet[14 + 2] = 0xff & ((64 - 14) >> 8);
      packet[14 + 3] = 0xff & (64 - 14);
      packet[14 + 20 + 4] = 0xff & ((64 - 14 - 20) >> 8);
      packet[14 + 20 + 5] = 0xff & (64 - 14 - 20);
      *reinterpret_cast<u_long*>(packet + 14 + 12) = reinterpret_cast<sockaddr_in*>(addr->addr)->sin_addr.S_un.S_addr;
      uint32_t cksum = 0;
      for (int i = 14; i < 14 + 4 * (packet[14] & 0xf); i += 2) cksum += *reinterpret_cast<uint16_t*>(packet + i);
      while (cksum >> 16) cksum = (cksum & 0xffff) + (cksum >> 16);
      cksum = ~cksum;
      *reinterpret_cast<uint16_t*>(packet + 14 + 10) = static_cast<uint16_t>(cksum);
      switch (pcap_datalink(psock)) {
        case DLT_NULL:
          send_data = packet + (14 - 4);
          packet_len -= 14 - 4;
          send_data[0] = 2;
          send_data[1] = 0;
          send_data[2] = 0;
          send_data[3] = 0;
          break;
        case DLT_EN10MB:
          break;
        default:
          std::cerr << "Unknown data-link type: " << pcap_datalink(psock) << std::endl;
          return;
      }
    }

    std::cout << "collecting pcap send stats...";
    constexpr size_t ITER = 1000;
    std::vector<int64_t> stats;
    stats.reserve(ITER);
    for (size_t i = 0; i < ITER; i++) {
      auto start = std::chrono::high_resolution_clock::now();
      const int res = pcap_sendpacket(psock, send_data, packet_len);
      auto end = std::chrono::high_resolution_clock::now();
      if (res != 0) {
        std::cerr << "Error sending packet: " << pcap_geterr(psock) << std::endl;
        return;
      }
      stats.emplace_back(std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count());
      std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }

    int64_t ave = 0;
    int64_t min = std::numeric_limits<int64_t>::max();
    int64_t max = 0;
    for (const auto t : stats) {
      ave += t;
      min = std::min(min, t);
      max = std::max(max, t);
    }
    ave /= static_cast<int64_t>(stats.size());
    int64_t std = 0;
    for (const auto t : stats) std += (t - ave) * (t - ave);
    std /= static_cast<int64_t>(stats.size());
    std = static_cast<int64_t>(std::sqrt(static_cast<double>(std)));

    std::cout << "\r\x1b[K";
    std::cout << "Send time mean\t\t: " << static_cast<double>(ave) / 1000.0 << " [us]" << std::endl;
    std::cout << "           std\t\t: " << static_cast<double>(std) / 1000.0 << " [us]" << std::endl;
    std::cout << "           min\t\t: " << static_cast<double>(min) / 1000.0 << " [us]" << std::endl;
    std::cout << "           max\t\t: " << static_cast<double>(max) / 1000.0 << " [us]" << std::endl;
  }

  std::cout << "================================ pcap diagnotics ================================" << std::endl;
}

}  // namespace autd3::link
