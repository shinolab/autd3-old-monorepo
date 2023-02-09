// File: emem.cpp
// Project: emem
// Created Date: 04/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/02/2023
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

    _io_map.resize(geometry.device_map());
    const auto res = _master->initialize();
    if (res.is_err()) throw std::runtime_error("Failed to initialize");
    const auto wc = res.value();
    if (wc == 0) throw std::runtime_error("No slaves found");

    if (const auto r = _master->config(_io_map.get()); r.is_err()) throw std::runtime_error("Failed to configure IO map");

    const uint32_t cyc_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle;
    if (_sync_mode == SyncMode::DC) {
      for (size_t i = 1; i < wc; i++)
        (*_master)[i].po_to_so_config = [cyc_time, i, this] {
          if (const auto r = _master->set_dc_sync0(i, true, cyc_time, 0U); r.is_err()) throw std::runtime_error("Failed to configure DC");
        };
    }

    if (const auto r = _master->config_dc(); r.is_err()) throw std::runtime_error("Failed to configure DC");

    (void)_master->state_check(0, ethercat::EcState::SafeOp, EC_TIMEOUT_SAFE);
    (void)_master->read_state();

    if ((*_master)[0].state != ethercat::EcState::SafeOp) {
      // for (size_t i = 1; i < wc; i++)
      //    if ((*_master)[i].state != ethercat::EcState::SafeOp)
      //        log state
      throw std::runtime_error("One ore more slaves did not reach safe operational state");
    }

    const auto expected_wkc = _master->expected_wkc();

    (void)_master->write_state(0, ethercat::EcState::Operational);

    _is_open.store(true);
    _ecat_thread = std::thread([this] { ecat_run(); });

    (void)_master->state_check(0, ethercat::EcState::Operational, 5 * EC_TIMEOUT_STATE);
    if ((*_master)[0].state != ethercat::EcState::Operational) {
      _is_open.store(false);
      if (_ecat_thread.joinable()) _ecat_thread.join();
      //(void)_master->read_state();
      // for (size_t i = 1; i < wc; i++)
      //    if ((*_master)[i].state != ethercat::EcState::SafeOp)
      //        log state
      throw std::runtime_error("One ore more slaves are not responding.");
    }

    if (_sync_mode == SyncMode::FreeRun) {
      for (size_t i = 1; i < wc; i++)
        if (const auto r = _master->set_dc_sync0(i, true, cyc_time, 0U); r.is_err()) throw std::runtime_error("Failed to configure DC");
    }

    _ecat_check_thread = std::thread([this, expected_wkc] {
      while (this->_is_open.load()) {
        if (this->_wkc.load() < expected_wkc) (void)0;  // do check here
        //     if (!error_handle(_logger, this->_on_lost)) break;
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
    (void)_master->write_state(0, ethercat::EcState::Init);

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

      if (const auto res = tester.initialize(); res.is_err() || res.value() == 0) {
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

      if (const auto res = _master->receive_process_data(EC_TIMEOUT); res.is_err())
        _wkc.store(0);
      else
        _wkc.store(res.value());

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
