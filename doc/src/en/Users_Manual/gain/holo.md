# Holo

Holo is a `Gain` for generating multiple foci.
Several algorithms for generating multiple foci have been proposed, and the following algorithms are implemented in SDK.

* `SDP` - Semi-definite programming, based on Inoue et al.[^inoue2015]
* `Naive` - Linear synthesis of single-focus solutions
* `GS` - Gershberg-Saxon, based on Marzo et al.[^marzo2019]
* `GSPAT` - Gershberg-Saxon for Phased Arrays of Transducers, based on Plasencia et al.[^plasencia2020]
* `LM` - Levenberg-Marquardt, LM method proposed by Levenberg [^levenberg1944] and Marquardt [^marquardt1963] for optimization of nonlinear least-squares problems, implementation based on Madsen's text[^madsen2004]
* `Greedy` - Greedy algorithm and Brute-force search, based on Suzuki et al.[^suzuki2021]

You can select the backend for the calculation of the algorithm from the following.

* `NalgebraBackend` - uses [Nalgebra](hthttps://nalgebra.org/)
* `CUDABackend` - uses CUDA, which runs on GPUs

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/holo_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/holo_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/holo_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/holo_0.py}}
```

The constructor argument of each algorithm is `backend`.

The `add_focus` function specifies the position of each focus and the amplitude.

## Amplitude constraint

Each algorithm's calculation result must be limited to the range that the transducer can output.
This can be controlled by `with_constraint`, and one of the following four must be specified.

- DontCare: Do nothing.
- Normalize: Divide the amplitude of all transducers by the maximum amplitude and normalize it.
- Uniform: Set the amplitude of all transducers to the specified value.
- Clamp: Clamp the amplitude to the specified range.

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/holo_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/holo_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/holo_1.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/holo_1.py}}
```

## Optimization parameters

Each algorithm has additional parameters.
These are all specified by `with_xxx`.

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/holo_2.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/holo_2.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/holo_2.cs}}
```

```python
{{#include ../../../codes/Users_Manual/gain/holo_2.py}}
```

Please refar to each paper for more details.

[^inoue2015]: Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.

[^marzo2019]: Marzo, Asier, and Bruce W. Drinkwater. "Holographic acoustic tweezers." Proceedings of the National Academy of Sciences 116.1 (2019): 84-89.

[^plasencia2020]: Plasencia, Diego Martinez, et al. "GS-PAT: high-speed multi-point sound-fields for phased arrays of transducers." ACM Transactions on Graphics (TOG) 39.4 (2020): 138-1.

[^levenberg1944]: Levenberg, Kenneth. "A method for the solution of certain non-linear problems in least squares." Quarterly of applied mathematics 2.2 (1944): 164-168.

[^marquardt1963]: Marquardt, Donald W. "An algorithm for least-squares estimation of nonlinear parameters." Journal of the society for Industrial and Applied Mathematics 11.2 (1963): 431-441.

[^madsen2004]: Madsen, Kaj, Hans Bruun Nielsen, and Ole Tingleff. "Methods for non-linear least squares problems." (2004).

[^suzuki2021]: Suzuki, Shun, et al. "Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search." IEEE Transactions on Haptics (2021).
