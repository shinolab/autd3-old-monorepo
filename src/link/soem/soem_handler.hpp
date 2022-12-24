// File: link_soem_impl.hpp
// Project: soem
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <atomic>
#include <cstdint>
#include <cstring>
#include <functional>
#include <memory>
#include <mutex>
#include <queue>
#include <string>
#include <thread>
#include <utility>
#include <vector>

#include "../../spdlog.hpp"
#include "autd3/driver/common/cpu/datagram.hpp"
#include "autd3/driver/common/cpu/ec_config.hpp"
#include "autd3/link/soem.hpp"
#include "ecat_thread.hpp"
#include "error_handler.hpp"

namespace autd3::link {

class SOEMHandler final {
 public:
  SOEMHandler(const bool high_precision, std::string ifname, const uint16_t sync0_cycle, const uint16_t send_cycle,
              std::function<void(std::string)> on_lost, const SyncMode sync_mode, const std::chrono::milliseconds state_check_interval)
      : _high_precision(high_precision),
        _ifname(std::move(ifname)),
        _sync0_cycle(sync0_cycle),
        _send_cycle(send_cycle),
        _on_lost(std::move(on_lost)),
        _sync_mode(sync_mode),
        _is_open(false),
        _state_check_interval(state_check_interval) {}
  ~SOEMHandler() {
    try {
      close();
    } catch (std::exception& ex) {
      spdlog::error(ex.what());
    }
  }
  SOEMHandler(const SOEMHandler& v) noexcept = delete;
  SOEMHandler& operator=(const SOEMHandler& obj) = delete;
  SOEMHandler(SOEMHandler&& obj) = delete;
  SOEMHandler& operator=(SOEMHandler&& obj) = delete;

  static std::vector<EtherCATAdapter> enumerate_adapters() {
    auto* adapter = ec_find_adapters();
    std::vector<EtherCATAdapter> adapters;
    while (adapter != nullptr) {
      EtherCATAdapter info(std::string(adapter->desc), std::string(adapter->name));
      adapters.emplace_back(info);
      adapter = adapter->next;
    }
    ec_free_adapters(adapter);
    return adapters;
  }

  static std::string lookup_autd() {
    spdlog::debug("looking for AUTD...");
    auto* adapters = ec_find_adapters();
    for (const auto* adapter = adapters; adapter != nullptr; adapter = adapter->next) {
      spdlog::debug("Checking on {} ({})...", adapter->name, adapter->desc);
      if (ec_init(adapter->name) <= 0) {
        ec_close();
        continue;
      }
      const auto wc = ec_config_init(0);
      if (wc <= 0) {
        ec_close();
        continue;
      }
      bool found = true;
      for (auto i = 1; i <= wc; i++)
        if (std::strcmp(ec_slave[i].name, "AUTD") != 0) {
          found = false;
          spdlog::warn("EtherCAT slaves were found on {} ({}), but {}-th device is not AUTD3", adapter->name, adapter->desc, i);
          ec_close();
          break;
        }
      if (found) {
        spdlog::debug("AUTD3 found on {} ({})", adapter->name, adapter->desc);
        auto ifname = std::string(adapter->name);
        ec_free_adapters(adapters);
        ec_close();
        return ifname;
      }
    }
    ec_free_adapters(adapters);
    spdlog::error("No AUTD3 devices found");
    return "";
  }

