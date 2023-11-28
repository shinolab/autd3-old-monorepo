#include "autd3.hpp"

const auto m = autd3::modulation::Sine(100) +
               autd3::modulation::Sine(150).with_phase(autd3::pi / 2.0);