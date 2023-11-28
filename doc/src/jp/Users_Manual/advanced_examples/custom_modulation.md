# Modulationの自作

`Modulation`も独自のものを作成することができる.
ここでは, 周期中のある一瞬だけ出力する`Burst`を作ってみる[^fn_burst].

以下が, この`Burst`のサンプルである.

```rust,edition2021
{{#include ../../../codes/Users_Manual/advanced_examples/custom_modulation_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/advanced_examples/custom_modulation_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/advanced_examples/custom_modulation_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/advanced_examples/custom_modulation_0.py}}
```

`Modulation`も`Gain`と同じく, `send`内部で`calc`メソッドが呼ばれ, その返り値の変調データが使用される.
したがって, この`calc`の中で, 変調データを計算すれば良い.

[^fn_burst]: SDKにはない.
