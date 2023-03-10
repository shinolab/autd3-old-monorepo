# Modulation

`Modulation`はAM変調を制御するための仕組みである.
`Modulation`は, バッファに貯められた$\SI{8}{bit}$データから, 一定のサンプリングレートでデータを順番にサンプリングし, Duty比に掛け合わせることで実現されている.
現在, `Modulation`には以下の制約がある.

* バッファサイズは最大で65536
* サンプリングレートは$\SI{163.84}{MHz}/N$で, $N$は32-bit符号なし整数であり, $1160$以上の値である必要がある.
* Modulationは全デバイスで共通
* Modulationは自動でループする. 1ループだけ, 等の制御は不可能.
* Modulationの開始/終了タイミングは制御できない.

SDKにはデフォルトでいくつかの種類のAMを生成するための`Modulation`がデフォルトで用意されている.

[[_TOC_]]

## Static

変調なし.

```cpp
  autd3::modulation::Static m(amp);
```
第1引数は規格化された振幅である.
$\[0, 1\]$の範囲外の値は$\[0, 1\]$にクランプされる (すなわち, $0$未満の値は$0$に, $1$より大きい値は$1$になる).
第1引数を省略した場合は1となる.

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
ただし, 上記で$\[0, 1\]$を超えるような値は$\[0, 1\]$に収まるように変換される.
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
  autd3::modulation::Square m(f, low, high, duty); 
```
第1引数は周波数$f$, 第2引数はlow (デフォルトで0), 第3引数はhigh (デフォルトで1)になっており, 音圧の波形はlowとhighが周波数$f$で繰り返される.
また, 第4引数にduty比を指定できる.
duty比は$t_\text{high}/T = t_\text{high}f$で定義される, ここで, $t_\text{high}$は1周期$T=1/f$の内, highを出力する時間である.

## Cache

`Cache`は`Modulation`の計算結果をキャッシュしておくための`Modulation`である.
変調データ計算が重く, かつ, 複数回同じ`Modulation`を送信する場合に使用する.
また, 変調データ計算後に変調データを確認, 変更するために使うこともできる.

`Cache`を使用するには, 任意の`Modulation`型を型引数に指定し, コンストラクタに元の型のコンストラクタ引数を指定する.
```cpp
  autd3::modulation::Cache<autd3::modulation::Sine> m(...);
```

変調データには, `buffer`関数, または, インデクサでアクセスできる.
ただし, `calc`関数を事前に呼び出す必要がある.

```cpp
  autd3::modulation::Cache<autd3::modulation::Sine> m(...);
  m.calc();
  m[0] = 0;
```
上記の例では, 0番目の変調データを0にしている.

## Transform

`Transform`は`Modulation`の計算結果を改変する`Modulation`である.

`Transform`を使用するには, 任意の`Modulation`型を型引数に指定し, コンストラクタの第1引数に`double`を引数に`double`を返す変換関数を, 第2引数以降に元の型のコンストラクタ引数を指定する.
```cpp
  autd3::modulation::Transform<autd3::modulation::Sine> m([](const double v) {return std::clamp(v, 0.5, 1.0); }, 150);
```
例えば, 上記の例では, $\SI{150}{Hz}$のSin波を半波整流したような変調データになる.

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

## Modulationの共通API

### sampling_frequency_division

`sampling_frequency_division`でサンプリング周波数の分周比$N$の確認, 設定ができる.
サンプリング周波数の基本周波数は$\SI{163.84}{MHz}$である.
`sampling_frequency_division`は$1160$以上の整数が指定できる.

```cpp
    m.sampling_frequency_division = 20480; // 163.84MHz/20480 = 8kHz
```

### Sampling周波数

`sampling_frequency`でサンプリング周波数を取得できる.

また, `set_sampling_frequency`でサンプリング周波数を設定できる.
ただし, `Modulation`の制約上, 指定したサンプリング周波数になるとは限らない.

### size

`size`で変調データバッファの長さを取得できる.

## Modulation Delay

Modulationはすべての振動子に同時に作用し, 伝搬遅延を考慮しない.
そのため, 振動子と焦点位置との間の距離に応じて, 変調がずれる可能性がある.

これを補償するために, 振動子毎にサンプリングするインデックスを遅らせる機能が備わっている.

例えば, 以下のようにすると, $0$番目の振動子は他のすべての振動子に対して, サンプリングするインデックスが一つ遅れる.

```cpp
  autd.geometry()[0].mod_delay = 1;
  autd.send(autd3::ModDelayConfig());
```

サンプリングされるインデックスに対する遅れであるため, どの程度遅れるかは`Modulation`のサンプリング周波数に依存する.
`mod_delay`が$1$でサンプリング周波数が$\SI{40}{kHz}$の場合は$\SI{25}{\text{μ}s}$, $\SI{4}{kHz}$の場合は$\SI{250}{\text{μ}s}$の遅れになる.

また, `mod_delay`の値は変調データバッファの長さ未満でなくてはならない.
