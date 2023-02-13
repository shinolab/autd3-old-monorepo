// File: master.hpp
// Project: emem
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>
#include <thread>

#include "ethercat/slave.hpp"
#include "ethercat/status.hpp"
#include "ethercat/time_utils.hpp"
#include "ethercat_driver.hpp"

namespace autd3::link {

using ethercat::EcState;

constexpr uint64_t SYNC_DELAY = 100000000;

template <typename I>
class Master {
 public:
  explicit Master(I interf) : _ethercat_driver(interf) {
    _slaves.fill(ethercat::Slave{});
    _io_segment.fill(0);
  }

  void close() { _ethercat_driver.close(); }

  [[nodiscard]] uint16_t expected_wkc() const noexcept { return _output_wkc * 2 + _input_wkc; }

  void send_process_data() {
    {
      auto current_segment = _input_segment;
      auto* data = _p_input;
      auto length = static_cast<int32_t>(_input_bytes);
      auto log_addr = _output_bytes;
      auto first = true;
      for (;;) {
        const auto sub_len = current_segment == _input_segment ? _io_segment[current_segment] - _input_offset : _io_segment[current_segment];
        current_segment++;
        _ethercat_driver.process_data_segment_trans_lrd(data, log_addr, sub_len, first, _slaves[1].config_addr, _dc_time);
        first = false;
        length -= static_cast<int32_t>(sub_len);
        log_addr += sub_len;
        data += sub_len;
        if (length <= 0 || current_segment >= _num_segments) break;
      }
    }

    {
      auto current_segment = 0;
      auto* data = _p_output;
      auto length = static_cast<int32_t>(_output_bytes);
      auto log_addr = 0;
      for (;;) {
        const auto sub_len = (std::min)(_io_segment[current_segment], static_cast<uint32_t>(length));
        current_segment++;
        _ethercat_driver.process_data_segment_trans_lwr(data, log_addr, sub_len);
        length -= static_cast<int32_t>(sub_len);
        log_addr += sub_len;
        data += sub_len;
        if (length <= 0 || current_segment >= _num_segments) break;
      }
    }
  }

  EmemResult receive_process_data(Duration timeout, uint16_t* wkc) { return _ethercat_driver.receive_process_data(timeout, &_dc_time, wkc); }

  EmemResult set_dc_sync0(const size_t slave_idx, const bool act, const uint32_t cyc_time, const uint32_t cyc_shift) {
    const auto slave_h = _slaves[slave_idx].config_addr;

    uint16_t unused{};

    uint8_t ra{0x00};
    EMEM_CHECK_RESULT(_ethercat_driver.fpwr(ethercat::NodeAddress{slave_h, ethercat::registers::DCSYNCACT}, &ra, 1, EC_TIMEOUT, &unused));

    if (act) ra = 1 + 2;

    uint8_t h{0x00};
    EMEM_CHECK_RESULT(_ethercat_driver.fpwr(ethercat::NodeAddress{slave_h, ethercat::registers::DCCUC}, &h, 1, EC_TIMEOUT, &unused));

    uint8_t t1_buf[sizeof(int64_t)];
    EMEM_CHECK_RESULT(
        _ethercat_driver.fprd(ethercat::NodeAddress{slave_h, ethercat::registers::DCSYSTIME}, t1_buf, sizeof(int64_t), EC_TIMEOUT, &unused));
    const auto t1 = i64_from_le_bytes(t1_buf);

    const auto t = cyc_time > 0 ? ((t1 + SYNC_DELAY) / cyc_time + 1) * cyc_time + cyc_shift : t1 + SYNC_DELAY + cyc_shift;
    const auto t_le = to_le_bytes(static_cast<int64_t>(t));
    EMEM_CHECK_RESULT(_ethercat_driver.fpwr(ethercat::NodeAddress{slave_h, ethercat::registers::DCSTART0}, reinterpret_cast<const uint8_t*>(&t_le),
                                            sizeof(uint64_t), EC_TIMEOUT, &unused));

    const auto cyc_time_le = to_le_bytes(cyc_time);
    EMEM_CHECK_RESULT(_ethercat_driver.fpwr(ethercat::NodeAddress{slave_h, ethercat::registers::DCCYCLE0},
                                            reinterpret_cast<const uint8_t*>(&cyc_time_le), sizeof(uint32_t), EC_TIMEOUT, &unused));

    EMEM_CHECK_RESULT(
        _ethercat_driver.fpwr(ethercat::NodeAddress{slave_h, ethercat::registers::DCSYNCACT}, &ra, sizeof(uint8_t), EC_TIMEOUT, &unused));

    return EmemResult::Ok;
  }

