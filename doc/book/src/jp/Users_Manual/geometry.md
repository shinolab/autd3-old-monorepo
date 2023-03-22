# Geometry

この章ではGeometryについて解説する.
GeometryはAUTD3デバイスが現実世界でどのように配置されているかを管理している.

[[_TOC_]]

## 複数デバイスの接続

AUTD3のデバイスは複数台をデイジーチェーンで接続できるようになっている.
SDKは複数台を接続したとしても, 透過的に使用できるように設計されている.

複数のデバイスを接続する場合は,
PCと1台目のEtherCAT Inをケーブルでつなぎ, $i$台目のEtherCAT Outと$i+1$台目のEtherCAT Inをケーブルで接続する ([Concept](concept.md)参照).

なお, 電源も相互に接続でき, 電源コネクタは3つの内で好きなところを使って良い.

> NOTE: AUTD3は最大でデバイスあたり$\SI{2}{A}$の電流を消費する. 電源の最大出力電流に注意されたい.

SDKで複数台のデバイスを使用する場合は`add_device`関数を接続したデバイスの順に呼び出す必要がある.

<figure>
  <img src="../fig/Users_Manual/autd_hori.jpg"/>
  <figcaption>Horizontal alignment</figcaption>
</figure>

例えば, 上図のように配置・接続しており, 図左側のデバイスが1台目, 右側のデバイスが2台目だとする.
さらに, グローバル座標を1台目のローカル座標と同じようにとるとすると,

```cpp
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .build();
```

とすれば良い.
ここで, `autd3::AUTD3`コンストラクタの第1引数は位置, 第2引数は回転を表す.
回転はZYZのオイラー角, または, クオータニオンで指定する.
また, `AUTD3::DEVICE_WIDTH`はデバイスの (基板外形を含めた) 横幅である.
この例では, 回転はしないので, 第2引数はゼロで良い.

また, 例えば, グローバル座標を2台目のローカル座標と同じようにとるとすると,

```cpp
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3(-autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .build();
```

とすれば良い.

<figure>
  <img src="../fig/Users_Manual/autd_vert.jpg"/>
  <figcaption>Vertical alignment</figcaption>
</figure>

さらに, 例えば, 上図のように配置されており, 下が1台目, 左が2台目で, グローバル座標を1台目のローカル座標と同じだとすると,
```cpp
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(0, 0, autd3::AUTD3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi / 2.0, 0)))
                      .build();
```
のように指定する.

SDKにおけるAPIでは, すべてグローバル座標を用いるため, 接続するデバイスの数に依存せず透過的に使用できる.


## デバイス/振動子のインデックス

デバイスには接続された順に0から始まるインデックスが割り当てられる.

また, 各デバイスは$249$個の振動子が配置されており, ローカルインデックスが割り振られている ([コンセプト](./concept.md)の「AUTDの表面写真」を参照).
振動子のグローバルインデックスは
$$
  \text{グローバルインデックス} = \text{デバイスインデックス} \times 249 + \text{ローカルインデックス},
$$
となる.
例えば, 2台目デバイスの振動子のインデックスは$249$から$497$となる.

## GeometryのAPI

### 音速の設定

振動子の位相を計算する際に, 波長が必要な場面がある.
音波の波長$\lambda$は, 音速$v$と周波数$f$から$\lambda = v/f$と計算される.
`Geometry`の`sound_speed`メンバーがこの音速$v$を表している.
```cpp
  autd.geometry().sound_speed = 340e3;
```
音速の単位はmm/sである.

温度からも音速を設定できる.
これには, `set_sound_speed_from_temp`関数を使用する.
```cpp
  autd.geometry().set_sound_speed_from_temp(15);
```
温度の単位は摂氏である.

なお, デフォルトの音速は$340\times 10^{3}\,\mathrm{mm/s}$となっており, これは, およそ摂氏15度での空気の音速に相当する.

## 減衰係数の設定

SDKでは, 振動子から放射された超音波の位置$\br$における音圧$p(\br)$は
$$
  p(\br) = \frac{D(\theta)}{\|\br\|}\rme^{-\|\br\|\alpha}\rme^{-\im k \|\br\|}
