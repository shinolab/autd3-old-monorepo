# Getting Started

ここでは, 実際にAUTD3を動かす手順について述べる. なお, 本章ではOSとしてWindows 11 64bitを使用する.
他のOSの場合は適宜読み替えられたい.

## Install

まず, 必要なツールをインストールする. 本節で使用するツールとversionは以下の通りである. 各々公式の手順に従ってインストールすること. Visual
Studio Community 2022は「C++によるデスクトップ開発」にチェックを入れてインストールすれば良い. なお, Linuxの場合はgccが,
macOSの場合はclangが使えれば良い. また, 以下はターミナルから操作するため, PATHを通しておくことを推奨する.

- Visual Studio Community 2022 17.3.5
- CMake 3.24.2
- git 2.38.0.windows.1[^fn_git]
- npcap 1.71[^fn_npcap]

## Setup Device

次にデバイスをセットアップする. ここでは一台のAUTD3のみを使うこととする. PCのイーサネットポートとAUTD3デバイスのEtherCAT In
([Concept](concept.md)参照) をイーサネットケーブルで接続する. 次に, $\SI{24}{V}$電源を接続する.

### Firmware update

firmwareが古い場合, 動作は保証されない. 本文章におけるfirmwareのversionは2.5が想定される.

firmwareのupdateには[Vivado Design Suite](https://www.xilinx.com/products/design-tools/vivado.html),
及び, [J-Link Software](https://www.segger.com/downloads/jlink/)をインストールしたWindows
10/11 64bit PCが必要である[^fn_vivado]. なお, Vivado 2022.1, 及び, J-Link Software v7.58b
(x64)での動作を確認している.

まず,
AUTD3デバイスとPCを[XILINX Platform Cable](https://www.xilinx.com/products/boards-and-kits/hw-usb-ii-g.html),
及び,
[J-Link 9-Pin Cortex-M Adapter](https://www.segger-pocjapan.com/j-link-9-pin-cortex-m-adapter)付きの[J-Link Plus](https://www.segger.com/products/debug-probes/j-link/models/j-link-plus/)で接続し,
AUTD3の電源を入れる. 次に,
[SDK](https://github.com/shinolab/autd3)内の`dist/firmware/autd_firmware_writer.ps1`,
または,
[GitHub Release](https://github.com/shinolab/autd3/releases)で配布されているパッケージ内の`firmware/autd_firmware_writer.ps1`をpowershellから実行し,
指示に従えばよい. updateには数分の時間を要する.

## Building first program

まず, ターミナルを開き, 適当なディレクトリを用意する.

```
mkdir autd3_sample
cd autd3_sample
```

次に, `CMakeLists.txt`, `main.cpp`ファイルを作成する.

```
└─autd3_sample
        CMakeLists.txt
        main.cpp
```

次に, SDKの最新のバイナリをダウンロードしてくる.
バイナリは[GitHub Release](https://github.com/shinolab/autd3/releases)にて公開されている.
ダウンロードしたものを解凍して, `include`フォルダと`lib`フォルダを`autd3_sample`フォルダにコピーする.

```
└─autd3_sample
    │  CMakeLists.txt
    │  main.cpp
    ├─include
    └─lib
```

次に, Eigen3をダウンロードしてくる. Eigen3は行列計算用のヘッダーオンリーライブラリである. ここでは,
カレントディレクトリを`autd3_sample`に変更し, gitのサブモジュールとして追加する.

```
git init
git submodule add https://gitlab.com/libeigen/eigen.git eigen
cd eigen
git checkout 3.4.0
cd ..
```

あるいは, 直接[Eigen3](https://gitlab.com/libeigen/eigen)をダウンロードしてきて,
`autd3_sample`フォルダ以下に置いても良い. SDKで使用しているversionは3.4.0である.

この時点で, ディレクトリ構成は以下のようになっている.

```
└─autd3_sample
    │  CMakeLists.txt
    │  main.cpp
    ├─include
    ├─lib
    └─eigen
        ├─bench
        ├─blas
        ├─ci
        ├─cmake
        ...
```

次に, `CMakeLists.txt`を以下のようにする.

```
{{#include ../../../samples/cpp/CMakeLists.txt}}
```

また, `main.cpp`を以下のようにする. これは単一焦点に$\SI{150}{Hz}$のAM変調をかける場合のソースコードである.

```cpp
{{#include ../../../samples/cpp/main.cpp}}
```

次に, CMakeでbuildする.

```
mkdir build
cd build
cmake ..
```

これで, buildディレクトリ以下に`autd3_sample.sln`が生成されているはずなので, これを開き, mainプロジェクトを実行する.
**なお, 実行に際して, Visual StudioのConfigurationをDebugからReleaseに変更すること.** また,
Linux/macOSの場合は, 実行時にroot権限が必要な場合がある.

## Explanation

上記コードの解説を以下に示す.

SDKを使用するには, `autd3.hpp`ヘッダーをインクルードする. また,
`link::SOEM`を使用するために`autd3/link/soem.hpp`ヘッダーも必要となる.

```cpp
#include "autd3.hpp"
#include "autd3/link/soem.hpp"
```

次に, `Controller`を作成する.

```cpp
autd3::Controller autd;
```

その後, デバイスの配置を指定する.

```cpp
autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
```

`add_device`の第一引数は位置, 第2引数は回転を表す. 位置は自分の設定したグローバル座標系におけるデバイスの原点を指定する. また,
回転はZYZのオイラー角, または, クオータニオンで指定する. ここでは, 回転も移動も行わないものとする.

次に, `Link`を作成し, デバイスと接続する.

```cpp
  auto link = autd3::link::SOEM().high_precision(true).build();
  autd.open(std::move(link));
```

linkの型は`unique_ptr`であるため, `Controller`に渡す際は`move`する必要がある.

次に, `check_trials`を50にセットしている. これは変更しなくても良いが, セットしておくと信頼性が増す. SOEM
link使用時は50程度の値にセットしておくことをお勧めする.

```cpp
autd.check_trials = 50;
```

次に, AUTDデバイスの初期化と同期を行う. 
電源投入時に初期化されるので`clear`は必要ないかもしれない.

```cpp
autd << autd3::clear << autd3::synchronize;
```

**例え, 一台のデバイスしか使用しない場合でも, 同期は一度行う必要がある.**

次に, firmwareのバージョンを確認している. これも必須ではない.

```cpp
const auto firm_infos = autd.firmware_infos();
std::copy(firm_infos.begin(), firm_infos.end(), std::ostream_iterator<autd3::FirmwareInfo>(std::cout, "\n"));
```

ここで, v2.6以外のヴァージョンが表示される場合は, 動作が保証されないので注意する.

次に, silencerを設定する.

```cpp
autd3::SilencerConfig config;
```

デフォルトで設定されているので, これも実際には送信する必要はない.
OFFにしたい場合は`SilencerConfig::none()`を使用する.
silencerは, 振動子に与える位相/振幅パラメータをLow-pass filterに通すことで, 静音化を行う.

その後, 単一焦点を表す`Gain`と$\SI{150}{Hz}$のSin波変調をかける`Modulation`を作成し, デバイスに送信する.

```cpp
const auto focus = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
autd3::gain::Focus g(focus);
autd3::modulation::Sine m(150);

autd << config << m, g;
```

ここで, `focus`は振動子アレイの中心から直上$\SI{150}{mm}$を表す.

最後に, デバイスとの接続を切ってお終いである.

```cpp
autd.close();
```

次頁では基本的な関数について解説していく. なお,
[オンラインのAPIドキュメント](https://shinolab.github.io/autd3/api/index.html)もあるのでそちらも参考にする事.

[^fn_git]: 動かすのに必須ではないが, 作業の単純化のため使用

[^fn_npcap]: SOEM linkを使用するのに使う. それ以外のlinkの場合は必要ない.

[^fn_vivado]: Vivadoは最小構成でも数十GBの容量を食うので, 適当なPCを用意しておくことをおすすめする.
