# Bessel

`Bessel`ではBessel beamを生成する.
この`Gain`は長谷川らの論文[^hasegawa2017]に基づく.

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/bessel_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/bessel_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/bessel_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/bessel_0.py}}
```

コンストラクタの第1引数はビームを生成する仮想円錐の頂点であり, 第2引数はビームの方向, 第3引数はビームに垂直な面とビームを生成する仮想円錐の側面となす角度である (下図の$\theta_z$).

<figure>
  <img src="../../fig/Users_Manual/1.4985159.figures.online.f1.jpg"/>
  <figcaption>Bessel beam (長谷川らの論文より引用)</figcaption>
</figure>

## 振幅の指定

`with_intensity`にて, 出力振幅を指定できる.

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/bessel_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/bessel_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/bessel_1.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/bessel_1.py}}
```

[^hasegawa2017]: Hasegawa, Keisuke, et al. "Electronically steerable ultrasound-driven long narrow air stream." Applied Physics Letters 111.6 (2017): 064104.
