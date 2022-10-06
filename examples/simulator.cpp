// File: simulator_example.cpp
// Project: examples
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/simulator/simulator.hpp"

#include <filesystem>

#include "autd3.hpp"
#include "autd3/link/simulator.hpp"
#include "runner.hpp"

int main([[maybe_unused]] int argc, char* argv[]) try {
  autd3::Controller autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
  autd.geometry().add_device(autd3::Vector3(autd3::DEVICE_WIDTH, 0.0, 0.0), autd3::Vector3::Zero());

  // autd.geometry().mode() = std::make_unique<autd3::NormalMode>();
  // for (auto& dev : autd.geometry())
  //   for (auto& tr : dev) tr.set_frequency(70e3);

  autd3::extra::simulator::Settings settings;
  settings.slice_pos_x = static_cast<float>(autd.geometry().center().x());
  settings.slice_pos_y = static_cast<float>(autd.geometry().center().y());
  settings.slice_pos_z = static_cast<float>(autd.geometry().center().z()) + 150.0f;
  settings.slice_rot_x = 90.0f;
  settings.camera_pos_x = settings.slice_pos_x;
  settings.camera_pos_y = settings.slice_pos_y - 600.0f;
  settings.camera_pos_z = settings.slice_pos_z;
  settings.camera_rot_x = 90.0f;
  settings.font_path = AUTD3_SIMULATOR_FONT_PATH;
  settings.image_save_path = std::filesystem::path(argv[0]).parent_path().append("image.png").string();

  auto link = autd3::link::Simulator(settings).build();
  autd.open(std::move(link));

  run(std::move(autd));

  return 0;
} catch (std::exception& e) {
  std::cerr << e.what() << std::endl;
  return -1;
}
