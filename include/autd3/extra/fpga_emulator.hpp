// File: emulator.hpp
// Project: fpga
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 15/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <utility>
#include <vector>

#include "autd3/driver/common/fpga/defined.hpp"
#include "autd3/driver/hardware.hpp"

namespace autd3::extra::fpga {
constexpr uint8_t VERSION_NUM = 0x86;

constexpr uint16_t BRAM_SELECT_CONTROLLER = 0x0;
constexpr uint16_t BRAM_SELECT_MOD = 0x1;
constexpr uint16_t BRAM_SELECT_NORMAL = 0x2;
constexpr uint16_t BRAM_SELECT_STM = 0x3;

constexpr size_t ADDR_CTL_REG = 0x0000;
// constexpr size_t ADDR_FPGA_INFO = 0x0001;
// constexpr size_t ADDR_EC_SYNC_TIME_0 = ADDR_EC_SYNC_CYCLE_TICKS + 1;
// constexpr size_t ADDR_EC_SYNC_TIME_1 = ADDR_EC_SYNC_CYCLE_TICKS + 2;
// constexpr size_t ADDR_EC_SYNC_TIME_2 = ADDR_EC_SYNC_CYCLE_TICKS + 3;
// constexpr size_t ADDR_EC_SYNC_TIME_3 = ADDR_EC_SYNC_CYCLE_TICKS + 4;
constexpr size_t ADDR_MOD_ADDR_OFFSET = 0x0020;
constexpr size_t ADDR_MOD_CYCLE = 0x0021;
constexpr size_t ADDR_MOD_FREQ_DIV_0 = 0x0022;
constexpr size_t ADDR_MOD_FREQ_DIV_1 = 0x0023;
constexpr size_t ADDR_VERSION_NUM = 0x003F;
constexpr size_t ADDR_SILENT_CYCLE = 0x0040;
constexpr size_t ADDR_SILENT_STEP = 0x0041;
constexpr size_t ADDR_STM_ADDR_OFFSET = 0x0050;
constexpr size_t ADDR_STM_CYCLE = 0x0051;
constexpr size_t ADDR_STM_FREQ_DIV_0 = 0x0052;
constexpr size_t ADDR_STM_FREQ_DIV_1 = 0x0053;
constexpr size_t ADDR_SOUND_SPEED_0 = 0x0054;
constexpr size_t ADDR_SOUND_SPEED_1 = 0x0055;
constexpr size_t ADDR_CYCLE_BASE = 0x0100;
constexpr size_t ADDR_MOD_DELAY_BASE = 0x0200;

constexpr uint16_t CTL_REG_LEGACY_MODE_BIT = 0;
constexpr uint16_t CTL_REG_FORCE_FAN_BIT = 4;
constexpr uint16_t CTL_REG_OP_MODE_BIT = 5;
constexpr uint16_t CTL_REG_STM_GAIN_MODE_BIT = 6;
// constexpr size_t CTL_REG_SYNC_BIT = 8;

constexpr uint8_t ENABLED_STM_BIT = 0x01;
constexpr uint8_t ENABLED_MODULATOR_BIT = 0x02;
constexpr uint8_t ENABLED_SILENCER_BIT = 0x04;
constexpr uint8_t ENABLED_MOD_DELAY_BIT = 0x08;
constexpr uint8_t ENABLED_EMULATOR_BIT = 0x80;
constexpr uint8_t ENABLED_FEATURES_BITS =
    ENABLED_MOD_DELAY_BIT | ENABLED_STM_BIT | ENABLED_MODULATOR_BIT | ENABLED_SILENCER_BIT | ENABLED_EMULATOR_BIT;

constexpr uint32_t TR_POS[driver::NUM_TRANS_IN_UNIT] = {
    0x00000000, 0x01960000, 0x032c0000, 0x04c30000, 0x06590000, 0x07ef0000, 0x09860000, 0x0b1c0000, 0x0cb30000, 0x0e490000, 0x0fdf0000, 0x11760000,
    0x130c0000, 0x14a30000, 0x16390000, 0x17d00000, 0x19660000, 0x1afc0000, 0x00000196, 0x04c30196, 0x06590196, 0x07ef0196, 0x09860196, 0x0b1c0196,
    0x0cb30196, 0x0e490196, 0x0fdf0196, 0x11760196, 0x130c0196, 0x14a30196, 0x16390196, 0x17d00196, 0x1afc0196, 0x0000032c, 0x0196032c, 0x032c032c,
    0x04c3032c, 0x0659032c, 0x07ef032c, 0x0986032c, 0x0b1c032c, 0x0cb3032c, 0x0e49032c, 0x0fdf032c, 0x1176032c, 0x130c032c, 0x14a3032c, 0x1639032c,
    0x17d0032c, 0x1966032c, 0x1afc032c, 0x000004c3, 0x019604c3, 0x032c04c3, 0x04c304c3, 0x065904c3, 0x07ef04c3, 0x098604c3, 0x0b1c04c3, 0x0cb304c3,
    0x0e4904c3, 0x0fdf04c3, 0x117604c3, 0x130c04c3, 0x14a304c3, 0x163904c3, 0x17d004c3, 0x196604c3, 0x1afc04c3, 0x00000659, 0x01960659, 0x032c0659,
    0x04c30659, 0x06590659, 0x07ef0659, 0x09860659, 0x0b1c0659, 0x0cb30659, 0x0e490659, 0x0fdf0659, 0x11760659, 0x130c0659, 0x14a30659, 0x16390659,
    0x17d00659, 0x19660659, 0x1afc0659, 0x000007ef, 0x019607ef, 0x032c07ef, 0x04c307ef, 0x065907ef, 0x07ef07ef, 0x098607ef, 0x0b1c07ef, 0x0cb307ef,
    0x0e4907ef, 0x0fdf07ef, 0x117607ef, 0x130c07ef, 0x14a307ef, 0x163907ef, 0x17d007ef, 0x196607ef, 0x1afc07ef, 0x00000986, 0x01960986, 0x032c0986,
    0x04c30986, 0x06590986, 0x07ef0986, 0x09860986, 0x0b1c0986, 0x0cb30986, 0x0e490986, 0x0fdf0986, 0x11760986, 0x130c0986, 0x14a30986, 0x16390986,
    0x17d00986, 0x19660986, 0x1afc0986, 0x00000b1c, 0x01960b1c, 0x032c0b1c, 0x04c30b1c, 0x06590b1c, 0x07ef0b1c, 0x09860b1c, 0x0b1c0b1c, 0x0cb30b1c,
    0x0e490b1c, 0x0fdf0b1c, 0x11760b1c, 0x130c0b1c, 0x14a30b1c, 0x16390b1c, 0x17d00b1c, 0x19660b1c, 0x1afc0b1c, 0x00000cb3, 0x01960cb3, 0x032c0cb3,
    0x04c30cb3, 0x06590cb3, 0x07ef0cb3, 0x09860cb3, 0x0b1c0cb3, 0x0cb30cb3, 0x0e490cb3, 0x0fdf0cb3, 0x11760cb3, 0x130c0cb3, 0x14a30cb3, 0x16390cb3,
    0x17d00cb3, 0x19660cb3, 0x1afc0cb3, 0x00000e49, 0x01960e49, 0x032c0e49, 0x04c30e49, 0x06590e49, 0x07ef0e49, 0x09860e49, 0x0b1c0e49, 0x0cb30e49,
    0x0e490e49, 0x0fdf0e49, 0x11760e49, 0x130c0e49, 0x14a30e49, 0x16390e49, 0x17d00e49, 0x19660e49, 0x1afc0e49, 0x00000fdf, 0x01960fdf, 0x032c0fdf,
    0x04c30fdf, 0x06590fdf, 0x07ef0fdf, 0x09860fdf, 0x0b1c0fdf, 0x0cb30fdf, 0x0e490fdf, 0x0fdf0fdf, 0x11760fdf, 0x130c0fdf, 0x14a30fdf, 0x16390fdf,
    0x17d00fdf, 0x19660fdf, 0x1afc0fdf, 0x00001176, 0x01961176, 0x032c1176, 0x04c31176, 0x06591176, 0x07ef1176, 0x09861176, 0x0b1c1176, 0x0cb31176,
    0x0e491176, 0x0fdf1176, 0x11761176, 0x130c1176, 0x14a31176, 0x16391176, 0x17d01176, 0x19661176, 0x1afc1176, 0x0000130c, 0x0196130c, 0x032c130c,
    0x04c3130c, 0x0659130c, 0x07ef130c, 0x0986130c, 0x0b1c130c, 0x0cb3130c, 0x0e49130c, 0x0fdf130c, 0x1176130c, 0x130c130c, 0x14a3130c, 0x1639130c,
    0x17d0130c, 0x1966130c, 0x1afc130c, 0x000014a3, 0x019614a3, 0x032c14a3, 0x04c314a3, 0x065914a3, 0x07ef14a3, 0x098614a3, 0x0b1c14a3, 0x0cb314a3,
    0x0e4914a3, 0x0fdf14a3, 0x117614a3, 0x130c14a3, 0x14a314a3, 0x163914a3, 0x17d014a3, 0x196614a3, 0x1afc14a3};
}  // namespace autd3::extra::fpga

