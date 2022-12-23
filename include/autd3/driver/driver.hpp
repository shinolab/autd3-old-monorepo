// File: driver.hpp
// Project: driver
// Created Date: 15/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <optional>

#include "autd3/driver/common/cpu/datagram.hpp"

namespace autd3::driver {

/**
 * @brief Driver provides a set of functions to drive autd3 firmware
 */
class Driver {
 public:
  virtual ~Driver() = default;
  Driver() = default;
  Driver(const Driver& v) noexcept = default;
  Driver& operator=(const Driver& obj) = default;
  Driver(Driver&& obj) = default;
  Driver& operator=(Driver&& obj) = default;

  /**
   * @return uint8_t firmware version number
   */
  [[nodiscard]] virtual uint8_t version_num() const = 0;

  /**
   * @brief Pack data for Clear operation
   * @param tx transmission data
   */
  virtual void clear(TxDatagram& tx) const = 0;

  /**
   * @brief Pack Header data that does nothing
   * @param msg_id Message ID
   * @param tx transmission data
   */
  virtual void null_header(uint8_t msg_id, TxDatagram& tx) const = 0;

  /**
   * @brief Pack Body data that does nothing
   * @param tx transmission data
   */
  virtual void null_body(TxDatagram& tx) const = 0;

  /**
   * @brief Pack data for Synchronize operation
   * @param cycles ultrasound cycle data of all transducers
   * @param tx transmission data
   */
  virtual void sync(const std::vector<uint16_t>& cycles, TxDatagram& tx) const = 0;

  /**
   * @brief Pack Modulation delay data
   * @param delays modulation delay data of all transducers
   * @param tx transmission data
   */
  virtual void mod_delay(const std::vector<uint16_t>& delays, TxDatagram& tx) const = 0;

  /**
   * @brief Pack modulation data
   * @param msg_id Message ID
   * @param mod_data Modulation data
   * @param sent Number of data already sent
   * @param freq_div Modulation sampling frequency division
   * @param tx transmission data
   * @return true if freq_div is valid
   */
  [[nodiscard]] virtual bool modulation(uint8_t msg_id, const std::vector<uint8_t>& mod_data, size_t& sent, uint32_t freq_div,
                                        TxDatagram& tx) const = 0;

  /**
   * @brief Pack silencer data
   * @param msg_id Message ID
   * @param cycle Silencer cycle
   * @param step Silencer step
   * @param tx transmission data
   * @return true if cycle and step are valid
   */
  [[nodiscard]] virtual bool config_silencer(uint8_t msg_id, uint16_t cycle, uint16_t step, TxDatagram& tx) const = 0;

  /**
   * @brief Pack Header data for normal operation in Legacy mode
   * @param tx transmission data
   */
  virtual void normal_legacy_header(TxDatagram& tx) const = 0;

  /**
   * @brief Pack Body data for normal operation in Legacy mode
   * @param drives Drive of all transducers
   * @param tx transmission data
   */
  virtual void normal_legacy_body(const std::vector<Drive>& drives, TxDatagram& tx) const = 0;

  /**
   * @brief Pack Header data for normal operation in Normal mode
   * @param tx transmission data
   */
  virtual void normal_header(TxDatagram& tx) const = 0;

  /**
   * @brief Pack duty ratio data to Body for normal operation in Normal mode
   * @param drives Drive of all transducers
   * @param tx transmission data
   */
  virtual void normal_duty_body(const std::vector<Drive>& drives, TxDatagram& tx) const = 0;

  /**
   * @brief Pack phase data to Body for normal operation in Normal mode
   * @param drives Drive of all transducers
   * @param tx transmission data
   */
  virtual void normal_phase_body(const std::vector<Drive>& drives, TxDatagram& tx) const = 0;

  /**
   * @brief Pack Header data for FocusSTM
   * @param tx transmission data
   */
  virtual void focus_stm_header(TxDatagram& tx) const = 0;

  /**
   * @param total_size Total number of STMFocus to be sent
   * @param sent Number of STMFocus already sent
   * @param device_map Number of transducers in each device
   * @return size_t Number of STMFocus that can be sent
   */
  [[nodiscard]] virtual size_t focus_stm_send_size(size_t total_size, size_t sent, const std::vector<size_t>& device_map) const = 0;

