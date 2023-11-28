# Transform

`Transform`は`Modulation`になんらかの後処理を適用するための機能である.

```rust,edition2021
{{#include ../../../codes/Users_Manual/modulation/transform_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/modulation/transform_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/modulation/transform_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/modulation/transform_0.py}}
```

`with_transform`の引数は`Fn(usize, EmitIntensity) -> EmitIntensity`であり, 第1引数はインデックス, 第2引数は変調データである.
