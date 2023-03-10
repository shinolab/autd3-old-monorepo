# Modeの設定/周波数の変更

AUTD3のSDKでは, 超音波の周波数を$\ufreq$から変更できる.
従来の$\ufreq$固定のモードをLegacyモードと呼び, 周波数を可変にできるモードをAdvancedモードと呼ぶ.

デフォルトはLegacyモードになっており, Advancedモードを使用する場合は, 以下のようにする.

```cpp
  auto geometry = autd3::Geometry::Builder()
                      ...
                      .advanced_mode()
                      .build();
```

振動子の周波数は`Transducer`の`set_frequency`で指定するか, `cycle`を直接変更する.
`Transducer`には, `Geometry`のイテレータ, または, インデクサを経由してアクセスできる.
指定できる周波数は$\clkf/N,N=2,...,8191$となっている[^freq_range].
`cycle`はこの$N$を表している.
`set_frequency`の場合は可能な$N$の中でもっとも近い$N$が選ばれる.

周波数, または, 周期の変更は, `Synchronize`を送信する前に行う必要があることに注意する.

```cpp
  for (auto& tr : autd.geometry())
    tr.set_frequency(70e3); // 163.84MHz/2341 ~ 69987 Hz

  autd.send(autd3::Synchronize());
```

## AdvancedPhaseモード

Advancedモードは振幅/位相データをそれぞれ1フレームで送信する必要があるため, 通信のレイテンシがLegacyモードの2倍になる.
実際には振幅データは頻繁に更新されることはないと考えられるため, 位相データのみを送信するAdvancedPhaseモードが用意されている.

```cpp
  auto geometry = autd3::Geometry::Builder()
                      ...
                      .advanced_phase_mode()
                      .build();
```

このモードの場合, 振幅はあらかじめ`Amplitudes`クラスを送信することで制御する.
`Gain`の振幅パラメータはすべて無視される.
```cpp
  autd3::Amplitudes amp(1.0);
  autd.send(amp);
```

[^freq_range]: ただし, 当然ながら振動子の共振周波数は$\ufreq$であるため, ここから大きく異なる周波数を指定しても, 超音波はほとんど出力されない.
