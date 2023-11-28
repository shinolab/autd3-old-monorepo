# Spatio-Temporal Modulation/時空間変調

SDKでは, `Gain`を周期的に切り替えるための機能 (Spatio-Temporal Modulation, STM) が用意されている.
SDKには単一焦点のみをサポートする`FocusSTM`と, 任意の`Gain`をサポートする`GainSTM`が用意されている.
`FocusSTM`と`GainSTM`はAUTD3ハードウェア上のタイマを使用するので時間精度が高いが, 制約も多い.

- [FocusSTM](./stm/focus.md)
- [GainSTM](./stm/gain.md)

## FocusSTM/GainSTMの共通API

### frequency

STMの周波数を取得する.

### sampling_config

サンプリング設定を取得する.

### start_idx/finish_idx

`FocusSTM`/`GainSTM`は通常, 何番目の焦点/`Gain`からスタートするかは決められていない.
これを指定するには, 以下のように`with_start_idx`で指定する.

```rust,edition2021
{{#include ../../codes/Users_Manual/stm_0.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/stm_0.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/stm_0.cs}}
```

```python
{{#include ../../codes/Users_Manual/stm_0.py}}
```

これにより, `start_idx`で指定したインデックスの焦点/`Gain`からスタートするようになる.

また, 同様に, 何番目の焦点/`Gain`で終了するかは`finish_idx`で決定できる.

注意点として, `finish_idx`で指定したインデックスの焦点/`Gain`は最後に出力されない.
`finish_idx`の1つ前の焦点/`Gain`を出力したあと, 終了する.

`start_idx`と`finish_idx`は, 通常の`Gain`から`FocusSTM`/`GainSTM`への遷移, 及び, `FocusSTM`/`GainSTM`から通常の`Gain`への遷移の場合にのみ有効となる.

これらの設定を無効 (デフォルト) にするには, 以下のようにする.

```rust,edition2021
{{#include ../../codes/Users_Manual/stm_1.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/stm_1.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/stm_1.cs}}
```

```python
{{#include ../../codes/Users_Manual/stm_1.py}}
```
