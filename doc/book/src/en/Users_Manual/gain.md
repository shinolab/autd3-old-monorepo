# Gain

AUTD can control phase/amplitude of each transducer individually, and thus can generate various sound fields.
`Gain` manages phase/amplitude of each transducer, and SDK has some `Gain` classes to generate several types of sound fields.

[[_TOC_]]

## Focus

`Focus` is the simplest `Gain` and generates a single focal point.

```cpp
    autd3::gain::Focus g(autd3::Vector3(x, y, z));
```

The first argument of the constructor is the position of the focus.
The second argument is a normalized sound pressure amplitude of 0-1 (1 by default).

## BesselBeam

`BesselBeam` generates a Bessel beam.
This `Gain` is based on the paper by Hasegawa et al [^hasegawa2017].

```cpp
  const autd3::Vector3 apex(x, y, z);
  const autd3::Vector3 dir = autd3::Vector3::UnitZ();
  const double theta_z = 0.3;
  autd3::gain::BesselBeam g(apex, dir, theta_z);
```

The first argument of the constructor is the apex of the virtual cone producing the beam, the second argument is the direction of the beam, and the third argument is the angle between the plane perpendicular to the beam and the side of the virtual cone producing the beam ($\theta_z$ in the figure below).

The fourth argument is a normalized sound pressure amplitude of 0-1 (1 by default).

<figure>
  <img src="../fig/Users_Manual/1.4985159.figures.online.f1.jpg"/>
  <figcaption>Bessel beam (cited from the paper[^hasegawa2017])</figcaption>
</figure>

## PlaneWave

`PlaneWave`  outputs a plane wave

```cpp
    autd3::gain::PlaneWave g(autd3::Vector3(x, y, z));
```

The first argument of the constructor specifies the direction of the plane wave.
The second argument is a normalized sound pressure amplitude of 0-1 (1 by default).

## Null

`Null` is a `Gain` with amplitude 0.

```cpp
    autd3::gain::Null g;
```

## Holo (Multiple foci)

Holo is a `Gain` for generating multiple foci.
Several algorithms for generating multiple foci have been proposed, and the following algorithms are implemented in SDK.

* `SDP` - Semi-definite programming, based on Inoue et al.[^inoue2015]
* `EVD` - Eigen value decomposition, based on Long et al.[^long2014]
* `LSS` - Linear Synthesis Scheme of single-focus solutions
* `GS` - Gershberg-Saxon, based on Marzo et al.[^marzo2019]
* `GSPAT` - Gershberg-Saxon for Phased Arrays of Transducers, based on Plasencia et al.[^plasencia2020]
* `LM` - Levenberg-Marquardt, LM method proposed by Levenberg [^levenberg1944] and Marquardt [^marquardt1963] for optimization of nonlinear least-squares problems, implementation based on Madsen's text[^madsen2004]
* `Greedy` - Greedy algorithm and Brute-force search, based on Suzuki et al.[^suzuki2021]
* `LSSGreedy` - Greedy algorithm on LSS, based on Chen et al.[^chen2022]
* `APO` - Acoustic Power Optimization, based on Hasegawa et al.[^hasegawa2020]

In addition, each method has a choice of computational backend.
The following `Backend` is provided in the SDK.

