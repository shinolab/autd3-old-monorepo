// File: debug.cpp
// Project: debug
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/debug.hpp"

#include "../../spdlog.hpp"
#include "autd3/core/link.hpp"
#include "autd3/extra/cpu_emulator.hpp"

namespace autd3::link {

class DebugImpl final : public core::Link {
 public:
  explicit DebugImpl() : Link(), _is_open(false) {}
  ~DebugImpl() override = default;
  DebugImpl(const DebugImpl& v) noexcept = delete;
  DebugImpl& operator=(const DebugImpl& obj) = delete;
  DebugImpl(DebugImpl&& obj) = delete;
  DebugImpl& operator=(DebugImpl&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    spdlog::info("Open Debug link");
    if (is_open()) {
      spdlog::info("Link is already opened");
      return true;
    }

    _cpus.clear();
    _cpus.reserve(geometry.num_devices());
    size_t i = 0;
    std::transform(geometry.device_map().begin(), geometry.device_map().end(), std::back_inserter(_cpus), [&i](const size_t dev) {
      extra::CPU cpu(i++, dev);
      cpu.init();
      return cpu;
    });
    spdlog::info("Initialize emulator");

    _is_open = true;
    return true;
  }

  bool close() override {
    spdlog::info("Close Debug link");
    if (!is_open()) spdlog::info("Link is not opened");
    _is_open = false;
    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    for (auto& cpu : _cpus) cpu.send(tx);

    spdlog::info("Send data");

    switch (tx.header().msg_id) {
      case driver::MSG_CLEAR:
        spdlog::info("\tOP: CLEAR");
        return true;
      case driver::MSG_RD_CPU_VERSION:
        spdlog::info("\tOP: READ CPU VERSION");
        return true;
      case driver::MSG_RD_FPGA_VERSION:
        spdlog::info("\tOP: READ FPGA VERSION");
        return true;
      case driver::MSG_RD_FPGA_FUNCTION:
        spdlog::info("\tOP: READ FPGA FUNCTION");
        return true;
      default:
        break;
    }

    spdlog::info("\tCPU Flag: {}", tx.header().cpu_flag.to_string());
    spdlog::info("\tFPGA Flag: {}", tx.header().fpga_flag.to_string());

    for (auto& cpu : _cpus) {
      spdlog::info("Status: {}", cpu.id());
      const auto& fpga = cpu.fpga();
      if (fpga.is_stm_mode()) {
        if (fpga.is_stm_gain_mode())
          if (fpga.is_legacy_mode())
            spdlog::info("\tGain STM Legacy mode");
          else
            spdlog::info("\tGain STM mode");
        else
          spdlog::info("\tFocus STM mode");
        if (tx.header().cpu_flag.contains(driver::CPUControlFlags::STMBegin)) spdlog::info("\t\tSTM BEGIN");
        if (tx.header().cpu_flag.contains(driver::CPUControlFlags::STMEnd)) {
          spdlog::info("\t\tSTM END (cycle = {}, frequency_division = {})", fpga.stm_cycle(), fpga.stm_frequency_division());
          for (size_t j = 0; j < fpga.stm_cycle(); j++) {
            const auto [duties, phases] = fpga.drives(j);
            spdlog::debug("\tSTM[{}]:", j);
            for (size_t k = 0; k < duties.size(); k++) spdlog::debug("\t\t{:<3}: duty = {:<4}, phase = {:<4}", k, duties[k].duty, phases[k].phase);
          }
        }
      } else if (fpga.is_legacy_mode())
        spdlog::info("\tNormal Legacy mode");
      else
        spdlog::info("\tNormal mode");
      spdlog::info("\tSilencer step = {}, cycle={}", fpga.silencer_step(), fpga.silencer_cycle());
      const auto m = fpga.modulation();
      const auto freq_div_m = fpga.modulation_frequency_division();
      spdlog::info("\tModulation size = {}, frequency_division = {}", m.size(), freq_div_m);
      if (fpga.is_outputting()) {
        spdlog::debug("\t\tmodulation = [{}]", fmt::join(m, ", "));
        if (!fpga.is_stm_mode()) {
          const auto [duties, phases] = fpga.drives(0);
          for (size_t k = 0; k < duties.size(); k++) spdlog::debug("\t\t{:<3}: duty = {:<4}, phase = {:<4}", k, duties[k].duty, phases[k].phase);
        }
      } else
        spdlog::info("\tWithout output");
    }
    return true;
  }
  bool receive(driver::RxDatagram& rx) override {
    spdlog::info("Receive data");
    std::transform(_cpus.begin(), _cpus.end(), rx.messages().begin(), [](const auto& cpu) { return driver::RxMessage(cpu.ack(), cpu.msg_id()); });
    return true;
  }

  bool is_open() override { return _is_open; }

 private:
  bool _is_open;
  std::vector<extra::CPU> _cpus;
};

core::LinkPtr Debug::build() const {
  core::LinkPtr link = std::make_unique<DebugImpl>();
  return link;
}

}  // namespace autd3::link
