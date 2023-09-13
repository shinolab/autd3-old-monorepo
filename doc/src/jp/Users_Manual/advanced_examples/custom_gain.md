# Gainの自作

ライブラリでは自前の`Gain`を作成することができる.

ここでは, `Focus`と同じように単一焦点を生成する`FocalPoint`を実際に定義してみることにする.

```rust,edition2021
# extern crate autd3;
use std::collections::HashMap;
use autd3::{
    driver::derive::prelude::*,
    derive::Gain,
    prelude::*,
};

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
public class FocalPoint : Gain
{
    private readonly Vector3d _point;

    public FocalPoint(Vector3d point)
    {
        _point = point;
    }

    public override Drive[] Calc(Geometry geometry)
    {
        var soundSpeed = geometry.SoundSpeed;
        return Transform(geometry, tr =>
        {
            var tp = tr.Position;
            var dist = (tp - _point).L2Norm;
            var phase = dist * tr.Wavenumber(soundSpeed);
            return new Drive{Phase = phase, Amp= 1.0};
        });
    }
}
```

```python
from pyautd3.gain import Gain, Drive

class FocalPoint(Gain):
    def __init__(self, point):
        self.point = np.array(point)

    def calc(self, geometry: Geometry):
        sound_speed = geometry.sound_speed
        return Gain.transform(
            geometry,
            lambda tr: Drive(
                np.linalg.norm(tr.position - self.point) * tr.wavenumber(sound_speed),
                1.0,
            ),
        )
```

`send`関数は`Gain`型を継承したクラスを引数に取ることができる.
そのため, `Gain`型を継承をしておく.

`send`関数内部では`Geometry`を引数にした`calc`メソッドが呼ばれ, その返り値の振幅/位相データが使用される.
そのため, この`calc`メソッド内で位相/振幅の計算を行えば良い.

`calc`の返り値は, デバイスのインデックスをキー, そのデバイスの振幅位相データベクトルを値とする`HashMap`である.
