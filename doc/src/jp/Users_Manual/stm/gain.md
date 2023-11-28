# GainSTM

`GainSTM`は`GainSTM`とは異なり, 任意の`Gain`を扱える. ただし, 使用できる`Gain`の個数は
- Legacyモードの場合2048
- Advanced/AdvancedPhaseモードの場合1024
となる.

`GainSTM`の使用方法は以下のようになる.
これは, アレイの中心から直上$\SI{150}{mm}$の点を中心とした半径$\SI{30}{mm}$の円周上で焦点を回すサンプルである.
円周上を200点サンプリングし, 一周を$\SI{1}{Hz}$で回るようにしている. (すなわち, サンプリング周波数は$\SI{200}{Hz}$である.)

```rust,edition2021
{{#include ../../../codes/Users_Manual/stm/gain_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/stm/gain_0.cpp}}

```

```cs
{{#include ../../../codes/Users_Manual/stm/gain_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/stm/gain_0.py}}
```

## サンプリング周波数の指定

周波数ではなく, サンプリング周波数を指定することもできる.

```rust,edition2021
{{#include ../../../codes/Users_Manual/stm/gain_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/stm/gain_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/stm/gain_1.cs}}
```

```python
{{#include ../../../codes/Users_Manual/stm/gain_1.py}}
```

## GainSTMMode

`GainSTM`は位相/振幅データをすべて送信するため, レイテンシが大きい[^fn_gain_seq].
この問題に対処するため, `GainSTM`には位相のみを送信して送信にかかる時間を半分にする`PhaseFull`モードと, 位相を4bitに圧縮して送信時間を4分の1にする`PhaseHalf`モード[^phase_half]が用意されている.

このモードの切り替えは`with_mode`で行う.

```rust,edition2021
{{#include ../../../codes/Users_Manual/stm/gain_2.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/stm/gain_2.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/stm/gain_2.cs}}
```

```python
{{#include ../../../codes/Users_Manual/stm/gain_2.py}}
```

デフォルトはすべての情報を送る`PhaseDutyFull`モードである.


[^fn_gain_seq]: `FocusSTM`のおよそ75倍のレイテンシ

[^phase_half]: Legacyモード限定