$$
のようにモデル化されている.
ここで, $D(\theta)$は指向性, $k = 2\pi / \lambda$は波数であり, $\alpha$が減衰係数である.
`Geometry`の`attenuation`メンバーがこの減衰係数$\alpha$を表している.
```cpp
geometry().attenuation = 0.0;
```
単位はNp/mmである.

デフォルトでは, $0$に設定されている.

### center/center_of

`center`で全振動子の中心, `center_of`で特定のデバイス内の全振動子の中心を取得できる.

### num_devices/num_transducers

`num_devices`でデバイスの数, `num_transducers`で振動子の数を取得できる.

### デバイスの移動/回転

`Geometry`に追加したデバイスの位置関係を変更するには, 以下の関数を使用する.

- `translate`: 平行移動
- `rotate`: 回転
- `affine`: アフィン変換 (平行移動/回転)

第1引数にデバイスのインデックスを指定すると, そのデバイスのみの適用となり, インデックスを指定しない場合はすべてのデバイスに適用される.

### Transducerの取得

`Geometry`は`Transducer`のコンテナになっており, `Transducer`は各振動子の情報を格納している.

`Transducer`を取得するには, インデクサを使用する.
例えば, 0番目の振動子を取得するには以下のようにする.
```cpp
  const auto& tr = autd.geometry()[0];
```
あるいは, `begin/end`で, それぞれ先頭の振動子と末尾の次の振動子を指すイテレータを取得できる.
また, これらの関数にデバイスのインデックスを指定すると, それぞれそのデバイス内の先頭の振動子と末尾の次の振動子を指すイテレータを取得できる.

## TransducerのAPI

### idx

振動子のインデックスを取得する.

```cpp
  const auto tr_idx = autd.geometry()[0].idx();
```

### position/rotation

位置, 及び, 回転を取得する.
回転はクオータニオンで表される.

```cpp
  const auto pos = autd.geometry()[0].position();
  const auto rot = autd.geometry()[0].rotation();
```

### x_direction/y_direction/z_direction

振動子のx,y,z方向ベクトルを取得する.

```cpp
  const auto x_dir = autd.geometry()[0].x_direction();
  const auto y_dir = autd.geometry()[0].y_direction();
  const auto z_dir = autd.geometry()[0].z_direction();
```

### mod_delay

振動子のModulation delayを取得/設定する.
詳細は「[Modulation](./modulation.md)」を参照されたい.

```cpp
  autd.geometry()[0].mod_delay = 1;
```

### cycle

振動子の周期$N$を取得, 設定する.
周波数は周期$N$に対して, $\clkf/N$となる.

```cpp
  const auto tr_cycle = autd.geometry()[0].cycle;
  autd.geometry()[0].cycle = 4096;
```

デフォルトは$4096 (\ufreq)$ である.

詳細は「[Modeの設定/周波数の変更](./advanced_examples/freq_config.md)」を参照されたい.

### frequency/set_frequency

周波数を取得, 設定する.
周波数$f$を設定する場合, $\clkf/f$にもっとも近い周期$N$が選択される.

```cpp
  const auto tr_frequency = autd.geometry()[0].frequency();
  autd.geometry()[0].set_frequency(40e3);
```

デフォルトは$\ufreq$である.

詳細は「[Modeの設定/周波数の変更](./advanced_examples/freq_config.md)」を参照されたい.

### wavelength/wavenumber

波長, 及び, 波数を取得する.

引数に音速を渡す必要がある.

```cpp
  const auto sound_speed = autd.geometry().sound_speed;
  const auto tr_wavelength = autd.geometry()[0].wavelength(sound_speed);
  const auto tr_wavenumber = autd.geometry()[0].wavenumber(sound_speed);
```

### align_phase_at

ある位置における超音波の位相を揃えるための振動子の位相を計算する.

## GeometryViewer

`GeometryViewer`を使用すると, `Geometry`の位置関係が確認できる.

```cpp

#include "autd3/extra/geometry_viewer.hpp"

...

  autd3::extra::GeometryViewer().window_size(800, 600).vsync(true).view(autd.geometry());
```

`GeometryViewer`を使用するには, [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/)をインストールし, CMakeで`BUILD_GEOMETRY_VIEWER`オプションをOnにする必要がある
或いは, 配布している`autd3_model`及び, `geometry_viewer`ライブラリをリンクされたい.