  [[nodiscard]] EmemResult state_check(const size_t slave_idx, const EcState req_state, const Duration timeout, EcState* state) {
    if (slave_idx > _slave_num) {
      *state = EcState{};
      return EmemResult::Ok;
    }

    const auto config_addr = _slaves[slave_idx].config_addr;
    const auto expire_time = std::chrono::high_resolution_clock::now() + timeout;

    uint16_t state_v;
    for (;;) {
      EcState ret{};
      if (slave_idx == 0) {
        uint16_t tmp{};
        uint16_t unused{};
        EMEM_CHECK_RESULT(_ethercat_driver.brd_word(ethercat::BroadcastAddress{0, ethercat::registers::ALSTAT}, EC_TIMEOUT, &unused, &tmp));
        ret = EcState::from(tmp);
      } else {
        uint8_t slave_states[sizeof(ethercat::EcAlStatus)];
        uint16_t unused{};
        EMEM_CHECK_RESULT(_ethercat_driver.fprd(ethercat::NodeAddress{config_addr, ethercat::registers::ALSTAT}, slave_states,
                                                sizeof(ethercat::EcAlStatus), EC_TIMEOUT, &unused));

        const auto* p_slave_states = reinterpret_cast<const ethercat::EcAlStatus*>(slave_states);
        _slaves[slave_idx].al_status_code = u16_from_le(p_slave_states->al_status_code);
        ret = EcState::from(u16_from_le(p_slave_states->al_status));
      }

      _slaves[slave_idx].state = ret;
      state_v = ret.value() & 0x000F;
      if (state_v == req_state.value() || std::chrono::high_resolution_clock::now() > expire_time) break;
      std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }

    *state = EcState::from(state_v);
    return EmemResult::Ok;
  }

  [[nodiscard]] EmemResult read_state(EcState* state) {
    uint8_t rval_le[sizeof(uint16_t)];
    uint16_t wkc{};
    EMEM_CHECK_RESULT(_ethercat_driver.brd(ethercat::BroadcastAddress{0, ethercat::registers::ALSTAT}, rval_le, sizeof(uint16_t), EC_TIMEOUT, &wkc));

    const auto all_slaves_present = wkc >= _slave_num;

    auto rval = u16_from_le_bytes(rval_le[0], rval_le[1]);
    const auto bitwise_state = EcState::from(rval & 0x000F);

    bool no_error;
    if ((rval & EcState::Error) == 0) {
      _slaves[0].al_status_code = 0;
      no_error = true;
    } else {
      no_error = false;
    }

    bool all_slaves_same_state{false};
    switch (bitwise_state.value()) {
      case EcState::Init:
      case EcState::PreOp:
      case EcState::SafeOp:
      case EcState::Operational:
        _slaves[0].state = bitwise_state;
        all_slaves_same_state = true;
        break;
      case EcState::None:
      case EcState::Ack:
        break;
    }

    if (no_error && all_slaves_same_state && all_slaves_present) {
      for (size_t i = 0; i < _slave_num; i++) {
        _slaves[i].al_status_code = 0x0000;
        _slaves[i].state = bitwise_state;
      }
      *state = bitwise_state;
      return EmemResult::Ok;
    }

    _slaves[0].al_status_code = 0x0000;
    uint16_t lowest = 0x00FF;
    uint16_t fslave = 1;

    EcAlStatusBytes sl[MAX_FPRD_MULTI];
    std::array<uint16_t, MAX_FPRD_MULTI> sl_ca{};
    uint16_t lslave = _slave_num;

    for (;;) {
      if (lslave >= MAX_FPRD_MULTI + fslave) lslave = fslave + MAX_FPRD_MULTI - 1;

      for (auto slave = fslave; slave <= lslave; slave++) {
        EcAlStatusBytes zero;
        zero.fill(0);
        const auto config_addr = _slaves[slave].config_addr;
        sl_ca[slave - fslave] = config_addr;
        sl[slave - fslave] = zero;
      }

      EMEM_CHECK_RESULT(_ethercat_driver.fprd_multi(lslave - fslave + 1, sl_ca.data(), sl, EC_TIMEOUT3, &wkc));

      for (auto slave = fslave; slave <= lslave; slave++) {
        const auto* p_al_status = reinterpret_cast<const ethercat::EcAlStatus*>(sl[slave - fslave].data());
        _slaves[slave].al_status_code = p_al_status->al_status_code;
        rval = p_al_status->al_status;
        if ((rval & 0x000F) < lowest) lowest = rval & 0x000F;

        _slaves[slave].state = EcState::from(rval);
        _slaves[0].al_status_code |= _slaves[slave].al_status_code;
      }
      fslave = lslave + 1;
      if (lslave >= _slave_num) break;
    }

    _slaves[0].state = EcState::from(lowest);
    return EmemResult::Ok;
  }

