# Custom Gain Tutorial

You can create your own `Gain`.

Here, we will define a `FocalPoint` that generates a single focus just like `Focus`.

```rust,edition2021
# extern crate autd3;
# extern crate autd3_driver;
use std::collections::HashMap;
use autd3::{
    derive::Gain,
    prelude::*,
};
use autd3_driver::derive::prelude::*;

#[derive(Gain, Clone, Copy)]
pub struct FocalPoint {
    position: Vector3,
}

impl FocalPoint {
    pub fn new(position: Vector3) -> Self {
        Self {position}
    }
}

impl<T: Transducer> Gain<T> for FocalPoint {
    fn calc(&self, geometry: &Geometry<T>, filter: GainFilter) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        Ok(Self::transform(geometry, filter, |dev, tr: &T| Drive {
            phase: (tr.position() - self.position).norm() * tr.wavelength(dev.sound_speed),
            amp: 1.0,
        }))
    }
}

# fn main() { 
# }
#
```

```cpp
#include "autd3.hpp"

class FocalPoint final : public autd3::Gain {
 public:
  explicit FocalPoint(autd3::Vector3 point) : _point(std::move(point)) {}

  std::unordered_map<size_t, std::vector<autd3::Drive>> calc(const autd3::Geometry& geometry) const override {
    return autd3::Gain::transform(geometry, [&](const auto& dev, const auto& tr) {
      const auto phase = (tr.position() - _point).norm() * tr.wavelength(dev.sound_speed());
      return autd3::Drive{phase, 1.0};
    });
  }

 private:
  autd3::Vector3 _point;
};
```

```cs
public class FocalPoint : Gain.Gain
{
    private readonly Vector3d _point;

    public FocalPoint(Vector3d point)
    {
        _point = point;
    }

    public override Dictionary<int, Drive[]> Calc(Geometry geometry)
    {
        return Transform(geometry, (dev, tr) =>
        {
            var tp = tr.Position;
            var dist = (tp - _point).L2Norm;
            var phase = dist * tr.Wavenumber(dev.SoundSpeed);
            return new Drive { Phase = phase, Amp = 1.0 };
        });
    }
}
```

```python
from pyautd3 import Geometry, Drive
from pyautd3.gain import Gain

class Focus(Gain):
    def __init__(self, point):
        self.point = np.array(point)

    def calc(self, geometry: Geometry) -> Dict[int, np.ndarray]:
        return Gain.transform(
            geometry,
            lambda dev, tr: Drive(
                np.linalg.norm(tr.position - self.point) * tr.wavenumber(dev.sound_speed),
                1.0,
            ),
        )
```
