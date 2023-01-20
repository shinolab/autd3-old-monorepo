// File: simulator.hpp
// Project: simulator
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>
#include <thread>

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26495 26800 26819 28020)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#include "nlohmann/json.hpp"
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26451)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#include "tinycolormap.hpp"
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif

namespace autd3::extra {

namespace simulator {
#ifdef AUTD3_USE_METER
constexpr float scale = 1e-3f;
#else
constexpr float scale = 1;
#endif
#ifdef AUTD3_USE_METER
constexpr float z_parity = -1;
#else
constexpr float z_parity = 1;
#endif
}  // namespace simulator

/**
 * @brief Settings for Simulator
 */
struct SimulatorSettings {
  int32_t window_width{800};
  int32_t window_height{600};
  bool vsync{true};
  int32_t gpu_idx{0};
  float slice_pos_x{86.6252f * simulator::scale};
  float slice_pos_y{66.7133f * simulator::scale};
  float slice_pos_z{150.0f * simulator::scale * simulator::z_parity};
  float slice_width{300.0f * simulator::scale};
  float slice_height{300.0f * simulator::scale};
  float slice_pixel_size{1.0f * simulator::scale};
  float camera_pos_x{86.6252f * simulator::scale};
  float camera_pos_y{-533.2867f * simulator::scale};
  float camera_pos_z{150.0f * simulator::scale * simulator::z_parity};
  float camera_near_clip{0.1f * simulator::scale};
  float camera_far_clip{1000.0f * simulator::scale};
  float camera_move_speed{10.0f * simulator::scale};
  float sound_speed{340.0f * simulator::scale};

  float slice_rot_x{90.0f * simulator::z_parity};
  float slice_rot_y{0};
  float slice_rot_z{0};
  float slice_color_scale{2};
  float slice_alpha{1};
  tinycolormap::ColormapType coloring_method{tinycolormap::ColormapType::Inferno};
  bool show_radiation_pressure{false};
  float camera_rot_x{90.0f * simulator::z_parity};
  float camera_rot_y{0};
  float camera_rot_z{0};
  float camera_fov{45};
  float font_size{16};
  float background_r{0.3f};
  float background_g{0.3f};
  float background_b{0.3f};
  float background_a{1.0f};
  bool show_mod_plot{false};
  bool show_mod_plot_raw{false};
  bool mod_enable{false};
  bool mod_auto_play{false};
  bool stm_auto_play{false};

  std::string image_save_path{"image.png"};

  size_t max_dev_num{50};
  size_t max_trans_num{10000};

