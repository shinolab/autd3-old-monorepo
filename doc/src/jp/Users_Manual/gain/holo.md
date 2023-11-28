# Holo

`Holo`は多焦点を生成するための`Gain`である.
多焦点を生成するアルゴリズムが幾つか提案されており, SDKには以下のアルゴリズムが実装されている.

* `SDP` - Semidefinite programming, 井上らの論文[^inoue2015]に基づく
* `LSS` - Linear Synthesis Scheme 単一焦点解の重ね合わせ
* `GS` - Gershberg-Saxon, Marzoらの論文[^marzo2019]に基づく
* `GSPAT` - Gershberg-Saxon for Phased Arrays of Transducers, Plasenciaらの論文[^plasencia2020]に基づく
* `LM` - Levenberg-Marquardt, LM法はLevenberg[^levenberg1944]とMarquardt[^marquardt1963]で提案された非線形最小二乗問題の最適化法, 実装はMadsenのテキスト[^madsen2004]に基づく.
* `Greedy` - Greedy algorithm and Brute-force search, 鈴木らの論文[^suzuki2021]に基づく

また, 各手法は計算Backendを選べるようになっている.
SDKには以下の`Backend`が用意されている

* `NalgebraBackend` - [Nalgebra](hthttps://nalgebra.org/)を使用
* `CUDABackend` - CUDAを使用, GPUで実行
* `ArrayFireBackend` - [ArrayFire](https://arrayfire.com/)を使用

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/holo_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/holo_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/holo_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/holo_0.py}}
```

各アルゴリズムのコンストラクタの引数は`backend`である.
また, `add_focus`関数により各焦点の位置と音圧を指定する.

## 振幅制約

各アルゴリズムの計算結果の振幅は最終的に振動子が出力できる範囲に制限する必要がある.
これは`with_constraint`で制御でき, 以下の4つのいずれかを指定する必要がある.

- DontCare: 何もケアしない.
- Normalize: 振幅の最大値ですべての振動子の振幅を割り, 規格化する.
- Uniform: すべての振動子の振幅を指定した値にする.
- Clamp: 振幅を指定の範囲にクランプする.

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/holo_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/holo_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/holo_1.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/holo_1.py}}
```

## 最適化パラメータ

各アルゴリズムごとに追加のパラメータが存在する.
これらはすべて`with_xxx`で指定する.

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/holo_2.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/holo_2.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/holo_2.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/holo_2.py}}
```

各パラメータの詳細はそれぞれの論文を参照されたい.

[^inoue2015]: Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.

[^marzo2019]: Marzo, Asier, and Bruce W. Drinkwater. "Holographic acoustic tweezers." Proceedings of the National Academy of Sciences 116.1 (2019): 84-89.

[^plasencia2020]: Plasencia, Diego Martinez, et al. "GS-PAT: high-speed multi-point sound-fields for phased arrays of transducers." ACM Transactions on Graphics (TOG) 39.4 (2020): 138-1.

[^levenberg1944]: Levenberg, Kenneth. "A method for the solution of certain non-linear problems in least squares." Quarterly of applied mathematics 2.2 (1944): 164-168.

[^marquardt1963]: Marquardt, Donald W. "An algorithm for least-squares estimation of nonlinear parameters." Journal of the society for Industrial and Applied Mathematics 11.2 (1963): 431-441.

[^madsen2004]: Madsen, Kaj, Hans Bruun Nielsen, and Ole Tingleff. "Methods for non-linear least squares problems." (2004).

[^suzuki2021]: Suzuki, Shun, et al. "Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search." IEEE Transactions on Haptics (2021).
