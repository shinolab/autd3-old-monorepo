// File: emem.cpp
// Project: emem
// Created Date: 04/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "autd3/link/emem.hpp"

#include <mutex>
#include <queue>

#include "../../spdlog.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "iomap.hpp"
#include "master.hpp"
#include "pcap/adapter.hpp"
#include "pcap/pcap_interface.hpp"

#if WIN32
#include "ecat_thread/win32.hpp"
#elif __APPLE__
#include "ecat_thread/macosx.hpp"
#else
#include "ecat_thread/linux.hpp"
#endif

namespace autd3::link {

class EmemLink final : public core::Link {
 public:
  EmemLink(const bool high_precision, std::string ifname, const uint16_t sync0_cycle, const uint16_t send_cycle,
           std::function<void(std::string)> on_lost, const SyncMode sync_mode, const std::chrono::milliseconds state_check_interval)
      : _high_precision(high_precision),
        _ifname(std::move(ifname)),
        _sync0_cycle(sync0_cycle),
        _send_cycle(send_cycle),
        _on_lost(std::move(on_lost)),
        _sync_mode(sync_mode),
        _is_open(false),
        _state_check_interval(state_check_interval) {}

  [[nodiscard]] bool open(const core::Geometry& geometry) override {
    if (is_open()) return false;

    std::queue<driver::TxDatagram>().swap(_send_buf);

    if (_ifname.empty()) _ifname = lookup_autd();
    if (_ifname.empty()) return false;

    spdlog::warn("IFNAME: {}", _ifname);

    _master = std::make_unique<Master<pcap::PcapInterface>>(pcap::PcapInterface(_ifname));

    uint16_t wc{};
    if (const auto res = _master->initialize(&wc); res != EmemResult::Ok) throw std::runtime_error("Failed to initialize");
    if (wc == 0) throw std::runtime_error("No slaves found");

    const uint32_t cyc_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle;
    if (_sync_mode == SyncMode::DC) {
      for (size_t i = 1; i <= wc; i++)
        (*_master)[i].po_to_so_config = [cyc_time, i, this] {
          if (const auto r = _master->set_dc_sync0(i, true, cyc_time, 0U); r != EmemResult::Ok) throw std::runtime_error("Failed to configure DC");
        };
    }

    if (const auto r = _master->config_dc(); r != EmemResult::Ok) throw std::runtime_error("Failed to configure DC");

    _io_map.resize(geometry.device_map());
    if (const auto r = _master->config(_io_map.get()); r != EmemResult::Ok) throw std::runtime_error("Failed to configure IO map");

    EcState unused_state{};
    (void)_master->state_check(0, EcState::SafeOp, EC_TIMEOUT_SAFE, &unused_state);
    (void)_master->read_state(&unused_state);

    if ((*_master)[0].state != EcState::SafeOp) throw std::runtime_error("One ore more slaves did not reach safe operational state");

    const auto expected_wkc = _master->expected_wkc();

    (void)_master->write_state(0, EcState::Operational);

    _is_open.store(true);
    _ecat_thread = std::thread([this] { ecat_run(); });

    (void)_master->state_check(0, EcState::Operational, 5 * EC_TIMEOUT_STATE, &unused_state);
    if ((*_master)[0].state != EcState::Operational) {
      _is_open.store(false);
      if (_ecat_thread.joinable()) _ecat_thread.join();
      throw std::runtime_error("One ore more slaves are not responding.");
    }

    if (_sync_mode == SyncMode::FreeRun) {
      for (size_t i = 1; i < wc; i++)
        if (const auto r = _master->set_dc_sync0(i, true, cyc_time, 0U); r != EmemResult::Ok) throw std::runtime_error("Failed to configure DC");
    }

    _ecat_check_thread = std::thread([this, expected_wkc] {
      while (this->_is_open.load()) {
        if (this->_wkc.load() < expected_wkc)
          if (!error_handle()) break;
        std::this_thread::sleep_for(_state_check_interval);
      }
    });

    return static_cast<size_t>(wc);
  }

  [[nodiscard]] bool close() override {
    if (!is_open()) return true;
    _is_open.store(false);

    if (_ecat_thread.joinable()) _ecat_thread.join();
    if (_ecat_check_thread.joinable()) _ecat_check_thread.join();

    const uint32_t cyc_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle;
    for (size_t i = 1; i < _master->num_slaves(); i++) (void)_master->set_dc_sync0(i, true, cyc_time, 0U);
    (void)_master->write_state(0, EcState::Init);

    _master->close();

    return true;
  }

  [[nodiscard]] bool send(const driver::TxDatagram& tx) override {
    if (!is_open()) throw std::runtime_error("link is closed");
    std::lock_guard lock(_send_mtx);
    _send_buf.push(tx.clone());
    return true;
  }

  [[nodiscard]] bool receive(driver::RxDatagram& rx) override {
    if (!is_open()) throw std::runtime_error("link is closed");
    rx.copy_from(_io_map.input());
    return true;
  }

  bool is_open() override { return _is_open; }

 private:
  static std::string lookup_autd() {
    const auto adapters = pcap::Adapter::enumerate_adapters();
    for (const auto& adapter : adapters) {
      Master tester(pcap::PcapInterface(adapter.name()));

      uint16_t wc{};
      if (const auto res = tester.initialize(&wc); res != EmemResult::Ok || wc == 0) {
        tester.close();
        continue;
      }

      auto ifname = std::string(adapter.name());
      tester.close();
      return ifname;
    }

    throw std::runtime_error("No AUTD3 devices found");
  }

