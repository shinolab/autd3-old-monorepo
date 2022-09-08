// File: debug.cpp
// Project: debug
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/08/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/debug.hpp"

#include "autd3/core/link.hpp"
#include "autd3/extra/firmware-emulator/cpu/emulator.hpp"

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 6285 26437 26800 26498)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#include "spdlog/spdlog.h"
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif

namespace autd3::link {

class DebugImpl final : public core::Link {
 public:
  explicit DebugImpl() : Link(), _is_open(false) {}
  ~DebugImpl() override = default;
  DebugImpl(const DebugImpl& v) noexcept = delete;
  DebugImpl& operator=(const DebugImpl& obj) = delete;
  DebugImpl(DebugImpl&& obj) = delete;
  DebugImpl& operator=(DebugImpl&& obj) = delete;

  void open(const core::Geometry& geometry) override {
    spdlog::info("Open Debug link");
    if (is_open()) {
      spdlog::info("Link is already opened");
      return;
    }

    _cpus.clear();
    _cpus.reserve(geometry.num_devices());
    for (size_t i = 0; i < geometry.num_devices(); i++) {
      extra::firmware_emulator::cpu::CPU cpu(i);
      cpu.init();
      _cpus.emplace_back(cpu);
    }
    spdlog::info("Initialize emulator");

    _is_open = true;
  }

  void close() override {
    spdlog::info("Close Debug link");
    if (!is_open()) {
      spdlog::info("Link is not opened");
      return;
    }
    _is_open = false;
  }

  bool send(const driver::TxDatagram& tx) override {
    for (size_t i = 0; i < _cpus.size(); i++) _cpus[i].send(tx.header(), tx.bodies()[i]);

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
          spdlog::info("\tPoint STM mode");
        if (tx.header().cpu_flag.contains(driver::CPUControlFlags::STM_BEGIN)) spdlog::info("\t\tSTM BEGIN");
        if (tx.header().cpu_flag.contains(driver::CPUControlFlags::STM_END)) {
          spdlog::info("\t\tSTM END (cycle = {}, frequency_division = {})", fpga.stm_cycle(), fpga.stm_frequency_division());
          const auto [duties, phases] = fpga.drives();
          for (size_t j = 0; j < duties.size(); j++) {
            spdlog::debug("\tSTM[{}]:", j);
            for (size_t k = 0; k < driver::NUM_TRANS_IN_UNIT; k++)
              spdlog::debug("\t\t{}: duty = {}, phase = {}", k, duties[j][k].duty, phases[j][k].phase);
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
        std::stringstream ss;
        ss << static_cast<int>(m[0]);
        for (size_t j = 1; j < m.size(); j++) ss << ", " << static_cast<int>(m[j]);
        spdlog::debug("\t\tmodulation = {}", ss.str());
        if (!fpga.is_stm_mode()) {
          const auto [duties, phases] = fpga.drives();
          for (size_t k = 0; k < driver::NUM_TRANS_IN_UNIT; k++)
            spdlog::debug("\t\t{}: duty = {}, phase = {}", k, duties[0][k].duty, phases[0][k].phase);
        }
      } else
        spdlog::info("\tWithout output");
    }
    return true;
  }
  bool receive(driver::RxDatagram& rx) override {
    spdlog::info("Receive data");

    for (size_t i = 0; i < _cpus.size(); i++) {
      rx.messages()[i].msg_id = _cpus[i].msg_id();
      rx.messages()[i].ack = _cpus[i].ack();
    }

    return true;
  }

  bool is_open() override { return _is_open; }

 private:
  bool _is_open;
  std::vector<extra::firmware_emulator::cpu::CPU> _cpus;
};

core::LinkPtr Debug::build() const {
  core::LinkPtr link = std::make_unique<DebugImpl>();
  return link;
}

}  // namespace autd3::link
