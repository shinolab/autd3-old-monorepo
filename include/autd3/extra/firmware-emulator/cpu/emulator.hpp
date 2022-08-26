// File: emulator.hpp
// Project: cpu
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/08/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../fpga/emulator.hpp"
#include "./params.hpp"
#include "autd3/driver/cpu/body.hpp"
#include "autd3/driver/cpu/header.hpp"
#include "autd3/driver/hardware.hpp"

namespace autd3::extra::firmware_emulator::cpu {

constexpr uint16_t GAIN_STM_MODE_PHASE_DUTY_FULL = 0x0001;
constexpr uint16_t GAIN_STM_MODE_PHASE_FULL = 0x0002;
constexpr uint16_t GAIN_STM_MODE_PHASE_HALF = 0x0004;

using driver::Body;
using driver::CPUControlFlags;
using driver::FPGAControlFlags;
using driver::GlobalHeader;
using driver::NUM_TRANS_IN_UNIT;

class CPU {
 public:
  explicit CPU(const size_t id) : _id(id), _msg_id(0), _ack(0), _mod_cycle(0), _stm_cycle(0), _gain_stm_mode(GAIN_STM_MODE_PHASE_DUTY_FULL) {
    _cycles.fill(0);
  }

  [[nodiscard]] size_t id() const { return _id; }

  [[nodiscard]] uint8_t msg_id() const { return _msg_id; }
  [[nodiscard]] uint8_t ack() const { return _ack; }

  [[nodiscard]] const fpga::FPGA& fpga() const { return _fpga; }

  void send(const GlobalHeader& header, const Body& body) { ecat_recv(header, body); }

  void init() {
    _fpga.init();
    _cycles.fill(0x1000);
    clear();
  }

 private:
  static uint16_t get_addr(const uint8_t select, const uint16_t addr) { return static_cast<uint16_t>((select & 0x0003) << 14) | (addr & 0x3FFF); }

  [[nodiscard]] uint16_t bram_read(const uint8_t select, const uint16_t addr) const { return _fpga.read(get_addr(select, addr)); }

  void bram_write(const uint8_t select, const uint16_t addr, const uint16_t data) { _fpga.write(get_addr(select, addr), data); }

  void bram_cpy(const uint8_t select, const uint16_t addr_base, const uint16_t* const data, const size_t size) {
    auto addr = get_addr(select, addr_base);
    for (size_t i = 0; i < size; i++) _fpga.write(addr++, data[i]);
  }

  void bram_set(const uint8_t select, const uint16_t addr_base, const uint16_t data, const size_t size) {
    auto addr = get_addr(select, addr_base);
    for (size_t i = 0; i < size; i++) _fpga.write(addr++, data);
  }

  void synchronize(const Body& body) {
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CYCLE_BASE, body.data, NUM_TRANS_IN_UNIT);
    std::copy_n(body.data, NUM_TRANS_IN_UNIT, _cycles.begin());
    // Do nothing to sync
  }