  static int64_t ec_sync(const int64_t reftime, const int64_t cycletime, int64_t* integral) {
    auto delta = (reftime - 50000) % cycletime;
    if (delta > cycletime / 2) delta -= cycletime;
    if (delta > 0) *integral += 1;
    if (delta < 0) *integral -= 1;
    return -(delta / 100) - *integral / 20;
  }

  using WaitFunc = void(const timespec&);

  template <WaitFunc W>
  void ecat_run_() {
    const auto cycletime_ns = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _send_cycle;

    ecat_init();

#if WIN32
    auto* h_process = GetCurrentProcess();
    const auto priority = GetPriorityClass(h_process);
    SetPriorityClass(h_process, REALTIME_PRIORITY_CLASS);
#endif

    auto ts = ecat_setup(cycletime_ns);
    int64_t toff = 0;
    _master->send_process_data();
    while (_is_open.load()) {
      ec_sync(_master->dc_time(), cycletime_ns, &toff);

      uint16_t wkc{};
      if (const auto res = _master->receive_process_data(EC_TIMEOUT, &wkc); res != EmemResult::Ok)
        _wkc.store(0);
      else
        _wkc.store(wkc);

      if (!_send_buf.empty()) {
        _io_map.copy_from(_send_buf.front());
        {
          std::lock_guard lock(_send_mtx);
          _send_buf.pop();
        }
      }

      add_timespec(ts, cycletime_ns + toff);
      W(ts);

      _master->send_process_data();
    }

#if WIN32
    SetPriorityClass(h_process, priority);
#endif
  }

  void ecat_run() {
    if (_high_precision)
      ecat_run_<timed_wait_h>();
    else
      ecat_run_<timed_wait>();
  }

  void check_state(const uint16_t slave) {
    if ((*_master)[slave].state == EcState::Operational) return;

    EcState state{};

    // ec_group[0].docheckstate = 1;
    if ((*_master)[slave].state == EcState::SafeOp | EcState::Error)
      (void)_master->write_state(slave, EcState::from(static_cast<uint16_t>(EcState::SafeOp | EcState::Ack)));
    else if ((*_master)[slave].state == EcState::SafeOp)
      (void)_master->write_state(slave, EcState::Operational);
    if (const auto r = _master->reconfig_slave(slave, std::chrono::nanoseconds(500 * 1000), &state); r != EmemResult::Ok && state != EcState::None) {
      (*_master)[slave].is_lost = false;
    } else if (!(*_master)[slave].is_lost) {
      (void)_master->state_check(slave, EcState::Operational, EC_TIMEOUT, &state);
      if ((*_master)[slave].state == EcState::None) (*_master)[slave].is_lost = true;
    }
  }

  void check_lost(const uint16_t slave) {
    if (!(*_master)[slave].is_lost) return;
    EcState state{};
    if ((*_master)[slave].state == EcState::None) {
      if (const auto r = _master->reconfig_slave(slave, std::chrono::nanoseconds(500 * 1000), &state); r != EmemResult::Ok && state != EcState::None)
        (*_master)[slave].is_lost = false;
    } else
      (*_master)[slave].is_lost = false;
  }

  bool error_handle() {
    std::stringstream ss;

    EcState state{};

    // ec_group[0].docheckstate = 0;
    (void)_master->read_state(&state);
    for (uint16_t slave = 1; slave <= static_cast<uint16_t>(_master->num_slaves()); slave++) {
      check_state(slave);
      check_lost(slave);
    }
    // if (ec_group[0].docheckstate == 0) return true;

    spdlog::error("check state");

    for (uint16_t slave = 1; slave <= static_cast<uint16_t>(_master->num_slaves()); slave++) {
      if (!(*_master)[slave].is_lost) continue;
      if (_on_lost != nullptr) _on_lost(ss.str());
      return false;
    }
    return true;
  }

  std::unique_ptr<Master<pcap::PcapInterface>> _master{nullptr};

  bool _high_precision;
  std::string _ifname;
  uint16_t _sync0_cycle;
  uint16_t _send_cycle;

  std::atomic<int32_t> _wkc;

  std::function<void(std::string)> _on_lost = nullptr;

  SyncMode _sync_mode;

  IOMap _io_map;

  std::atomic<bool> _is_open;

  std::thread _ecat_thread;
  std::thread _ecat_check_thread;

  std::queue<driver::TxDatagram> _send_buf;
  std::mutex _send_mtx;

  std::chrono::milliseconds _state_check_interval;
};

core::LinkPtr Emem::build() {
  return std::make_unique<EmemLink>(_high_precision, std::move(_ifname), _sync0_cycle, _send_cycle, std::move(_callback), _sync_mode,
                                    _state_check_interval);
}

std::vector<EtherCATAdapter> Emem::enumerate_adapters() {
  const auto adapters = pcap::Adapter::enumerate_adapters();
  std::vector<EtherCATAdapter> res;
  res.reserve(adapters.size());
  std::transform(adapters.begin(), adapters.end(), std::back_inserter(res),
                 [](const auto& adapter) { return EtherCATAdapter(adapter.desc(), adapter.name()); });
  return res;
}

}  // namespace autd3::link