namespace autd3::extra {

class FPGA {
 public:
  FPGA() {
    _controller_bram.resize(1024);
    _modulator_bram.resize(32768);
    _normal_op_bram.resize(512);
    _stm_op_bram.resize(524288);
  }

  void init() { _controller_bram[fpga::ADDR_VERSION_NUM] = static_cast<uint16_t>(fpga::ENABLED_FEATURES_BITS << 8 | fpga::VERSION_NUM); }

  [[nodiscard]] uint16_t read(const uint16_t addr) const {
    const auto select = (addr >> 14) & 0x0003;
    const size_t addr_in_bram = (addr & 0x3FFF);
    switch (select) {
      case fpga::BRAM_SELECT_CONTROLLER:
        return _controller_bram[addr_in_bram];
      case fpga::BRAM_SELECT_MOD:
        return _modulator_bram[static_cast<size_t>(_controller_bram[fpga::ADDR_MOD_ADDR_OFFSET]) << 14 | addr_in_bram];
      case fpga::BRAM_SELECT_NORMAL:
        return _normal_op_bram[addr_in_bram];
      case fpga::BRAM_SELECT_STM:
        return _stm_op_bram[static_cast<size_t>(_controller_bram[fpga::ADDR_STM_ADDR_OFFSET]) << 14 | addr_in_bram];
      default:
        return 0;
    }
  }

