#include "autd3.hpp"

auto stm = autd3::GainSTM::new_with_sampling_config(
    autd3::SamplingConfiguration::new_with_frequency(1));