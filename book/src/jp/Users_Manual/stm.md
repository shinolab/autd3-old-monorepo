# STM

`STM`はHardwareのタイマでSpatio-Temporal Modulationを実現する機能である.
SDKには単一焦点のみをサポートする`PointSTM`と任意の`Gain`をサポートする`GainSTM`が用意されている.

### PointSTM

`PointSTM`には以下の制約がある.
* 最大サンプリング点数は65536
* サンプリング周波数は$\SI{163.84}{MHz}/N$. ここで, $N$は32-bit符号なし整数であり, $1612$以上の値である必要がある.

`PointSTM`の使用方法は以下のようになる.
```cpp
  PointSTM stm;

  const Vector3 center = autd.geometry().center() + Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
  vector<size_t> points(points_num);
  iota(points.begin(), points.end(), 0);
  transform(points.begin(), points.end(), back_inserter(stm), [&](const size_t i) {
    const auto theta = 2.0 * pi * static_cast<double>(i) / static_cast<double>(points_num);
    return Point(center + Vector3(radius * cos(theta), radius * sin(theta), 0));
  });

  const auto actual_freq = stm.set_frequency(1);
  cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd.send(stm);
```

サンプリング点数とサンプリング周期に関する制約によって, 指定した周波数と実際の周波数は異なる可能性がある.
例えば, 上記の例は, 200点を$\SI{1}{Hz}$で回すため, サンプリング周波数は$\SI{200}{Hz}=\SI{163.84}{MHz}/819200$とすればよく, 制約を満たす.
しかし, `point_num`=199にすると, サンプリング周波数を$\SI{199}{Hz}$にしなければならないが, $\SI{199}{Hz}=\SI{163.84}{MHz}/N$を満たすような$N$は存在しない, そのため, 最も近い$N$が選択される.
これによって, 指定した周波数と実際の周波数がずれる.
`set_frequency`関数はこの実際の周波数を返してくる.

### GainSTM

`GainSTM`は`PointSTM`とは異なり, 任意の`Gain`を扱える.
ただし, 使用できる`Gain`の個数は

- Legacyモードの場合2048
- Normlaモードの場合1024

となる.

`GainSTM`の使用サンプルは`PointSTM`とほぼ同じである.
```cpp
  GainSTM stm(autd.geometry());

  const Vector3 center = autd.geometry().center() + Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
  vector<size_t> points(points_num);
  iota(points.begin(), points.end(), 0);
  for_each(points.begin(), points.end(), [&](const size_t i) {
    const auto theta = 2.0 * pi * static_cast<double>(i) / static_cast<double>(points_num);
    gain::Focus g(center + Vector3(radius * cos(theta), radius * sin(theta), 0.0));
    stm.add(g);
  });

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd.send(stm);
```
周波数の制約も`PointSTM`と同じである.

`GainSTM`は位相/振幅データをすべて送信するため, レイテンシが大きい[^fn_gain_seq].

この問題に対処するために, `GainSTM`には位相のみを送信して送信にかかる時間を半分にする`PhaseFull`モードと, 位相を$\SI{4}{bit}$に圧縮して送信時間を4分の1にする`PhaseHalf`モード[^phase_half]が用意されている.
モードの切り替えは`mode`関数で行う.

```cpp
stm.mode() = Mode::PhaseFull;
```

デフォルトはすべての情報を送る`PhaseDutyFull`モードである.

### STM common functions

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

# SoftwareSTM

`SoftwareSTM`はSoftwareのタイマでSpatio-Temporal Modulationを実現する機能である.
AUTD3ハードウェア上の制約はないが, その精度はホスト側のタイマの精度によって決まる[^timer_precision].

`SoftwareSTM`の使用方法は以下のようになる.
基本的な使い方は`GainSTM`と同様である.
```cpp
  autd3::SoftwareSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
  std::vector<size_t> points(points_num);
  std::iota(points.begin(), points.end(), 0);
  std::for_each(points.begin(), points.end(), [&](const size_t i) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    autd3::gain::Focus g(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0));
    stm.add(g);
  });

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";

  auto handle = stm.start(std::move(autd));

  std::cout << "press any key to stop software stm..." << std::endl;
  std::cin.ignore();

  autd = handle.finish();
```

[^fn_gain_seq]: `PointSTM`のおよそ60倍のレイテンシ.

[^phase_half]: Legacyモード限定.

[^timer_precision]: Windowsでは例えば, \SI{1}{ms}程度が限界である.
