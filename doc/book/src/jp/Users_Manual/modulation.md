# Modulation

`Modulation`はAM変調を制御するための仕組みである.
`Modulation`は, バッファに貯められた$\SI{8}{bit}$データから, 一定のサンプリングレートでデータを順番にサンプリングし, Duty比に掛け合わせることで実現されている.
現在, `Modulation`には以下の制約がある.

* バッファサイズは最大で65536
* サンプリングレートは$\SI{163.84}{MHz}/N$で, $N$は32-bit符号なし整数であり, $1160$以上の値である必要がある.
* Modulationは全デバイスで共通
* Modulationは自動でループする. 1ループだけ, 等の制御は不可能.

SDKにはデフォルトでいくつかの種類のAMを生成するための`Modulation`がデフォルトで用意されている.

[[_TOC_]]

## Static

変調なし.

```cpp
  autd3::modulation::Static m;
```

なお, 第1引数に0-1の規格化された振幅を引数に取れ, 超音波の出力を一律で変更するために使うことができる.

## Sine

音圧をSin波状に変形するための`Modulation`.
```cpp
  autd3::modulation::Sine m(f, amplitude, offset); 
```

第1引数は周波数$f$, 第2引数は$amplitude$ (デフォルトで1), 第3引数は$offset$ (デフォルトで0.5)になっており, 音圧の波形が
$$
    \frac{amplitude}{2} \times \sin(2\pi ft) + offset
$$
となるようなAMをかける.
ただし, 上記で$\[0,1\]$を超えるような値は$\[0,1\]$に収まるように変換される.
また, サンプリング周波数はデフォルトで$\SI{4}{kHz}$ ($N=40960$) になっている.

## SineSquared

放射圧, すなわち, 音圧の二乗をSin波状に変形するための`Modulation`.
引数等は`Sine`と同じ.

## SineLegacy

古いversionにあった`Sine Modulation`と互換.
周波数として, `double`の値を取れるが, 厳密に指定周波数になるのではなく, 出力可能な周波数の内, 最も近い周波数が選ばれる.
また, 音圧ではなくDuty比がSin波状になる.

## Square

矩形波状の`Modulation`.

```cpp
  autd3::modulation::Square m(f, low, high); 
```
第1引数は周波数$f$, 第2引数はlow (デフォルトで0), 第3引数はhigh (デフォルトで1)になっており, 音圧の波形はlowとhighが周波数$f$で繰り返される.
また, 第4引数にduty比を指定できる.
duty比は$t_\text{high}/T = t_\text{high}f$で定義される, ここで, $t_\text{high}$は1周期$T=1/f$の内, highを出力する時間である.

## Cache

`Modulation`は使い捨てであり, 複数回使用した場合は, 都度変調データの計算が行われる.
`Cache`は計算データをキャッシュしておくために使用できる.

```cpp
  autd3::modulation::Cache<autd3::modulation::Sine> m(150);
```

元の`Modulation`には`modulation`メンバでアクセスできる.

## Wav

`Wav`はWavファイルをもとに構成される`Modulation`である.

```cpp
  const std::filesystem::path path("sin150.wav");
  autd3::modulation::Wav m(path);
```

`Wav`を使用するには`BUILD_MODULATION_AUDIO_FILE`フラグをONにしてビルドするか, 或いは, 配布している`modulation_audio_file`ライブラリをリンクされたい.

## RawPCM

`RawPCM`は符号なし8-bitのバイナリデータファイルをもとに構成される`Modulation`である.

```cpp
  const std::filesystem::path path = std::filesystem::path("sin150.wav");
  autd3::modulation::RawPCM m(path, 4e3);
```

`RawPCM`を使用するには`BUILD_MODULATION_AUDIO_FILE`フラグをONにしてビルドするか, 或いは, 配布している`modulation_audio_file`ライブラリをリンクされたい.

## Modulationに共通の関数

### Sampling周波数分周比$N$

`sampling_freq_div_ratio`でサンプリング周波数の分周比$N$の確認, 設定ができる.
サンプリング周波数の基本周波数は$\SI{163.84}{MHz}$である.
`sampling_freq_div_ratio`は$1160$以上の整数が指定できる.

```cpp
    m.sampling_frequency_division() = 20480; // 163.84MHz/20480 = 8kHz
```

### Sampling周波数

`sampling_frequency`でサンプリング周波数を取得できる.

また, `set_sampling_frequency`でサンプリング周波数を設定できる.
ただし, `Modulation`の制約上, 指定したサンプリング周波数になるとは限らない.

## Modulation Delay

Modulationはすべての振動子に同時に作用し, 伝搬遅延を考慮しない.
そのため, 振動子と焦点位置との間の距離に応じて, 変調がずれる可能性がある.

これを補償するために, 振動子毎にサンプリングするインデックスを遅らせる機能が備わっている.

例えば, 以下のようにすると, $0$番目のデバイスの$17$番目の振動子は他のすべての振動子に対して, サンプリングするインデックスが一つ遅れる.

```cpp
  autd.geometry()[0][17].mod_delay() = 1;
  autd.send(autd3::mod_delay_config());
```

サンプリングされるインデックスに対する遅れであるため, どの程度遅れるかは`Modulation`のサンプリング周波数に依存する.
`mod_delay`が$1$でサンプリング周波数が$\SI{40}{kHz}$の場合は$\SI{25}{\text{μ}s}$, $\SI{4}{kHz}$の場合は$\SI{250}{\text{μ}s}$の遅れになる.

また, `mod_delay`の値は変調の長さ, すなわち, `buffer`サイズ未満でなくてはならない.