  EmemResult write_state(const size_t slave_idx, const EcState state) {
    _slaves[0].state = state;
    uint16_t unused{};
    return slave_idx == 0 ? _ethercat_driver.bwr_word(ethercat::BroadcastAddress{0, ethercat::registers::ALCTL}, state.value(), EC_TIMEOUT3, &unused)
                          : _ethercat_driver.fpwr_word(ethercat::NodeAddress{_slaves[slave_idx].config_addr, ethercat::registers::ALCTL},
                                                       state.value(), EC_TIMEOUT3, &unused);
  }

  EmemResult initialize(uint16_t* wkc) {
    EMEM_CHECK_RESULT(detect_slaves(wkc));

    _slave_num = *wkc;

    EMEM_CHECK_RESULT(reset_slaves());

    for (uint16_t slave = 1; slave <= _slave_num; slave++) {
      uint16_t unused{};

      const auto adp = static_cast<uint16_t>(1 - slave);

      auto addr = ethercat::PositionAddr{adp, ethercat::registers::STADR};
      const auto w = to_le_bytes(static_cast<uint16_t>(slave + EC_NODE_OFFSET));
      EMEM_CHECK_RESULT(_ethercat_driver.apwr(addr, reinterpret_cast<const uint8_t*>(&w), sizeof(uint16_t), EC_TIMEOUT3, &unused));

      const uint16_t b = slave == 1 ? 1 : 0;
      addr.offset = ethercat::registers::DLCTL;
      EMEM_CHECK_RESULT(_ethercat_driver.apwr_word(addr, b, EC_TIMEOUT3, &unused));

      addr.offset = ethercat::registers::STADR;
      uint16_t config_addr{};
      EMEM_CHECK_RESULT(_ethercat_driver.aprd_word(addr, EC_TIMEOUT3, &unused, &config_addr));
      _slaves[slave].config_addr = config_addr;

      const auto node_addr = ethercat::NodeAddress{config_addr, ethercat::registers::ALIAS};
      uint16_t alias_addr{};
      EMEM_CHECK_RESULT(_ethercat_driver.fprd_word(node_addr, EC_TIMEOUT3, &unused, &alias_addr));
      _slaves[slave].alias_addr = alias_addr;
    }

    for (uint16_t slave = 1; slave <= _slave_num; slave++) {
      _slaves[slave].eep_man = 0x08A9;
      _slaves[slave].eep_id = 0x0001;
      _slaves[slave].eep_rev = 0x0001;
      _slaves[slave].mbx_wr_offset = 0x1000;
      _slaves[slave].mbx_wr_size = 0x0080;
      _slaves[slave].mbx_rd_offset = 0x1400;
      _slaves[slave].mbx_rd_size = 0x0080;
    }

    for (uint16_t slave = 1; slave <= _slave_num; slave++) {
      uint16_t unused{};

      const auto config_addr = _slaves[slave].config_addr;
      _slaves[slave].has_dc = true;

      auto addr = ethercat::NodeAddress{config_addr, ethercat::registers::DLSTAT};
      uint16_t topology{};
      EMEM_CHECK_RESULT(_ethercat_driver.fprd_word(addr, EC_TIMEOUT3, &unused, &topology));

      uint8_t h = 0;
      uint8_t b = 0;
      if ((topology & 0x0300) == 0x0200) {
        h++;
        b |= 0x01;
      }
      if ((topology & 0x0C00) == 0x0800) {
        h++;
        b |= 0x02;
      }
      if ((topology & 0x3000) == 0x2000) {
        h++;
        b |= 0x04;
      }
      if ((topology & 0xC000) == 0x8000) {
        h++;
        b |= 0x08;
      }
      _slaves[slave].ptype = 0x0F;
      _slaves[slave].topology = h;
      _slaves[slave].active_ports = b;

      _slaves[slave].parent = slave - 1;

      EcState unused_state{};
      EMEM_CHECK_RESULT(state_check(slave, EcState::Init, EC_TIMEOUT_STATE, &unused_state));

      initialize_sii(slave);

      addr = ethercat::NodeAddress{config_addr, ethercat::registers::SM0};
      uint8_t d[sizeof(ethercat::SM) * 2];

      std::memcpy(&d[0], _slaves[slave].sm.data(), sizeof(ethercat::SM));
      std::memcpy(&d[sizeof(ethercat::SM)], &_slaves[slave].sm[1], sizeof(ethercat::SM));
      EMEM_CHECK_RESULT(_ethercat_driver.fpwr(addr, d, sizeof(ethercat::SM) * 2, EC_TIMEOUT3, &unused));

      EMEM_CHECK_RESULT(set_eeprom_to_pdi(slave, &unused));

      addr = ethercat::NodeAddress{config_addr, ethercat::registers::ALCTL};
      constexpr uint16_t w = EcState::PreOp | EcState::Ack;
      EMEM_CHECK_RESULT(_ethercat_driver.fpwr_word(addr, w, EC_TIMEOUT3, &unused));
    }

    return EmemResult::Ok;
  }

