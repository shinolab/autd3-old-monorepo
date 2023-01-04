// File: driver.hpp
// Project: v2_7
// Created Date: 14/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 04/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <optional>

#include "cpu/datagram.hpp"
#include "fpga/defined.hpp"

namespace autd3::driver {

struct Legacy {};
struct Normal {};
struct NormalDuty {};
struct NormalPhase {};

struct Driver {
  virtual ~Driver() = default;
  virtual bool pack(TxDatagram& tx) = 0;
};

struct Clear final : Driver {
  bool pack(TxDatagram& tx) override {
    tx.header().msg_id = MSG_CLEAR;
    tx.num_bodies = 0;
    return true;
  }
};

struct NullHeader final : Driver {
  NullHeader& msg_id(const uint8_t msg_id) {
    _msg_id = msg_id;
    return *this;
  }

  bool pack(TxDatagram& tx) override {
    tx.header().msg_id = _msg_id;
    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
    tx.header().cpu_flag.remove(CPUControlFlags::ConfigSync);
    tx.header().size = 0;
    return true;
  }

 private:
  uint8_t _msg_id{};
};

struct NullBody final : Driver {
  bool pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.num_bodies = 0;
    return true;
  }
};

template <typename T>
struct Sync final {
  Sync& cycles(const std::vector<uint16_t>& cycles) {
    _cycles = cycles.data();
    _size = cycles.size();
    return *this;
  }

  [[nodiscard]] bool pack(TxDatagram& tx);

 private:
  size_t _size{};
  const uint16_t* _cycles{};
};

template <>
bool Sync<Legacy>::pack(TxDatagram& tx);
template <>
bool Sync<Normal>::pack(TxDatagram& tx);

struct ModDelay final : Driver {
  ModDelay& delays(const std::vector<uint16_t>& delays) {
    _delays = delays.data();
    return *this;
  }

  [[nodiscard]] bool pack(TxDatagram& tx) override {
    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.set(CPUControlFlags::ModDelay);
    tx.num_bodies = tx.num_devices();

    std::memcpy(tx.bodies_raw_ptr(), _delays, tx.bodies_size());
    return true;
  }

 private:
  const uint16_t* _delays{};
};

struct Modulation final : Driver {
  Modulation& msg_id(const uint8_t msg_id) {
    _msg_id = msg_id;
    return *this;
  }

  Modulation& mod_data(const std::vector<uint8_t>& mod_data) {
    _mod_data = mod_data.data();
    _size = mod_data.size();
    return *this;
  }

  Modulation& sent(size_t* sent) {
    _sent = sent;
    return *this;
  }

  Modulation& freq_div(const uint32_t freq_div) {
    _freq_div = freq_div;
    return *this;
  }

  [[nodiscard]] bool pack(TxDatagram& tx) override;

 private:
  uint8_t _msg_id{};
  size_t _size{};
  const uint8_t* _mod_data{};
  size_t* _sent{};
  uint32_t _freq_div{};
};

struct ConfigSilencer final : Driver {
  ConfigSilencer& msg_id(const uint8_t msg_id) {
    _msg_id = msg_id;
    return *this;
  }

  ConfigSilencer& cycle(const uint16_t cycle) {
    _cycle = cycle;
    return *this;
  }

  ConfigSilencer& step(const uint16_t step) {
    _step = step;
    return *this;
  }

  [[nodiscard]] bool pack(TxDatagram& tx) override;

 private:
  uint8_t _msg_id{};
  uint16_t _cycle{};
  uint16_t _step{};
};

template <typename T>
struct GainHeader final : Driver {
  bool pack(TxDatagram& tx) override;
};

template <>
inline bool GainHeader<Legacy>::pack(TxDatagram& tx) {
  tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
  tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);

  tx.header().fpga_flag.set(FPGAControlFlags::LegacyMode);
  tx.header().fpga_flag.remove(FPGAControlFlags::STMMode);

  tx.num_bodies = 0;

  return true;
}

template <>
inline bool GainHeader<Normal>::pack(TxDatagram& tx) {
  tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
  tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);

  tx.header().fpga_flag.remove(FPGAControlFlags::LegacyMode);
  tx.header().fpga_flag.remove(FPGAControlFlags::STMMode);

  tx.num_bodies = 0;

  return true;
}

