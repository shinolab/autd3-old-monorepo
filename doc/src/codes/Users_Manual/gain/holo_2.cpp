#include "autd3/gain/holo.hpp"

const auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
auto g = autd3::gain::holo::GSPAT(backend).with_repeat(100);
