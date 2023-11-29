#include "autd3.hpp"

const auto m = autd3::modulation::Sine(150)
                   .with_intensity(autd3::EmitIntensity::maximum())
                   .with_offset(autd3::EmitIntensity::maximum() / 2);