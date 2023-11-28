#include "autd3.hpp"

autd3::ControllerBuilder()
    .add_device(autd3::AUTD3(autd3::Vector3::Zero()))
    .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0)))