  /**
   * @brief Pack STMFocus data to Body for FocusSTM
   * @param points STMFocus data to be sent
   * @param sent Number of STMFocus already sent
   * @param total_size Total number of STMFocus to be sent
   * @param freq_div STM sampling frequency division
   * @param sound_speed Sound speed
   * @param tx transmission data
   * @param start_idx stm start index
   * @param finish_idx stm finish index
   * @return true if total_size and freq_div are valid
   */
  [[nodiscard]] virtual bool focus_stm_body(const std::vector<std::vector<STMFocus>>& points, size_t& sent, size_t total_size, uint32_t freq_div,
                                            autd3_float_t sound_speed, std::optional<uint16_t> start_idx, std::optional<uint16_t> finish_idx,
                                            TxDatagram& tx) const = 0;

  /**
   * @brief Pack Header data for GainSTM in Legacy mode
   * @param tx transmission data
   */
  virtual void gain_stm_legacy_header(TxDatagram& tx) const = 0;

  /**
   * @brief Pack Body data for GainSTM
   * @param drives Drive of all transducers
   * @param sent Number of data already sent
   * @param freq_div STM sampling frequency division
   * @param mode GainSTMMode
   * @param start_idx stm start index
   * @param finish_idx stm finish index
   * @param tx transmission data
   * @return true if freq_div is valid
   */
  [[nodiscard]] virtual bool gain_stm_legacy_body(const std::vector<std::vector<Drive>>& drives, size_t& sent, uint32_t freq_div, GainSTMMode mode,
                                                  std::optional<uint16_t> start_idx, std::optional<uint16_t> finish_idx, TxDatagram& tx) const = 0;

  /**
   * @brief Pack Header data for GainSTM in Normal/NormalPhase mode
   * @param tx transmission data
   */
  virtual void gain_stm_normal_header(TxDatagram& tx) const = 0;

  /**
   * @brief Pack phase data to Body for GainSTM in Normal/NormalPhase mode
   * @param drives Drive of all transducers.
   * @param sent Number of data already sent
   * @param freq_div STM sampling frequency division
   * @param mode GainSTMMode
   * @param start_idx stm start index
   * @param finish_idx stm finish index
   * @param tx transmission data
   * @return true if freq_div is valid
   */
  [[nodiscard]] virtual bool gain_stm_normal_phase(const std::vector<std::vector<Drive>>& drives, size_t sent, uint32_t freq_div, GainSTMMode mode,
                                                   std::optional<uint16_t> start_idx, std::optional<uint16_t> finish_idx, TxDatagram& tx) const = 0;

  /**
   * @brief Pack duty data to Body for GainSTM in Normal/NormalPhase mode
   * @param drives Drive of all transducers.
   * @param sent Number of data already sent
   * @param freq_div STM sampling frequency division
   * @param mode GainSTMMode
   * @param start_idx stm start index
   * @param finish_idx stm finish index
   * @param tx transmission data
   * @return true if freq_div is valid
   */
  [[nodiscard]] virtual bool gain_stm_normal_duty(const std::vector<std::vector<Drive>>& drives, size_t sent, uint32_t freq_div, GainSTMMode mode,
                                                  std::optional<uint16_t> start_idx, std::optional<uint16_t> finish_idx, TxDatagram& tx) const = 0;

  /**
   * @brief Set force fan flag
   * @param tx transmission data
   * @param value if true, the fan is forced to start
   */
  virtual void force_fan(TxDatagram& tx, bool value) const = 0;

  /**
   * @brief Set reads FPGA information flag
   * @param tx transmission data
   * @param value if true, the device will return the FPGA state
   */
  virtual void reads_fpga_info(TxDatagram& tx, bool value) const = 0;

  /**
   * @brief Pack Header data for getting CPU version
   * @param tx transmission data
   */
  virtual void cpu_version(TxDatagram& tx) const = 0;

  /**
   * @brief Pack Header data for getting FPGA version
   * @param tx transmission data
   */
  virtual void fpga_version(TxDatagram& tx) const = 0;

  /**
   * @brief Pack Header data for getting FPGA function
   * @param tx transmission data
   */
  virtual void fpga_functions(TxDatagram& tx) const = 0;
};

}  // namespace autd3::driver
