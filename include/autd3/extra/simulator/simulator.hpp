// File: simulator.hpp
// Project: simulator
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3/core/geometry.hpp>
#include <memory>
#include <string>

namespace autd3::extra::simulator {

struct Settings {
  int32_t window_width{800};
  int32_t window_height{600};
  bool vsync{true};
  int32_t gpu_idx{0};

  float slice_pos_x{0};
  float slice_pos_y{0};
  float slice_pos_z{0};
  float slice_rot_x{0};
  float slice_rot_y{0};
  float slice_rot_z{0};
  int32_t slice_width{300};
  int32_t slice_height{300};
  int32_t slice_pixel_size{1};
  float slice_color_scale{2};
  float slice_alpha{1};
  float camera_pos_x{0};
  float camera_pos_y{0};
  float camera_pos_z{0};
  float camera_rot_x{0};
  float camera_rot_y{0};
  float camera_rot_z{0};
  float camera_fov{45};
  float camera_near_clip{0.1f};
  float camera_far_clip{1000};
  float camera_move_speed{10};
  float font_size{16};
  std::string font_path{};
  float background_r{0.3f};
  float background_g{0.3f};
  float background_b{0.3f};
  float background_a{1.0f};
  bool show_mod_plot{false};
  bool show_mod_plot_raw{false};

  std::string image_save_path{"image.png"};
};

class Simulator {
 public:
  /**
   * @brief Start simulator
   */
  virtual void start(const core::Geometry& geometry) = 0;

  /**
   * @brief Exit simulator
   */
  virtual void exit() = 0;

  virtual bool send(const driver::TxDatagram& tx) = 0;
  virtual bool receive(driver::RxDatagram& rx) = 0;

  /**
   * @brief Create Bundle link
   */
  [[nodiscard]] static std::unique_ptr<Simulator> create(Settings setting, std::function<void(Settings)> callback);

  Simulator() noexcept = default;
  virtual ~Simulator() = default;
  Simulator(const Simulator& v) noexcept = delete;
  Simulator& operator=(const Simulator& obj) = delete;
  Simulator(Simulator&& obj) = default;
  Simulator& operator=(Simulator&& obj) = default;
};

}  // namespace autd3::extra::simulator
