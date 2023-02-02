# チュートリアル

ここでは, 実際にAUTD3を動かす手順について述べる. なお, 本章ではOSとしてWindows 11 64bitを使用する.
他のOSの場合は適宜読み替えられたい.

## インストール

まず, 必要なツールをインストールする.
本節で使用するツールとバージョンは以下の通りである.
各々公式の手順に従ってインストールすること.
Visual Studio Community 2022は「C++によるデスクトップ開発」にチェックを入れてインストールすれば良い.
なお, Linuxの場合はgccが, macOSの場合はclangが使えれば良い.
また, 以下はターミナルから操作するため, PATHを通しておくことを推奨する.

- Visual Studio Community 2022 17.4.4
- CMake 3.25.1
- git 2.39.0.windows.1[^fn_git]
- npcap 1.72[^fn_npcap]

## デバイスのセットアップ

次にデバイスをセットアップする. ここでは一台のAUTD3のみを使うこととする.
PCのイーサネットポートとAUTD3デバイスのEtherCAT In ([Concept](concept.md)参照) をイーサネットケーブルで接続する.
次に, $\SI{24}{V}$電源を接続する.

### ファームウェアアップデート

ファームウェアが古い場合, 動作は保証されない.
本文章におけるファームウェアのバージョンはv2.8が想定される.

> NOTE: 実際には, (少なくとも以下のプログラムは) v2.7以下のファームウェアでも動作すると思われる. しかし, v2.8を使用することを推奨する.

ファームウェアのアップデートには[Vivado](https://www.xilinx.com/products/design-tools/vivado.html), 及び, [J-Link Software](https://www.segger.com/downloads/jlink/)をインストールしたWindows 10/11 64bit PCが必要である.
なお, Vivado 2022.2, 及び, J-Link Software v7.82a (x64)での動作を確認している.

> NOTE: ファームウェアのアップデートだけが目的であれば, "Vivado Lab Edition"の使用を強く推奨する. 
> ML Edition はインストールに60 GB以上のディスク容量を要求する. Lab Edition は6 GB程度のディスク容量で済む. 

まず, AUTD3デバイスとPCを[XILINX Platform Cable](https://www.xilinx.com/products/boards-and-kits/hw-usb-ii-g.html), 及び, [J-Link 9-Pin Cortex-M Adapter](https://www.segger-pocjapan.com/j-link-9-pin-cortex-m-adapter)付きの[J-Link Plus](https://www.segger.com/products/debug-probes/j-link/models/j-link-plus/)で接続し, AUTD3の電源を入れる.
次に, [SDK](https://github.com/shinolab/autd3)内の`dist/firmware/autd_firmware_writer.ps1`, または, [GitHub Release](https://github.com/shinolab/autd3/releases)で配布されているパッケージ内の`firmware/autd_firmware_writer.ps1`をpowershellから実行し, 指示に従えばよい. updateには数分の時間を要する.

## AUTD3クライアントプログラムの作成

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
`autd3_sample`フォルダ以下に置いても良い. SDKで使用しているバージョンは3.4.0である.

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

次に, CMakeでビルドする.

```
mkdir build
cd build
cmake ..
```

これで, buildディレクトリ以下に`autd3_sample.sln`が生成されているはずなので, これを開き, mainプロジェクトを実行する.
**なお, 実行に際して, Visual StudioのConfigurationをDebugからReleaseに変更すること.** また,
Linux/macOSの場合は, 実行時にroot権限が必要な場合がある.

次頁では基本的な関数について解説していく.
なお, [オンラインのAPIドキュメント](https://shinolab.github.io/autd3/api/index.html)もあるのでそちらも参考にする事.

[^fn_git]: 動かすのに必須ではないが, 作業の単純化のため使用

[^fn_npcap]: SOEM linkを使用するのに使う. それ以外のlinkの場合は必要ない.
