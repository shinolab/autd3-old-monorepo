// File: emulator.hpp
// Project: cpu
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmaybe-uninitialized"
#pragma GCC diagnostic ignored "-Wuninitialized"
#endif

#include <algorithm>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/hardware.hpp"
#include "fpga_emulator.hpp"

namespace autd3::extra::cpu {
constexpr uint16_t CPU_VERSION = 0x86;

constexpr uint8_t BRAM_SELECT_CONTROLLER = 0x0;
constexpr uint8_t BRAM_SELECT_MOD = 0x1;
constexpr uint8_t BRAM_SELECT_NORMAL = 0x2;
constexpr uint8_t BRAM_SELECT_STM = 0x3;

constexpr uint16_t BRAM_ADDR_CTL_REG = 0x000;
constexpr uint16_t BRAM_ADDR_FPGA_INFO = 0x001;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_0 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 1;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_1 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 2;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_2 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 3;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_3 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 4;
constexpr uint16_t BRAM_ADDR_MOD_ADDR_OFFSET = 0x020;
constexpr uint16_t BRAM_ADDR_MOD_CYCLE = 0x021;
constexpr uint16_t BRAM_ADDR_MOD_FREQ_DIV_0 = 0x022;
constexpr uint16_t BRAM_ADDR_VERSION_NUM = 0x03F;
constexpr uint16_t BRAM_ADDR_SILENT_CYCLE = 0x040;
constexpr uint16_t BRAM_ADDR_SILENT_STEP = 0x041;
constexpr uint16_t BRAM_ADDR_STM_ADDR_OFFSET = 0x050;
constexpr uint16_t BRAM_ADDR_STM_CYCLE = 0x051;
constexpr uint16_t BRAM_ADDR_STM_FREQ_DIV_0 = 0x052;
constexpr uint16_t BRAM_ADDR_SOUND_SPEED_0 = 0x054;
constexpr uint16_t BRAM_ADDR_CYCLE_BASE = 0x100;
constexpr uint16_t BRAM_ADDR_MOD_DELAY_BASE = 0x200;

constexpr uint32_t MOD_BUF_SEGMENT_SIZE_WIDTH = 15;
constexpr uint32_t MOD_BUF_SEGMENT_SIZE = 1 << MOD_BUF_SEGMENT_SIZE_WIDTH;
constexpr uint32_t MOD_BUF_SEGMENT_SIZE_MASK = MOD_BUF_SEGMENT_SIZE - 1;
constexpr uint32_t POINT_STM_BUF_SEGMENT_SIZE_WIDTH = 11;
constexpr uint32_t POINT_STM_BUF_SEGMENT_SIZE = 1 << POINT_STM_BUF_SEGMENT_SIZE_WIDTH;
constexpr uint32_t POINT_STM_BUF_SEGMENT_SIZE_MASK = POINT_STM_BUF_SEGMENT_SIZE - 1;
constexpr uint32_t GAIN_STM_BUF_SEGMENT_SIZE_WIDTH = 5;
constexpr uint32_t GAIN_STM_BUF_SEGMENT_SIZE = 1 << GAIN_STM_BUF_SEGMENT_SIZE_WIDTH;
constexpr uint32_t GAIN_STM_BUF_SEGMENT_SIZE_MASK = GAIN_STM_BUF_SEGMENT_SIZE - 1;
constexpr uint32_t GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH = 6;
constexpr uint32_t GAIN_STM_LEGACY_BUF_SEGMENT_SIZE = 1 << GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH;
constexpr uint32_t GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK = GAIN_STM_LEGACY_BUF_SEGMENT_SIZE - 1;

constexpr uint16_t GAIN_STM_MODE_PHASE_DUTY_FULL = 0x0001;
constexpr uint16_t GAIN_STM_MODE_PHASE_FULL = 0x0002;
constexpr uint16_t GAIN_STM_MODE_PHASE_HALF = 0x0004;

}  // namespace autd3::extra::cpu

