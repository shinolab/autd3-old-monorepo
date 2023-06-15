# FocusSTM

- 最大サンプリング点数は$65536$.
- サンプリング周波数は$\clklf/N$.

`FocusSTM`の使用方法は以下のようになる.
これは, アレイの中心から直上$\SI{150}{mm}$の点を中心とした半径$\SI{30}{mm}$の円周上で焦点を回すサンプルである.
円周上を200点サンプリングし, 一周を$\SI{1}{Hz}$で回るようにしている. (すなわち, サンプリング周波数は$\SI{200}{Hz}$である.)

```rust,should_panic
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
let point_num = 200;
let radius = 30.0 * MILLIMETER;
let stm = FocusSTM::new(1.0).add_foci_from_iter((0..point_num).map(|i| {
    let theta = 2.0 * PI * i as float / point_num as float;
    let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
    center + p
}));
autd.send(stm)?;
# Ok(())
# }
```

```cpp
const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
constexpr size_t points_num = 200;
constexpr auto radius = 30.0;
autd3::FocusSTM stm(1);
for (size_t i = 0; i < points_num; i++) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    stm.add_focus(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
}
autd.send(stm);
```

```cs
var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
const int pointNum = 200;
const double radius = 30.0;
var stm = new FocusSTM(1);
for (var i = 0; i < pointNum; i++)
{
    var theta = 2.0 * Math.PI * i / pointNum;
    var p = radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0);
    stm.AddFocus(center + p);
}
autd.Send(stm);
```

```python
from pyautd3.stm import FocusSTM

center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
size = 200
radius = 30.0
stm = FocusSTM(1.0)
for i in range(size):
    theta = 2.0 * np.pi * i / size
    p = radius * np.array([np.cos(theta), np.sin(theta), 0])
    stm.add_focus(center + p)
autd.send(stm)
```

`FocusSTM`のコンストラクタにはSTM周波数を指定する.
なお, サンプリング点数とサンプリング周期に関する制約によって, 指定した周波数と実際の周波数は異なる可能性がある.
例えば, 上記の例は200点を$\SI{1}{Hz}$で回すため, サンプリング周波数は$\SI{200}{Hz}=\clklf/102400$とすれば良い.
しかし, 例えば`point_num=199`にすると, サンプリング周波数を$\SI{199}{Hz}$にしなければならないが, $\SI{199}{Hz}=\clklf/N$を満たすような整数$N$は存在しない.
そのため, もっとも近い$N$が選択される.
これによって, 指定した周波数と実際の周波数がずれる.
`frequency`によって実際の周波数を確認することができる.
