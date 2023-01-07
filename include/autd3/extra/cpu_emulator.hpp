// File: emulator.hpp
// Project: cpu
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
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

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 6285 6385 26437 26800 26498 26451 26495 26450)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#endif
#include "spdlog/spdlog.h"
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

#include <algorithm>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/extra/fpga_emulator.hpp"

namespace autd3::extra::cpu {
constexpr uint16_t CPU_VERSION = 0x87;

constexpr uint8_t BRAM_SELECT_CONTROLLER = 0x0;
constexpr uint8_t BRAM_SELECT_MOD = 0x1;
constexpr uint8_t BRAM_SELECT_NORMAL = 0x2;
constexpr uint8_t BRAM_SELECT_STM = 0x3;

constexpr uint16_t CTL_REG_OP_MODE_BIT = 9;
constexpr uint16_t CTL_REG_OP_MODE = 1 << CTL_REG_OP_MODE_BIT;

constexpr uint16_t BRAM_ADDR_CTL_REG = 0x000;
constexpr uint16_t BRAM_ADDR_FPGA_INFO = 0x001;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_0 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 1;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_1 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 2;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_2 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 3;
// constexpr uint16_t BRAM_ADDR_EC_SYNC_TIME_3 = BRAM_ADDR_EC_SYNC_CYCLE_TICKS + 4;
constexpr uint16_t BRAM_ADDR_MOD_ADDR_OFFSET = 0x020;
constexpr uint16_t BRAM_ADDR_MOD_CYCLE = 0x021;
constexpr uint16_t BRAM_ADDR_MOD_FREQ_DIV_0 = 0x022;
constexpr uint16_t BRAM_ADDR_MOD_FREQ_DIV_1 = BRAM_ADDR_MOD_FREQ_DIV_0 + 1;
constexpr uint16_t BRAM_ADDR_VERSION_NUM = 0x03F;
constexpr uint16_t BRAM_ADDR_SILENT_CYCLE = 0x040;
constexpr uint16_t BRAM_ADDR_SILENT_STEP = 0x041;
constexpr uint16_t BRAM_ADDR_STM_ADDR_OFFSET = 0x050;
constexpr uint16_t BRAM_ADDR_STM_CYCLE = 0x051;
constexpr uint16_t BRAM_ADDR_STM_FREQ_DIV_0 = 0x052;
constexpr uint16_t BRAM_ADDR_STM_FREQ_DIV_1 = 0x053;
constexpr uint16_t BRAM_ADDR_SOUND_SPEED_0 = 0x054;
constexpr uint16_t BRAM_ADDR_SOUND_SPEED_1 = 0x055;
constexpr uint16_t BRAM_ADDR_STM_START_IDX = 0x056;
constexpr uint16_t BRAM_ADDR_STM_FINISH_IDX = 0x057;
constexpr uint16_t BRAM_ADDR_CYCLE_BASE = 0x100;
constexpr uint16_t BRAM_ADDR_MOD_DELAY_BASE = 0x200;

constexpr uint32_t MOD_BUF_SEGMENT_SIZE_WIDTH = 15;
constexpr uint32_t MOD_BUF_SEGMENT_SIZE = 1 << MOD_BUF_SEGMENT_SIZE_WIDTH;
constexpr uint32_t MOD_BUF_SEGMENT_SIZE_MASK = MOD_BUF_SEGMENT_SIZE - 1;
constexpr uint32_t FOCUS_STM_BUF_SEGMENT_SIZE_WIDTH = 11;
constexpr uint32_t FOCUS_STM_BUF_SEGMENT_SIZE = 1 << FOCUS_STM_BUF_SEGMENT_SIZE_WIDTH;
constexpr uint32_t FOCUS_STM_BUF_SEGMENT_SIZE_MASK = FOCUS_STM_BUF_SEGMENT_SIZE - 1;
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

/**
 * @brief CPU board firmware emulator
 */
class CPU {
 public:
  explicit CPU(const size_t id, const size_t num_transducers)
      : _id(id),
        _num_transducers(num_transducers),
        _msg_id(0),
        _ack(0),
        _mod_cycle(0),
        _stm_write(0),
        _stm_cycle(0),
        _fpga(num_transducers),
        _gain_stm_mode(cpu::GAIN_STM_MODE_PHASE_DUTY_FULL),
        _fpga_flags(driver::FPGAControlFlags::None),
        _cpu_flags(driver::CPUControlFlags::None) {
    _cycles.resize(num_transducers, 0x0000);
  }