  void write(const uint16_t addr, const uint16_t data) {
    const auto select = (addr >> 14) & 0x0003;
    const size_t addr_in_bram = (addr & 0x3FFF);
    switch (select) {
      case fpga::BRAM_SELECT_CONTROLLER:
        _controller_bram[addr_in_bram] = data;
        break;
      case fpga::BRAM_SELECT_MOD:
        _modulator_bram[static_cast<size_t>(_controller_bram[fpga::ADDR_MOD_ADDR_OFFSET]) << 14 | addr_in_bram] = data;
        break;
      case fpga::BRAM_SELECT_NORMAL:
        _normal_op_bram[addr_in_bram] = data;
        break;
      case fpga::BRAM_SELECT_STM:
        _stm_op_bram[static_cast<size_t>(_controller_bram[fpga::ADDR_STM_ADDR_OFFSET]) << 14 | addr_in_bram] = data;
        break;
      default:
        break;
    }
  }

  [[nodiscard]] bool is_legacy_mode() const { return (_controller_bram[fpga::ADDR_CTL_REG] & (1 << fpga::CTL_REG_LEGACY_MODE_BIT)) != 0; }

  [[nodiscard]] bool is_force_fan() const { return (_controller_bram[fpga::ADDR_CTL_REG] & (1 << fpga::CTL_REG_FORCE_FAN_BIT)) != 0; }

  [[nodiscard]] bool is_stm_mode() const { return (_controller_bram[fpga::ADDR_CTL_REG] & (1 << fpga::CTL_REG_OP_MODE_BIT)) != 0; }
  [[nodiscard]] bool is_stm_gain_mode() const { return (_controller_bram[fpga::ADDR_CTL_REG] & (1 << fpga::CTL_REG_STM_GAIN_MODE_BIT)) != 0; }

  [[nodiscard]] uint16_t silencer_cycle() const { return _controller_bram[fpga::ADDR_SILENT_CYCLE]; }

  [[nodiscard]] uint16_t silencer_step() const { return _controller_bram[fpga::ADDR_SILENT_STEP]; }

  [[nodiscard]] std::array<uint16_t, driver::NUM_TRANS_IN_UNIT> cycles() const {
    std::array<uint16_t, driver::NUM_TRANS_IN_UNIT> cycles{};
    for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) cycles[i] = _controller_bram[fpga::ADDR_CYCLE_BASE + i];
    return cycles;
  }

