#include "autd3.hpp"

auto g = autd3::gain::Plane(autd3::Vector3(nx, ny, nz))
             .with_intensity(autd3::EmitIntensity::maximum());