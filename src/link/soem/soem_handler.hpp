// File: link_soem_impl.hpp
// Project: soem
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 20/03/2023
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
#include "autd3/core/utils/hint.hpp"
#include "autd3/core/utils/osal_timer.hpp"
#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "autd3/link/soem.hpp"
#include "ecat.hpp"
#include "error_handler.hpp"

namespace autd3::link {

struct SOEMCallback final : core::CallbackHandler {
  ~SOEMCallback() override = default;
  SOEMCallback(const SOEMCallback& v) noexcept = delete;
  SOEMCallback& operator=(const SOEMCallback& obj) = delete;
  SOEMCallback(SOEMCallback&& obj) = delete;
  SOEMCallback& operator=(SOEMCallback&& obj) = delete;

  explicit SOEMCallback(std::atomic<int32_t>& wkc, std::queue<driver::TxDatagram>& send_buf, std::mutex& send_mtx, IOMap& io_map)
      : _rt_lock(false), _wkc(wkc), _send_buf(send_buf), _send_mtx(send_mtx), _io_map(io_map) {}

  void callback() override {
    if (auto expected = false; _rt_lock.compare_exchange_weak(expected, true)) {
      ec_send_processdata();
      _wkc.store(ec_receive_processdata(EC_TIMEOUTRET));
      if (!_send_buf.empty()) {
        _io_map.copy_from(_send_buf.front());
        {
          std::lock_guard lock(_send_mtx);
          _send_buf.pop();
        }
      }
      _rt_lock.store(false, std::memory_order_release);
    }
  }

 private:
  std::atomic<bool> _rt_lock;

