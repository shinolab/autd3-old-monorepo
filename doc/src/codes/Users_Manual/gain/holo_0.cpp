#include "autd3/gain/holo.hpp"

using autd3::gain::holo::Pascal;
const auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
auto g = autd3::gain::holo::GSPAT(backend)
             .add_focus(autd3::Vector3(x1, y1, z1), 5e3 * Pascal)
             .add_focus(autd3::Vector3(x2, y2, z2), 5e3 * Pascal);