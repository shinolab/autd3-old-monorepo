// File: audio_file.cpp
// Project: audio_file
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 20/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include "autd3/modulation/audio_file.hpp"

#include <cmath>
#include <cstring>
#include <fstream>

#include "autd3/core/modulation.hpp"

namespace autd3::modulation {

RawPCM::RawPCM(const std::string& filename, const double sampling_freq, const uint32_t mod_sampling_freq_div)
    : Modulation(), _sampling_freq(sampling_freq) {
  this->_props.freq_div = mod_sampling_freq_div;

  std::ifstream ifs;
  ifs.open(filename, std::ios::binary);

  if (ifs.fail()) throw std::runtime_error("Error on opening file");

  char buf[sizeof(uint8_t)];
  while (ifs.read(buf, sizeof(uint8_t))) {
    int value;
    std::memcpy(&value, buf, sizeof(uint8_t));
    _buf.emplace_back(value);
  }
}

void RawPCM::calc() {
  const auto mod_sf = this->sampling_frequency();
  // up sampling
  std::vector<int32_t> sample_buf;
  const auto freq_ratio = static_cast<double>(mod_sf) / _sampling_freq;
  sample_buf.resize(this->_buf.size() * static_cast<size_t>(freq_ratio));

  for (size_t i = 0; i < sample_buf.size(); i++) {
    const auto v = static_cast<double>(i) / freq_ratio;
    const auto tmp = std::fmod(v, double{1}) < 1 / freq_ratio ? this->_buf[static_cast<int>(v)] : 0;
    sample_buf[i] = tmp;
  }

  this->_props.buffer.resize(sample_buf.size());
  for (size_t i = 0; i < sample_buf.size(); i++) {
    const auto amp = static_cast<double>(sample_buf[i]) / static_cast<double>(std::numeric_limits<uint8_t>::max());
    const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp(amp, 0.0, 1.0)) / driver::pi * 510.0));
    this->_props.buffer[i] = duty;
  }
}

namespace {
template <class T>
T read_from_stream(std::ifstream& fsp) {
  char buf[sizeof(T)];
  if (!fsp.read(buf, sizeof(T))) throw std::runtime_error("Invalid data length");
  T v{};
  std::memcpy(&v, buf, sizeof(T));
  return v;
}
}  // namespace

Wav::Wav(const std::string& filename, const uint32_t mod_sampling_freq_div) : Modulation() {
  this->_props.freq_div = mod_sampling_freq_div;

  std::ifstream fs;
  fs.open(filename, std::ios::binary);
  if (fs.fail()) throw std::runtime_error("Error on opening file");

  if (const auto riff_tag = read_from_stream<uint32_t>(fs); riff_tag != 0x46464952u) throw std::runtime_error("Invalid data format");

  [[maybe_unused]] const auto chunk_size = read_from_stream<uint32_t>(fs);

  if (const auto wav_desc = read_from_stream<uint32_t>(fs); wav_desc != 0x45564157u) throw std::runtime_error("Invalid data format");
  if (const auto fmt_desc = read_from_stream<uint32_t>(fs); fmt_desc != 0x20746d66u) throw std::runtime_error("Invalid data format");
  if (const auto fmt_chunk_size = read_from_stream<uint32_t>(fs); fmt_chunk_size != 0x00000010u) throw std::runtime_error("Invalid data format");

  if (const auto wave_fmt = read_from_stream<uint16_t>(fs); wave_fmt != 0x0001u)
    throw std::runtime_error("Invalid data format. This supports only uncompressed linear PCM data.");
  if (const auto channel = read_from_stream<uint16_t>(fs); channel != 0x0001u)
    throw std::runtime_error("Invalid data format. This supports only monaural audio.");

  _sampling_freq = read_from_stream<uint32_t>(fs);
  [[maybe_unused]] const auto bytes_per_sec = read_from_stream<uint32_t>(fs);
  [[maybe_unused]] const auto block_size = read_from_stream<uint16_t>(fs);

  const auto bits_per_sample = read_from_stream<uint16_t>(fs);

  if (const auto data_desc = read_from_stream<uint32_t>(fs); data_desc != 0x61746164u) throw std::runtime_error("Invalid data format");

  const auto data_chunk_size = read_from_stream<uint32_t>(fs);

  if (bits_per_sample != 8 && bits_per_sample != 16) throw std::runtime_error("This only supports 8 or 16 bits per sampling data.");

  const auto data_size = data_chunk_size / (bits_per_sample / 8);
  for (size_t i = 0; i < data_size; i++) {
    if (bits_per_sample == 8) {
      auto amp = static_cast<double>(read_from_stream<uint8_t>(fs)) / static_cast<double>(std::numeric_limits<uint8_t>::max());
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp(amp, 0.0, 1.0)) / driver::pi * 510.0));
      _buf.emplace_back(duty);
    } else if (bits_per_sample == 16) {
      const auto d32 = static_cast<int32_t>(read_from_stream<int16_t>(fs)) - std::numeric_limits<int16_t>::min();
      auto amp = static_cast<double>(d32) / static_cast<double>(std::numeric_limits<uint16_t>::max());
      const auto duty = static_cast<uint8_t>(std::round(std::asin(std::clamp(amp, 0.0, 1.0)) / driver::pi * 510.0));
      _buf.emplace_back(duty);
    }
  }
}

void Wav::calc() {
  const auto mod_sf = this->sampling_frequency();

  // down sampling
  std::vector<uint8_t> sample_buf;
  const auto freq_ratio = mod_sf / static_cast<double>(_sampling_freq);
  auto buffer_size = static_cast<size_t>(static_cast<double>(this->_buf.size()) * freq_ratio);
  buffer_size = std::min(buffer_size, driver::MOD_BUF_SIZE_MAX);

  sample_buf.resize(buffer_size);
  for (size_t i = 0; i < sample_buf.size(); i++) {
    const auto idx = static_cast<size_t>(static_cast<double>(i) / freq_ratio);
    sample_buf[i] = _buf[idx];
  }

  this->_props.buffer = std::move(sample_buf);
}
}  // namespace autd3::modulation
