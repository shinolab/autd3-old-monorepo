// File: emem.cpp
// Project: emem
// Created Date: 04/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 21/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "autd3/link/emem.hpp"

#include <mutex>
#include <queue>

#include "../../spdlog.hpp"
#include "autd3/core/utils/hint.hpp"
#include "autd3/core/utils/osal_timer.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "iomap.hpp"
#include "master.hpp"
#include "pcap/adapter.hpp"
#include "pcap/pcap_interface.hpp"

//
#include "ecat.hpp"

namespace autd3::link {

struct EMEMCallback final : core::CallbackHandler {
  ~EMEMCallback() override = default;
  EMEMCallback(const EMEMCallback& v) noexcept = delete;
  EMEMCallback& operator=(const EMEMCallback& obj) = delete;
  EMEMCallback(EMEMCallback&& obj) = delete;
  EMEMCallback& operator=(EMEMCallback&& obj) = delete;

  explicit EMEMCallback(Master& master, std::atomic<int32_t>& wkc, std::queue<driver::TxDatagram>& send_buf, std::mutex& send_mtx, IOMap& io_map)
      : _rt_lock(false), _master(master), _wkc(wkc), _send_buf(send_buf), _send_mtx(send_mtx), _io_map(io_map) {}

  void callback() override {
    if (auto expected = false; _rt_lock.compare_exchange_weak(expected, true)) {
      _master.send_process_data();
      uint16_t wkc{};
      (void)_master.receive_process_data(EC_TIMEOUT, &wkc);
      _wkc.store(wkc);
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

  Master& _master;
  std::atomic<int32_t>& _wkc;
  std::queue<driver::TxDatagram>& _send_buf;
  std::mutex& _send_mtx;
  IOMap& _io_map;
};

class EmemLink final : public core::Link {
 public:
  EmemLink(const TimerStrategy timer_strategy, std::string ifname, const size_t buf_size, const uint16_t sync0_cycle, const uint16_t send_cycle,
           std::function<void(std::string)> on_lost, const SyncMode sync_mode, const std::chrono::milliseconds state_check_interval)
      : _timer_strategy(timer_strategy),
        _ifname(std::move(ifname)),
        _buf_size(buf_size),
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

    _master.open(_ifname);

    uint16_t wc{};
    if (const auto res = _master.initialize(&wc); res != EmemResult::Ok) throw std::runtime_error("Failed to initialize");
    if (wc == 0) throw std::runtime_error("No slaves found");

    const uint32_t cyc_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle;
    if (_sync_mode == SyncMode::DC) {
      for (size_t i = 1; i <= wc; i++)
        _master[i].po_to_so_config = [cyc_time, i, this] {
          if (const auto r = _master.set_dc_sync0(i, true, cyc_time, 0U); r != EmemResult::Ok) throw std::runtime_error("Failed to configure DC");
        };
    }

    if (const auto r = _master.config_dc(); r != EmemResult::Ok) throw std::runtime_error("Failed to configure DC");

    _io_map.resize(geometry.device_map());
    if (const auto r = _master.config(_io_map.get()); r != EmemResult::Ok) throw std::runtime_error("Failed to configure IO map");

    EcState unused_state{};
    (void)_master.state_check(0, EcState::SafeOp, EC_TIMEOUT_SAFE, &unused_state);
    (void)_master.read_state(&unused_state);

    if (_master[0].state != EcState::SafeOp) throw std::runtime_error("One ore more slaves did not reach safe operational state");

    const auto expected_wkc = _master.expected_wkc();

    _master[0].state = EcState::Operational;
    (void)_master.write_state(0);

    _is_open.store(true);

    const auto cycle_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _send_cycle;
    switch (_timer_strategy) {
      case TimerStrategy::BusyWait:
        _ecat_thread = std::thread([this, cycle_time] { ecat_run<busy_wait>(cycle_time); });
        break;
      case TimerStrategy::Sleep:
        _ecat_thread = std::thread([this, cycle_time] { ecat_run<wait_with_sleep>(cycle_time); });
        break;
      case TimerStrategy::NativeTimer:
        _timer = core::Timer<EMEMCallback>::start(std::make_unique<EMEMCallback>(_master, _wkc, _send_buf, _send_mtx, _io_map), cycle_time);
        break;
    }

    (void)_master.state_check(0, EcState::Operational, 5 * EC_TIMEOUT_STATE, &unused_state);
    if (_master[0].state != EcState::Operational) {
      _is_open.store(false);
      close_th();
      throw std::runtime_error("One ore more slaves are not responding:" + _master[0].state.to_string());
    }

    if (_sync_mode == SyncMode::FreeRun) {
      for (size_t i = 1; i < wc; i++)
        if (const auto r = _master.set_dc_sync0(i, true, cyc_time, 0U); r != EmemResult::Ok) throw std::runtime_error("Failed to configure DC");
    }

    _ecat_check_thread = std::thread([this, expected_wkc] {
      while (this->_is_open.load()) {
        if (this->_wkc.load() < expected_wkc || _do_check_state)
          if (!error_handle()) break;
        std::this_thread::sleep_for(_state_check_interval);
      }
    });

    return static_cast<size_t>(wc);
  }

  void close_th() {
    switch (_timer_strategy) {
      case TimerStrategy::BusyWait:
      case TimerStrategy::Sleep:
        if (_ecat_thread.joinable()) _ecat_thread.join();
        break;
      case TimerStrategy::NativeTimer:
        const auto _ = _timer->stop();
        break;
    }
  }

  [[nodiscard]] bool close() override {
    if (!is_open()) return true;
    _is_open.store(false);

    close_th();
    if (_ecat_check_thread.joinable()) _ecat_check_thread.join();

    const uint32_t cyc_time = driver::EC_CYCLE_TIME_BASE_NANO_SEC * _sync0_cycle;
    for (size_t i = 1; i < _master.num_slaves(); i++) (void)_master.set_dc_sync0(i, true, cyc_time, 0U);
    _master[0].state = EcState::Init;
    (void)_master.write_state(0);

    _master.close();

    return true;
  }

  [[nodiscard]] bool send(const driver::TxDatagram& tx) override {
    if (!is_open()) throw std::runtime_error("link is closed");

    if (_buf_size != 0)
      while (_send_buf.size() >= _buf_size) std::this_thread::sleep_for(std::chrono::milliseconds(1));

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
      Master tester;
      tester.open(adapter.name());

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
    _master.send_process_data();
    while (_is_open.load()) {
      ec_sync(_master.dc_time(), cycletime_ns, &toff);

      uint16_t wkc{};
      (void)_master.receive_process_data(EC_TIMEOUT, &wkc);
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

      _master.send_process_data();
    }
  }

  void check_state(const uint16_t slave, std::stringstream& ss) {
    if (_master[slave].state == EcState::Operational) return;

    EcState state{};

    _do_check_state = true;
    if (_master[slave].state == EcState::from(EcState::SafeOp + EcState::Error)) {
      spdlog::warn("slave {} is in SAFE_OP + ERROR, attempting ack", slave);
      _master[slave].state = EcState::from(EcState::SafeOp + EcState::Ack);
      (void)_master.write_state(slave);
    } else if (_master[slave].state == EcState::SafeOp) {
      spdlog::warn("slave {} is in SAFE_OP, change to OPERATIONAL", slave);
      _master[slave].state = EcState::Operational;
      (void)_master.write_state(slave);
    } else if (_master[slave].state != EcState::None) {
      if (_master.re_config_slave(slave, std::chrono::nanoseconds(500 * 1000), &state) == EmemResult::Ok && state != EcState::None) {
        _master[slave].is_lost = false;
        spdlog::info("slave {} reconfigured", slave);
      }
    } else if (!_master[slave].is_lost) {
      (void)_master.state_check(slave, EcState::Operational, EC_TIMEOUT, &state);
      if (_master[slave].state == EcState::None) {
        _master[slave].is_lost = true;
        ss << "ERROR: slave " << slave << " lost\n";
        spdlog::warn("slave {} lost", slave);
      }
    }
  }

  void check_lost(const uint16_t slave) {
    if (!_master[slave].is_lost) return;
    if (_master[slave].state == EcState::None) {
      if (_master.recover_slave(slave, std::chrono::nanoseconds(500 * 1000)) == EmemResult::Ok) {
        _master[slave].is_lost = false;
        spdlog::info("slave {} recovered.", slave);
      }
    } else {
      _master[slave].is_lost = false;
      spdlog::info("slave {} found.", slave);
    }
  }

  bool error_handle() {
    std::stringstream ss;

    EcState state{};

    _do_check_state = false;
    (void)_master.read_state(&state);
    for (uint16_t slave = 1; slave <= static_cast<uint16_t>(_master.num_slaves()); slave++) {
      check_state(slave, ss);
      check_lost(slave);
    }
    if (!_do_check_state) return true;

    for (uint16_t slave = 1; slave <= static_cast<uint16_t>(_master.num_slaves()); slave++) {
      if (!_master[slave].is_lost) continue;
      if (_on_lost != nullptr) _on_lost(ss.str());
      return false;
    }
    return true;
  }

  Master _master;
  bool _do_check_state{false};

  TimerStrategy _timer_strategy;
  std::string _ifname;
  size_t _buf_size;
  uint16_t _sync0_cycle;
  uint16_t _send_cycle;

  std::atomic<int32_t> _wkc;

  std::function<void(std::string)> _on_lost = nullptr;

  SyncMode _sync_mode;

  IOMap _io_map;

  std::atomic<bool> _is_open;

  std::thread _ecat_thread;
  std::thread _ecat_check_thread;
  std::unique_ptr<core::Timer<EMEMCallback>> _timer;

  std::queue<driver::TxDatagram> _send_buf;
  std::mutex _send_mtx;

  std::chrono::milliseconds _state_check_interval;
};

core::LinkPtr Emem::build() {
  return std::make_unique<EmemLink>(_timer_strategy, std::move(_ifname), _buf_size, _sync0_cycle, _send_cycle, std::move(_callback), _sync_mode,
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