  void write_mod(const GlobalHeader& header) {
    const auto write = header.size;

    if (header.cpu_flag.contains(CPUControlFlags::MOD_BEGIN)) {
      _mod_cycle = 0;
      bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_ADDR_OFFSET, 0);
      const auto freq_div = header.mod_head().freq_div;
      bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
    }
    const auto* data =
        reinterpret_cast<const uint16_t*>((header.cpu_flag.contains(CPUControlFlags::MOD_BEGIN)) ? header.mod_head().data : header.mod_body().data);
    if (const auto segment_capacity = (_mod_cycle & ~MOD_BUF_SEGMENT_SIZE_MASK) + MOD_BUF_SEGMENT_SIZE - _mod_cycle; write <= segment_capacity) {
      bram_cpy(BRAM_SELECT_MOD, static_cast<uint16_t>((_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1), data, ((static_cast<size_t>(write) + 1) >> 1));
      _mod_cycle += write;
    } else {
      bram_cpy(BRAM_SELECT_MOD, static_cast<uint16_t>((_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1), data, (segment_capacity >> 1));
      _mod_cycle += segment_capacity;
      data += segment_capacity;
      bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_ADDR_OFFSET,
                 static_cast<uint16_t>((_mod_cycle & ~MOD_BUF_SEGMENT_SIZE_MASK) >> MOD_BUF_SEGMENT_SIZE_WIDTH));
      bram_cpy(BRAM_SELECT_MOD, ((_mod_cycle & MOD_BUF_SEGMENT_SIZE_MASK) >> 1), data, ((static_cast<size_t>(write) - segment_capacity + 1) >> 1));
      _mod_cycle += write - segment_capacity;
    }

    if (header.cpu_flag.contains(CPUControlFlags::MOD_END))
      bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_CYCLE, static_cast<uint16_t>(std::max(_mod_cycle, 1u) - 1u));
  }

  void config_silencer(const GlobalHeader& header) {
    const auto step = header.silencer_header().step;
    const auto cycle = header.silencer_header().cycle;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, step);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_CYCLE, cycle);
  }

  void set_mod_delay(const Body& body) { bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_DELAY_BASE, body.data, NUM_TRANS_IN_UNIT); }

  void write_normal_op(const GlobalHeader& header, const Body& body) {
    if (header.fpga_flag.contains(FPGAControlFlags::LEGACY_MODE))
      for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++) bram_write(BRAM_SELECT_NORMAL, static_cast<uint16_t>(i << 1), body.data[i]);
    else if (header.cpu_flag.contains(CPUControlFlags::IS_DUTY))
      for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++) bram_write(BRAM_SELECT_NORMAL, static_cast<uint16_t>(i << 1) + 1, body.data[i]);
    else
      for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++) bram_write(BRAM_SELECT_NORMAL, static_cast<uint16_t>(i << 1), body.data[i]);
  }

  void write_point_stm(const GlobalHeader& header, const Body& body) {
    uint32_t size;
    const uint16_t* src;

    if (header.cpu_flag.contains(CPUControlFlags::STM_BEGIN)) {
      _stm_cycle = 0;
      bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
      size = body.point_stm_head().data()[0];
      const auto freq_div = (static_cast<uint32_t>(body.point_stm_head().data()[2]) << 16) | static_cast<uint32_t>(body.point_stm_head().data()[1]);
      const auto sound_speed =
          (static_cast<uint32_t>(body.point_stm_head().data()[4]) << 16) | static_cast<uint32_t>(body.point_stm_head().data()[3]);

      bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
      bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SOUND_SPEED_0, reinterpret_cast<const uint16_t*>(&sound_speed), 2);
      src = body.point_stm_head().data() + 5;
    } else {
      size = body.point_stm_body().data()[0];
      src = body.point_stm_body().data() + 1;
    }

    if (const auto segment_capacity = (_stm_cycle & ~POINT_STM_BUF_SEGMENT_SIZE_MASK) + POINT_STM_BUF_SEGMENT_SIZE - _stm_cycle;
        size <= segment_capacity) {
      auto dst = static_cast<uint16_t>((_stm_cycle & POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3);
      for (uint32_t i = 0; i < size; i++, dst += 4) {
        bram_write(BRAM_SELECT_STM, dst++, *src++);
        bram_write(BRAM_SELECT_STM, dst++, *src++);
        bram_write(BRAM_SELECT_STM, dst++, *src++);
        bram_write(BRAM_SELECT_STM, dst++, *src++);
      }
      _stm_cycle += size;
    } else {
      auto dst = static_cast<uint16_t>((_stm_cycle & POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3);
      for (uint32_t i = 0; i < segment_capacity; i++, dst += 4) {
        bram_write(BRAM_SELECT_STM, dst++, *src++);
        bram_write(BRAM_SELECT_STM, dst++, *src++);
        bram_write(BRAM_SELECT_STM, dst++, *src++);
        bram_write(BRAM_SELECT_STM, dst++, *src++);
      }
      _stm_cycle += segment_capacity;
      bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET,
                 static_cast<uint16_t>((_stm_cycle & ~POINT_STM_BUF_SEGMENT_SIZE_MASK) >> POINT_STM_BUF_SEGMENT_SIZE_WIDTH));
      dst = static_cast<uint16_t>((_stm_cycle & POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3);
      const auto cnt = size - segment_capacity;
      for (uint32_t i = 0; i < cnt; i++, dst += 4) {
        bram_write(BRAM_SELECT_STM, dst++, *src++);
        bram_write(BRAM_SELECT_STM, dst++, *src++);
        bram_write(BRAM_SELECT_STM, dst++, *src++);
        bram_write(BRAM_SELECT_STM, dst++, *src++);
      }
      _stm_cycle += cnt;
    }
    if (header.cpu_flag.contains(CPUControlFlags::STM_END))
      bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, static_cast<uint16_t>(std::max(_stm_cycle, 1u) - 1u));
  }

  void write_gain_stm(const GlobalHeader& header, const Body& body) {
    if (header.cpu_flag.contains(CPUControlFlags::STM_BEGIN)) {
      _stm_cycle = 0;
      bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET, 0);
      const auto freq_div = (static_cast<uint32_t>(body.gain_stm_head().data()[1]) << 16) | static_cast<uint32_t>(body.gain_stm_head().data()[0]);
      bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
      _gain_stm_mode = body.gain_stm_head().data()[2];
      return;
    }

    auto* src = body.gain_stm_body().data();
    auto dst = static_cast<uint16_t>((_stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9);

    switch (_gain_stm_mode) {
      case GAIN_STM_MODE_PHASE_DUTY_FULL:
        if (header.fpga_flag.contains(FPGAControlFlags::LEGACY_MODE))
          _stm_cycle += 1;
        else if (header.cpu_flag.contains(CPUControlFlags::IS_DUTY)) {
          dst += 1;
          _stm_cycle += 1;
        }
        for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++, dst += 2) bram_write(BRAM_SELECT_STM, dst, *src++);
        break;
      case GAIN_STM_MODE_PHASE_FULL:
        if (header.fpga_flag.contains(FPGAControlFlags::LEGACY_MODE)) {
          for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++, dst += 2) bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (*src++ & 0x00FF));
          _stm_cycle += 1;

          src = body.gain_stm_body().data();
          dst = static_cast<uint16_t>((_stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9);
          for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++, dst += 2) bram_write(BRAM_SELECT_STM, dst, 0xFF00 | (((*src++) >> 8) & 0x00FF));
          _stm_cycle += 1;
        } else if (!header.cpu_flag.contains(CPUControlFlags::IS_DUTY)) {
          for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++) {
            bram_write(BRAM_SELECT_STM, dst++, *src++);
            bram_write(BRAM_SELECT_STM, dst++, _cycles[i] >> 1);
          }
          _stm_cycle += 1;
        }
        break;
      case GAIN_STM_MODE_PHASE_HALF:
        if (header.fpga_flag.contains(FPGAControlFlags::LEGACY_MODE)) {
          for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++, dst += 2) {
            const auto phase = static_cast<uint16_t>(*src++ & 0x000F);
            bram_write(BRAM_SELECT_STM, dst, static_cast<uint16_t>(0xFF00 | (phase << 4) | phase));
          }
          _stm_cycle += 1;

          src = body.gain_stm_body().data();
          dst = static_cast<uint16_t>((_stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9);
          for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++, dst += 2) {
            const auto phase = static_cast<uint16_t>((*src++ >> 4) & 0x000F);
            bram_write(BRAM_SELECT_STM, dst, static_cast<uint16_t>(0xFF00 | (phase << 4) | phase));
          }
          _stm_cycle += 1;

          src = body.gain_stm_body().data();
          dst = static_cast<uint16_t>((_stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9);
          for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++, dst += 2) {
            const auto phase = static_cast<uint16_t>((*src++ >> 8) & 0x000F);
            bram_write(BRAM_SELECT_STM, dst, static_cast<uint16_t>(0xFF00 | (phase << 4) | phase));
          }
          _stm_cycle += 1;

          src = body.gain_stm_body().data();
          dst = static_cast<uint16_t>((_stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9);
          for (size_t i = 0; i < NUM_TRANS_IN_UNIT; i++, dst += 2) {
            const auto phase = static_cast<uint16_t>((*src++ >> 12) & 0x000F);
            bram_write(BRAM_SELECT_STM, dst, static_cast<uint16_t>(0xFF00 | (phase << 4) | phase));
          }
          _stm_cycle += 1;
        }
        break;
      default:
        throw std::runtime_error("Not supported GainSTM mode");
    }

    if ((_stm_cycle & GAIN_STM_BUF_SEGMENT_SIZE_MASK) == 0)
      bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_ADDR_OFFSET,
                 ((_stm_cycle & ~GAIN_STM_BUF_SEGMENT_SIZE_MASK) >> GAIN_STM_BUF_SEGMENT_SIZE_WIDTH));

    if (header.cpu_flag.contains(CPUControlFlags::STM_END))
      bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_STM_CYCLE, static_cast<uint16_t>(std::max(_stm_cycle, 1u) - 1u));
  }

  static uint16_t get_cpu_version() { return CPU_VERSION; }

  uint16_t get_fpga_version() { return bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_VERSION_NUM); }

  uint16_t read_fpga_info() { return bram_read(BRAM_SELECT_CONTROLLER, BRAM_ADDR_FPGA_INFO); }

  void clear() {
    constexpr uint32_t freq_div = 40960;

    constexpr auto ctl_reg = FPGAControlFlags::LEGACY_MODE;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, ctl_reg);

    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_STEP, 10);
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_SILENT_CYCLE, 4096);

    _stm_cycle = 0;

    _mod_cycle = 2;
    bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_CYCLE, static_cast<uint16_t>(std::max(_mod_cycle, 1u) - 1u));
    bram_cpy(BRAM_SELECT_CONTROLLER, BRAM_ADDR_MOD_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
    bram_write(BRAM_SELECT_MOD, 0, 0x0000);

    bram_set(BRAM_SELECT_NORMAL, 0, 0x0000, NUM_TRANS_IN_UNIT << 1);
  }

  void ecat_recv(const GlobalHeader& header, const Body& body) {
    if (_msg_id == header.msg_id) return;

    _msg_id = header.msg_id;
    if (header.fpga_flag.contains(FPGAControlFlags::READS_FPGA_INFO)) _ack = static_cast<uint8_t>(read_fpga_info());

    switch (_msg_id) {
      case driver::MSG_CLEAR:
        clear();
        break;
      case driver::MSG_RD_CPU_VERSION:
        _ack = get_cpu_version() & 0xFF;
        break;
      case driver::MSG_RD_FPGA_VERSION:
        _ack = get_fpga_version() & 0xFF;
        break;
      case driver::MSG_RD_FPGA_FUNCTION:
        _ack = (get_fpga_version() >> 8) & 0xFF;
        break;
      default:
        if (_msg_id > driver::MSG_END) return;

        const auto ctl_reg = header.fpga_flag;
        bram_write(BRAM_SELECT_CONTROLLER, BRAM_ADDR_CTL_REG, ctl_reg.value());

        if (header.cpu_flag.contains(CPUControlFlags::MOD))
          write_mod(header);
        else if (header.cpu_flag.contains(CPUControlFlags::CONFIG_SILENCER))
          config_silencer(header);
        else if (header.cpu_flag.contains(CPUControlFlags::CONFIG_SYNC)) {
          synchronize(body);
          return;
        }

        if (!header.cpu_flag.contains(CPUControlFlags::WRITE_BODY)) return;

        if (header.cpu_flag.contains(CPUControlFlags::MOD_DELAY)) {
          set_mod_delay(body);
          return;
        }

        if (!ctl_reg.contains(FPGAControlFlags::STM_MODE)) {
          write_normal_op(header, body);
          return;
        }

        if (!ctl_reg.contains(FPGAControlFlags::STM_GAIN_MODE))
          write_point_stm(header, body);
        else
          write_gain_stm(header, body);

        break;
    }
  }

  size_t _id;
  uint8_t _msg_id;
  uint8_t _ack;
  uint32_t _mod_cycle;
  uint32_t _stm_cycle;
  fpga::FPGA _fpga;
  uint16_t _gain_stm_mode;
  std::array<uint16_t, NUM_TRANS_IN_UNIT> _cycles{};
};

};  // namespace autd3::extra::firmware_emulator::cpu
