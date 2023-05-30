# Custom Gain Tutorial

You can create your own `Gain` by inheriting from the `Gain` class.
Here, we will define a `FocalPoint` that generates a single focus just like `Focus`.

```cpp
#include "autd3.hpp"

class FocalPoint final : public autd3::Gain {
 public:
  explicit FocalPoint(autd3::Vector3 point) : _point(std::move(point)) {}

  std::vector<autd3::driver::Drive> calc(const autd3::Geometry& geometry) override {
    std::vector<autd3::driver::Drive> drives;
    drives.reserve(geometry.num_transducers());
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(drives), [&](const auto& transducer) {
        const auto phase = transducer.align_phase_at(_point, geometry.sound_speed);
        return driver::Drive{phase, 1.0};
      });
    return drives;
  } 

 private:
  autd::Vector3 _point;
};
```

The `Gain::calc` method is called in the `Controller::send` function whose argument is `Geometry`.
In this function, you have to calculate and return the phase/amplitude data.
`Geometry` defines an iterator that return `Transducer`, from which the position of the transducer can be obtained.

In order to maximize the sound pressure of the emitted ultrasound from transducers at a certain point $\bp$, the phases at $\bp$ should be aligned.
This can be calculated by the function `align_phase_at` provided in the `Transducer` class.
