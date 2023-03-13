// File: audio_file.cpp
// Project: audio_file
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/modulation/audio_file.hpp"

#include <cmath>
#include <cstring>
#include <fstream>

#include "autd3/core/modulation.hpp"

namespace autd3::modulation {

RawPCM::RawPCM(std::filesystem::path filename, const driver::autd3_float_t sampling_freq, const uint32_t mod_sampling_freq_div)
    : Modulation(mod_sampling_freq_div), _filename(std::move(filename)), _sampling_freq(sampling_freq) {}

std::vector<driver::autd3_float_t> RawPCM::calc() {
  std::ifstream ifs;
  ifs.open(_filename, std::ios::binary);
  if (ifs.fail()) throw std::runtime_error("Error on opening file");

  std::vector<uint8_t> buf;
  char read_buf[sizeof(uint8_t)];
  while (ifs.read(read_buf, sizeof(uint8_t))) {
    int value;
    std::memcpy(&value, read_buf, sizeof(uint8_t));
    buf.emplace_back(value);
  }

  const auto mod_sf = this->sampling_frequency();
  // up sampling
  std::vector<int32_t> sample_buf;
  const auto freq_ratio = mod_sf / _sampling_freq;
  sample_buf.resize(buf.size() * static_cast<size_t>(freq_ratio));
  size_t i = 0;
  std::generate(sample_buf.begin(), sample_buf.end(), [&i, freq_ratio, &buf] {
    const auto v = static_cast<driver::autd3_float_t>(i++) / freq_ratio;
    return std::fmod(v, driver::autd3_float_t{1}) < 1 / freq_ratio ? buf[static_cast<size_t>(v)] : 0;
  });

  std::vector<driver::autd3_float_t> buffer;
  buffer.reserve(sample_buf.size());
  std::transform(sample_buf.begin(), sample_buf.end(), std::back_inserter(buffer), [](const auto& v) {
    return static_cast<driver::autd3_float_t>(v / static_cast<driver::autd3_float_t>(std::numeric_limits<uint8_t>::max()));
  });
  return buffer;
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

Wav::Wav(std::filesystem::path filename, const uint32_t mod_sampling_freq_div) : Modulation(mod_sampling_freq_div), _filename(std::move(filename)) {}

std::vector<driver::autd3_float_t> Wav::calc() {
  std::ifstream fs;
  fs.open(_filename, std::ios::binary);
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

  const auto sampling_freq = read_from_stream<uint32_t>(fs);
  [[maybe_unused]] const auto bytes_per_sec = read_from_stream<uint32_t>(fs);
  [[maybe_unused]] const auto block_size = read_from_stream<uint16_t>(fs);

  const auto bits_per_sample = read_from_stream<uint16_t>(fs);

  if (const auto data_desc = read_from_stream<uint32_t>(fs); data_desc != 0x61746164u) throw std::runtime_error("Invalid data format");

  const auto data_chunk_size = read_from_stream<uint32_t>(fs);

  if (bits_per_sample != 8 && bits_per_sample != 16) throw std::runtime_error("This only supports 8 or 16 bits per sampling data.");

  std::vector<driver::autd3_float_t> buf;
  const auto data_size = data_chunk_size / (bits_per_sample / 8);
  buf.resize(data_size);
  std::generate(buf.begin(), buf.end(), [&] {
    if (bits_per_sample == 8)
      return static_cast<driver::autd3_float_t>(read_from_stream<uint8_t>(fs)) /
             static_cast<driver::autd3_float_t>(std::numeric_limits<uint8_t>::max());

    if (bits_per_sample == 16) {
      const auto d32 = static_cast<int32_t>(read_from_stream<int16_t>(fs)) - std::numeric_limits<int16_t>::min();
      return static_cast<driver::autd3_float_t>(d32) / static_cast<driver::autd3_float_t>(std::numeric_limits<uint16_t>::max());
    }
    throw std::runtime_error("Unsupported format.");
  });

  const auto mod_sf = this->sampling_frequency();

  // down sampling
  const auto freq_ratio = mod_sf / static_cast<driver::autd3_float_t>(sampling_freq);
  const auto buffer_size = static_cast<size_t>(static_cast<driver::autd3_float_t>(buf.size()) * freq_ratio);
  return generate_iota(buffer_size, [&](const size_t i) { return buf[static_cast<size_t>(static_cast<driver::autd3_float_t>(i) / freq_ratio)]; });
}
}  // namespace autd3::modulation
