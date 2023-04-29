// File: debug.cpp
// Project: debug
// Created Date: 11/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "autd3/link/debug.hpp"

#include "../../spdlog.hpp"
#include "autd3/core/link.hpp"
#include "autd3/extra/cpu_emulator.hpp"

namespace autd3::link {

class DebugImpl final : public core::Link {
 public:
  explicit DebugImpl(const core::Duration timeout, std::shared_ptr<spdlog::logger> logger) : Link(timeout), _logger(std::move(logger)) {}
  ~DebugImpl() override = default;
  DebugImpl(const DebugImpl& v) noexcept = delete;
  DebugImpl& operator=(const DebugImpl& obj) = delete;
  DebugImpl(DebugImpl&& obj) = delete;
  DebugImpl& operator=(DebugImpl&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    _logger->debug("Open Debug link");

    _cpus.clear();
    _cpus.reserve(geometry.num_devices());
    size_t i = 0;
    std::transform(geometry.device_map().begin(), geometry.device_map().end(), std::back_inserter(_cpus), [&i](const size_t dev) {
      extra::CPU cpu(i++, dev);
      cpu.init();
      return cpu;
    });
    _logger->trace("Initialize emulator");

    _is_open = true;

    return true;
  }

  bool close() override {
    _is_open = false;
    _logger->debug("Close Debug link");
    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    for (auto& cpu : _cpus) cpu.send(tx);

    switch (tx.header().msg_id) {
      case driver::MSG_CLEAR:
        _logger->debug("\tOP: CLEAR");
        return true;
      case driver::MSG_RD_CPU_VERSION_MAJOR:
        _logger->debug("\tOP: READ CPU VERSION");
        return true;
      case driver::MSG_RD_FPGA_VERSION_MAJOR:
        _logger->debug("\tOP: READ FPGA VERSION");
        return true;
      case driver::MSG_RD_CPU_VERSION_MINOR:
        _logger->debug("\tOP: READ CPU VERSION MINOR");
        return true;
      case driver::MSG_RD_FPGA_VERSION_MINOR:
        _logger->debug("\tOP: READ FPGA VERSION MINOR");
        return true;
      case driver::MSG_RD_FPGA_FUNCTION:
        _logger->debug("\tOP: READ FPGA FUNCTION");
        return true;
      default:
        break;
    }

    _logger->debug("\tCPU Flag: {}", to_string(tx.header().cpu_flag));
    _logger->debug("\tFPGA Flag: {}", to_string(tx.header().fpga_flag));

    for (auto& cpu : _cpus) {
      _logger->debug("Status: {}", cpu.id());
      const auto& fpga = cpu.fpga();
      if (fpga.is_stm_mode()) {
        if (fpga.is_stm_gain_mode())
          if (fpga.is_legacy_mode())
            _logger->debug("\tGain STM Legacy mode");
          else
            _logger->debug("\tGain STM mode");
        else
          _logger->debug("\tFocus STM mode");
        if (tx.header().cpu_flag.contains(driver::CPUControlFlags::STMBegin)) _logger->debug("\t\tSTM BEGIN");
        if (tx.header().cpu_flag.contains(driver::CPUControlFlags::STMEnd)) {
          _logger->debug("\t\tSTM END (cycle = {}, frequency_division = {})", fpga.stm_cycle(), fpga.stm_frequency_division());
          if (_logger->level() == spdlog::level::trace) {
            const auto cycles = fpga.cycles();
            for (size_t j = 0; j < fpga.stm_cycle(); j++) {
              const auto [duties, phases] = fpga.drives(j);
              _logger->trace("\tSTM[{}]:", j);
              for (size_t k = 0; k < duties.size(); k++)
                _logger->trace("\t\t{:<3}: duty = {:<4}, phase = {:<4}, cycle = {:<4}", k, duties[k], phases[k], cycles[k]);
            }
          }
        }
      } else if (fpga.is_legacy_mode())
        _logger->debug("\tNormal Legacy mode");
      else
        _logger->debug("\tNormal mode");
      _logger->debug("\tSilencer step = {}, cycle={}", fpga.silencer_step(), fpga.silencer_cycle());
      const auto m = fpga.modulation();
      const auto freq_div_m = fpga.modulation_frequency_division();
      _logger->debug("\tModulation size = {}, frequency_division = {}", m.size(), freq_div_m);
      if (fpga.is_outputting()) {
        _logger->trace("\t\tmodulation = [{}]", fmt::join(m, ", "));
        if (!fpga.is_stm_mode() && _logger->level() == spdlog::level::trace) {
          const auto cycles = fpga.cycles();
          const auto [duties, phases] = fpga.drives(0);
          for (size_t k = 0; k < duties.size(); k++)
            _logger->trace("\t\t{:<3}: duty = {:<4}, phase = {:<4}, cycle = {:<4}", k, duties[k], phases[k], cycles[k]);
        }
      } else
        _logger->debug("\tWithout output");
    }
    return true;
  }
  bool receive(driver::RxDatagram& rx) override {
    std::transform(_cpus.begin(), _cpus.end(), rx.messages().begin(), [](const auto& cpu) { return driver::RxMessage(cpu.ack(), cpu.msg_id()); });
    return true;
  }

  bool is_open() override { return _is_open; }

 private:
  bool _is_open{false};
  std::vector<extra::CPU> _cpus;
  std::shared_ptr<spdlog::logger> _logger;
};

core::LinkPtr Debug::build_() {
  const auto name = "AUTD3";
  spdlog::sink_ptr sink =
      _out == nullptr || _flush == nullptr ? get_default_sink() : std::make_shared<CustomSink<std::mutex>>(std::move(_out), std::move(_flush));
  auto logger = std::make_shared<spdlog::logger>(name, std::move(sink));
  logger->set_level(static_cast<spdlog::level::level_enum>(_level));
  register_logger(logger);
  return std::make_unique<DebugImpl>(_timeout, std::move(logger));
}

}  // namespace autd3::link