template <typename T>
struct GainBody final : Driver {
  GainBody& drives(const std::vector<Drive>& drives) {
    _drives = drives.data();
    _size = drives.size();
    return *this;
  }

  GainBody& cycles(const std::vector<uint16_t>& cycles) {
    _cycles = cycles.data();
    return *this;
  }

  bool pack(TxDatagram& tx) override;

 private:
  size_t _size{};
  const Drive* _drives{};
  const uint16_t* _cycles{};
};

template <>
inline bool GainBody<Legacy>::pack(TxDatagram& tx) {
  auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies_raw_ptr());
  for (size_t i = 0; i < _size; i++) p[i].set(_drives[i]);

  tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

  tx.num_bodies = tx.num_devices();

  return true;
}

template <>
inline bool GainBody<NormalDuty>::pack(TxDatagram& tx) {
  tx.header().cpu_flag.set(CPUControlFlags::IsDuty);

  auto* p = reinterpret_cast<Duty*>(tx.bodies_raw_ptr());
  for (size_t i = 0; i < _size; i++) p[i].set(_drives[i], _cycles[i]);

  tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

  tx.num_bodies = tx.num_devices();

  return true;
}

template <>
inline bool GainBody<NormalPhase>::pack(TxDatagram& tx) {
  tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

  auto* p = reinterpret_cast<Phase*>(tx.bodies_raw_ptr());
  for (size_t i = 0; i < _size; i++) p[i].set(_drives[i], _cycles[i]);

  tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

  tx.num_bodies = tx.num_devices();

  return true;
}

struct FocusSTMHeader final : Driver {
  bool pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().cpu_flag.remove(CPUControlFlags::STMBegin);
    tx.header().cpu_flag.remove(CPUControlFlags::STMEnd);

    tx.header().fpga_flag.set(FPGAControlFlags::STMMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::STMGainMode);
    tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMStartIdx);
    tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMFinishIdx);

    tx.num_bodies = 0;

    return true;
  }
};

struct FocusSTMBody final : Driver {
  FocusSTMBody& points(const std::vector<std::vector<STMFocus>>& points) {
    _points = points.data();
    return *this;
  }

  FocusSTMBody& sent(size_t* sent) {
    _sent = sent;
    return *this;
  }

  FocusSTMBody& total_size(const size_t total_size) {
    _total_size = total_size;
    return *this;
  }

  FocusSTMBody& freq_div(const uint32_t freq_div) {
    _freq_div = freq_div;
    return *this;
  }

  FocusSTMBody& sound_speed(const autd3_float_t sound_speed) {
    _sound_speed = sound_speed;
    return *this;
  }

  FocusSTMBody& start_idx(const std::optional<uint16_t> start_idx) {
    _start_idx = start_idx;
    return *this;
  }

  FocusSTMBody& finish_idx(const std::optional<uint16_t> finish_idx) {
    _finish_idx = finish_idx;
    return *this;
  }

  [[nodiscard]] static size_t send_size(const size_t total_size, const size_t sent, const std::vector<size_t>& device_map) noexcept {
    const size_t tr_num = *std::min_element(device_map.begin(), device_map.end());
    const size_t data_len = tr_num * sizeof(uint16_t);
    const auto max_size =
        sent == 0 ? (data_len - sizeof(uint16_t) - sizeof(uint32_t) - sizeof(uint32_t) - sizeof(uint16_t) - sizeof(uint16_t)) / sizeof(STMFocus)
                  : (data_len - sizeof(uint16_t)) / sizeof(STMFocus);
    return (std::min)(total_size - sent, max_size);
  }

  [[nodiscard]] bool pack(TxDatagram& tx) override;

 private:
  const std::vector<STMFocus>* _points{};
  size_t* _sent{};
  size_t _total_size{};
  uint32_t _freq_div{};
  autd3_float_t _sound_speed{};
  std::optional<uint16_t> _start_idx;
  std::optional<uint16_t> _finish_idx;
};

template <typename T>
struct GainSTMHeader final : Driver {
  bool pack(TxDatagram& tx) override;
};

