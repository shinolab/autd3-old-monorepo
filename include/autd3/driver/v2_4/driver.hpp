// File: driver.hpp
// Project: v2_4
// Created Date: 15/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/common/fpga/defined.hpp"
#include "autd3/driver/driver.hpp"
#include "autd3/driver/v2_4/defined.hpp"

namespace autd3::driver {

class DriverV2_4 final : public Driver {
 public:
  ~DriverV2_4() override = default;

  uint8_t version_num() const noexcept override;

  void clear(TxDatagram& tx) const noexcept override;

  void null_header(const uint8_t msg_id, TxDatagram& tx) const noexcept override;

  void null_body(TxDatagram& tx) const noexcept override;

  void sync(const uint16_t* const cycles, TxDatagram& tx) const noexcept override;

  void mod_delay(const uint16_t* const delays, TxDatagram& tx) const noexcept override;

  bool modulation(const uint8_t msg_id, const std::vector<uint8_t>& mod_data, size_t& sent, const uint32_t freq_div, TxDatagram& tx) const override;

  bool config_silencer(const uint8_t msg_id, const uint16_t cycle, const uint16_t step, TxDatagram& tx) const override;

  void normal_legacy_header(TxDatagram& tx) noexcept;

  void normal_legacy_header(TxDatagram& tx) const noexcept override;

  void normal_legacy_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept override;

  void normal_header(TxDatagram& tx) const noexcept override;

  void normal_duty_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept override;

  void normal_phase_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept override;

  void point_stm_header(TxDatagram& tx) const noexcept override;

  size_t point_stm_send_size(const size_t total_size, const size_t sent) const noexcept override;

  bool point_stm_body(const std::vector<std::vector<STMFocus>>& points, size_t& sent, const size_t total_size, const uint32_t freq_div,
                      const double sound_speed, TxDatagram& tx) const override;

  void gain_stm_legacy_header(TxDatagram& tx) const noexcept override;

  bool gain_stm_legacy_body(const std::vector<std::vector<driver::Drive>>& drives, size_t& sent, const uint32_t freq_div, const GainSTMMode mode,
                            TxDatagram& tx) const override;

  void gain_stm_normal_header(TxDatagram& tx) const noexcept override;

  bool gain_stm_normal_phase(const std::vector<std::vector<driver::Drive>>& drives, const size_t sent, const uint32_t freq_div,
                             const GainSTMMode mode, TxDatagram& tx) const override;

  bool gain_stm_normal_duty(const std::vector<std::vector<driver::Drive>>& drives, const size_t sent, const uint32_t freq_div, const GainSTMMode mode,
                            TxDatagram& tx) const override;

  void force_fan(TxDatagram& tx, const bool value) const noexcept override;

  void reads_fpga_info(TxDatagram& tx, const bool value) const noexcept override;

  void cpu_version(TxDatagram& tx) const noexcept override;

  void fpga_version(TxDatagram& tx) const noexcept override;

  void fpga_functions(TxDatagram& tx) const noexcept override;
};

}  // namespace autd3::driver
