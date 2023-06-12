# Bessel

`Bessel`ではBessel beamを生成する.
この`Gain`は長谷川らの論文[^hasegawa2017]に基づく.

```rust
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
# let x = 0.;
# let y = 0.;
# let z = 0.;
# let nx = 0.;
# let ny = 0.;
# let nz = 0.;
# let theta = 0.;
let g = autd3::gain::Bessel::new(Vector3::new(x, y, z), Vector3::new(nx, ny, nz), theta);
# }
```

```cpp
const auto g = autd3::gain::Bessel(autd3::Vector3(x, y, z), autd3::Vector3(nx, ny, nz), theta);
```

```cs
var g = new Bessel(new Vector3d(x, y, z), new Vector3d(nx, ny, nz), theta);
```

```python
from pyautd3.gain import Bessel

g = Bessel([x, y, z], [nx, ny, nz], theta)
```

コンストラクタの第1引数はビームを生成する仮想円錐の頂点であり, 第2引数はビームの方向, 第3引数はビームに垂直な面とビームを生成する仮想円錐の側面となす角度である (下図の$\theta_z$).

<figure>
  <img src="../../fig/Users_Manual/1.4985159.figures.online.f1.jpg"/>
  <figcaption>Bessel beam (長谷川らの論文より引用)</figcaption>
</figure>

## 振幅の指定

`with_amp`にて, 0-1の規格化された音圧振幅を指定できる.
$\[0, 1\]$の範囲外の値は$\[0, 1\]$にクランプされる (すなわち, $0$未満の値は$0$に, $1$より大きい値は$1$になる).

```rust
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
# let x = 0.;
# let y = 0.;
# let z = 0.;
# let nx = 0.;
# let ny = 0.;
# let nz = 0.;
# let theta = 0.;
let g = autd3::gain::Bessel::new(Vector3::new(x, y, z), Vector3::new(nx, ny, nz), theta)
            .with_amp(1.);
# }
```

```cpp
const auto g = autd3::gain::Bessel(autd3::Vector3(x, y, z), theta)
                .with_amp(1.);
```

```cs
var g = new Bessel(new Vector3d(x, y, z), new Vector3d(nx, ny, nz), theta)
            .WithAmp(1);
```

```python
g = Bessel([x, y, z], [nx, ny, nz], theta).with_amp(1)
```

[^hasegawa2017]: Hasegawa, Keisuke, et al. "Electronically steerable ultrasound-driven long narrow air stream." Applied Physics Letters 111.6 (2017): 064104.
