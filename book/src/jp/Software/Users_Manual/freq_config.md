# Frequency Configuration

Version 2.0から, すべての振動子の周波数を個別に指定できる機能が追加された.

従来の$\SI{40}{kHz}$固定のモードをLegacyモードと呼び, 周波数を可変にできるモードをNormalモードと呼ぶ.

デフォルトの`Controller`はLegacyモード専用になっており, Normalモードを使用する場合は, `ControllerX`クラスを使用し, その型引数に`NormalTransducer`を渡せば良い.

> NOTE: `Controller`は`ControllerX<LegacyTransducer>`のエイリアスである.

振動子の周波数は`Geometry`→`Device`→`Transducer`とアクセスし, `Transducer`の`set_frequency`, または, `set_cycle`関数で指定する.

指定できる周波数は$\SI{163.84}{MHz}/N, N=1,2,...,8191$となっている.
`set_cycle`ではこの$N$を指定する. 
`set_frequency`の場合は最も近い$N$が選ばれる.

周波数, または, 周期を変更したあとに`synchronize`関数を一度呼び出す必要があることに注意する.


```cpp
  autd3::ControllerX<autd3::NormalTransducer> autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());

  for (auto& dev : autd.geometry())
    for (auto& tr : dev) tr.set_frequency(70e3); // actual frequency is 163.84MHz/2341 ~ 69987
```

なお, Normalモードでは, `Gain`, `STM`などのインスタンス化の際に`ControllerX`と同様に型引数を指定する必要がある.

```cpp
  autd3::gain::Focus<autd3::NormalTransducer> g(center);
```
