#include "autd3.hpp"

const auto g =
    autd3::gain::Uniform(autd3::EmitIntensity::maximum())
        .with_transform([](const autd3::Device& dev,
                           const autd3::Transducer& tr,
                           autd3::Drive d) -> autd3::Drive {
          d.intensity = autd3::EmitIntensity(d.intensity.value() / 2);
          d.phase = autd3::Phase::from_rad(d.phase.radian() + autd3::pi);
          return d;
        });