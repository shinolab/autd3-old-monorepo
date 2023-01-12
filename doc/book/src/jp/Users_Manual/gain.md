# Gain

AUTDは各振動子の位相/振幅を個別に制御することができ, これによって様々な音場を生成できる.
`Gain`はこれを管理するクラスであり, SDKにはデフォルトでいくつかの種類の音場を生成するための`Gain`がデフォルトでいくつか用意されている.

[[_TOC_]]

## Focus

`Focus`は最も単純な`Gain`であり, 単一焦点を生成する.
```cpp
    autd3::gain::Focus g(autd3::Vector3(x, y, z));
```
コンストラクタの第1引数には焦点の位置を指定する.
第2引数として, 0-1の規格化された音圧振幅を指定できる.

## BesselBeam

`BesselBeam`ではその名の通りBessel beamを生成する.
この`Gain`は長谷川らの論文[^hasegawa2017]に基づく.
```cpp
  const autd3::Vector3 apex(x, y, z);
  const autd3::Vector3 dir = autd3::Vector3::UnitZ();
  const double theta_z = 0.3;
  autd3::gain::BesselBeam g(apex, dir, theta_z);
```
コンストラクタの第1引数はビームを生成する仮想円錐の頂点であり, 第2引数はビームの方向, 第3引数はビームに垂直な面とビームを生成する仮想円錐の側面となす角度である (下図の$\theta_z$).
第4引数として, 0-1の規格化された音圧振幅で指定できる.

<figure>
  <img src="../fig/Users_Manual/1.4985159.figures.online.f1.jpg"/>
  <figcaption>Bessel beam (長谷川らの論文より引用)</figcaption>
</figure>

## PlaneWave

`PlaneWave`は平面波を出力する
```cpp
    autd3::gain::PlaneWave g(autd3::Vector3(x, y, z));
```
コンストラクタの第1引数には平面波の進行方向を指定する.
第2引数として, 0-1の規格化された音圧振幅を指定できる.

## Null

`Null`は振幅0の`Gain`である.
```cpp
    autd3::gain::Null g;
```

## Holo (Multiple foci)

`Holo`は多焦点を生成するための`Gain`である.
多焦点を生成するアルゴリズムが幾つか提案されており, SDKには以下のアルゴリズムが実装されている.

* `SDP` - Semidefinite programming, 井上らの論文[^inoue2015]に基づく
* `EVD` - Eigen value decomposition, Longらの論文[^long2014]に基づく
* `LSS` - Linear Synthesis Scheme 単一焦点解の重ね合わせ
* `GS` - Gershberg-Saxon, Marzoらの論文[^marzo2019]に基づく
* `GSPAT` - Gershberg-Saxon for Phased Arrays of Transducers, Plasenciaらの論文[^plasencia2020]に基づく
* `LM` - Levenberg-Marquardt, LM法はLevenberg[^levenberg1944]とMarquardt[^marquardt1963]で提案された非線形最小二乗問題の最適化法, 実装はMadsenのテキスト[^madsen2004]に基づく.
* `Greedy` - Greedy algorithm and Brute-force search, 鈴木らの論文[^suzuki2021]に基づく
* `LSSGreedy` - Greedy algorithm on LSS, Chenらの論文[^chen2022]に基づく
* `APO` - Acoustic Power Optimization, 長谷川らの論文[^hasegawa2020]に基づく

また, 各手法は計算Backendを選べるようになっている.
SDKには以下の`Backend`が用意されている

