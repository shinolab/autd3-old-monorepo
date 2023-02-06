// File: slave.hpp
// Project: ethercat
// Created Date: 07/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <array>
#include <cstdint>

#include "fmmu.hpp"
#include "sync_manager.hpp"

namespace autd3::link::ethercat {

constexpr size_t MAX_SM = 8;
constexpr size_t MAX_FMMU = 4;

struct Slave {
  uint16_t state;
  uint16_t al_status_code;
  uint16_t config_addr;
  uint16_t alias_addr;
  bool eep_pdi;
  uint32_t eep_man;
  uint32_t eep_id;
  uint32_t eep_rev;
  uint16_t mbx_wr_offset;
  uint16_t mbx_wr_size;
  uint16_t mbx_rd_offset;
  uint16_t mbx_rd_size;
  uint16_t mbx_proto;
  bool has_dc;
  uint8_t ptype;
  uint8_t topology;
  uint8_t active_ports;
  uint16_t parent;
  std::array<SMType, MAX_SM> sm_type;
  std::array<SM, MAX_SM> sm;
  std::array<FMMU, MAX_FMMU> fmmu;
  uint8_t fmmu0func;
  uint8_t fmmu1func;
  uint8_t fmmu2func;
  uint8_t fmmu3func;
  uint16_t out_bits;
  uint8_t out_start_bit;
  uint8_t in_start_bit;
  uint16_t in_bits;
  uint32_t out_bytes;
  uint32_t in_bytes;
  uint8_t* p_output;
  uint8_t* p_input;
  int32_t dc_rt_a;
  int32_t dc_rt_b;
  int32_t propagation_delay;
};

}  // namespace autd3::link::ethercat
