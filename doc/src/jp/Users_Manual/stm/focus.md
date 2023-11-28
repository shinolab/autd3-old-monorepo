# FocusSTM

- 最大サンプリング点数は$65536$.
- サンプリング周波数は$\clkf/N$.

`FocusSTM`の使用方法は以下のようになる.
これは, アレイの中心から直上$\SI{150}{mm}$の点を中心とした半径$\SI{30}{mm}$の円周上で焦点を回すサンプルである.
円周上を200点サンプリングし, 一周を$\SI{1}{Hz}$で回るようにしている. (すなわち, サンプリング周波数は$\SI{200}{Hz}$である.)

```rust,edition2021
{{#include ../../../codes/Users_Manual/stm/focus_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/stm/focus_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/stm/focus_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/stm/focus_0.py}}
```

`FocusSTM`のコンストラクタにはSTM周波数を指定する.
なお, サンプリング点数とサンプリング周期に関する制約によって, 指定した周波数と実際の周波数は異なる可能性がある.
例えば, 上記の例は200点を$\SI{1}{Hz}$で回すため, サンプリング周波数は$\SI{200}{Hz}=\clkf/102400$とすれば良い.
しかし, 例えば`point_num=199`にすると, サンプリング周波数を$\SI{199}{Hz}$にしなければならないが, $\SI{199}{Hz}=\clkf/N$を満たすような整数$N$は存在しない.
そのため, もっとも近い$N$が選択される.
これによって, 指定した周波数と実際の周波数がずれる.
`frequency`によって実際の周波数を確認することができる.

## サンプリング設定の指定

周波数ではなく, サンプリング周波数等を指定することもできる.

```rust,edition2021
{{#include ../../../codes/Users_Manual/stm/focus_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/stm/focus_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/stm/focus_1.cs}}
```

```python
{{#include ../../../codes/Users_Manual/stm/focus_1.py}}
```