template <>
inline bool GainSTMHeader<Legacy>::pack(TxDatagram& tx) {
  tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
  tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
  tx.header().cpu_flag.remove(CPUControlFlags::STMBegin);
  tx.header().cpu_flag.remove(CPUControlFlags::STMEnd);

  tx.header().fpga_flag.set(FPGAControlFlags::LegacyMode);
  tx.header().fpga_flag.set(FPGAControlFlags::STMMode);
  tx.header().fpga_flag.set(FPGAControlFlags::STMGainMode);
  tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMStartIdx);
  tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMFinishIdx);

  tx.num_bodies = 0;

  return true;
}

template <>
inline bool GainSTMHeader<Normal>::pack(TxDatagram& tx) {
  tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
  tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
  tx.header().cpu_flag.remove(CPUControlFlags::STMBegin);
  tx.header().cpu_flag.remove(CPUControlFlags::STMEnd);

  tx.header().fpga_flag.remove(FPGAControlFlags::LegacyMode);
  tx.header().fpga_flag.set(FPGAControlFlags::STMMode);
  tx.header().fpga_flag.set(FPGAControlFlags::STMGainMode);
  tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMStartIdx);
  tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMFinishIdx);

  tx.num_bodies = 0;

  return true;
}

template <typename T>
struct GainSTMBody final : Driver {
  GainSTMBody& drives(const std::vector<std::vector<Drive>>& drives) {
    _drives = drives.data();
    _size = drives.size();
    return *this;
  }

  GainSTMBody& cycles(const std::vector<uint16_t>& cycles) {
    _cycles = cycles.data();
    return *this;
  }

  GainSTMBody& sent(size_t* sent) {
    _sent = sent;
    return *this;
  }

  GainSTMBody& freq_div(const uint32_t freq_div) {
    _freq_div = freq_div;
    return *this;
  }

  GainSTMBody& mode(const GainSTMMode mode) {
    _mode = mode;
    return *this;
  }

  GainSTMBody& start_idx(const std::optional<uint16_t> start_idx) {
    _start_idx = start_idx;
    return *this;
  }

  GainSTMBody& finish_idx(const std::optional<uint16_t> finish_idx) {
    _finish_idx = finish_idx;
    return *this;
  }

  [[nodiscard]] bool pack(TxDatagram& tx) override;

 private:
  size_t _size{};
  const std::vector<Drive>* _drives{};
  const uint16_t* _cycles{};
  size_t* _sent{};
  uint32_t _freq_div{};
  GainSTMMode _mode{};
  std::optional<uint16_t> _start_idx;
  std::optional<uint16_t> _finish_idx;
};

template <>
bool GainSTMBody<Legacy>::pack(TxDatagram& tx);

template <>
bool GainSTMBody<NormalDuty>::pack(TxDatagram& tx);

template <>
bool GainSTMBody<NormalPhase>::pack(TxDatagram& tx);

struct ForceFan final : Driver {
  explicit ForceFan(const bool value) : _value(value) {}

  bool pack(TxDatagram& tx) override {
    if (_value)
      tx.header().fpga_flag.set(FPGAControlFlags::ForceFan);
    else
      tx.header().fpga_flag.remove(FPGAControlFlags::ForceFan);
    return true;
  }

 private:
  bool _value;
};

struct ReadsFPGAInfo final : Driver {
  explicit ReadsFPGAInfo(const bool value) : _value(value) {}

  bool pack(TxDatagram& tx) override {
    if (_value)
      tx.header().fpga_flag.set(FPGAControlFlags::ReadsFPGAInfo);
    else
      tx.header().fpga_flag.remove(FPGAControlFlags::ReadsFPGAInfo);
    return true;
  }

 private:
  bool _value;
};

struct CPUVersion final : Driver {
  bool pack(TxDatagram& tx) override {
    tx.header().msg_id = MSG_RD_CPU_VERSION;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_CPU_VERSION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;

    return true;
  }
};

struct FPGAVersion final : Driver {
  bool pack(TxDatagram& tx) override {
    tx.header().msg_id = MSG_RD_FPGA_VERSION;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_FPGA_VERSION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;

    return true;
  }
};

struct FPGAFunctions final : Driver {
  bool pack(TxDatagram& tx) override {
    tx.header().msg_id = MSG_RD_FPGA_FUNCTION;
    tx.header().cpu_flag = static_cast<CPUControlFlags::Value>(MSG_RD_FPGA_FUNCTION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;

    return true;
  }
};
}  // namespace autd3::driver
