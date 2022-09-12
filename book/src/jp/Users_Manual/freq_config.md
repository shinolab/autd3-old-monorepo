# Frequency Configuration

Version 2.0から, すべての振動子の周波数を個別に指定できる機能が追加された.

従来の$\SI{40}{kHz}$固定のモードをLegacyモードと呼び, 周波数を可変にできるモードをNormalモードと呼ぶ.

デフォルトはLegacyモードになっており, Normalモードを使用する場合は, `Geometry`の`mode`を`NormalMode`にすれば良い.

```cpp
autd.geometry().mode() = std::make_unique<autd3::NormalMode>();
```

振動子の周波数は`Geometry`→`Device`→`Transducer`とアクセスし, `Transducer`の`set_frequency`, または, `set_cycle`関数で指定する.

指定できる周波数は$\SI{163.84}{MHz}/N, N=1,2,...,8191$となっている.
`set_cycle`ではこの$N$を指定する. 
`set_frequency`の場合は最も近い$N$が選ばれる.

> NOTE: Legacyモードで周波数を変更しても, 実際の周波数は$\SI{40}{kHz}$から変化しない.

周波数, または, 周期を変更したあとに`synchronize`関数を一度呼び出す必要があることに注意する.

```cpp
  for (auto& dev : autd.geometry())
    for (auto& tr : dev) tr.set_frequency(70e3); // actual frequency is 163.84MHz/2341 ~ 69987 Hz
```

## NormalPhaseモード

Normalモードは振幅/位相データをそれぞれ1フレームで送信する必要があるため, 若干通信のレイテンシが大きい.
実際には振幅データは頻繁に更新されることはないと思われるため, 位相データのみを送信する`NormalPhase`モードも用意されている.

```cpp
autd.geometry().mode() = std::make_unique<autd3::NormalPhaseMode>();
```

このモードの場合, 振幅は予め`Amplitudes`クラスを送信することで制御する.

```cpp
  autd3::Amplitudes amp(1.0);
  autd.send(amp);
```
