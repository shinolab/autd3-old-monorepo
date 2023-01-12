# STM/時空間変調

`STM`はハードウェアのタイマでSpatio-Temporal Modulation (STM, 時空間変調) を実現する機能である.
SDKには単一焦点のみをサポートする`FocusSTM`と任意の`Gain`をサポートする`GainSTM`が用意されている.

### FocusSTM

`FocusSTM`には以下の制約がある.

* 最大サンプリング点数は65536
* サンプリング周波数は$\SI{163.84}{MHz}/N$. ここで, $N$は32-bit符号なし整数であり, $806$以上の値である必要がある.

`FocusSTM`の使用方法は以下のようになる.
これは, アレイの中心から直上$\SI{150}{mm}$の点を中心とした半径$\SI{30}{mm}$の円周上で焦点を回すサンプルである.
円周上を200点サンプリングし, 一周を$\SI{1}{Hz}$で回るようにしている.
(すなわち, サンプリング周波数は$\SI{200}{Hz}$である.)

```cpp
  autd3::FocusSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
  std::vector<size_t> points(points_num);
  std::iota(points.begin(), points.end(), 0);
  std::transform(points.begin(), points.end(), std::back_inserter(stm), [&](const size_t i) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    return autd3::Point(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
  });

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd << stm;
```

サンプリング点数とサンプリング周期に関する制約によって, 指定した周波数と実際の周波数は異なる可能性がある.
例えば, 上記の例は, 200点を$\SI{1}{Hz}$で回すため, サンプリング周波数は$\SI{200}{Hz}=\SI{163.84}{MHz}/819200$とすればよく, 制約を満たす.
しかし, `point_num`=199にすると, サンプリング周波数を$\SI{199}{Hz}$にしなければならないが, $\SI{199}{Hz}=\SI{163.84}{MHz}/N$を満たすような整数$N$は存在しない, そのため, 最も近い$N$が選択される.
これによって, 指定した周波数と実際の周波数がずれる.
`set_frequency`関数はこの実際の周波数を返してくる.

### GainSTM

`GainSTM`は`FocusSTM`とは異なり, 任意の`Gain`を扱える.
ただし, 使用できる`Gain`の個数は

- Legacyモードの場合2048
- Normlaモードの場合1024

となる.
また, サンプリング周波数分周比$N$の最小値が

- Legacyモードの場合76
- Normlaモードの場合138

となっている.

`GainSTM`の使用サンプルは`FocusSTM`とほぼ同じである.

```cpp
  autd3::GainSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 50;
  for (size_t i = 0; i < points_num; i++) {
    constexpr auto radius = 30.0;
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    autd3::gain::Focus g(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0));
    stm.add(g);
  }

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd << stm;
```

周波数の制約も`FocusSTM`と同じである.

`GainSTM`は位相/振幅データをすべて送信するため, レイテンシが大きい[^fn_gain_seq].

この問題に対処するために, `GainSTM`には位相のみを送信して送信にかかる時間を半分にする`PhaseFull`モードと, 位相を$\SI{4}{bit}$に圧縮して送信時間を4分の1にする`PhaseHalf`モード[^phase_half]が用意されている.
モードの切り替えは`mode`関数で行う.

```cpp
stm.mode() = autd3::GainSTMMode::PhaseFull;
```

デフォルトはすべての情報を送る`PhaseDutyFull`モードである.

### STMに共通の関数

#### frequency

`STM`の周波数を取得する.

#### sampling_frequency

`STM`のサンプリング周波数を取得する.

#### sampling_frequency_division

`STM`のサンプリング周波数の分周比を取得, 設定する.
サンプリング周波数の基本周波数は$\SI{163.84}{MHz}$である.

```cpp
    stm.sampling_frequency_division() = 20480; // 163.84MHz/20480 = 8kHz
```

#### start_idx/finish_idx

`Focus/GainSTM`は通常, 何番目の焦点/`Gain`からスタートするかは決められていない.
これを指定するには, 以下のように`start_idx`を指定する.

```cpp
  stm.start_idx() = 0;
```

これにより, `start_idx`で指定したインデックスの焦点/`Gain`からスタートするようになる.

また, 同様に, 何番目の焦点/`Gain`で終了するかは`finish_idx`で決定できる.

```cpp
  stm.finish_idx() = 0;
```

注意点として, `finish_idx`で指定したインデックスの焦点/`Gain`は最後に出力されない.
`finish_idx`の一つ前の焦点/`Gain`を出力したあと, 終了する.

また, 以上の2つの設定は, 通常の`Gain`→`Focus/GainSTM`への遷移, 及び, `Focus/GainSTM`→通常の`Gain`への遷移の場合にのみ有効となる.

これらの設定を無効 (デフォルト) にするには, `std::nullopt`を指定する.

```cpp
  stm.start_idx() = std::nullopt;
  stm.finish_idx() = std::nullopt;
```

# SoftwareSTM

`SoftwareSTM`はソフトウェアのタイマでSpatio-Temporal Modulationを実現する機能である.
AUTD3ハードウェア上の制約はないが, その精度はホスト側のタイマの精度によって決まる[^timer_precision].

`SoftwareSTM`の使用方法は以下のようになる.
基本的な使い方は`GainSTM`と同様である.

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

[^fn_gain_seq]: `FocusSTM`のおよそ60倍のレイテンシ

[^phase_half]: Legacyモード限定

[^timer_precision]: 例えば, Windowsでは$\SI{1}{ms}$程度が限界である
