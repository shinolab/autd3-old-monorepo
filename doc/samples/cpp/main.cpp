#include <iostream>

#include "autd3.hpp"
#include "autd3/link/soem.hpp"

void on_lost(const char* msg) {
  std::cerr << "Link is lost\n";
  std::cerr << msg;
  exit(-1);
}

int main() try {
  // create and open controller
  auto autd =
      autd3::ControllerBuilder()
          // The  argument is the position.
          // The position is the origin of the device in the global coordinate
          // system you set.
          .add_device(autd3::AUTD3(autd3::Vector3::Zero()))
          .open_with_async(autd3::link::SOEM::builder().with_on_lost(&on_lost))
          .get();

  // check firmware version
  const auto firm_infos = autd.firmware_infos_async().get();
  std::copy(firm_infos.begin(), firm_infos.end(),
            std::ostream_iterator<autd3::FirmwareInfo>(std::cout, "\n"));

  // Silencer is used to quiet down the transducers' noise by passing the
  // phase/amplitude parameters through a low-pass filter.
  autd3::Silencer silencer;
  autd.send_async(silencer).get();

  // focus is 150.0 mm above array center
  const autd3::Vector3 focus =
      autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(focus);

  // Amplitude Modulation of 150 Hz sine wave
  autd3::modulation::Sine m(150);

  // send data
  autd.send_async(m, g).get();

  std::cout << "press enter to finish..." << std::endl;
  std::cin.ignore();

  // close controller
  (void)autd.close_async().get();

  return 0;
} catch (std::exception& ex) {
  std::cerr << ex.what() << std::endl;
}