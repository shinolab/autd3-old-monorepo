// File: driver.hpp
// Project: v2_6
// Created Date: 15/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/common/fpga/defined.hpp"
#include "autd3/driver/driver.hpp"

namespace autd3::driver {

/**
 * @brief Driver for v2.6 firmware
 */
class DriverV2_6 final : public Driver {
 public:
  DriverV2_6() = default;
  ~DriverV2_6() override = default;
  DriverV2_6(const DriverV2_6& v) = default;
  DriverV2_6& operator=(const DriverV2_6& obj) = default;
  DriverV2_6(DriverV2_6&& obj) = default;
  DriverV2_6& operator=(DriverV2_6&& obj) = default;

  [[nodiscard]] uint8_t version_num() const noexcept override;

  void clear(TxDatagram& tx) const noexcept override;

  void null_header(uint8_t msg_id, TxDatagram& tx) const noexcept override;

  void null_body(TxDatagram& tx) const noexcept override;

  void sync(const std::vector<uint16_t>& cycles, TxDatagram& tx) const noexcept override;

  void mod_delay(const std::vector<uint16_t>& delays, TxDatagram& tx) const noexcept override;

  [[nodiscard]] bool modulation(uint8_t msg_id, const std::vector<uint8_t>& mod_data, size_t& sent, uint32_t freq_div, TxDatagram& tx) const override;

  [[nodiscard]] bool config_silencer(uint8_t msg_id, uint16_t cycle, uint16_t step, TxDatagram& tx) const override;

  void normal_legacy_header(TxDatagram& tx) noexcept;

  void normal_legacy_header(TxDatagram& tx) const noexcept override;

  void normal_legacy_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept override;

  void normal_header(TxDatagram& tx) const noexcept override;

  void normal_duty_body(const std::vector<Drive>& drives, const std::vector<uint16_t>& cycles, TxDatagram& tx) const noexcept override;

  void normal_phase_body(const std::vector<Drive>& drives, const std::vector<uint16_t>& cycles, TxDatagram& tx) const noexcept override;

  void focus_stm_header(TxDatagram& tx) const noexcept override;

  [[nodiscard]] size_t focus_stm_send_size(size_t total_size, size_t sent, const std::vector<size_t>& device_map) const noexcept override;

  [[nodiscard]] bool focus_stm_body(const std::vector<std::vector<STMFocus>>& points, size_t& sent, size_t total_size, uint32_t freq_div,
                                    autd3_float_t sound_speed, std::optional<uint16_t> start_idx, std::optional<uint16_t> finish_idx,
                                    TxDatagram& tx) const override;

  void gain_stm_legacy_header(TxDatagram& tx) const noexcept override;

  [[nodiscard]] bool gain_stm_legacy_body(const std::vector<std::vector<Drive>>& drives, size_t& sent, uint32_t freq_div, GainSTMMode mode,
                                          std::optional<uint16_t> start_idx, std::optional<uint16_t> finish_idx, TxDatagram& tx) const override;

  void gain_stm_normal_header(TxDatagram& tx) const noexcept override;

  [[nodiscard]] bool gain_stm_normal_phase(const std::vector<std::vector<Drive>>& drives, const std::vector<uint16_t>& cycles, size_t sent,
                                           uint32_t freq_div, GainSTMMode mode, std::optional<uint16_t> start_idx, std::optional<uint16_t> finish_idx,
                                           TxDatagram& tx) const override;

  [[nodiscard]] bool gain_stm_normal_duty(const std::vector<std::vector<Drive>>& drives, const std::vector<uint16_t>& cycles, size_t sent,
                                          uint32_t freq_div, GainSTMMode mode, std::optional<uint16_t> start_idx, std::optional<uint16_t> finish_idx,
                                          TxDatagram& tx) const override;

  void force_fan(TxDatagram& tx, bool value) const noexcept override;

  void reads_fpga_info(TxDatagram& tx, bool value) const noexcept override;

  void cpu_version(TxDatagram& tx) const noexcept override;

  void fpga_version(TxDatagram& tx) const noexcept override;

  void fpga_functions(TxDatagram& tx) const noexcept override;
};

}  // namespace autd3::driver