  EmemResult config(uint8_t* p_map) {
    for (size_t slave = 1; slave <= _slave_num; slave++) EMEM_CHECK_RESULT(map_coe_soe(slave));
    for (size_t slave = 1; slave <= _slave_num; slave++) EMEM_CHECK_RESULT(map_sm(slave));

    uint32_t log_addr = 0;
    uint32_t o_log_addr = log_addr;
    uint8_t bit_pos = 0;
    uint16_t current_segment = 0;
    uint32_t segment_size = 0;

    for (size_t slave = 1; slave <= _slave_num; slave++) {
      EMEM_CHECK_RESULT(map_output(slave, p_map, log_addr, bit_pos));

      const auto diff = log_addr - o_log_addr;
      o_log_addr = log_addr;
      if (segment_size + diff > EC_MAX_LRW_DATA - EC_FIRST_DC_DATAGRAM) {
        _io_segment[current_segment] = segment_size;
        if (current_segment < MAX_IO_SEGMENT - 1) {
          current_segment++;
          segment_size = diff;
        }
      } else {
        segment_size += diff;
      }
    }

    if (bit_pos > 0) {
      log_addr++;
      o_log_addr = log_addr;
      bit_pos = 0;
      if (segment_size + 1 > EC_MAX_LRW_DATA - EC_FIRST_DC_DATAGRAM) {
        _io_segment[current_segment] = segment_size;
        if (current_segment < MAX_IO_SEGMENT - 1) {
          current_segment++;
          segment_size = 1;
        }
      } else {
        segment_size++;
      }
    }

    _p_output = p_map;
    _output_bytes = log_addr;
    _num_segments = current_segment + 1;
    _input_segment = current_segment;
    _input_offset = segment_size;
    _slaves[0].p_output = p_map;
    _slaves[0].out_bytes = log_addr;

    uint16_t unused{};
    for (size_t slave = 1; slave <= _slave_num; slave++) {
      EMEM_CHECK_RESULT(map_input(slave, p_map, log_addr, bit_pos));

      const auto diff = log_addr - o_log_addr;
      o_log_addr = log_addr;
      if (segment_size + diff > EC_MAX_LRW_DATA - EC_FIRST_DC_DATAGRAM) {
        _io_segment[current_segment] = segment_size;
        if (current_segment < MAX_IO_SEGMENT - 1) {
          current_segment++;
          segment_size = diff;
        }
      } else {
        segment_size += diff;
      }

      EMEM_CHECK_RESULT(set_eeprom_to_pdi(slave, &unused));

      const auto config_addr = _slaves[slave].config_addr;
      const auto addr = ethercat::NodeAddress{config_addr, ethercat::registers::ALCTL};
      EMEM_CHECK_RESULT(_ethercat_driver.fpwr_word(addr, EcState::SafeOp, EC_TIMEOUT3, &unused));
    }
    if (bit_pos > 0) {
      log_addr++;
      if (segment_size + 1 > EC_MAX_LRW_DATA - EC_FIRST_DC_DATAGRAM) {
        _io_segment[current_segment] = segment_size;
        if (current_segment < MAX_IO_SEGMENT - 1) {
          current_segment += 1;
          segment_size = 1;
        }
      } else {
        segment_size += 1;
      }
    }

    _io_segment[current_segment] = segment_size;
    _num_segments = current_segment + 1;
    _p_input = p_map + _output_bytes;
    _input_bytes = log_addr - _output_bytes;
    _slaves[0].p_input = _p_input;
    _slaves[0].in_bytes = log_addr - _slaves[0].out_bytes;

    return EmemResult::Ok;
  }