  std::atomic<int32_t>& _wkc;
  std::queue<driver::TxDatagram>& _send_buf;
  std::mutex& _send_mtx;
  IOMap& _io_map;
};

class SOEMHandler final {
 public:
  SOEMHandler(const core::TimerStrategy timer_strategy, std::string ifname, const uint16_t sync0_cycle, const uint16_t send_cycle,
              std::function<void(std::string)> on_lost, const SyncMode sync_mode, const std::chrono::milliseconds state_check_interval,
              std::shared_ptr<spdlog::logger> logger)
      : _timer_strategy(timer_strategy),
        _ifname(std::move(ifname)),
        _sync0_cycle(sync0_cycle),
        _send_cycle(send_cycle),
        _on_lost(std::move(on_lost)),
        _sync_mode(sync_mode),
        _is_open(false),
        _state_check_interval(state_check_interval),
        _logger(std::move(logger)) {}
  ~SOEMHandler() {
    try {
      close();
    } catch (std::exception& ex) {
      _logger->debug(ex.what());
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

  std::string lookup_autd() {
    _logger->debug("looking for AUTD...");
    auto* adapters = ec_find_adapters();
    for (const auto* adapter = adapters; adapter != nullptr; adapter = adapter->next) {
      _logger->debug("Checking on {} ({})...", adapter->name, adapter->desc);
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
          _logger->debug("EtherCAT slaves were found on {} ({}), but {}-th device is not AUTD3", adapter->name, adapter->desc, i);
          ec_close();
          break;
        }
      if (found) {
        _logger->debug("AUTD3 found on {} ({})", adapter->name, adapter->desc);
        auto ifname = std::string(adapter->name);
        ec_free_adapters(adapters);
        ec_close();
        return ifname;
      }
    }
    ec_free_adapters(adapters);
    throw std::runtime_error("No AUTD3 devices found");
  }

  size_t open(const std::vector<size_t>& device_map) {
    if (is_open()) return 0;

    std::queue<driver::TxDatagram>().swap(_send_buf);

    const uint32_t cycle_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _send_cycle;
    _logger->debug("send interval: {} [ns]", cycle_time);

    if (_ifname.empty()) _ifname = lookup_autd();
    if (_ifname.empty()) return 0;

    _logger->debug("interface name: {}", _ifname);
    if (ec_init(_ifname.c_str()) <= 0) throw std::runtime_error("No socket connection on " + _ifname);

    const auto wc = ec_config_init(0);
    if (wc <= 0) throw std::runtime_error("No slaves found");
    _logger->debug("Found {} devices", wc);

    const auto auto_detect = device_map.empty();
    if (!auto_detect && static_cast<size_t>(wc) != device_map.size())
      throw std::runtime_error("The number of slaves you configured: " + std::to_string(device_map.size()) + ", but found: " + std::to_string(wc));
    std::vector<size_t> dev_map;
    for (auto i = 1; i <= wc; i++)
      if (std::strcmp(ec_slave[i].name, "AUTD") == 0) {
        dev_map.emplace_back(auto_detect ? 249 : device_map[static_cast<size_t>(i) - 1]);
      } else
        throw std::runtime_error("Slave[" + std::to_string(i) + "] is not AUTD3");

    _user_data = std::make_unique<uint32_t[]>(1);
    _user_data[0] = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle;
    ecx_context.userdata = _user_data.get();
    _logger->debug("Sync0 interval: {} [ns]", driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle);
    if (_sync_mode == SyncMode::DC) {
      _logger->debug("run mode: DC");
      for (int cnt = 1; cnt <= ec_slavecount; cnt++)
        ec_slave[cnt].PO2SOconfigx = [](auto* context, auto slave) -> int {
          const auto cyc_time = static_cast<uint32_t*>(context->userdata)[0];
          ec_dcsync0(slave, true, cyc_time, 0U);
          return 0;
        };
    }

    ec_configdc();

    _io_map.resize(dev_map);
    ec_config_map(_io_map.get());

    ec_statecheck(0, EC_STATE_SAFE_OP, EC_TIMEOUTSTATE);
    ec_readstate();
    if (ec_slave[0].state != EC_STATE_SAFE_OP) {
      for (size_t slave = 1; slave <= static_cast<size_t>(ec_slavecount); slave++)
        if (ec_slave[slave].state != EC_STATE_SAFE_OP)
          _logger->debug("Slave[{}]: {} (State={:#02x} StatusCode={:#04x})", slave, ec_ALstatuscode2string(ec_slave[slave].ALstatuscode),
                         ec_slave[slave].state, ec_slave[slave].ALstatuscode);
      throw std::runtime_error("One ore more slaves did not reach safe operational state");
    }

    const auto expected_wkc = ec_group[0].outputsWKC * 2 + ec_group[0].inputsWKC;
    _logger->debug("Calculated workcounter {}", expected_wkc);

    ec_slave[0].state = EC_STATE_OPERATIONAL;
    ec_writestate(0);

    _is_open.store(true);

    switch (_timer_strategy) {
      case core::TimerStrategy::BusyWait:
        _ecat_thread = std::thread([this, cycle_time] { ecat_run<busy_wait>(cycle_time); });
        break;
      case core::TimerStrategy::Sleep:
        _ecat_thread = std::thread([this, cycle_time] { ecat_run<wait_with_sleep>(cycle_time); });
        break;
      case core::TimerStrategy::NativeTimer:
        _timer = core::Timer<SOEMCallback>::start(std::make_unique<SOEMCallback>(_wkc, _send_buf, _send_mtx, _io_map), cycle_time);
        break;
    }

    ec_statecheck(0, EC_STATE_OPERATIONAL, 5 * EC_TIMEOUTSTATE);
    if (ec_slave[0].state != EC_STATE_OPERATIONAL) {
      _is_open.store(false);
      close_th();
      ec_readstate();
      for (size_t slave = 1; slave <= static_cast<size_t>(ec_slavecount); slave++)
        if (ec_slave[slave].state != EC_STATE_SAFE_OP)
          _logger->debug("Slave {} State={:#02x} StatusCode={:#04x} : {}", slave, ec_slave[slave].state, ec_slave[slave].ALstatuscode,
                         ec_ALstatuscode2string(ec_slave[slave].ALstatuscode));
      throw std::runtime_error("One ore more slaves are not responding.");
    }

    if (_sync_mode == SyncMode::FreeRun) {
      _logger->debug("run mode: FreeRun");
      for (int slave = 1; slave <= ec_slavecount; slave++) {
        ec_dcsync0(static_cast<uint16_t>(slave), true, driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle, 0U);
        _logger->debug("Sync0 configured on slave[{}]", slave);
      }
    }

    _logger->debug("Run EC state check thread, interval: {} [ms]", _state_check_interval.count());
    _ecat_check_thread = std::thread([this, expected_wkc] {
      while (this->_is_open.load()) {
        if (this->_wkc.load() < expected_wkc || ec_group[0].docheckstate)
          if (!error_handle(_logger, this->_on_lost)) break;
        std::this_thread::sleep_for(_state_check_interval);
      }
    });

    return static_cast<size_t>(wc);
  }

  bool send(const driver::TxDatagram& tx) {
    if (!is_open()) throw std::runtime_error("link is closed");

    std::lock_guard lock(_send_mtx);
    _send_buf.push(tx.clone());
    return true;
  }

  bool receive(driver::RxDatagram& rx) const {
    if (!is_open()) throw std::runtime_error("link is closed");

    rx.copy_from(_io_map.input());
    return true;
  }

  void close_th() {
    switch (_timer_strategy) {
      case core::TimerStrategy::BusyWait:
      case core::TimerStrategy::Sleep:
        if (_ecat_thread.joinable()) _ecat_thread.join();
        break;
      case core::TimerStrategy::NativeTimer:
        const auto _ = _timer->stop();
        break;
    }
  }

  bool close() {
    if (!is_open()) return true;
    _is_open.store(false);

    close_th();
    if (_ecat_check_thread.joinable()) _ecat_check_thread.join();

    const auto cyc_time = static_cast<uint32_t*>(ecx_context.userdata)[0];
    for (uint16_t slave = 1; slave <= static_cast<uint16_t>(ec_slavecount); slave++) ec_dcsync0(slave, false, cyc_time, 0U);

    ec_slave[0].state = EC_STATE_INIT;
    ec_writestate(0);

    ec_close();

    return true;
  }

  bool is_open() const { return _is_open.load(); }

 private:
  core::TimerStrategy _timer_strategy;
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

  std::unique_ptr<core::Timer<SOEMCallback>> _timer;

  std::queue<driver::TxDatagram> _send_buf;
  std::mutex _send_mtx;

  std::chrono::milliseconds _state_check_interval;

  std::shared_ptr<spdlog::logger> _logger;

  using WaitFunc = void(const timespec&);

  static void wait_with_sleep(const timespec& abs_time) {
    auto tp = timeval{0, 0};
    gettimeofday(&tp, nullptr);
    if (const auto sleep = (static_cast<int64_t>(abs_time.tv_sec) - static_cast<int64_t>(tp.tv_sec)) * 1000000000LL +
                           (static_cast<int64_t>(abs_time.tv_nsec) - static_cast<int64_t>(tp.tv_usec) * 1000LL);
        sleep > 0)
      std::this_thread::sleep_for(std::chrono::nanoseconds(sleep));
  }

  static void busy_wait(const timespec& abs_time) {
    auto tp = timeval{0, 0};
    gettimeofday(&tp, nullptr);

    const auto sleep = (static_cast<int64_t>(abs_time.tv_sec) - static_cast<int64_t>(tp.tv_sec)) * 1000000000LL +
                       (static_cast<int64_t>(abs_time.tv_nsec) - static_cast<int64_t>(tp.tv_usec) * 1000LL);
    const auto expired = std::chrono::high_resolution_clock::now() + std::chrono::nanoseconds(sleep);
    while (std::chrono::high_resolution_clock::now() < expired) core::spin_loop_hint();
  }

  template <WaitFunc W>
  void ecat_run(const uint32_t cycletime_ns) {
    auto ts = ecat_setup(cycletime_ns);
    int64_t toff = 0;
    ec_send_processdata();
    while (_is_open.load()) {
      ec_sync(ec_DCtime, cycletime_ns, &toff);

      _wkc.store(ec_receive_processdata(EC_TIMEOUTRET));
      if (!_send_buf.empty()) {
        _io_map.copy_from(_send_buf.front());
        {
          std::lock_guard lock(_send_mtx);
          _send_buf.pop();
        }
      }

      add_timespec(ts, cycletime_ns + toff);
      W(ts);

      ec_send_processdata();
    }
  }
};

}  // namespace autd3::link