  size_t open(const std::vector<size_t>& device_map, const int remaining) {
    if (is_open()) return 0;

    std::queue<driver::TxDatagram>().swap(_send_buf);

    const auto cycle_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _send_cycle;
    spdlog::debug("send interval: {} [ns]", cycle_time);

    if (_ifname.empty()) _ifname = lookup_autd();
    if (_ifname.empty()) return 0;

    spdlog::debug("interface name: {}", _ifname);
    if (ec_init(_ifname.c_str()) <= 0) {
      spdlog::error("No socket connection on {}", _ifname);
      return 0;
    }

    const auto wc = ec_config_init(0);
    if (wc <= 0) {
      spdlog::error("No slaves found");
      return 0;
    }
    spdlog::debug("Found {} devices", wc);

    const auto auto_detect = device_map.empty();
    if (!auto_detect && static_cast<size_t>(wc) != device_map.size()) {
      spdlog::error("The number of slaves you configured: {}, but found: {}", device_map.size(), wc);
      return 0;
    }
    std::vector<size_t> dev_map;
    for (auto i = 1; i <= wc; i++)
      if (std::strcmp(ec_slave[i].name, "AUTD") == 0) {
        dev_map.emplace_back(auto_detect ? 249 : device_map[static_cast<size_t>(i) - 1]);
      } else {
        spdlog::error("Slave[{}] is not AUTD3", i);
        return 0;
      }

    _user_data = std::make_unique<uint32_t[]>(1);
    _user_data[0] = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle;
    ecx_context.userdata = _user_data.get();
    spdlog::debug("Sync0 interval: {} [ns]", driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle);
    if (_sync_mode == SyncMode::DC) {
      for (int cnt = 1; cnt <= ec_slavecount; cnt++)
        ec_slave[cnt].PO2SOconfigx = [](auto* context, auto slave) -> int {
          const auto cyc_time = static_cast<uint32_t*>(context->userdata)[0];
          ec_dcsync0(slave, true, cyc_time, 0U);
          return 0;
        };
      spdlog::debug("run mode: DC sync");
      spdlog::debug("Sync0 configured");
    }

    _io_map.resize(dev_map);
    ec_config_map(_io_map.get());

    ec_configdc();

    ec_statecheck(0, EC_STATE_SAFE_OP, EC_TIMEOUTSTATE);
    if (ec_slave[0].state != EC_STATE_SAFE_OP) {
      spdlog::error("One ore more slaves did not reach safe operational state: {}", ec_slave[0].state);
      ec_readstate();
      for (size_t slave = 1; slave <= static_cast<size_t>(ec_slavecount); slave++)
        if (ec_slave[slave].state != EC_STATE_SAFE_OP)
          spdlog::error("Slave[{}]: {} (State={:#02x} StatusCode={:#04x})", slave, ec_ALstatuscode2string(ec_slave[slave].ALstatuscode),
                        ec_slave[slave].state, ec_slave[slave].ALstatuscode);
      return false;
    }

    const auto expected_wkc = ec_group[0].outputsWKC * 2 + ec_group[0].inputsWKC;
    spdlog::debug("Calculated workcounter {}", expected_wkc);

    ec_slave[0].state = EC_STATE_OPERATIONAL;

    ec_send_processdata();
    ec_receive_processdata(EC_TIMEOUTRET);

    _is_open.store(true);
    _ecat_thread = std::thread([this, cycle_time] {
      ecat_run(this->_high_precision, &this->_is_open, &this->_wkc, cycle_time, this->_send_mtx, this->_send_buf, this->_io_map);
    });

    ec_writestate(0);

    ec_statecheck(0, EC_STATE_OPERATIONAL, EC_TIMEOUTSTATE);
    if (ec_slave[0].state != EC_STATE_OPERATIONAL) {
      _is_open.store(false);
      if (_ecat_thread.joinable()) _ecat_thread.join();
      if (remaining == 0) {
        spdlog::error("One ore more slaves are not responding: {}", ec_slave[0].state);
        ec_readstate();
        for (size_t slave = 1; slave <= static_cast<size_t>(ec_slavecount); slave++)
          if (ec_slave[slave].state != EC_STATE_SAFE_OP)
            spdlog::error("Slave {} State={:#02x} StatusCode={:#04x} : {}", slave, ec_slave[slave].state, ec_slave[slave].ALstatuscode,
                          ec_ALstatuscode2string(ec_slave[slave].ALstatuscode));
        return static_cast<size_t>(wc);
      }
      spdlog::debug("Failed to reach op mode. retry opening...");
      return open(device_map, remaining - 1);
    }

    if (_sync_mode == SyncMode::FreeRun) {
      for (int slave = 1; slave <= ec_slavecount; slave++)
        ec_dcsync0(static_cast<uint16_t>(slave), true, driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle, 0U);
      spdlog::debug("run mode: Free Run");
      spdlog::debug("Sync0 configured");
    }

    spdlog::debug("Run EC state check thread, interval: {} [ms]", _state_check_interval.count());
    _ecat_check_thread = std::thread([this, expected_wkc] {
      while (this->_is_open.load()) {
        if (this->_wkc.load() < expected_wkc || ec_group[0].docheckstate)
          if (!error_handle(this->_on_lost)) break;
        std::this_thread::sleep_for(_state_check_interval);
      }
    });

    return static_cast<size_t>(wc);
  }

  bool send(const driver::TxDatagram& tx) {
    if (!is_open()) {
      spdlog::error("link is closed");
      return false;
    }

    std::lock_guard lock(_send_mtx);
    _send_buf.push(tx.clone());
    return true;
  }

  bool receive(driver::RxDatagram& rx) const {
    if (!is_open()) {
      spdlog::error("link is closed");
      return false;
    }
    rx.copy_from(_io_map.input());
    return true;
  }

  bool close() {
    if (!is_open()) return true;
    _is_open.store(false);

    spdlog::debug("Stopping ethercat thread...");
    if (_ecat_thread.joinable()) _ecat_thread.join();
    spdlog::debug("Stopping ethercat thread...done");
    spdlog::debug("Stopping state check thread...");
    if (_ecat_check_thread.joinable()) _ecat_check_thread.join();
    spdlog::debug("Stopping state check thread...done");

    const auto cyc_time = static_cast<uint32_t*>(ecx_context.userdata)[0];
    for (uint16_t slave = 1; slave <= static_cast<uint16_t>(ec_slavecount); slave++) ec_dcsync0(slave, false, cyc_time, 0U);

    ec_slave[0].state = EC_STATE_INIT;
    ec_writestate(0);

    ec_close();

    return true;
  }

  bool is_open() const { return _is_open.load(); }

 private:
  bool _high_precision;
  std::string _ifname;
  uint16_t _sync0_cycle;
  uint16_t _send_cycle;

  std::atomic<int32_t> _wkc;

  std::function<void(std::string)> _on_lost = nullptr;

  SyncMode _sync_mode;

  IOMap _io_map;

  std::atomic<bool> _is_open;
  std::unique_ptr<uint32_t[]> _user_data;

  std::thread _ecat_thread;
  std::thread _ecat_check_thread;

  std::queue<driver::TxDatagram> _send_buf;
  std::mutex _send_mtx;

  std::chrono::milliseconds _state_check_interval;
};

}  // namespace autd3::link