* `EigenBackend` - [Eigen](https://eigen.tuxfamily.org/index.php?title=Main_Page)を使用
* `CUDABackend` - CUDAを使用, GPUで実行
* `BLASBackend` - BLASを使用

Holo gainを使用するには`BUILD_GAIN_HOLO`フラグをONにしてビルドするか, 或いは, 配布している`gain_holo`ライブラリをリンクされたい.
また, 適当なバックエンドライブラリをビルド, または, リンクする必要がある.

Holo gainを使用する際は`autd3/gain/holo.hpp`を`include`する.

```cpp
#include "autd3/gain/holo.hpp"
...

  const auto backend = autd3::gain::holo::EigenBackend::create();
  autd3::gain::holo::GSPAT g(backend);
  g.add_focus(autd3::Vector3(x1, y1, z1), 1.0);
  g.add_focus(autd3::Vector3(x2, y2, z2), 1.0);
```

各アルゴリズムのコンストラクタの引数は`backend`である.
`add_focus`関数により各焦点の位置と音圧を指定する.
また, 各アルゴリズムごとに追加のパラメータが存在する.
各パラメータの詳細はそれぞれの論文を参照されたい.

### CUDA Backend

CUDA backendを使用するには[CUDA Toolkit](https://developer.nvidia.com/cuda-toolkit)をインストールし, `BUILD_BACKEND_CUDA`フラグをONにしてビルドするか, 或いは, 配布している`backend_cuda`ライブラリをリンクされたい.

```
  cmake .. -DBUILD_GAIN_HOLO=ON -DBUILD_BACKEND_CUDA=ON
```

なお,  CUDA Toolkit version 11.8で動作を確認している.

### BLAS Backend

BLAS backendを使用する場合, ビルド済みのライブラリは配布されていないので, 自分でビルドする必要ある.
BLAS backendをビルドするには, `BUILD_BACKEND_BLAS`フラグをONにし, BLASのinclude/lib directoryとBLASベンダーを指定する.

```
cmake .. -DBUILD_HOLO_GAIN=ON -DBUILD_BLAS_BACKEND=ON -DBLAS_LIB_DIR=<your BLAS library path> -DBLAS_INCLUDE_DIR=<your BLAS include path> -DBLA_VENDOR=<your BLAS vendor>
```

* Intel MKLを使用する場合は更に`USE_MKL`フラグをONにする.
    ```
    cmake .. -DBUILD_HOLO_GAIN=ON -DBUILD_BLAS_BACKEND=ON -DBLAS_LIB_DIR=<your MKL library path> -DBLAS_INCLUDE_DIR=<your MKL include path> -DBLA_VENDOR=Intel10_64lp -DUSE_MKL=ON
    ```

#### WindowsにおけるOpenBLASインストールガイド

ここでは, BLASの実装の一つである[OpenBLAS](https://github.com/xianyi/OpenBLAS)のインストール例を載せる.
[official instruction](https://github.com/xianyi/OpenBLAS/wiki/How-to-use-OpenBLAS-in-Microsoft-Visual-Studio)も参考にすること.

まず, Visual Studio 2022とAnaconda (or miniconda)をインストールし, Anaconda Promptを開き以下のコマンドを入力する.

```
git clone https://github.com/xianyi/OpenBLAS
cd OpenBLAS
conda update -n base conda
conda config --add channels conda-forge
conda install -y cmake flang clangdev perl libflang ninja
"c:/Program Files/Microsoft Visual Studio/2022/Community/VC/Auxiliary/Build/vcvars64.bat"
set "LIB=%CONDA_PREFIX%\Library\lib;%LIB%"
set "CPATH=%CONDA_PREFIX%\Library\include;%CPATH%"
mkdir build
cd build
cmake .. -G "Ninja" -DCMAKE_CXX_COMPILER=clang-cl -DCMAKE_C_COMPILER=clang-cl -DCMAKE_Fortran_COMPILER=flang -DCMAKE_MT=mt -DBUILD_WITHOUT_LAPACK=no -DNOFORTRAN=0 -DDYNAMIC_ARCH=ON -DCMAKE_BUILD_TYPE=Release
cmake --build . --config Release
cmake --install . --prefix D:\lib\openblas -v
```

ここでは, `D:/lib/open`にインストールしたが, どこでも良い.
また, `%CONDA_HOME%\Library\bin`をPATHに追加する必要がある. ここで`CONDA_HOME`はAnaconda (or miniconda)のホームディレクトリである.

上記の例に従った場合は, BLASBackendのオプションは以下のように指定する.

```
cmake .. -DBUILD_HOLO_GAIN=ON -DBUILD_BLAS_BACKEND=ON -DBLAS_LIB_DIR=D:/lib/openblas -DBLAS_INCLUDE_DIR=D:/lib/openblas/include/openblas -DBLA_VENDOR=OpenBLAS
```

もし, `flangxxx.lib`関連のlinkエラーが発生した場合は, `-DBLAS_DEPEND_LIB_DIR=%CONDA_HOME%/Library/lib`オプションを追加する.

## Grouped

`Grouped`は複数のデバイスを使用する際に,
各デバイスで別々の`Gain`を使用するための`Gain`である.

`Grouped`では, デバイスIDと任意の`Gain`を紐付けて使用する.
```cpp
  const auto g0 = ...;
  const auto g1 = ...;

  autd3::gain::Grouped g;
  g.add(0, g0);
  g.add(1, g1);
```
上の場合は, デバイス0が`Gain g0`, デバイス1が`Gain g1`を使用する.

[^hasegawa2017]: Hasegawa, Keisuke, et al. "Electronically steerable ultrasound-driven long narrow air stream." Applied Physics Letters 111.6 (2017): 064104.

[^inoue2015]: Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.

[^long2014]: Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.

[^marzo2019]: Marzo, Asier, and Bruce W. Drinkwater. "Holographic acoustic tweezers." Proceedings of the National Academy of Sciences 116.1 (2019): 84-89.

[^plasencia2020]: Plasencia, Diego Martinez, et al. "GS-PAT: high-speed multi-point sound-fields for phased arrays of transducers." ACM Transactions on Graphics (TOG) 39.4 (2020): 138-1.

[^levenberg1944]: Levenberg, Kenneth. "A method for the solution of certain non-linear problems in least squares." Quarterly of applied mathematics 2.2 (1944): 164-168.

[^marquardt1963]: Marquardt, Donald W. "An algorithm for least-squares estimation of nonlinear parameters." Journal of the society for Industrial and Applied Mathematics 11.2 (1963): 431-441.

[^madsen2004]: Madsen, Kaj, Hans Bruun Nielsen, and Ole Tingleff. "Methods for non-linear least squares problems." (2004).

[^suzuki2021]: Suzuki, Shun, et al. "Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search." IEEE Transactions on Haptics (2021).

[^chen2022]: Jianyu Chen, et al., "Sound Pressure Field Reconstruction for Ultrasound Phased Array by Linear Synthesis Scheme Optimization,” in Haptics: Science, Technology, Applications. EuroHaptics 2022.

[^hasegawa2020]: Keisuke Hasegawa, et al., "Volumetric acoustic holography and its application to self-positioning by single channel measurement," Journal of Applied Physics,127(24):244904, 2020.7
