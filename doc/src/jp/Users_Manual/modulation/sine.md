# Sine

音圧をSin波状に変形するための`Modulation`.

コンストラクタには周波数$f$を整数で指定する.

```rust,edition2021
{{#include ../../../codes/Users_Manual/modulation/sine_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/modulation/sine_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/modulation/sine_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/modulation/sine_0.py}}
```

## 振幅とオフセットの指定

`Sine`は音圧の波形が
$$
    \frac{amplitude}{2} \times \sin(2\pi ft) + offset
$$
となるようなAMをかける.
ここで, $amplitude$と$offset$はそれぞれ, `with_amp`と`with_offset`にて指定できる (デフォルトはそれぞれ$1$, $0.5$).

ただし, 上記で$\[0, 1\]$を超えるような値は$\[0, 1\]$に収まるように変換される.

```rust,edition2021
{{#include ../../../codes/Users_Manual/modulation/sine_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/modulation/sine_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/modulation/sine_1.cs}}
```

```python
{{#include ../../../codes/Users_Manual/modulation/sine_1.py}}
```