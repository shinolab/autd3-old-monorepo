// File: driver.hpp
// Project: driver
// Created Date: 15/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/common/cpu/datagram.hpp"

namespace autd3::driver {

class Driver {
 public:
  virtual ~Driver() = default;
  Driver() = default;
  Driver(const Driver& v) noexcept = default;
  Driver& operator=(const Driver& obj) = default;
  Driver(Driver&& obj) = default;
  Driver& operator=(Driver&& obj) = default;

  [[nodiscard]] virtual uint8_t version_num() const = 0;
  virtual void clear(TxDatagram& tx) const = 0;
  virtual void null_header(uint8_t msg_id, TxDatagram& tx) const = 0;
  virtual void null_body(TxDatagram& tx) const = 0;
  virtual void sync(const uint16_t* cycles, TxDatagram& tx) const = 0;
  virtual void mod_delay(const uint16_t* delays, TxDatagram& tx) const = 0;
  [[nodiscard]] virtual bool modulation(uint8_t msg_id, const std::vector<uint8_t>& mod_data, size_t& sent, uint32_t freq_div,
                                        TxDatagram& tx) const = 0;
  [[nodiscard]] virtual bool config_silencer(uint8_t msg_id, uint16_t cycle, uint16_t step, TxDatagram& tx) const = 0;
  virtual void normal_legacy_header(TxDatagram& tx) const = 0;
  virtual void normal_legacy_body(const std::vector<Drive>& drives, TxDatagram& tx) const = 0;
  virtual void normal_header(TxDatagram& tx) const = 0;
  virtual void normal_duty_body(const std::vector<Drive>& drives, TxDatagram& tx) const = 0;
  virtual void normal_phase_body(const std::vector<Drive>& drives, TxDatagram& tx) const = 0;
  virtual void focus_stm_initialer(TxDatagram& tx) const = 0;
  [[nodiscard]] virtual size_t focus_stm_send_size(size_t total_size, size_t sent, const std::vector<size_t>& device_map) const = 0;
  virtual bool focus_stm_subsequent(const std::vector<std::vector<STMFocus>>& points, size_t& sent, size_t total_size, uint32_t freq_div,
                                    double sound_speed, TxDatagram& tx) const = 0;
  virtual void gain_stm_legacy_header(TxDatagram& tx) const = 0;
  [[nodiscard]] virtual bool gain_stm_legacy_body(const std::vector<std::vector<Drive>>& drives, size_t& sent, uint32_t freq_div, GainSTMMode mode,
                                                  TxDatagram& tx) const = 0;
  virtual void gain_stm_normal_header(TxDatagram& tx) const = 0;
  [[nodiscard]] virtual bool gain_stm_normal_phase(const std::vector<std::vector<Drive>>& drives, size_t sent, uint32_t freq_div, GainSTMMode mode,
                                                   TxDatagram& tx) const = 0;
  [[nodiscard]] virtual bool gain_stm_normal_duty(const std::vector<std::vector<Drive>>& drives, size_t sent, uint32_t freq_div, GainSTMMode mode,
                                                  TxDatagram& tx) const = 0;
  virtual void force_fan(TxDatagram& tx, bool value) const = 0;
  virtual void reads_fpga_info(TxDatagram& tx, bool value) const = 0;
  virtual void cpu_version(TxDatagram& tx) const = 0;
  virtual void fpga_version(TxDatagram& tx) const = 0;
  virtual void fpga_functions(TxDatagram& tx) const = 0;
};

}  // namespace autd3::driver
