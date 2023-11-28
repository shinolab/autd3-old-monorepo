#include "autd3.hpp"

const auto m = autd3::modulation::Sine(150).with_sampling_config(
    autd3::SamplingConfiguration::new_with_frequency(4e3));
