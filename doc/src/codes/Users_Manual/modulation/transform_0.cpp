#include "autd3.hpp"

const auto m = autd3::modulation::Sine(150).with_transform(
    [](const size_t idx, const autd3::EmitIntensity d) -> autd3::EmitIntensity {
      return autd3::EmitIntensity(d.value() / 2);
    });