namespace autd3::extra {

class CPU {
 public:
  explicit CPU(const size_t id, const bool enable_fpga = true)
      : _id(id), _enable_fpga(enable_fpga), _msg_id(0), _ack(0), _mod_cycle(0), _stm_cycle(0), _gain_stm_mode(cpu::GAIN_STM_MODE_PHASE_DUTY_FULL) {
    _cycles.fill(0);
  }

  [[nodiscard]] size_t id() const { return _id; }

  [[nodiscard]] uint8_t msg_id() const { return _msg_id; }
  [[nodiscard]] uint8_t ack() const { return _ack; }
  [[nodiscard]] driver::FPGAControlFlags fpga_flags() const { return _fpga_flags; }
  [[nodiscard]] driver::CPUControlFlags cpu_flags() const { return _cpu_flags; }

  [[nodiscard]] const FPGA& fpga() const { return _fpga; }

  void send(const driver::GlobalHeader* header, const driver::Body* body) { ecat_recv(header, body); }
  void send(const driver::TxDatagram& tx) { ecat_recv(&tx.header(), tx.num_bodies > _id ? tx.bodies() + _id : nullptr); }

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

  void synchronize(const driver::Body* body) {
    if (body == nullptr) return;
    bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_CYCLE_BASE, body->data, driver::NUM_TRANS_IN_UNIT);
    std::copy_n(body->data, driver::NUM_TRANS_IN_UNIT, _cycles.begin());
    // Do nothing to sync
  }