  [[nodiscard]] size_t id() const { return _id; }

  [[nodiscard]] uint8_t msg_id() const { return _msg_id; }
  [[nodiscard]] uint8_t ack() const { return _ack; }
  [[nodiscard]] driver::FPGAControlFlags fpga_flags() const { return _fpga_flags; }
  [[nodiscard]] driver::CPUControlFlags cpu_flags() const { return _cpu_flags; }

  [[nodiscard]] const FPGA& fpga() const { return _fpga; }

  void send(const driver::GlobalHeader* header, const driver::Body* body) { ecat_recv(header, body); }
  void send(const driver::TxDatagram& tx) { ecat_recv(&tx.header(), tx.num_bodies > _id ? &tx.body(_id) : nullptr); }

  void init() {
    _fpga.init();
    std::fill(_cycles.begin(), _cycles.end(), 0x1000);
    clear();
  }

  [[nodiscard]] bool configure_local_trans_pos(const std::vector<driver::Vector3>& local_trans_pos) {
    return _fpga.configure_local_trans_pos(local_trans_pos);
  }

 private:
  static uint16_t get_addr(const uint8_t select, const uint16_t addr) {
    const auto h = static_cast<uint16_t>((select & 0x0003) << 14);
    const uint16_t l = addr & 0x3FFF;
    return h | l;
  }

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
    bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_CYCLE_BASE, reinterpret_cast<const uint16_t*>(body), _num_transducers);
    std::copy_n(reinterpret_cast<const uint16_t*>(body), _num_transducers, _cycles.begin());
    // Do nothing to sync
  }

  void write_mod(const driver::GlobalHeader* header) {
    const auto write = header->size;

    if (header->cpu_flag.contains(driver::CPUControlFlags::ModBegin)) {
      _mod_cycle = 0;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_ADDR_OFFSET, 0);
      const auto freq_div = header->mod_initial().freq_div;
      bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
    }
    const auto* data = reinterpret_cast<const uint16_t*>(
        header->cpu_flag.contains(driver::CPUControlFlags::ModBegin) ? header->mod_initial().data : header->mod_subsequent().data);
    if (const auto segment_capacity = (_mod_cycle & ~cpu::MOD_BUF_SEGMENT_SIZE_MASK) + cpu::MOD_BUF_SEGMENT_SIZE - _mod_cycle;
        write <= segment_capacity) {
      bram_cpy(cpu::BRAM_SELECT_MOD, static_cast<uint16_t>((_mod_cycle & cpu::MOD_BUF_SEGMENT_SIZE_MASK) >> 1), data,
               (static_cast<size_t>(write) + 1) / 2);
      _mod_cycle += write;
    } else {
      bram_cpy(cpu::BRAM_SELECT_MOD, static_cast<uint16_t>((_mod_cycle & cpu::MOD_BUF_SEGMENT_SIZE_MASK) >> 1), data, segment_capacity >> 1);
      _mod_cycle += segment_capacity;
      data += segment_capacity;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_ADDR_OFFSET,
                 static_cast<uint16_t>((_mod_cycle & ~cpu::MOD_BUF_SEGMENT_SIZE_MASK) >> cpu::MOD_BUF_SEGMENT_SIZE_WIDTH));
      bram_cpy(cpu::BRAM_SELECT_MOD, (_mod_cycle & cpu::MOD_BUF_SEGMENT_SIZE_MASK) >> 1, data,
               (static_cast<size_t>(write) - segment_capacity + 1) / 2);
      _mod_cycle += write - segment_capacity;
    }

    if (header->cpu_flag.contains(driver::CPUControlFlags::ModEnd))
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_CYCLE, static_cast<uint16_t>((std::max)(_mod_cycle, 1u) - 1u));
  }

  void config_silencer(const driver::GlobalHeader* header) {
    const auto step = header->silencer().step;
    const auto cycle = header->silencer().cycle;
    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SILENT_STEP, step);
    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SILENT_CYCLE, cycle);
  }

  void set_mod_delay(const driver::Body* body) {
    if (body == nullptr) return;
    bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_DELAY_BASE, reinterpret_cast<const uint16_t*>(body), _num_transducers);
  }

  void write_normal_op(const driver::GlobalHeader* header, const driver::Body* body) {
    if (body == nullptr) return;
    if (header->fpga_flag.contains(driver::FPGAControlFlags::LegacyMode))
      for (size_t i = 0; i < _num_transducers; i++)
        bram_write(cpu::BRAM_SELECT_NORMAL, static_cast<uint16_t>(i << 1), reinterpret_cast<const uint16_t*>(body)[i]);
    else if (header->cpu_flag.contains(driver::CPUControlFlags::IsDuty))
      for (size_t i = 0; i < _num_transducers; i++)
        bram_write(cpu::BRAM_SELECT_NORMAL, static_cast<uint16_t>(i << 1) + 1, reinterpret_cast<const uint16_t*>(body)[i]);
    else
      for (size_t i = 0; i < _num_transducers; i++)
        bram_write(cpu::BRAM_SELECT_NORMAL, static_cast<uint16_t>(i << 1), reinterpret_cast<const uint16_t*>(body)[i]);
  }

  void write_focus_stm(const driver::GlobalHeader* header, const driver::Body* body) {
    if (body == nullptr) return;
    uint32_t size;
    const uint16_t* src;

    if (header->cpu_flag.contains(driver::CPUControlFlags::STMBegin)) {
      _stm_write = 0;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET, 0);
      size = body->focus_stm_initial().data()[0];
      const auto freq_div =
          static_cast<uint32_t>(body->focus_stm_initial().data()[2]) << 16 | static_cast<uint32_t>(body->focus_stm_initial().data()[1]);
      const auto sound_speed =
          static_cast<uint32_t>(body->focus_stm_initial().data()[4]) << 16 | static_cast<uint32_t>(body->focus_stm_initial().data()[3]);
      const auto start_idx = body->focus_stm_initial().data()[5];
      const auto finish_idx = body->focus_stm_initial().data()[6];

      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_FREQ_DIV_0, static_cast<uint16_t>(freq_div & 0xFFFF));
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_FREQ_DIV_1, static_cast<uint16_t>(freq_div >> 16 & 0xFFFF));
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SOUND_SPEED_0, static_cast<uint16_t>(sound_speed & 0xFFFF));
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SOUND_SPEED_1, static_cast<uint16_t>(sound_speed >> 16 & 0xFFFF));
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_START_IDX, start_idx);
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_FINISH_IDX, finish_idx);
      src = body->focus_stm_initial().data() + 7;
    } else {
      size = body->focus_stm_subsequent().data()[0];
      src = body->focus_stm_subsequent().data() + 1;
    }

    if (const auto segment_capacity = (_stm_write & ~cpu::FOCUS_STM_BUF_SEGMENT_SIZE_MASK) + cpu::FOCUS_STM_BUF_SEGMENT_SIZE - _stm_write;
        size <= segment_capacity) {
      auto dst = static_cast<uint16_t>((_stm_write & cpu::FOCUS_STM_BUF_SEGMENT_SIZE_MASK) << 3);
      for (uint32_t i = 0; i < size; i++, dst += 4) {
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
      }
      _stm_write += size;
    } else {
      auto dst = static_cast<uint16_t>((_stm_write & cpu::FOCUS_STM_BUF_SEGMENT_SIZE_MASK) << 3);
      for (uint32_t i = 0; i < segment_capacity; i++, dst += 4) {
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
      }
      _stm_write += segment_capacity;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET,
                 static_cast<uint16_t>((_stm_write & ~cpu::FOCUS_STM_BUF_SEGMENT_SIZE_MASK) >> cpu::FOCUS_STM_BUF_SEGMENT_SIZE_WIDTH));
      dst = static_cast<uint16_t>((_stm_write & cpu::FOCUS_STM_BUF_SEGMENT_SIZE_MASK) << 3);
      const auto cnt = size - segment_capacity;
      for (uint32_t i = 0; i < cnt; i++, dst += 4) {
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
      }
      _stm_write += cnt;
    }
    if (header->cpu_flag.contains(driver::CPUControlFlags::STMEnd)) {
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_CYCLE, static_cast<uint16_t>((std::max)(_stm_write, 1u) - 1u));
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_CTL_REG, header->fpga_flag.value() | cpu::CTL_REG_OP_MODE);
    }
  }

  void write_gain_stm_legacy(const driver::GlobalHeader* header, const driver::Body* body) {
    if (body == nullptr) return;
    if (header->cpu_flag.contains(driver::CPUControlFlags::STMBegin)) {
      _stm_write = 0;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET, 0);
      const auto freq_div =
          static_cast<uint32_t>(body->gain_stm_initial().data()[1]) << 16 | static_cast<uint32_t>(body->gain_stm_initial().data()[0]);
      bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
      _gain_stm_mode = body->gain_stm_initial().data()[2];
      _stm_cycle = body->gain_stm_initial().data()[3];

      const auto start_idx = body->gain_stm_initial().data()[4];
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_START_IDX, start_idx);
      const auto finish_idx = body->gain_stm_initial().data()[5];
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_FINISH_IDX, finish_idx);

      return;
    }

    auto* src = body->gain_stm_subsequent().data();
    auto dst = static_cast<uint16_t>((_stm_write & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);

    switch (_gain_stm_mode) {
      case cpu::GAIN_STM_MODE_PHASE_DUTY_FULL:
        _stm_write += 1;
        for (size_t i = 0; i < _num_transducers; i++) bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
        break;
      case cpu::GAIN_STM_MODE_PHASE_FULL:
        for (size_t i = 0; i < _num_transducers; i++) bram_write(cpu::BRAM_SELECT_STM, dst++, 0xFF00 | *src++);
        _stm_write += 1;
        src = body->gain_stm_subsequent().data();
        dst = static_cast<uint16_t>((_stm_write & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        for (size_t i = 0; i < _num_transducers; i++) bram_write(cpu::BRAM_SELECT_STM, dst++, 0xFF00 | *src++ >> 8);
        _stm_write += 1;
        break;
      case cpu::GAIN_STM_MODE_PHASE_HALF:
        for (size_t i = 0; i < _num_transducers; i++) {
          const auto phase = static_cast<uint16_t>(*src++ & 0x000F);
          bram_write(cpu::BRAM_SELECT_STM, dst++, static_cast<uint16_t>(0xFF00 | phase << 4 | phase));
        }
        _stm_write += 1;

        src = body->gain_stm_subsequent().data();
        dst = static_cast<uint16_t>((_stm_write & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        for (size_t i = 0; i < _num_transducers; i++) {
          const auto phase = static_cast<uint16_t>(*src++ >> 4 & 0x000F);
          bram_write(cpu::BRAM_SELECT_STM, dst++, static_cast<uint16_t>(0xFF00 | phase << 4 | phase));
        }
        _stm_write += 1;

        src = body->gain_stm_subsequent().data();
        dst = static_cast<uint16_t>((_stm_write & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        for (size_t i = 0; i < _num_transducers; i++) {
          const auto phase = static_cast<uint16_t>(*src++ >> 8 & 0x000F);
          bram_write(cpu::BRAM_SELECT_STM, dst++, static_cast<uint16_t>(0xFF00 | phase << 4 | phase));
        }
        _stm_write += 1;

        src = body->gain_stm_subsequent().data();
        dst = static_cast<uint16_t>((_stm_write & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) << 8);
        for (size_t i = 0; i < _num_transducers; i++) {
          const auto phase = static_cast<uint16_t>(*src++ >> 12 & 0x000F);
          bram_write(cpu::BRAM_SELECT_STM, dst++, static_cast<uint16_t>(0xFF00 | phase << 4 | phase));
        }
        _stm_write += 1;
        break;
      default:
        spdlog::error("Not supported GainSTM mode");
    }

    if ((_stm_write & cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) == 0)
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET,
                 static_cast<uint16_t>((_stm_write & ~cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_MASK) >> cpu::GAIN_STM_LEGACY_BUF_SEGMENT_SIZE_WIDTH));

    if (header->cpu_flag.contains(driver::CPUControlFlags::STMEnd)) {
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_CYCLE, static_cast<uint16_t>((std::max)(_stm_cycle, 1u) - 1u));
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_CTL_REG, header->fpga_flag.value() | cpu::CTL_REG_OP_MODE);
    }
  }

  void write_gain_stm(const driver::GlobalHeader* header, const driver::Body* body) {
    if (body == nullptr) return;
    if (header->cpu_flag.contains(driver::CPUControlFlags::STMBegin)) {
      _stm_write = 0;
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET, 0);
      const auto freq_div =
          static_cast<uint32_t>(body->gain_stm_initial().data()[1]) << 16 | static_cast<uint32_t>(body->gain_stm_initial().data()[0]);
      bram_cpy(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_FREQ_DIV_0, reinterpret_cast<const uint16_t*>(&freq_div), 2);
      _gain_stm_mode = body->gain_stm_initial().data()[2];
      _stm_cycle = body->gain_stm_initial().data()[3];

      const auto start_idx = body->gain_stm_initial().data()[4];
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_START_IDX, start_idx);
      const auto finish_idx = body->gain_stm_initial().data()[5];
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_FINISH_IDX, finish_idx);

      return;
    }

    auto* src = body->gain_stm_subsequent().data();
    auto dst = static_cast<uint16_t>((_stm_write & cpu::GAIN_STM_BUF_SEGMENT_SIZE_MASK) << 9);

    switch (_gain_stm_mode) {
      case cpu::GAIN_STM_MODE_PHASE_DUTY_FULL:
        if (header->cpu_flag.contains(driver::CPUControlFlags::IsDuty)) {
          dst += 1;
          _stm_write += 1;
        }
        for (size_t i = 0; i < _num_transducers; i++, dst += 2) bram_write(cpu::BRAM_SELECT_STM, dst, *src++);
        break;
      case cpu::GAIN_STM_MODE_PHASE_FULL:
        if (!header->cpu_flag.contains(driver::CPUControlFlags::IsDuty)) {
          for (size_t i = 0; i < _num_transducers; i++) {
            bram_write(cpu::BRAM_SELECT_STM, dst++, *src++);
            bram_write(cpu::BRAM_SELECT_STM, dst++, _cycles[i] >> 1);
          }
          _stm_write += 1;
        }
        break;
      case cpu::GAIN_STM_MODE_PHASE_HALF:
        spdlog::error("Phase half mode is not supported in Normal GainSTM");
        return;
      default:
        spdlog::error("Not supported GainSTM mode");
        return;
    }

    if ((_stm_write & cpu::GAIN_STM_BUF_SEGMENT_SIZE_MASK) == 0)
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_ADDR_OFFSET,
                 static_cast<uint16_t>((_stm_write & ~cpu::GAIN_STM_BUF_SEGMENT_SIZE_MASK) >> cpu::GAIN_STM_BUF_SEGMENT_SIZE_WIDTH));

    if (header->cpu_flag.contains(driver::CPUControlFlags::STMEnd)) {
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_STM_CYCLE, static_cast<uint16_t>((std::max)(_stm_cycle, 1u) - 1u));
      bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_CTL_REG, header->fpga_flag.value() | cpu::CTL_REG_OP_MODE);
    }
  }

  static uint16_t get_cpu_version() { return cpu::CPU_VERSION; }

  [[nodiscard]] uint16_t get_fpga_version() const { return bram_read(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_VERSION_NUM); }

  [[nodiscard]] uint16_t read_fpga_info() const { return bram_read(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_FPGA_INFO); }

  void clear() {
    constexpr uint32_t freq_div = 40960;

    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_CTL_REG, 0x0000);

    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SILENT_STEP, 10);
    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_SILENT_CYCLE, 4096);

    _stm_write = 0;

    _mod_cycle = 2;
    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_CYCLE, static_cast<uint16_t>((std::max)(_mod_cycle, 1u) - 1u));
    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_FREQ_DIV_0, static_cast<uint16_t>(freq_div & 0xFFFF));
    bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_MOD_FREQ_DIV_1, static_cast<uint16_t>(freq_div >> 16 & 0xFFFF));
    bram_write(cpu::BRAM_SELECT_MOD, 0, 0x0000);

    bram_set(cpu::BRAM_SELECT_NORMAL, 0, 0x0000, _num_transducers << 1);
  }

  void ecat_recv(const driver::GlobalHeader* header, const driver::Body* body) {
    if (_msg_id == header->msg_id) return;

    _msg_id = header->msg_id;
    _fpga_flags = header->fpga_flag;
    _cpu_flags = header->cpu_flag;
    if (header->fpga_flag.contains(driver::FPGAControlFlags::ReadsFPGAInfo)) _ack = static_cast<uint8_t>(read_fpga_info());

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
        _ack = get_fpga_version() >> 8;
        break;
      default:
        if (_msg_id > driver::MSG_END) return;

        const auto ctl_reg = header->fpga_flag;
        bram_write(cpu::BRAM_SELECT_CONTROLLER, cpu::BRAM_ADDR_CTL_REG, ctl_reg.value());

        if (header->cpu_flag.contains(driver::CPUControlFlags::Mod))
          write_mod(header);
        else if (header->cpu_flag.contains(driver::CPUControlFlags::ConfigSilencer))
          config_silencer(header);
        else if (header->cpu_flag.contains(driver::CPUControlFlags::ConfigSync)) {
          synchronize(body);
          return;
        }

        if (!header->cpu_flag.contains(driver::CPUControlFlags::WriteBody)) return;

        if (header->cpu_flag.contains(driver::CPUControlFlags::ModDelay)) {
          set_mod_delay(body);
          return;
        }

        if (!ctl_reg.contains(driver::FPGAControlFlags::STMMode)) {
          write_normal_op(header, body);
          return;
        }

        if (!ctl_reg.contains(driver::FPGAControlFlags::STMGainMode))
          write_focus_stm(header, body);
        else if (header->fpga_flag.contains(driver::FPGAControlFlags::LegacyMode))
          write_gain_stm_legacy(header, body);
        else
          write_gain_stm(header, body);

        break;
    }
  }

  size_t _id;
  size_t _num_transducers;
  uint8_t _msg_id;
  uint8_t _ack;
  uint32_t _mod_cycle;
  uint32_t _stm_write;
  uint32_t _stm_cycle;
  FPGA _fpga;
  uint16_t _gain_stm_mode;
  std::vector<uint16_t> _cycles{};

  driver::FPGAControlFlags _fpga_flags;
  driver::CPUControlFlags _cpu_flags;
};

}  // namespace autd3::extra

#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
