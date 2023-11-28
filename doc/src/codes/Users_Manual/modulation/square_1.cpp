#include "autd3.hpp"

const auto m = autd3::modulation::Square(150)
                   .with_low(autd3::EmitIntensity::minimum())
                   .with_high(autd3::EmitIntensity::maximum());