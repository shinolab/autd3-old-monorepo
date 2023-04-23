#include <iostream>

#include "autd3.hpp"
#include "autd3/link/soem.hpp"

int main() try {
  // geometry contains information about where devices are placed
  auto geometry = autd3::Geometry::Builder()
                      // The first argument is the position, the second is the rotation.
                      // The position is the origin of the device in the global coordinate system you set.
                      // The rotation is specified in ZYZ Euler angles or quaternions.
                      // Here, neither rotation nor translation is assumed.
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .build();

  // create SOEM link
  auto link = autd3::link::SOEM().build();

  // create and open controller
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  // initialize and synchronize devices
  // You MUST synchronize devices once after initialization, even if you are using only one device.
  autd.send(autd3::Clear());
  autd.send(autd3::Synchronize());

  // check firmware version
  const auto firm_infos = autd.firmware_infos();
  std::copy(firm_infos.begin(), firm_infos.end(), std::ostream_iterator<autd3::FirmwareInfo>(std::cout, "\n"));

  // Silencer is used to quiet down the transducers' noise by passing the phase/amplitude parameters through a low-pass filter.
  autd3::SilencerConfig silencer;
  autd.send(silencer);

  // focus is 150.0 mm above array center
  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(focus);

  // Amplitude Modulation of 150 Hz sine wave
  autd3::modulation::Sine m(150);

  // send data
  autd.send(m, g);

  std::cout << "press enter to finish..." << std::endl;
  std::cin.ignore();

  // close controller
  autd.close();

  return 0;
} catch (std::exception& ex) {
  std::cerr << ex.what() << std::endl;
}