# Modeの設定/周波数の変更

AUTD3のSDKでは, 超音波の周波数を$\ufreq$から変更できる.
従来の$\ufreq$固定のモードをLegacyモードと呼び, 周波数を可変にできるモードをAdvancedモードと呼ぶ.

デフォルトはLegacyモードになっており, Advancedモードを使用する場合は, 以下のようにする.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
                  .advanced()
#        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#        .open_with(Debug::new())?;
#
#    Ok(())
# }
```
```cpp
auto autd = autd3::Controller::builder()
               .advanced()
```
```cs
var autd = Controller.Builder()
        .Advanced()
```
```python
autd = Controller.builder().advanced()
```

振動子の周波数は`Transducer`の`set_frequency`で指定するか, `cycle`を直接変更する.
指定できる周波数は$\clkf/N,N=2,...,8191$となっている[^freq_range].
`cycle`はこの$N$を表している.
`set_frequency`の場合は可能な$N$の中でもっとも近い$N$が選ばれる.

周波数, または, 周期の変更を行った後, `Synchronize`を送信する必要があることに注意する.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#                  .advanced()
#        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#        .open_with(Debug::new())?;
for dev in autd.geometry_mut().iter_mut() {
    for tr in dev.iter_mut() {
        tr.set_frequency(70e3)?;
    }
}

autd.send(Synchronize::new())?;
#    Ok(())
# }
```
```cpp
for (auto& dev : autd.geometry())
    for (auto& tr : dev)
        tr.set_frequency(70e3);

autd.send(autd3::Synchronize());
```
```cs
foreach (var dev in autd.Geometry)
    foreach (var tr in dev)
        tr.Frequency = 70e3;

autd.Send(new Synchronize());
```
```python
from pyautd3 import Synchronize

for dev in autd.geometry:
    for tr in dev:
        tr.frequency = 70e3

autd.send(Synchronize())
```

## AdvancedPhaseモード

Advancedモードは振幅/位相データをそれぞれ1フレームで送信する必要があるため, 通信のレイテンシがLegacyモードの2倍になる.
実際には振幅データは頻繁に更新されることはないと考えられるため, 位相データのみを送信するAdvancedPhaseモードが用意されている.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
                  .advanced_phase()
#        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#        .open_with(Debug::new())?;
#
#    Ok(())
# }
```
```cpp
auto autd = autd3::Controller::builder()
               .advanced_phase()
```
```cs
var autd = Controller.Builder()
        .AdvancedPhase()
```
```python
autd = Controller.builder().advanced_phase()
```

このモードの場合, 振幅はあらかじめ`Amplitudes`クラスを送信することで制御する.
`Gain`の振幅パラメータはすべて無視される.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#                  .advanced_phase()
#        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#        .open_with(Debug::new())?;
let amp = Amplitudes::uniform(1.);
autd.send(amp)?;
#    Ok(())
# }
```
```cpp
const auto amp = autd3::Amplitudes(1.0);
autd.send(amp);
```
```cs
var amp = new Amplitudes(1.0);
autd.Send(amp);
```
```python
from pyautd3 import Amplitudes

amp = Amplitudes(1.)
autd.send(amp)
```

[^freq_range]: ただし, 当然ながら振動子の共振周波数は$\ufreq$であるため, ここから大きく異なる周波数を指定しても, 超音波はほとんど出力されない.
