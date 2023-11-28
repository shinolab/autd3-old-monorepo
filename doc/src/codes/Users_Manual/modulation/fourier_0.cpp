#include "autd3.hpp"

auto m =
    autd3::modulation::Fourier(autd3::modulation::Sine(100))
        .add_component(autd3::modulation::Sine(150))
        .add_components_from_iter(std::vector{autd3::modulation::Sine(200)});