  EmemResult reconfig_slave(const uint16_t slave, const Duration timeout, EcState* state) {
    const uint16_t config_addr = _slaves[slave].config_addr;
    auto addr = ethercat::NodeAddress{config_addr, ethercat::registers::ALCTL};
    uint16_t v{};
    EMEM_CHECK_RESULT(_ethercat_driver.fpwr_word(addr, EcState::Init, timeout, &v));

    if (v == 0) {
      *state = EcState{};
      return EmemResult::Ok;
    }

    EMEM_CHECK_RESULT(set_eeprom_to_pdi(slave, &v));

    EMEM_CHECK_RESULT(state_check(slave, EcState::Init, EC_TIMEOUT_STATE, state));

    if (*state == EcState::Init) {
      for (uint16_t n_sm = 0; n_sm < ethercat::MAX_SM; n_sm++) {
        if (_slaves[slave].sm[n_sm].start_addr > 0) {
          addr.offset = ethercat::registers::SM0 + sizeof(ethercat::SM) * n_sm;
          _ethercat_driver.fpwr(addr, reinterpret_cast<const uint8_t*>(&_slaves[slave].sm[n_sm]), sizeof(ethercat::SM), timeout, &v);
        }
      }

      addr.offset = ethercat::registers::ALCTL;
      EMEM_CHECK_RESULT(_ethercat_driver.fpwr_word(addr, EcState::PreOp, timeout, &v));
      EMEM_CHECK_RESULT(state_check(slave, EcState::PreOp, EC_TIMEOUT_STATE, state));

      if (*state == EcState::PreOp) {
        if (_slaves[slave].po_to_so_config) _slaves[slave].po_to_so_config();

        EMEM_CHECK_RESULT(_ethercat_driver.fpwr_word(addr, EcState::SafeOp, timeout, &v));
        EMEM_CHECK_RESULT(state_check(slave, EcState::PreOp, EC_TIMEOUT_STATE, state));

        for (uint16_t fmmu_c = 0; fmmu_c < 2; fmmu_c++) {
          addr.offset = ethercat::registers::FMMU0 + sizeof(ethercat::FMMU) * fmmu_c;
          EMEM_CHECK_RESULT(_ethercat_driver.fpwr(addr, reinterpret_cast<const uint8_t*>(&_slaves[fmmu_c]), sizeof(ethercat::FMMU), timeout, &v));
        }
      }
    }

    return EmemResult::Ok;
  }

  EmemResult config_dc() {
    constexpr auto addr = ethercat::BroadcastAddress{0, ethercat::registers::DCTIME0};
    uint8_t ht[sizeof(int32_t)] = {0};
    uint16_t wkc{};
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, ht, sizeof(int32_t), EC_TIMEOUT3, &wkc));

    const auto master_time_ns = ethercat::get_master_ec_time();
    for (size_t slave = 1; slave <= _slave_num; slave++) {
      const auto slave_h = _slaves[slave].config_addr;
      auto node_addr = ethercat::NodeAddress{slave_h, 0};

      node_addr.offset = ethercat::registers::DCTIME0;
      EMEM_CHECK_RESULT(_ethercat_driver.fprd(node_addr, ht, sizeof(int32_t), EC_TIMEOUT, &wkc));

      _slaves[slave].dc_rt_a = i32_from_le_bytes(ht);

      uint8_t htr[sizeof(int64_t)];
      node_addr.offset = ethercat::registers::DCSOF;
      EMEM_CHECK_RESULT(_ethercat_driver.fprd(node_addr, htr, sizeof(int64_t), EC_TIMEOUT, &wkc));

      uint64_t htr_u64 = to_le_bytes(master_time_ns - static_cast<uint64_t>(i64_from_le_bytes(htr)));
      node_addr.offset = ethercat::registers::DCSYSOFFSET;
      EMEM_CHECK_RESULT(_ethercat_driver.fpwr(node_addr, reinterpret_cast<const uint8_t*>(&htr_u64), sizeof(uint64_t), EC_TIMEOUT, &wkc));

      node_addr.offset = ethercat::registers::DCTIME1;
      EMEM_CHECK_RESULT(_ethercat_driver.fprd(node_addr, ht, sizeof(int32_t), EC_TIMEOUT, &wkc));
      _slaves[slave].dc_rt_b = i32_from_le_bytes(ht);

      const auto child = slave;

      if (const auto parent = _slaves[slave].parent; parent > 0) {
        const auto dt3 = port_time(parent, 1) - port_time(parent, 0);
        auto dt1 = _slaves[slave].topology > 1 ? port_time(slave, prev_port(slave, 0)) - port_time(slave, 0) : 0;

        if (dt1 > dt3) dt1 = -dt1;

        auto dt2 = child - parent > 1 ? port_time(parent, prev_port(parent, 1)) - port_time(parent, 0) : 0;
        if (dt2 < 0) dt2 = -dt2;

        _slaves[slave].propagation_delay = (dt3 - dt1) / 2 + dt2 + _slaves[parent].propagation_delay;

        auto p = to_le_bytes(_slaves[slave].propagation_delay);
        node_addr.offset = ethercat::registers::DCSYSDELAY;
        EMEM_CHECK_RESULT(_ethercat_driver.fpwr(node_addr, reinterpret_cast<const uint8_t*>(&p), sizeof(int32_t), EC_TIMEOUT, &wkc));
      }
    }

