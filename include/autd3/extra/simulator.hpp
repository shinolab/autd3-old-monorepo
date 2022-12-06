// File: simulator.hpp
// Project: simulator
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>
#include <thread>

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26495 26800 26819)
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

/**
 * @brief Settings for Simulator
 */
struct SimulatorSettings {
  int32_t window_width{800};
  int32_t window_height{600};
  bool vsync{true};
  int32_t gpu_idx{0};
#ifdef AUTD3_USE_METER
  bool use_meter{true};
#ifdef AUTD3_USE_LEFT_HANDED
  bool use_left_handed{true};
  float slice_pos_z{-150.0e-3f};
  float camera_pos_z{-150.0e-3f};
#else
  bool use_left_handed{false};
  float slice_pos_z{150.0e-3f};
  float camera_pos_z{150.0e-3f};
#endif
  float slice_pos_x{86.6252e-3f};
  float slice_pos_y{66.7133e-3f};
  float slice_width{300e-3f};
  float slice_height{300e-3f};
  float slice_pixel_size{1.0e-3f};
  float camera_pos_x{86.6252e-3f};
  float camera_pos_y{-533.2867e-3f};
  float camera_near_clip{0.1e-3f};
  float camera_far_clip{1000e-3f};
  float camera_move_speed{10e-3f};
  float sound_speed{340.0f};
#else
  bool use_meter{false};
#ifdef AUTD3_USE_LEFT_HANDED
  bool use_left_handed{true};
  float slice_pos_z{-150.0f};
  float camera_pos_z{-150.0f};
#else
  bool use_left_handed{false};
  float slice_pos_z{150.0f};
  float camera_pos_z{150.0f};
#endif
  float slice_pos_x{86.6252f};
  float slice_pos_y{66.7133f};
  float slice_width{300};
  float slice_height{300};
  float slice_pixel_size{1.0};
  float camera_pos_x{86.6252f};
  float camera_pos_y{-533.2867f};
  float camera_near_clip{0.1f};
  float camera_far_clip{1000};
  float camera_move_speed{10};
  float sound_speed{340.0e3f};
#endif

#ifdef AUTD3_USE_LEFT_HANDED
  float slice_rot_x{-90.0f};
  float camera_rot_x{-90.0f};
#else
  float slice_rot_x{90.0f};
  float camera_rot_x{90.0f};
#endif
  float slice_rot_y{0};
  float slice_rot_z{0};
  float slice_color_scale{2};
  float slice_alpha{1};
  tinycolormap::ColormapType coloring_method{tinycolormap::ColormapType::Inferno};
  bool show_radiation_pressure{false};
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

  std::string image_save_path{"image.png"};

  size_t max_dev_num{50};
  size_t max_trans_num{10000};

  void load_default(const bool use_meter_, const bool use_left_handed_) {
    use_meter = use_meter_;
    use_left_handed = use_left_handed_;
    if (use_meter) {
      slice_pos_x = 86.6252e-3f;
      slice_pos_y = 66.7133e-3f;
      slice_pos_z = use_left_handed ? -150.0e-3f : 150.0e-3f;
      slice_width = 300e-3f;
      slice_height = 300e-3f;
      slice_pixel_size = 1.0e-3f;
      camera_pos_x = 86.6252e-3f;
      camera_pos_y = -533.2867e-3f;
      camera_pos_z = use_left_handed ? -150.0e-3f : 150.0e-3f;
      camera_near_clip = 0.1e-3f;
      camera_far_clip = 1000e-3f;
      camera_move_speed = 10e-3f;
      sound_speed = 340.0f;
    } else {
      slice_pos_x = 86.6252f;
      slice_pos_y = 66.7133f;
      slice_pos_z = use_left_handed ? -150.0f : 150.0f;
      slice_width = 300;
      slice_height = 300;
      slice_pixel_size = 1.0;
      camera_pos_x = 86.6252f;
      camera_pos_y = -533.2867f;
      camera_pos_z = use_left_handed ? -150.0f : 150.0f;
      camera_near_clip = 0.1f;
      camera_far_clip = 1000;
      camera_move_speed = 10;
      sound_speed = 340.0e3f;
    }

    slice_rot_x = use_left_handed ? -90.0f : 90.0f;
    slice_rot_y = 0;
    slice_rot_z = 0;
    slice_color_scale = 2;
    slice_alpha = 1;
    coloring_method = tinycolormap::ColormapType::Inferno;
    show_radiation_pressure = false;
    camera_rot_x = use_left_handed ? -90.0f : 90.0f;
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
  }
};

inline void to_json(nlohmann::json& j, const SimulatorSettings& s) {
  j = nlohmann::json{
      {"window_width", s.window_width},
      {"window_height", s.window_height},
      {"vsync", s.vsync},
      {"gpu_idx", s.gpu_idx},
      {"use_meter", s.use_meter},
      {"use_left_handed", s.use_left_handed},
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
  };
}
inline void from_json(const nlohmann::json& j, SimulatorSettings& s) {
  j.at("window_width").get_to(s.window_width);
  j.at("window_height").get_to(s.window_height);
  j.at("vsync").get_to(s.vsync);
  j.at("vsync").get_to(s.vsync);
  j.at("use_meter").get_to(s.use_meter);
  j.at("use_left_handed").get_to(s.use_left_handed);
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
  [[nodiscard]] bool run();

  /**
   * @brief Set settings
   */
  Simulator& settings(SimulatorSettings* settings) {
    _settings = settings;
    return *this;
  }

 private:
  SimulatorSettings _default_settings{};
  SimulatorSettings* _settings{&_default_settings};

  std::thread _th;
};

}  // namespace autd3::extra
