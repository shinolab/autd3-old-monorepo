// File: defined.hpp
// Project: v2_2
// Created Date: 15/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/common/fpga/defined.hpp"
#include "autd3/driver/driver.hpp"

namespace autd3::driver {

class DriverV2_2 final : public Driver {
 public:
  DriverV2_2() = default;
  ~DriverV2_2() override = default;
  DriverV2_2(const DriverV2_2& v) = default;
  DriverV2_2& operator=(const DriverV2_2& obj) = default;
  DriverV2_2(DriverV2_2&& obj) = default;
  DriverV2_2& operator=(DriverV2_2&& obj) = default;

  [[nodiscard]] uint8_t version_num() const noexcept override;

  void clear(TxDatagram& tx) const noexcept override;

  void null_header(uint8_t msg_id, TxDatagram& tx) const noexcept override;

  void null_body(TxDatagram& tx) const noexcept override;

  void sync(const uint16_t* cycles, TxDatagram& tx) const noexcept override;

  void mod_delay(const uint16_t* delays, TxDatagram& tx) const noexcept override;

  bool modulation(uint8_t msg_id, const std::vector<uint8_t>& mod_data, size_t& sent, uint32_t freq_div, TxDatagram& tx) const override;

  bool config_silencer(uint8_t msg_id, uint16_t cycle, uint16_t step, TxDatagram& tx) const override;

  void normal_legacy_header(TxDatagram& tx) noexcept;

  void normal_legacy_header(TxDatagram& tx) const noexcept override;

  void normal_legacy_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept override;

  void normal_header(TxDatagram& tx) const noexcept override;

  void normal_duty_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept override;

  void normal_phase_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept override;

  void focus_stm_initialer(TxDatagram& tx) const noexcept override;

  [[nodiscard]] size_t focus_stm_send_size(size_t total_size, size_t sent, const std::vector<size_t>& device_map) const noexcept override;

  bool focus_stm_subsequent(const std::vector<std::vector<STMFocus>>& points, size_t& sent, size_t total_size, uint32_t freq_div, double sound_speed,
                            TxDatagram& tx) const override;

  void gain_stm_legacy_header(TxDatagram& tx) const noexcept override;

  bool gain_stm_legacy_body(const std::vector<std::vector<Drive>>& drives, size_t& sent, uint32_t freq_div, GainSTMMode mode,
                            TxDatagram& tx) const override;

  void gain_stm_normal_header(TxDatagram& tx) const noexcept override;

  bool gain_stm_normal_phase(const std::vector<std::vector<Drive>>& drives, size_t sent, uint32_t freq_div, GainSTMMode mode,
                             TxDatagram& tx) const override;

  bool gain_stm_normal_duty(const std::vector<std::vector<Drive>>& drives, size_t sent, uint32_t freq_div, GainSTMMode mode,
                            TxDatagram& tx) const override;

  void force_fan(TxDatagram& tx, bool value) const noexcept override;

  void reads_fpga_info(TxDatagram& tx, bool value) const noexcept override;

  void cpu_version(TxDatagram& tx) const noexcept override;

  void fpga_version(TxDatagram& tx) const noexcept override;

  void fpga_functions(TxDatagram& tx) const noexcept override;
};

}  // namespace autd3::driver
