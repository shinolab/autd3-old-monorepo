// File: point.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <algorithm>
#include <utility>
#include <vector>

#include "autd3/core/geometry/transducer.hpp"
#include "autd3/core/interface.hpp"
#include "autd3/driver/cpu/operation.hpp"
#include "stm.hpp"

namespace autd3::core {

template <typename T = LegacyTransducer, std::enable_if_t<std::is_base_of_v<Transducer<typename T::D>, T>, nullptr_t> = nullptr>
struct PointSTM final : STM, DatagramBody<T> {
  PointSTM() : STM(), DatagramBody<T>(), _sent(0) {}

  void add(const Vector3& point, uint8_t duty_shift = 0) {
    if (_points.size() + 1 > driver::POINT_STM_BUF_SIZE_MAX) {
      throw std::runtime_error("PointSTM out of buffer");
    }
    _points.emplace_back(point, duty_shift);
  }

  size_t size() override { return _points.size(); }
  void init() override { _sent = 0; }
  void pack(const uint8_t msg_id, Geometry<T>& geometry, driver::TxDatagram& tx) override {
    point_stm_header(msg_id, tx);
    if (is_finished()) {
      return;
    }

    const auto is_first_frame = _sent == 0;
    const auto max_size = is_first_frame ? driver::POINT_STM_HEAD_DATA_SIZE : driver::POINT_STM_BODY_DATA_SIZE;

    const auto send_size = std::min(_points.size() - _sent, max_size);
    const auto is_last_frame = _sent + send_size == _points.size();

    std::vector<std::vector<driver::STMFocus>> points;
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(points), [this, send_size](const Device<T>& dev) {
      std::vector<driver::STMFocus> lp;
      const auto src = gsl::span{_points}.subspan(_sent, send_size);
      std::transform(src.begin(), src.end(), std::back_inserter(lp), [dev](const auto ps) {
        const auto [pos, shift] = ps;
        const auto local = dev.to_local_position(pos);
        return driver::STMFocus(local.x(), local.y(), local.z(), shift);
      });
      return lp;
    });

    driver::point_stm_body(points, is_first_frame, this->_freq_div, geometry.sound_speed, is_last_frame, tx);

    _sent += send_size;
  }

  [[nodiscard]] bool is_finished() const override { return _sent == _points.size(); }

 private:
  std::vector<std::pair<Vector3, uint8_t>> _points;
  size_t _sent;
};

}  // namespace autd3::core
