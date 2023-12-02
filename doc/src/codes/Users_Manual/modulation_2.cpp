#include "autd3.hpp"

autd.send(autd3::ConfigureModDelay([](const autd3::Device& dev,
                                      const autd3::Transducer& tr) {
  return dev.idx() == 0 && tr.idx() == 0 ? 1 : 0;
}));