* `EigenBackend` - uses [Eigen](https://eigen.tuxfamily.org/index.php?title=Main_Page), available by default.
* `CUDABackend` - uses CUDA, which runs on GPU.
* `BLASBackend` - uses BLAS

To use `Holo`, `include` the file `autd3/gain/holo.hpp`.

```cpp
#include "autd3/gain/holo.hpp"

...

  const auto backend = autd3::gain::holo::EigenBackend::create();
  autd3::gain::holo::GSPAT g(backend);
  g.add_focus(autd3::Vector3(x1, y1, z1), 1.0);
  g.add_focus(autd3::Vector3(x2, y2, z2), 1.0);
```

The constructor argument of each algorithm is `backend`.
There are additional parameters for each algorithm.
For the details of each parameter, please refer to the respective papers.

The `add_focus` function specifies the position of each focus and the amplitude.

If you want to use other `Backend` than Eigen, you need to compile the respective `Backend` library.

### CUDA Backend

To build a CUDA backend, install the [CUDA Toolkit](https://developer.nvidia.com/cuda-toolkit) and set the `BUILD_BACKEND_CUDA` flag on in CMake.

```
  cmake ... -DBUILD_GAIN_HOLO=ON -DBUILD_BACKEND_CUDA=ON
```

We have confirmed that it works with CUDA Toolkit version 11.8.

### BLAS Backend

To build BLAS backend, set `BUILD_BACKEND_BLAS` flag ON and specify the include/lib directory of BLAS and the BLAS vendor.

```
cmake ... -DBUILD_HOLO_GAIN=ON -DBUILD_BLAS_BACKEND=ON -DBLAS_LIB_DIR=<your BLAS library path> -DBLAS_INCLUDE_DIR=<your BLAS include path> -DBLA_VENDOR=<your BLAS vendor>
```

* If you use Intel MKL, turn on the `USE_MKL` flag.
    ```
    cmake ... -DBUILD_HOLO_GAIN=ON -DBUILD_BLAS_BACKEND=ON -DBLAS_LIB_DIR=<your MKL library path> -DBLAS_INCLUDE_DIR=<your MKL include path> -DBLA_VENDOR=Intel10_64lp -DUSE_MKL=ON
    ```

#### OpenBLAS install example in Windows

* Here is an example of installing [OpenBLAS](https://github.com/xianyi/OpenBLAS), one of the BLAS implementations. Please also refer to [official instruction](https://github.com/xianyi/OpenBLAS/wiki/How-to-use-OpenBLAS-in-Microsoft-Visual-Studio).

    * First, install Visual Studio 2022 and Anaconda (or miniconda).
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
        cmake ... -G "Ninja" -DCMAKE_CXX_COMPILER=clang-cl -DCMAKE_C_COMPILER=clang-cl -DCMAKE_Fortran_COMPILER=flang -DCMAKE_MT=mt -DBUILD_WITHOUT_LAPACK=no -DNOFORTRAN=0 -DDYNAMIC_ARCH=ON -DCMAKE_BUILD_TYPE=Release
        cmake --build . --config Release
        cmake --install . --prefix D:\lib\openblas -v
        ```

    * Here, we installed to `D:/lib/open`, but it can be anywhere.
    * You also need to add `%CONDA_HOME%\Library\bin` to your PATH. Here `CONDA_HOME` is the home directory of Anaconda (or miniconda).

* If you follow the above example, the options of BLASBackend should be as follows.
    
    ```
    cmake ... -DBUILD_HOLO_GAIN=ON -DBUILD_BLAS_BACKEND=ON -DBLAS_LIB_DIR=D:/lib/openblas -DBLAS_INCLUDE_DIR=D:/lib/openblas/include/openblas -DBLA_VENDOR=OpenBLAS
    ```

    * If you get `flangxxx.lib` related link errors, add the option `-DBLAS_DEPEND_LIB_DIR=%CONDA_HOME%/Library/lib`.


## Grouped

`Grouped` is a `Gain` to use different `Gain` for each device.

In `Grouped`, a device ID is associated with an arbitrary `Gain`.
```cpp
  const auto g0 = ... ;
  const auto g1 = ... ;

  autd3::gain::Grouped g(autd.geometry());
  g.add(0, g0);
  g.add(1, g1);
```

In the above case, device 0 uses `g0` and device 1 uses `g1`.

## Create Custom Gain Tutorial

You can create your own `Gain` by inheriting from the `Gain` class.
Here, we will actually define a `FocalPoint` that generates a single focus just like `Focus`.

````cpp
#include "autd3.hpp"

class FocalPoint final : public autd3::Gain {
 public:
  explicit FocalPoint(autd3::Vector3 point) : _point(std::move(point)) {}

  std::vector<autd3::driver::Drive> calc(const autd3::Geometry& geometry) override {
    std::vector<autd3::driver::Drive> drives;
    drives.reserve(geometry.num_transducers());
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(drives), [&](const auto& transducer) { {
        const auto dist = (_point - transducer.position()).norm();
        const auto phase = transducer.align_phase_at(dist);
        return driver::Drive{phase, 1.0};
      });
    return drives;
  } 

 private:
  autd::Vector3 _point;
};
```

The `Controller::send` function takes a class inheriting from `Gain` as an argument.
Therefore, you should inherit `Gain`.

The `Gain::calc` method is called in the `Controller::send` function whose argument is `Geometry`.
Therefore, you should calculate the phase/amplitude in this `calc` method.
`Geometry` defines an iterator that return `Device`.
The `Device` also defines an iterator that returns `Transducer`, from which the position of the transducer can be obtained.

In order to maximize the sound pressure of the emitted ultrasound from transducers at a certain point $\bp$, the phases at $\bp$ should be aligned.
This can be calculated by the function `align_phase_at` provided in the `Transducer` class.

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
