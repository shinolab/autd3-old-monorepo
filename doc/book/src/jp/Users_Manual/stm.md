# Spatio-Temporal Modulation/時空間変調

SDKでは, `Gain`を周期的に切り替えるための機能 (Spatio-Temporal Modulation, STM) が用意されている.
SDKには単一焦点のみをサポートする`FocusSTM`と, 任意の`Gain`をサポートする`GainSTM`と`SoftwareSTM`が用意されている.
`FocusSTM`と`GainSTM`はAUTD3ハードウェア上のタイマを使用するので時間精度が高いが, 制約も多い.
一方, `SoftwareSTM`はSDKを使用するPC上のタイマを使用するので時間精度は低いが制約が少ない.

[[_TOC_]]

## FocusSTM

- 最大サンプリング点数は$65536$.
- サンプリング周波数は$\clkf/N$. ここで, $N$は$\SI{32}{bit}$符号なし整数であり, $1612$以上の値である必要がある.

`FocusSTM`の使用方法は以下のようになる.
これは, アレイの中心から直上$\SI{150}{mm}$の点を中心とした半径$\SI{30}{mm}$の円周上で焦点を回すサンプルである.
円周上を200点サンプリングし, 一周を$\SI{1}{Hz}$で回るようにしている. (すなわち, サンプリング周波数は$\SI{200}{Hz}$である.)

```cpp
  autd3::FocusSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  for (size_t i = 0; i < points_num; i++) {
    constexpr auto radius = 30.0;
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    stm.add(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
  }

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd.send(stm);
```
`FocusSTM::add`の第1引数は焦点の位置であり, 第2引数は`duty_shift`である.
`duty_shift`により振幅の調整が行える.
駆動信号のデューティー比$D$は$D=\SI{50}{\%} >> \text{duty\_shift}$となり, 理論上, 超音波の振幅$p$は
$$
  p \propto \sin\left(D\pi\right),
$$
となる.

> Note: この関係式は理論上であり, 実際の振動子では異なることが知られている. しかし, 依然として, $D\in \[\SI{0}{\%}, \SI{50}{\%}\]$に対して単調増加であるのは変わらない.

したがって, $0$を指定すると最大の出力となり, `duty_shift`を大きくするごとに振幅は減る.
この引数を省略した場合は0になる.

サンプリング点数とサンプリング周期に関する制約によって, 指定した周波数と実際の周波数は異なる可能性がある.
例えば, 上記の例は200点を$\SI{1}{Hz}$で回すため, サンプリング周波数は$\SI{200}{Hz}=\clkf/819200$とすれば良い.
しかし, 例えば`point_num=199`にすると, サンプリング周波数を$\SI{199}{Hz}$にしなければならないが, $\SI{199}{Hz}=\clkf/N$を満たすような整数$N$は存在しない.
そのため, もっとも近い$N$が選択される.
これによって, 指定した周波数と実際の周波数がずれる.
`set_frequency`関数はこの実際の周波数を返してくる.

## GainSTM

`GainSTM`は`FocusSTM`とは異なり, 任意の`Gain`を扱える. ただし, 使用できる`Gain`の個数は
- Legacyモードの場合2048
- Normlaモードの場合1024

となる.
また, サンプリング周波数分周比$N$の最小値が

- Legacyモードの場合152
- Normlaモードの場合276

となる.

`GainSTM`の使用方法は`FocusSTM`とほとんど同じである.
```cpp
  autd3::GainSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  for (size_t i = 0; i < points_num; i++) {
    constexpr auto radius = 30.0;
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    autd3::gain::Focus g(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0));
    stm.add(g);
  }

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd.send(stm);
```

`GainSTM`は位相/振幅データをすべて送信するため, レイテンシが大きい[^fn_gain_seq].
この問題に対処するため, `GainSTM`には位相のみを送信して送信にかかる時間を半分にする`PhaseFull`モードと, 位相を4bitに圧縮して送信時間を4分の1にする`PhaseHalf`モード[^phase_half]が用意されている.

このモードの切り替えはコンストラクタで行う.

```cpp
  autd3::GainSTM stm(autd3::GainSTMMode::PhaseFull);
```
デフォルトはすべての情報を送る`PhaseDutyFull`モードである.

## FocusSTM/GainSTMの共通API

### frequency/set_frequency

STMの周波数を取得, 設定する.

### sampling_frequency

サンプリング周波数を取得する.

### sampling_frequency_division

サンプリング周波数の分周比を取得, 設定する.
```cpp
    stm.sampling_frequency_division = 20480; // 163.84MHz/20480 = 8kHz
```

### start_idx/finish_idx

`FocusSTM`/`GainSTM`は通常, 何番目の焦点/`Gain`からスタートするかは決められていない.
これを指定するには, 以下のように`start_idx`を指定する.

```cpp
  stm.start_idx = 0;
```

これにより, `start_idx`で指定したインデックスの焦点/`Gain`からスタートするようになる.

また, 同様に, 何番目の焦点/`Gain`で終了するかは`finish_idx`で決定できる.

```cpp
  stm.finish_idx = 0;
```

注意点として, `finish_idx`で指定したインデックスの焦点/`Gain`は最後に出力されない.
`finish_idx`の1つ前の焦点/`Gain`を出力したあと, 終了する.

`start_idx`と`finish_idx`は, 通常の`Gain`から`FocusSTM`/`GainSTM`への遷移, 及び, `FocusSTM`/`GainSTM`から通常の`Gain`への遷移の場合にのみ有効となる.

これらの設定を無効 (デフォルト) にするには, `std::nullopt`を指定する.

```cpp
  stm.start_idx = std::nullopt;
  stm.finish_idx = std::nullopt;
```

## SoftwareSTM

`SoftwareSTM`はソフトウェアのタイマでSpatio-Temporal Modulationを実現する機能である.
AUTD3ハードウェア上の制約はないが, その精度はホスト側のタイマの精度によって決まる[^timer_precision].

`SoftwareSTM`の使用方法は以下のようになる. 基本的な使い方は`GainSTM`と同様である.

```cpp
  autd3::SoftwareSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  for (size_t i = 0; i < points_num; i++) {
    constexpr auto radius = 30.0;
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    stm.add(autd3::gain::Focus(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0)));
  }

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";

  auto handle = stm.start(autd);

  std::cout << "press any key to stop software stm..." << std::endl;
  std::cin.ignore();

  handle.finish();
```

### Timer strategy

`SoftwareSTM`コンストラクタの引数に`TimerStrategy`を指定できる.

```cpp
  autd3::SoftwareSTM stm(autd3::TimerStrategy::Sleep);
```

* `Sleep`       : 標準ライブラリの`std::chrono::sleep_for`を用いる
* `BusyWait`    : ビジーウェイトを用いる. 高解像度だが, CPU負荷が高い.
* `NativeTimer` : OSのタイマー機能を用いる
  * Windowsではマルチメディアタイマー, linuxではPOSIXタイマー, macOSではGrand Central Dispatch Timer

デフォルトは`Sleep`である.

[^fn_gain_seq]: `FocusSTM`のおよそ60倍のレイテンシ

[^phase_half]: Legacyモード限定

[^timer_precision]: 例えば, Windowsでは$\SI{1}{ms}$程度が限界である
