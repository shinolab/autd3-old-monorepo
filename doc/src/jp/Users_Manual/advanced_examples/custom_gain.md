# Gainの自作

ライブラリでは自前の`Gain`を作成することができる.

ここでは, `Focus`と同じように単一焦点を生成する`FocalPoint`を実際に定義してみることにする.

```rust,edition2021
{{#include ../../../codes/Users_Manual/advanced_examples/custom_gain_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/advanced_examples/custom_gain_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/advanced_examples/custom_gain_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/advanced_examples/custom_gain_0.py}}
```

`send`関数は`Gain`型を継承したクラスを引数に取ることができる.
そのため, `Gain`型を継承をしておく.

`send`関数内部では`Geometry`を引数にした`calc`メソッドが呼ばれ, その返り値の振幅/位相データが使用される.
そのため, この`calc`メソッド内で位相/振幅の計算を行えば良い.

`calc`の返り値は, デバイスのインデックスをキー, そのデバイスの振幅位相データベクトルを値とする`HashMap`である.
