# Frequency Configuration

Version 2.0から, すべての振動子の周波数を個別に指定できる機能が追加された.

従来の$\SI{40}{kHz}$固定のモードをLegacyモードと呼び, 周波数を可変にできるモードをNormalモードと呼ぶ.

デフォルトはLegacyモードになっており, Normalモードを使用する場合は, `Controller`の型引数に`NormalTransducer`を渡せば良い.

振動子の周波数は`Geometry`→`Device`→`Transducer`とアクセスし, `Transducer`の`set_frequency`, または, `set_cycle`関数で指定する.

指定できる周波数は$\SI{163.84}{MHz}/N, N=1,2,...,8191$となっている.
`set_cycle`ではこの$N$を指定する. 
`set_frequency`の場合は最も近い$N$が選ばれる.

なお, Normalモードの場合は, `Gain`, `STM`などのインスタンス化の際に`Controller`と同様に型引数を指定する必要がある.

```cpp
  autd3::Controller<autd3::NormalTransducer> autd;

  autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());

  for (auto& dev : autd.geometry())
    for (auto& tr : dev) tr.set_frequency(70e3); // actual frequency is 163.84MHz/2341 ~ 69987

...

  autd3::gain::Focus<autd3::NormalTransducer> g(center);
```