  [[nodiscard]] std::array<uint16_t, driver::NUM_TRANS_IN_UNIT> mod_delays() const {
    std::array<uint16_t, driver::NUM_TRANS_IN_UNIT> delays{};
    for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) delays[i] = _controller_bram[fpga::ADDR_MOD_DELAY_BASE + i];
    return delays;
  }

  [[nodiscard]] uint32_t stm_frequency_division() const {
    return (_controller_bram[fpga::ADDR_STM_FREQ_DIV_1] << 16 & 0xFFFF0000) | (_controller_bram[fpga::ADDR_STM_FREQ_DIV_0] & 0x0000FFFF);
  }

  [[nodiscard]] size_t stm_cycle() const { return static_cast<size_t>(_controller_bram[fpga::ADDR_STM_CYCLE]) + 1; }

  [[nodiscard]] uint32_t sound_speed() const {
    return (_controller_bram[fpga::ADDR_SOUND_SPEED_1] << 16 & 0xFFFF0000) | (_controller_bram[fpga::ADDR_SOUND_SPEED_0] & 0x0000FFFF);
  }

  [[nodiscard]] uint32_t modulation_frequency_division() const {
    return (_controller_bram[fpga::ADDR_MOD_FREQ_DIV_1] << 16 & 0xFFFF0000) | (_controller_bram[fpga::ADDR_MOD_FREQ_DIV_0] & 0x0000FFFF);
  }

  [[nodiscard]] size_t modulation_cycle() const { return static_cast<size_t>(_controller_bram[fpga::ADDR_MOD_CYCLE]) + 1; }

  [[nodiscard]] std::vector<uint8_t> modulation() const {
    const auto cycle = modulation_cycle();
    std::vector<uint8_t> m;
    m.reserve(cycle);

    for (size_t i = 0; i < cycle >> 1; i++) {
      const auto b = _modulator_bram[i];
      m.emplace_back(b & 0x00FF);
      m.emplace_back(b >> 8 & 0x00FF);
    }

    if (cycle % 2 != 0) {
      const auto b = _modulator_bram[(cycle + 1) >> 1];
      m.emplace_back(b & 0x00FF);
    }
    return m;
  }

  [[nodiscard]] bool is_outputting() const {
    const auto m = modulation();
    if (std::all_of(m.begin(), m.end(), [](const uint8_t x) { return x == 0; })) return false;
    if (!is_stm_mode()) {
      const auto [duties, phases] = drives();
      const auto& duty = duties[0];
      if (std::all_of(duty.begin(), duty.end(), [](const driver::Duty x) { return x.duty == 0; })) return false;
    }
    return true;
  }

  [[nodiscard]] std::pair<std::vector<std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT>>,
                          std::vector<std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT>>>
  drives() const {
    if (is_stm_mode()) {
      if (is_stm_gain_mode()) {
        if (is_legacy_mode()) return std::make_pair(gain_stm_legacy_duty(), gain_stm_legacy_phase());
        return std::make_pair(gain_stm_normal_duty(), gain_stm_normal_phase());
      }
      return std::make_pair(point_stm_duty(), point_stm_phase());
    }
    if (is_legacy_mode()) return std::make_pair(std::vector{legacy_duty()}, std::vector{legacy_phase()});
    return std::make_pair(std::vector{normal_duty()}, std::vector{normal_phase()});
  }

 private:
  [[nodiscard]] std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT> normal_duty() const {
    std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT> d{};
    for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) d[i] = driver::Duty{_normal_op_bram[2 * i + 1]};
    return d;
  }

  [[nodiscard]] std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT> normal_phase() const {
    std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT> d{};
    for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) d[i] = driver::Phase{_normal_op_bram[2 * i]};
    return d;
  }

  [[nodiscard]] std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT> legacy_duty() const {
    std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT> d{};
    for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) {
      auto duty = static_cast<uint16_t>((_normal_op_bram[2 * i] >> 8) & 0x00FF);
      duty = static_cast<uint16_t>(((duty << 3) | 0x07) + 1);
      d[i] = driver::Duty{duty};
    }
    return d;
  }

  [[nodiscard]] std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT> legacy_phase() const {
    std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT> d{};
    for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) {
      auto phase = static_cast<uint16_t>(_normal_op_bram[2 * i] & 0x00FF);
      phase <<= 4;
      d[i] = driver::Phase{phase};
    }
    return d;
  }

  [[nodiscard]] std::vector<std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT>> gain_stm_normal_duty() const {
    const auto cycle = stm_cycle();
    std::vector<std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT>> v;
    v.reserve(cycle);
    for (size_t i = 0; i < cycle; i++) {
      std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT> d{};
      for (size_t j = 0; j < driver::NUM_TRANS_IN_UNIT; j++) d[j] = driver::Duty{_stm_op_bram[512 * i + 2 * j + 1]};
      v.emplace_back(d);
    }
    return v;
  }

  [[nodiscard]] std::vector<std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT>> gain_stm_normal_phase() const {
    const auto cycle = stm_cycle();
    std::vector<std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT>> v;
    v.reserve(cycle);
    for (size_t i = 0; i < cycle; i++) {
      std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT> d{};
      for (size_t j = 0; j < driver::NUM_TRANS_IN_UNIT; j++) d[j] = driver::Phase{_stm_op_bram[512 * i + 2 * j]};
      v.emplace_back(d);
    }
    return v;
  }

  [[nodiscard]] std::vector<std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT>> gain_stm_legacy_duty() const {
    const auto cycle = stm_cycle();
    std::vector<std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT>> v;
    v.reserve(cycle);
    for (size_t i = 0; i < cycle; i++) {
      std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT> d{};
      for (size_t j = 0; j < driver::NUM_TRANS_IN_UNIT; j++) {
        auto duty = static_cast<uint16_t>((_stm_op_bram[256 * i + j] >> 8) & 0x00FF);
        duty = static_cast<uint16_t>(((duty << 3) | 0x07) + 1);
        d[j] = driver::Duty{duty};
      }
      v.emplace_back(d);
    }
    return v;
  }

  [[nodiscard]] std::vector<std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT>> gain_stm_legacy_phase() const {
    const auto cycle = stm_cycle();
    std::vector<std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT>> v;
    v.reserve(cycle);
    for (size_t i = 0; i < cycle; i++) {
      std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT> d{};
      for (size_t j = 0; j < driver::NUM_TRANS_IN_UNIT; j++) {
        auto phase = static_cast<uint16_t>(_stm_op_bram[256 * i + j] & 0x00FF);
        phase <<= 4;
        d[j] = driver::Phase{phase};
      }
      v.emplace_back(d);
    }
    return v;
  }

  [[nodiscard]] std::vector<std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT>> point_stm_duty() const {
    const auto cycle = stm_cycle();
    const auto ultrasound_cycles = cycles();
    std::vector<std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT>> v;
    v.reserve(cycle);
    for (size_t i = 0; i < cycle; i++) {
      std::array<driver::Duty, driver::NUM_TRANS_IN_UNIT> d{};
      for (size_t j = 0; j < driver::NUM_TRANS_IN_UNIT; j++) {
        const auto duty_shift = static_cast<uint16_t>((_stm_op_bram[8 * i + 3] >> 6) & 0x000F);
        d[j] = driver::Duty{static_cast<uint16_t>(ultrasound_cycles[j] >> (duty_shift + 1))};
      }
      v.emplace_back(d);
    }
    return v;
  }

  [[nodiscard]] std::vector<std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT>> point_stm_phase() const {
    const auto cycle = stm_cycle();
    const auto ultrasound_cycles = cycles();
    const auto sound_speed = static_cast<uint64_t>(this->sound_speed());
    std::vector<std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT>> v;
    v.reserve(cycle);
    for (size_t i = 0; i < cycle; i++) {
      std::array<driver::Phase, driver::NUM_TRANS_IN_UNIT> d{};
      auto x = (((_stm_op_bram[8 * i + 1]) << 16) & 0x30000) | _stm_op_bram[8 * i];
      if ((x & 0x20000) != 0) x = -131072 + (x & 0x1FFFF);
      auto y = (((_stm_op_bram[8 * i + 2]) << 14) & 0x3C000) | (((_stm_op_bram[8 * i + 1]) >> 2) & 0x3FFFF);
      if ((y & 0x20000) != 0) y = -131072 + (y & 0x1FFFF);
      auto z = (((_stm_op_bram[8 * i + 3]) << 12) & 0x3F000) | (((_stm_op_bram[8 * i + 2]) >> 4) & 0xFFF);
      if ((z & 0x20000) != 0) z = -131072 + (z & 0x1FFFF);
      for (size_t j = 0; j < driver::NUM_TRANS_IN_UNIT; j++) {
        const auto tr_x = ((fpga::TR_POS[j] >> 16) & 0xFFFF);
        const auto tr_y = (fpga::TR_POS[j] & 0xFFFF);
        const auto d2 = (x - tr_x) * (x - tr_x) + (y - tr_y) * (y - tr_y) + z * z;
        const auto dist = static_cast<uint64_t>(std::sqrt(d2));
        const auto q = (dist << 22) / sound_speed;
        const auto p = q % ultrasound_cycles[j];
        d[j] = driver::Phase{static_cast<uint16_t>(p)};
      }
      v.emplace_back(d);
    }
    return v;
  }

  std::vector<uint16_t> _controller_bram;
  std::vector<uint16_t> _modulator_bram;
  std::vector<uint16_t> _normal_op_bram;
  std::vector<uint16_t> _stm_op_bram;
};

};  // namespace autd3::extra
