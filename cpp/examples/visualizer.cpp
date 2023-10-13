// File: visualizer.cpp
// Project: examples
// Created Date: 12/10/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "autd3/link/visualizer.hpp"

#include "autd3.hpp"
#include "util.hpp"

#ifdef USE_PYTHON
using PlotConfig = autd3::link::PyPlotConfig;
#else
using PlotConfig = autd3::link::PlotConfig;
#endif

int main() try {
  auto autd = autd3::Controller::builder()
                  .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
#ifdef USE_PYTHON
                  .open_with(autd3::link::Visualizer::builder().with_backend<autd3::link::PythonBackend>());
#else
                  .open_with(autd3::link::Visualizer::builder());
#endif

  autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0, 0, 150);

  autd3::gain::Focus g(center);
  autd3::modulation::Square m(150);

  autd.send(m, g);

  PlotConfig config;
  config.fname = "phase.png";
  autd.link<autd3::link::Visualizer>().plot_phase(config, autd.geometry());

  config.fname = "x.png";
  autd.link<autd3::link::Visualizer>().plot_field(
      config, autd3::link::PlotRange(center.x() - 50, center.x() + 50, center.y(), center.y(), center.z(), center.z(), 1), autd.geometry());

  config.fname = "xy.png";
  autd.link<autd3::link::Visualizer>().plot_field(
      config, autd3::link::PlotRange(center.x() - 20, center.x() + 20, center.y() - 30, center.y() + 30, center.z(), center.z(), 1), autd.geometry());

  config.fname = "yz.png";
  autd.link<autd3::link::Visualizer>().plot_field(
      config, autd3::link::PlotRange(center.x(), center.x(), center.y() - 30, center.y() + 30, 0, center.z() + 50, 2), autd.geometry());

  config.fname = "zx.png";
  autd.link<autd3::link::Visualizer>().plot_field(
      config, autd3::link::PlotRange(center.x() - 30, center.x() + 30, center.y(), center.y(), 0, center.z() + 50, 2), autd.geometry());

  config.fname = "mod.png";
  autd.link<autd3::link::Visualizer>().plot_modulation(config);

  // Calculate acoustic pressure without plotting
  std::vector points{center};
  const auto p = autd.link<autd3::link::Visualizer>().calc_field(points, autd.geometry());
  std::cout << "Acoustic pressure at (" << center.x() << ", " << center.y() << ", " << center.z() << ") = " << p[0] << std::endl;

  autd.close();

  return 0;
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
