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
  auto link = autd3::link::SOEM().high_precision(true).build();

  // create and open controller
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  // You do not need to change this value, but setting it increases reliability.
  // For SOEM link, it is recommended to set `ack_check_timeout` to about 10 ms
  autd.set_ack_check_timeout(std::chrono::milliseconds(20));

  // initialize and synchronize devices
  // You MUST synchronize devices once after initialization, even if you are using only one device.
  autd << autd3::clear << autd3::synchronize;

  // check firmware version
  const auto firm_infos = autd.firmware_infos();
  std::copy(firm_infos.begin(), firm_infos.end(), std::ostream_iterator<autd3::FirmwareInfo>(std::cout, "\n"));

  // Silencer is used to quiet down the transducers' noise by passing the phase/amplitude parameters through a low-pass filter.
  autd3::SilencerConfig silencer;

  // focus is 150.0 mm above array center
  const autd3::Vector3 focus = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(focus);
  autd3::modulation::Sine m(150);  // Amplitude Modulation of 150 Hz sine wave

  // send data
  autd << silencer << m, g;

  std::cout << "press enter to finish..." << std::endl;
  std::cin.ignore();

  // close controller
  autd.close();

  return 0;
} catch (std::exception& ex) {
  std::cerr << ex.what() << std::endl;
}