  void write_mod(const driver::GlobalHeader* header) {
    const auto write = header->size;

    if (header->cpu_flag.contains(driver::CPUControlFlags::MOD_BEGIN)) {
      _mod_cycle = 0;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_ADDR_OFFSET, 0);
      const auto freq_div = header->mod_head().freq_div;
      bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
    }
    const auto* data = reinterpret_cast<const uint16_t*>((header->cpu_flag.contains(driver::CPUControlFlags::MOD_BEGIN)) ? header->mod_head().data
                                                                                                                         : header->mod_body().data);
    if (const auto segment_capacity = (_mod_cycle & ~cpu::MOD_BUF_SEGMENT_SIZE_MASK) + cpu::MOD_BUF_SEGMENT_SIZE - _mod_cycle;
        write <= segment_capacity) {
      bram_cpy(cpu::BRAM_SELECT_MOD, static_cast<uint16_t>((_mod_cycle & cpu::MOD_BUF_SEGMENT_SIZE_MASK) >> 1), data,
               ((static_cast<size_t>(write) + 1) >> 1));
      _mod_cycle += write;
    } else {
      bram_cpy(cpu::BRAM_SELECT_MOD, static_cast<uint16_t>((_mod_cycle & cpu::MOD_BUF_SEGMENT_SIZE_MASK) >> 1), data, (segment_capacity >> 1));
      _mod_cycle += segment_capacity;
      data += segment_capacity;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_ADDR_OFFSET,
                 static_cast<uint16_t>((_mod_cycle & ~cpu::MOD_BUF_SEGMENT_SIZE_MASK) >> cpu::MOD_BUF_SEGMENT_SIZE_WIDTH));
      bram_cpy(cpu::BRAM_SELECT_MOD, ((_mod_cycle & cpu::MOD_BUF_SEGMENT_SIZE_MASK) >> 1), data,
               ((static_cast<size_t>(write) - segment_capacity + 1) >> 1));
      _mod_cycle += write - segment_capacity;
    }

    if (header->cpu_flag.contains(driver::CPUControlFlags::MOD_END))
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_CYCLE, static_cast<uint16_t>((std::max)(_mod_cycle, 1u) - 1u));
  }

  void config_silencer(const driver::GlobalHeader* header) {
    const auto step = header->silencer_header().step;
    const auto cycle = header->silencer_header().cycle;
    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SILENT_STEP, step);
    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SILENT_CYCLE, cycle);
  }

  void set_mod_delay(const driver::Body* body) {
    if (body == nullptr) return;
    bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_DELAY_BASE, body->data, driver::NUM_TRANS_IN_UNIT);
  }

  void write_normal_op(const driver::GlobalHeader* header, const driver::Body* body) {
    if (body == nullptr) return;
    if (header->fpga_flag.contains(driver::FPGAControlFlags::LEGACY_MODE))
      for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) bram_write(cpu::BRAM_SELECT_NORMAL, static_cast<uint16_t>(i << 1), body->data[i]);
    else if (header->cpu_flag.contains(driver::CPUControlFlags::IS_DUTY))
      for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) bram_write(cpu::BRAM_SELECT_NORMAL, static_cast<uint16_t>(i << 1) + 1, body->data[i]);
    else
      for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) bram_write(cpu::BRAM_SELECT_NORMAL, static_cast<uint16_t>(i << 1), body->data[i]);
  }

  void write_point_stm(const driver::GlobalHeader* header, const driver::Body* body) {
    if (body == nullptr) return;
    uint32_t size;
    const uint16_t* src;

    if (header->cpu_flag.contains(driver::CPUControlFlags::STM_BEGIN)) {
      _stm_cycle = 0;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET, 0);
      size = body->point_stm_head().data()[0];
      const auto freq_div = (static_cast<uint32_t>(body->point_stm_head().data()[2]) << 16) | static_cast<uint32_t>(body->point_stm_head().data()[1]);
      const auto sound_speed =
          (static_cast<uint32_t>(body->point_stm_head().data()[4]) << 16) | static_cast<uint32_t>(body->point_stm_head().data()[3]);

      bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
      bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SOUND_SPEED_0, reinterpret_cast<const uint16_t*>(&sound_speed), 2);
      src = body->point_stm_head().data() + 5;
    } else {
      size = body->point_stm_body().data()[0];
      src = body->point_stm_body().data() + 1;
    }

    if (const auto segment_capacity = (_stm_cycle & ~cpu::POINT_STM_BUF_SEGMENT_SIZE_MASK) + cpu::POINT_STM_BUF_SEGMENT_SIZE - _stm_cycle;
        size <= segment_capacity) {
      auto dst = static_cast<uint16_t>((_stm_cycle & cpu::POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3);
      for (uint32_t i = 0; i < size; i++, dst += 4) {
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
      }
      _stm_cycle += size;
    } else {
      auto dst = static_cast<uint16_t>((_stm_cycle & cpu::POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3);
      for (uint32_t i = 0; i < segment_capacity; i++, dst += 4) {
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
      }
      _stm_cycle += segment_capacity;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET,
                 static_cast<uint16_t>((_stm_cycle & ~cpu::POINT_STM_BUF_SEGMENT_SIZE_MASK) >> cpu::POINT_STM_BUF_SEGMENT_SIZE_WIDTH));
      dst = static_cast<uint16_t>((_stm_cycle & cpu::POINT_STM_BUF_SEGMENT_SIZE_MASK) << 3);
      const auto cnt = size - segment_capacity;
      for (uint32_t i = 0; i < cnt; i++, dst += 4) {
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
      }
      _stm_cycle += cnt;
    }
    if (header->cpu_flag.contains(driver::CPUControlFlags::STM_END))
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_CYCLE, static_cast<uint16_t>((std::max)(_stm_cycle, 1u) - 1u));
  }

  void write_gain_stm_legacy(const driver::GlobalHeader* header, const driver::Body* body) {
    if (body == nullptr) return;
    if (header->cpu_flag.contains(driver::CPUControlFlags::STM_BEGIN)) {
      _stm_cycle = 0;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET, 0);
      const auto freq_div = (static_cast<uint32_t>(body->gain_stm_head().data()[1]) << 16) | static_cast<uint32_t>(body->gain_stm_head().data()[0]);
      bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
      _gain_stm_mode = body->gain_stm_head().data()[2];
      return;
    }

    auto* src = body->gain_stm_body().data();
    auto dst = static_cast<uint16_t>((_stm_cycle & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);

    switch (_gain_stm_mode) {
      case cpu::GAIN_STM_MODE_PHASE_DUTY_FULL:
        _stm_cycle += 1;
        for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        break;
      case cpu::GAIN_STM_MODE_PHASE_FULL:
        for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) bram_write(cpu::BRAM_SELECT_STM, dst++, 0xFF00 | (*src++ & 0x00FF));
        _stm_cycle += 1;
        src = body->gain_stm_body().data();
        dst = static_cast<uint16_t>((_stm_cycle & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) bram_write(cpu::BRAM_SELECT_STM, dst++, 0xFF00 | (((*src++) >> 8) & 0x00FF));
        _stm_cycle += 1;
        break;
      case cpu::GAIN_STM_MODE_PHASE_HALF:
        for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) {
          const auto phase = static_cast<uint16_t>(*src++ & 0x000F);
          bram_write(cpu::BRAM_SELECT_STM, dst++, static_cast<uint16_t>(0xFF00 | (phase << 4) | phase));
        }
        _stm_cycle += 1;

        src = body->gain_stm_body().data();
        dst = static_cast<uint16_t>((_stm_cycle & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) {
          const auto phase = static_cast<uint16_t>((*src++ >> 4) & 0x000F);
          bram_write(cpu::BRAM_SELECT_STM, dst++, static_cast<uint16_t>(0xFF00 | (phase << 4) | phase));
        }
        _stm_cycle += 1;

        src = body->gain_stm_body().data();
        dst = static_cast<uint16_t>((_stm_cycle & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) {
          const auto phase = static_cast<uint16_t>((*src++ >> 8) & 0x000F);
          bram_write(cpu::BRAM_SELECT_STM, dst++, static_cast<uint16_t>(0xFF00 | (phase << 4) | phase));
        }
        _stm_cycle += 1;

        src = body->gain_stm_body().data();
        dst = static_cast<uint16_t>((_stm_cycle & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) {
          const auto phase = static_cast<uint16_t>((*src++ >> 12) & 0x000F);
          bram_write(cpu::BRAM_SELECT_STM, dst++, static_cast<uint16_t>(0xFF00 | (phase << 4) | phase));
        }
        _stm_cycle += 1;
        break;
      default:
        throw std::runtime_error("Not supported GainSTM mode");
    }

    if ((_stm_cycle & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) == 0)
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET,
                 ((_stm_cycle & ~cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) >> cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH));

    if (header->cpu_flag.contains(driver::CPUControlFlags::STM_END))
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_CYCLE, static_cast<uint16_t>((std::max)(_stm_cycle, 1u) - 1u));
  }

  void write_gain_stm(const driver::GlobalHeader* header, const driver::Body* body) {
    if (body == nullptr) return;
    if (header->cpu_flag.contains(driver::CPUControlFlags::STM_BEGIN)) {
      _stm_cycle = 0;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET, 0);
      const auto freq_div = (static_cast<uint32_t>(body->gain_stm_head().data()[1]) << 16) | static_cast<uint32_t>(body->gain_stm_head().data()[0]);
      bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
      _gain_stm_mode = body->gain_stm_head().data()[2];
      return;
    }

    auto* src = body->gain_stm_body().data();
    auto dst = static_cast<uint16_t>((_stm_cycle & cpu::GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9);

    switch (_gain_stm_mode) {
      case cpu::GAIN_STM_MODE_PHASE_DUTY_FULL:
        if (header->cpu_flag.contains(driver::CPUControlFlags::IS_DUTY)) {
          dst += 1;
          _stm_cycle += 1;
        }
        for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++, dst += 2) bram_write(cpu::BRAM_SELECT_STM, dst, *src++);
        break;
      case cpu::GAIN_STM_MODE_PHASE_FULL:
        if (!header->cpu_flag.contains(driver::CPUControlFlags::IS_DUTY)) {
          for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) {
            bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
            bram_write(cpu::BRAM_SELECT_STM, dst++, _cycles[i] >> 1);
          }
          _stm_cycle += 1;
        }
        break;
      case cpu::GAIN_STM_MODE_PHASE_HALF:
        throw std::runtime_error("Phase half mode is not supported in Normal GainSTM");
        break;
      default:
        throw std::runtime_error("Not supported GainSTM mode");
    }

    if ((_stm_cycle & cpu::GAIN_STM_BUF_SEGMENT_SIZE_MASK) == 0)
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET,
                 ((_stm_cycle & ~cpu::GAIN_STM_BUF_SEGMENT_SIZE_MASK) >> cpu::GAIN_STM_BUF_SEGMENT_SIZE_WIDTH));

    if (header->cpu_flag.contains(driver::CPUControlFlags::STM_END))
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_CYCLE, static_cast<uint16_t>((std::max)(_stm_cycle, 1u) - 1u));
  }

  static uint16_t get_cpu_version() { return cpu::CPU_VERSION; }

  uint16_t get_fpga_version() { return bram_read(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_VERSION_NUM); }

  uint16_t read_fpga_info() { return bram_read(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_FPGA_INFO); }

  void clear() {
    constexpr uint32_t freq_div = 40960;

    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_CTL_REG, 0x0000);

    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SILENT_STEP, 10);
    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SILENT_CYCLE, 4096);

    _stm_cycle = 0;

    _mod_cycle = 2;
    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_CYCLE, static_cast<uint16_t>((std::max)(_mod_cycle, 1u) - 1u));
    bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
    bram_write(cpu::BRAM_SELECT_MOD, 0, 0x0000);

    bram_set(cpu::BRAM_SELECT_NORMAL, 0, 0x0000, driver::NUM_TRANS_IN_UNIT << 1);
  }

  void ecat_recv(const driver::GlobalHeader* header, const driver::Body* body) {
    if (_msg_id == header->msg_id) return;

    _msg_id = header->msg_id;
    _fpga_flags = header->fpga_flag;
    _cpu_flags = header->cpu_flag;
    if (header->fpga_flag.contains(driver::FPGAControlFlags::READS_FPGA_INFO)) _ack = static_cast<uint8_t>(read_fpga_info());

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
        if (!_enable_fpga || _msg_id > driver::MSG_END) return;

        const auto ctl_reg = header->fpga_flag;
        bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_CTL_REG, ctl_reg.value());

        if (header->cpu_flag.contains(driver::CPUControlFlags::MOD))
          write_mod(header);
        else if (header->cpu_flag.contains(driver::CPUControlFlags::CONFIG_SILENCER))
          config_silencer(header);
        else if (header->cpu_flag.contains(driver::CPUControlFlags::CONFIG_SYNC)) {
          synchronize(body);
          return;
        }

        if (!header->cpu_flag.contains(driver::CPUControlFlags::WRITE_BODY)) return;

        if (header->cpu_flag.contains(driver::CPUControlFlags::MOD_DELAY)) {
          set_mod_delay(body);
          return;
        }

        if (!ctl_reg.contains(driver::FPGAControlFlags::STM_MODE)) {
          write_normal_op(header, body);
          return;
        }

        if (!ctl_reg.contains(driver::FPGAControlFlags::STM_GAIN_MODE))
          write_point_stm(header, body);
        else if (header->fpga_flag.contains(driver::FPGAControlFlags::LEGACY_MODE))
          write_gain_stm_legacy(header, body);
        else
          write_gain_stm(header, body);

        break;
    }
  }

  size_t _id;
  bool _enable_fpga;
  uint8_t _msg_id;
  uint8_t _ack;
  uint32_t _mod_cycle;
  uint32_t _stm_cycle;
  FPGA _fpga;
  uint16_t _gain_stm_mode;
  std::array<uint16_t, driver::NUM_TRANS_IN_UNIT> _cycles{};

  driver::FPGAControlFlags _fpga_flags;
  driver::CPUControlFlags _cpu_flags;
};

};  // namespace autd3::extra

#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