  void load_default() {
    slice_pos_x = 86.6252f * simulator::scale;
    slice_pos_y = 66.7133f * simulator::scale;
    slice_pos_z = 150.0f * simulator::scale * simulator::z_parity;
    slice_width = 300 * simulator::scale;
    slice_height = 300 * simulator::scale;
    slice_pixel_size = 1.0f * simulator::scale;
    camera_pos_x = 86.6252f * simulator::scale;
    camera_pos_y = -533.2867f * simulator::scale;
    camera_pos_z = 150.0f * simulator::scale * simulator::z_parity;
    camera_near_clip = 0.1f * simulator::scale;
    camera_far_clip = 1000 * simulator::scale;
    camera_move_speed = 10 * simulator::scale;
    sound_speed = 340.0e3f * simulator::scale;

    slice_rot_x = 90.0f * simulator::z_parity;
    slice_rot_y = 0;
    slice_rot_z = 0;
    slice_color_scale = 2;
    slice_alpha = 1;
    coloring_method = tinycolormap::ColormapType::Inferno;
    show_radiation_pressure = false;
    camera_rot_x = 90.0f * simulator::z_parity;
    camera_rot_y = 0;
    camera_rot_z = 0;
    camera_fov = 45;
    font_size = 16;
    background_r = 0.3f;
    background_g = 0.3f;
    background_b = 0.3f;
    background_a = 1.0f;
    show_mod_plot = false;
    show_mod_plot_raw = false;
    mod_enable = false;
    mod_auto_play = false;
    stm_auto_play = false;
  }
};

inline void to_json(nlohmann::json& j, const SimulatorSettings& s) {
  j = nlohmann::json{{"window_width", s.window_width},
                     {"window_height", s.window_height},
                     {"vsync", s.vsync},
                     {"gpu_idx", s.gpu_idx},
                     {"slice_pos_x", s.slice_pos_x},
                     {"slice_pos_y", s.slice_pos_y},
                     {"slice_pos_z", s.slice_pos_z},
                     {"slice_rot_x", s.slice_rot_x},
                     {"slice_rot_y", s.slice_rot_y},
                     {"slice_rot_z", s.slice_rot_z},
                     {"slice_width", s.slice_width},
                     {"slice_height", s.slice_height},
                     {"slice_pixel_size", s.slice_pixel_size},
                     {"slice_color_scale", s.slice_color_scale},
                     {"slice_alpha", s.slice_alpha},
                     {"show_radiation_pressure", s.show_radiation_pressure},
                     {"coloring_method", s.coloring_method},
                     {"camera_pos_x", s.camera_pos_x},
                     {"camera_pos_y", s.camera_pos_y},
                     {"camera_pos_z", s.camera_pos_z},
                     {"camera_rot_x", s.camera_rot_x},
                     {"camera_rot_y", s.camera_rot_y},
                     {"camera_rot_z", s.camera_rot_z},
                     {"camera_fov", s.camera_fov},
                     {"camera_near_clip", s.camera_near_clip},
                     {"camera_far_clip", s.camera_far_clip},
                     {"camera_move_speed", s.camera_move_speed},
                     {"sound_speed", s.sound_speed},
                     {"font_size", s.font_size},
                     {"background_r", s.background_r},
                     {"background_g", s.background_g},
                     {"background_b", s.background_b},
                     {"background_a", s.background_a},
                     {"show_mod_plot", s.show_mod_plot},
                     {"show_mod_plot_raw", s.show_mod_plot_raw},
                     {"image_save_path", s.image_save_path},
                     {"max_dev_num", s.max_dev_num},
                     {"max_trans_num", s.max_trans_num},
                     {"mod_enable", s.mod_enable},
                     {"mod_auto_play", s.mod_auto_play},
                     {"stm_auto_play", s.stm_auto_play}};
}

inline void from_json(const nlohmann::json& j, SimulatorSettings& s) {
  j.at("window_width").get_to(s.window_width);
  j.at("window_height").get_to(s.window_height);
  j.at("vsync").get_to(s.vsync);
  j.at("vsync").get_to(s.vsync);
  j.at("slice_pos_x").get_to(s.slice_pos_x);
  j.at("slice_pos_y").get_to(s.slice_pos_y);
  j.at("slice_pos_z").get_to(s.slice_pos_z);
  j.at("slice_rot_x").get_to(s.slice_rot_x);
  j.at("slice_rot_y").get_to(s.slice_rot_y);
  j.at("slice_rot_z").get_to(s.slice_rot_z);
  j.at("slice_width").get_to(s.slice_width);
  j.at("slice_height").get_to(s.slice_height);
  j.at("slice_pixel_size").get_to(s.slice_pixel_size);
  j.at("slice_color_scale").get_to(s.slice_color_scale);
  j.at("slice_alpha").get_to(s.slice_alpha);
  j.at("show_radiation_pressure").get_to(s.show_radiation_pressure);
  j.at("coloring_method").get_to(s.coloring_method);
  j.at("camera_pos_x").get_to(s.camera_pos_x);
  j.at("camera_pos_y").get_to(s.camera_pos_y);
  j.at("camera_pos_z").get_to(s.camera_pos_z);
  j.at("camera_rot_x").get_to(s.camera_rot_x);
  j.at("camera_rot_y").get_to(s.camera_rot_y);
  j.at("camera_rot_z").get_to(s.camera_rot_z);
  j.at("camera_fov").get_to(s.camera_fov);
  j.at("camera_near_clip").get_to(s.camera_near_clip);
  j.at("camera_far_clip").get_to(s.camera_far_clip);
  j.at("camera_move_speed").get_to(s.camera_move_speed);
  j.at("sound_speed").get_to(s.sound_speed);
  j.at("font_size").get_to(s.font_size);
  j.at("background_r").get_to(s.background_r);
  j.at("background_g").get_to(s.background_g);
  j.at("background_b").get_to(s.background_b);
  j.at("background_a").get_to(s.background_a);
  j.at("show_mod_plot").get_to(s.show_mod_plot);
  j.at("show_mod_plot_raw").get_to(s.show_mod_plot_raw);
  j.at("image_save_path").get_to(s.image_save_path);
  j.at("max_dev_num").get_to(s.max_dev_num);
  j.at("max_trans_num").get_to(s.max_trans_num);
  j.at("mod_enable").get_to(s.mod_enable);
  j.at("mod_auto_play").get_to(s.mod_auto_play);
  j.at("stm_auto_play").get_to(s.stm_auto_play);
}

/**
 * @brief An acoustic field simulator of autd3
 */
class Simulator {
 public:
  Simulator() = default;
  ~Simulator() = default;
  Simulator(const Simulator& v) noexcept = delete;
  Simulator& operator=(const Simulator& obj) = delete;
  Simulator(Simulator&& obj) = default;
  Simulator& operator=(Simulator&& obj) = delete;

  /**
   * @brief Start simulator
   */
  void run();

  /**
   * @brief Set settings
   */
  Simulator& settings(const SimulatorSettings& settings) {
    _settings = settings;
    return *this;
  }

 private:
  SimulatorSettings _settings{};

  std::thread _th;
};

}  // namespace autd3::extra