    return EmemResult::Ok;
  }

  [[nodiscard]] size_t num_slaves() const noexcept { return _slave_num; }
  [[nodiscard]] int64_t dc_time() const noexcept { return _dc_time; }

  [[nodiscard]] const ethercat::Slave& operator[](const size_t i) const { return _slaves[i]; }
  [[nodiscard]] ethercat::Slave& operator[](const size_t i) { return _slaves[i]; }

 private:
  EmemResult detect_slaves(uint16_t* wkc) {
    auto addr = ethercat::BroadcastAddress{0x0000, ethercat::registers::DLALIAS};
    uint8_t b = 0;
    uint16_t unused{};
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, &b, sizeof(uint8_t), EC_TIMEOUT3, &unused));

    addr = ethercat::BroadcastAddress{0x0000, ethercat::registers::ALCTL};
    b = EcState::Init | EcState::Ack;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, &b, sizeof(uint8_t), EC_TIMEOUT3, &unused));
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, &b, sizeof(uint8_t), EC_TIMEOUT3, &unused));

    addr = ethercat::BroadcastAddress{0x0000, ethercat::registers::TYPE};
    uint8_t w = 0x00;
    EMEM_CHECK_RESULT(_ethercat_driver.brd(addr, &w, sizeof(uint8_t), EC_TIMEOUT_SAFE, wkc));
    if (*wkc > EC_SLAVE_MAX) return EmemResult::TooManySlaves;

    return EmemResult::Ok;
  }

  EmemResult reset_slaves() {
    uint8_t zero[64] = {};
    uint16_t wkc{};

    uint8_t b = 0x00;
    auto addr = ethercat::BroadcastAddress{0x0000, 0x0000};

    addr.offset = ethercat::registers::DLPORT;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, &b, sizeof(uint8_t), EC_TIMEOUT3, &wkc));

    uint16_t w = to_le_bytes(uint16_t{0x0004});
    addr.offset = ethercat::registers::IRQMASK;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, reinterpret_cast<const uint8_t*>(&w), sizeof(uint16_t), EC_TIMEOUT3, &wkc));

    addr.offset = ethercat::registers::RXERR;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, zero, 8, EC_TIMEOUT3, &wkc));

    addr.offset = ethercat::registers::FMMU0;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, zero, 16 * 3, EC_TIMEOUT3, &wkc));

    addr.offset = ethercat::registers::SM0;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, zero, 8 * 4, EC_TIMEOUT3, &wkc));

    addr.offset = ethercat::registers::DCSYNCACT;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, &b, sizeof(uint8_t), EC_TIMEOUT3, &wkc));

    addr.offset = ethercat::registers::DCSYSTIME;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, zero, 4, EC_TIMEOUT3, &wkc));

    w = to_le_bytes(uint16_t{0x1000});
    addr.offset = ethercat::registers::DCSPEEDCNT;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, reinterpret_cast<const uint8_t*>(&w), sizeof(uint16_t), EC_TIMEOUT3, &wkc));

    w = to_le_bytes(uint16_t{0x0C00});
    addr.offset = ethercat::registers::DCTIMEFILT;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, reinterpret_cast<const uint8_t*>(&w), sizeof(uint16_t), EC_TIMEOUT3, &wkc));

    addr.offset = ethercat::registers::DLALIAS;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, &b, sizeof(uint8_t), EC_TIMEOUT3, &wkc));

    b = EcState::Init | EcState::Ack;
    addr.offset = ethercat::registers::ALCTL;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, &b, sizeof(uint8_t), EC_TIMEOUT3, &wkc));

    b = 0x02;
    addr.offset = ethercat::registers::EEPCFG;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, &b, sizeof(uint8_t), EC_TIMEOUT3, &wkc));

    b = 0x00;
    addr.offset = ethercat::registers::EEPCFG;
    EMEM_CHECK_RESULT(_ethercat_driver.bwr(addr, &b, sizeof(uint8_t), EC_TIMEOUT3, &wkc));

    return EmemResult::Ok;
  }

  EmemResult set_eeprom_to_pdi(const size_t slave_idx, uint16_t* wkc) {
    *wkc = 1;
    if (!_slaves[slave_idx].eep_pdi) {
      const auto config_addr = _slaves[slave_idx].config_addr;
      constexpr uint8_t eepctl = 0x01;

      *wkc = 0;
      const auto addr = ethercat::NodeAddress{config_addr, ethercat::registers::EEPCFG};
      for (size_t i = 0; i < EC_DEFAULT_RETRIES; i++) {
        EMEM_CHECK_RESULT(_ethercat_driver.fpwr(addr, &eepctl, sizeof(uint8_t), EC_TIMEOUT, wkc));
        if (*wkc > 0) break;
      }
      _slaves[slave_idx].eep_pdi = true;
    }
    return EmemResult::Ok;
  }

  void initialize_sii(const size_t slave_idx) {
    _slaves[slave_idx].sm_type[0] = ethercat::SMType::MbxWr;
    _slaves[slave_idx].sm_type[1] = ethercat::SMType::MbxRd;
    _slaves[slave_idx].sm_type[2] = ethercat::SMType::Output;
    _slaves[slave_idx].sm_type[3] = ethercat::SMType::Input;
    _slaves[slave_idx].sm[0].start_addr = _slaves[slave_idx].mbx_wr_offset;
    _slaves[slave_idx].sm[0].sm_length = _slaves[slave_idx].mbx_wr_size;
    _slaves[slave_idx].sm[0].sm_flags = to_le_bytes(ethercat::EC_DEFAULTMBXSM0);
    _slaves[slave_idx].sm[1].start_addr = _slaves[slave_idx].mbx_rd_offset;
    _slaves[slave_idx].sm[1].sm_length = _slaves[slave_idx].mbx_rd_size;
    _slaves[slave_idx].sm[1].sm_flags = to_le_bytes(ethercat::EC_DEFAULTMBXSM1);
    _slaves[slave_idx].sm[2].start_addr = 0x1800;
    _slaves[slave_idx].sm[2].sm_length = 0x0272;
    _slaves[slave_idx].sm[2].sm_flags = to_le_bytes(0x00010064u);
    _slaves[slave_idx].sm[3].start_addr = 0x1F80;
    _slaves[slave_idx].sm[3].sm_length = 0x0002;
    _slaves[slave_idx].sm[3].sm_flags = to_le_bytes(0x00010020u);
    _slaves[slave_idx].mbx_proto = 0x000c;
    _slaves[slave_idx].fmmu0func = 1;
    _slaves[slave_idx].fmmu1func = 2;
    _slaves[slave_idx].fmmu2func = 3;
    _slaves[slave_idx].fmmu3func = 0;
  }

  EmemResult map_coe_soe(const size_t slave_idx) {
    EcState unused{};
    EMEM_CHECK_RESULT(state_check(slave_idx, EcState::PreOp, EC_TIMEOUT_STATE, &unused));

    if (_slaves[slave_idx].po_to_so_config) _slaves[slave_idx].po_to_so_config();

    return EmemResult::Ok;
  }

  EmemResult map_sm(const size_t slave_idx) {
    _slaves[slave_idx].out_bits = (128 + 498) * 8;
    _slaves[slave_idx].in_bits = 16;

    const auto config_addr = _slaves[slave_idx].config_addr;
    for (size_t n_sm = 2; n_sm < 4; n_sm++) {
      const auto addr = ethercat::NodeAddress{config_addr, static_cast<uint16_t>(ethercat::registers::SM0 + sizeof(ethercat::SM) * n_sm)};
      uint16_t wkc{};
      EMEM_CHECK_RESULT(_ethercat_driver.fpwr_struct(addr, &_slaves[slave_idx].sm[n_sm], EC_TIMEOUT3, &wkc));
    }

    _slaves[slave_idx].out_bytes = (_slaves[slave_idx].out_bits + 7) / 8;
    _slaves[slave_idx].in_bytes = (_slaves[slave_idx].in_bits + 7) / 8;

    return EmemResult::Ok;
  }

  EmemResult map_output(const size_t slave_idx, uint8_t* p_map, uint32_t& log_addr, uint8_t& bit_pos) {
    constexpr auto fmmu_c = 0;
    constexpr auto sm_c = 2;

    _slaves[slave_idx].fmmu[fmmu_c].phys_start = _slaves[slave_idx].sm[sm_c].start_addr;
    const auto sm_len = to_le_bytes(_slaves[slave_idx].sm[sm_c].sm_length);
    const auto byte_count = sm_len;

    if (bit_pos > 0) {
      log_addr++;
      bit_pos = 0;
    }

    _slaves[slave_idx].fmmu[fmmu_c].log_start = to_le_bytes(log_addr);
    _slaves[slave_idx].fmmu[fmmu_c].log_start_bit = bit_pos;
    bit_pos = 7;
    const auto fmmu_size = byte_count;
    log_addr += fmmu_size;

    _slaves[slave_idx].fmmu[fmmu_c].log_length = to_le_bytes(fmmu_size);
    _slaves[slave_idx].fmmu[fmmu_c].log_end_bit = bit_pos;
    bit_pos = 0;

    _slaves[slave_idx].fmmu[fmmu_c].phys_start_bit = 0;
    _slaves[slave_idx].fmmu[fmmu_c].fmmu_type = 2;
    _slaves[slave_idx].fmmu[fmmu_c].fmmu_active = 1;

    const auto config_addr = _slaves[slave_idx].config_addr;
    const auto addr = ethercat::NodeAddress{config_addr, ethercat::registers::FMMU0 + sizeof(ethercat::FMMU) * fmmu_c};
    uint16_t wkc{};
    EMEM_CHECK_RESULT(_ethercat_driver.fpwr_struct(addr, &_slaves[slave_idx].fmmu[fmmu_c], EC_TIMEOUT3, &wkc));

    _slaves[slave_idx].p_output = p_map + _slaves[slave_idx].fmmu[fmmu_c].log_start;
    _slaves[slave_idx].out_start_bit = _slaves[slave_idx].fmmu[fmmu_c].log_start_bit;

    _output_wkc++;

    return EmemResult::Ok;
  }

  EmemResult map_input(const size_t slave_idx, uint8_t* p_map, uint32_t& log_addr, uint8_t& bit_pos) {
    constexpr auto fmmu_c = 1;
    constexpr auto sm_c = 3;

    _slaves[slave_idx].fmmu[fmmu_c].phys_start = _slaves[slave_idx].sm[sm_c].start_addr;
    const auto sm_len = to_le_bytes(_slaves[slave_idx].sm[sm_c].sm_length);
    const auto byte_count = sm_len;

    if (bit_pos > 0) {
      log_addr += 1;
      bit_pos = 0;
    }

    _slaves[slave_idx].fmmu[fmmu_c].log_start = to_le_bytes(log_addr);
    _slaves[slave_idx].fmmu[fmmu_c].log_start_bit = bit_pos;
    bit_pos = 7;
    const auto fmmu_size = byte_count;
    log_addr += fmmu_size;

    _slaves[slave_idx].fmmu[fmmu_c].log_length = to_le_bytes(fmmu_size);
    _slaves[slave_idx].fmmu[fmmu_c].log_end_bit = bit_pos;
    bit_pos = 0;

    _slaves[slave_idx].fmmu[fmmu_c].phys_start_bit = 0;
    _slaves[slave_idx].fmmu[fmmu_c].fmmu_type = 1;
    _slaves[slave_idx].fmmu[fmmu_c].fmmu_active = 1;

    const auto config_addr = _slaves[slave_idx].config_addr;
    const auto addr = ethercat::NodeAddress{config_addr, ethercat::registers::FMMU0 + sizeof(ethercat::FMMU) * fmmu_c};
    uint16_t wkc{};
    EMEM_CHECK_RESULT(_ethercat_driver.fpwr_struct(addr, &_slaves[slave_idx].fmmu[fmmu_c], EC_TIMEOUT3, &wkc));

    _slaves[slave_idx].p_input = p_map + _slaves[slave_idx].fmmu[fmmu_c].log_start;
    _slaves[slave_idx].in_start_bit = _slaves[slave_idx].fmmu[fmmu_c].log_start_bit;

    _input_wkc++;

    return EmemResult::Ok;
  }

  [[nodiscard]] int32_t port_time(const size_t slave, const uint8_t port) const {
    if (port == 0) return _slaves[slave].dc_rt_a;
    if (port == 1) return _slaves[slave].dc_rt_b;
    return 0;
  }

  [[nodiscard]] uint8_t prev_port(const size_t slave, const uint8_t port) const {
    const auto active_port = _slaves[slave].active_ports;
    if (port == 0) return (active_port & 0x02) > 0 ? 1 : port;
    if (port == 1) return (active_port & 0x01) > 0 ? 0 : port;
    throw std::runtime_error("prev_port unreachable");
  }

  EtherCATDriver<I> _ethercat_driver;
  uint16_t _slave_num{0};
  std::array<ethercat::Slave, EC_SLAVE_MAX> _slaves{};
  std::array<uint32_t, MAX_IO_SEGMENT> _io_segment{};
  uint8_t* _p_output{nullptr};
  uint8_t* _p_input{nullptr};
  uint32_t _output_bytes{};
  uint32_t _input_bytes{};
  uint16_t _num_segments{};
  uint16_t _input_segment{};
  uint32_t _input_offset{};
  uint16_t _output_wkc{};
  uint16_t _input_wkc{};
  int64_t _dc_time{};
};
}  // namespace autd3::link
