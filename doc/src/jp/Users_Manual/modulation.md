# Modulation

`Modulation`はAM変調を制御するための仕組みである.
`Modulation`は, バッファに貯められた$\SI{8}{bit}$データから, 一定のサンプリングレートでデータを順番にサンプリングし, Duty比に掛け合わせることで実現されている.
現在, `Modulation`には以下の制約がある.

* バッファサイズは最大で65536
* サンプリングレートは$\clklf/N$で, $N$は32-bit符号なし整数であり, $512$以上の値である必要がある.
* Modulationは全デバイスで共通
* Modulationは自動でループする. 1ループだけ, 等の制御は不可能.
* Modulationの開始/終了タイミングは制御できない.

SDKにはデフォルトでいくつかの種類のAMを生成するための`Modulation`が用意されている.

* [Static](./modulation/static.md)
* [Sine](./modulation/sine.md)
* [SineLegacy](./modulation/sine_legacy.md)
* [Square](./modulation/square.md)
* [Wav](./modulation/wav.md)
* [RawPCM](./modulation/rawpcm.md)
* [Cache](./modulation/cache.md)

また, これらを加工するための機能も用意されている.

* [FIR](./modulation/fir.md)
* [Transform](./modulation/transform.md)

## Modulationの共通API

### Sampling周波数

`sampling_frequency`でサンプリング周波数を取得できる.

```rust
# use autd3::prelude::*;
# use autd3::core::modulation::ModulationProperty;
# #[allow(unused_variables)]
# fn main()  {
# let m = autd3::modulation::SineLegacy::new(150.);
let fs = m.sampling_frequency();
# }
```

```cpp
const auto fs = m.sampling_frequency();
```

```cs
var fs = m.SamplingFrequency;
```

```python
fs = m.sampling_frequency
```

また, 一部の`Modulation`は`with_sampling_frequency`でサンプリング周波数を設定できる.
ただし, `Modulation`の制約上, 必ずしも指定したサンプリング周波数になるとは限らない.

- e.g.,
  ```rust
  # use autd3::prelude::*;
  # #[allow(unused_variables)]
  # fn main()  {
  let m = autd3::modulation::Sine::new(150).with_sampling_frequency(4e3);
  # }
  ```

  ```cpp
  const auto m = autd3::modulation::Sine(150).with_sampling_frequency(4e3);
  ```

  ```cs
  var m = new Sine(150).WithSamplingFrequency(4e3);
  ```

  ```python
  m = Sine(150).with_sampling_frequency(4e3)
  ```

### Sampling周波数分周比

`sampling_frequency_division`でサンプリング周波数の分周比$N$を取得できる.

サンプリング周波数の基本周波数は$\clklf$である.

```rust
# use autd3::prelude::*;
# use autd3::core::modulation::ModulationProperty;
# #[allow(unused_variables)]
# fn main()  {
# let m = autd3::modulation::SineLegacy::new(150.);
let div = m.sampling_frequency_division();
# }
```

```cpp
const auto fs = m.sampling_frequency_division();
```

```cs
var fs = m.SamplingFrequencyDivision;
```

```python
fs = m.sampling_frequency_division
```

また, 一部の`Modulation`は`with_sampling_frequency_division`でサンプリング周波数分周比を設定できる.

- e.g.,
  ```rust
  # use autd3::prelude::*;
  # #[allow(unused_variables)]
  # fn main()  {
  let m = autd3::modulation::Sine::new(150).with_sampling_frequency_division(5120);
  # }
  ```

  ```cpp
  const auto m = autd3::modulation::Sine(150).with_sampling_frequency_division(5120);
  ```

  ```cs
  var m = new Sine(150).WithSamplingFrequencyDivision(5120);
  ```

  ```python
  m = Sine(150).with_sampling_frequency_division(5120)
  ```

## Modulation Delay

Modulationはすべての振動子に同時に作用し, 伝搬遅延を考慮しない.
そのため, 振動子と焦点位置との間の距離に応じて, 変調がずれる可能性がある.

これを補償するために, 振動子毎にサンプリングするインデックスを遅らせる機能が備わっている.

例えば, 以下のようにすると, $0$番目の振動子は他のすべての振動子に対して, サンプリングするインデックスが一つ遅れる.

```rust,should_panic
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
autd.geometry_mut()[0].set_mod_delay(1);
autd.send(ModDelay::new())?;
# Ok(())
# }
```

```cpp
autd.geometry()[0].set_mod_delay(1);
autd.send(autd3::ModDelayConfig());
```

```cs
autd.Geometry[0].ModDelay = 1;
autd.Send(new ModDelayConfig());
```

```python
from pyautd3 import ModDelayConfig

autd.geometry[0].mod_delay = 1
autd.send(ModDelayConfig())
```

サンプリングされるインデックスに対する遅れであるため, どの程度遅れるかは`Modulation`のサンプリング周波数に依存する.
`mod_delay`が$1$でサンプリング周波数が$\SI{40}{kHz}$の場合は$\SI{25}{\text{μ}s}$, $\SI{4}{kHz}$の場合は$\SI{250}{\text{μ}s}$の遅れになる.
