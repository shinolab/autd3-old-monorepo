# GainSTM

`GainSTM`は`FocusSTM`とは異なり, 任意の`Gain`を扱える. ただし, 使用できる`Gain`の個数は
- Legacyモードの場合2048
- Advanced/AdvancedPhaseモードの場合1024
となる.

`GainSTM`の使用方法は以下のようになる.
これは, アレイの中心から直上$\SI{150}{mm}$の点を中心とした半径$\SI{30}{mm}$の円周上で焦点を回すサンプルである.
円周上を200点サンプリングし, 一周を$\SI{1}{Hz}$で回るようにしている. (すなわち, サンプリング周波数は$\SI{200}{Hz}$である.)

```rust
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
let point_num = 200;
let radius = 30.0 * MILLIMETER;
let stm = GainSTM::new(1.0).add_gains_from_iter((0..point_num).map(|i| {
    let theta = 2.0 * PI * i as float / point_num as float;
    let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
    let g = Focus::new(center + p);
    Box::new(g) as _
}));
autd.send(stm)?;
# Ok(())
# }
```

```cpp
const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
constexpr size_t points_num = 200;
constexpr auto radius = 30.0;
autd3::GainSTM stm(1);
for (size_t i = 0; i < points_num; i++) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    stm.add_gain(autd3::gain::Focus(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0)));
}
autd.send(stm);
```

```cs
var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
const int pointNum = 200;
const double radius = 30.0;
var stm = new GainSTM(1);
for (var i = 0; i < pointNum; i++)
{
    var theta = 2.0 * Math.PI * i / pointNum;
    var p = radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0);
    stm.AddGain(new Focus(center + p));
}
autd.Send(stm);
```

```python
from pyautd3.stm import GainSTM

center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
size = 200
radius = 30.0
stm = GainSTM(1.0)
for i in range(size):
    theta = 2.0 * np.pi * i / size
    p = radius * np.array([np.cos(theta), np.sin(theta), 0])
    stm.add_gain(Focus(center + p))
autd.send(stm)
```

## GainSTMMode

`GainSTM`は位相/振幅データをすべて送信するため, レイテンシが大きい[^fn_gain_seq].
この問題に対処するため, `GainSTM`には位相のみを送信して送信にかかる時間を半分にする`PhaseFull`モードと, 位相を4bitに圧縮して送信時間を4分の1にする`PhaseHalf`モード[^phase_half]が用意されている.

このモードの切り替えは`with_mode`で行う.

```rust,should_panic
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
let stm = GainSTM::new(1.0).with_mode(GainSTMMode::PhaseFull);
# autd.send(stm)?;
# Ok(())
# }
```

```cpp
auto stm = autd3::GainSTM(1).with_mode(autd3::GainSTMMode::PhaseFull);
```

```cs
var stm = new GainSTM(1).WithMode(GainSTMMode.PhaseFull);
```

```python
from pyautd3.stm import GainSTM, GainSTMMode

stm = GainSTM(1).with_mode(GainSTMMode.PhaseFull)
```

デフォルトはすべての情報を送る`PhaseDutyFull`モードである.


[^fn_gain_seq]: `FocusSTM`のおよそ60倍のレイテンシ

[^phase_half]: Legacyモード限定
