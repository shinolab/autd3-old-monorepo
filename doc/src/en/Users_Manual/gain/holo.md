# Holo

Holo is a `Gain` for generating multiple foci.
Several algorithms for generating multiple foci have been proposed, and the following algorithms are implemented in SDK.

* `SDP` - Semi-definite programming, based on Inoue et al.[^inoue2015]
* `EVP` - Eigen value decomposition, based on Long et al.[^long2014]
* `Naive` - Linear synthesis of single-focus solutions
* `GS` - Gershberg-Saxon, based on Marzo et al.[^marzo2019]
* `GSPAT` - Gershberg-Saxon for Phased Arrays of Transducers, based on Plasencia et al.[^plasencia2020]
* `LM` - Levenberg-Marquardt, LM method proposed by Levenberg [^levenberg1944] and Marquardt [^marquardt1963] for optimization of nonlinear least-squares problems, implementation based on Madsen's text[^madsen2004]
* `Greedy` - Greedy algorithm and Brute-force search, based on Suzuki et al.[^suzuki2021]

You can select the backend for the calculation of the algorithm from the following.

* `NalgebraBackend` - uses [Nalgebra](hthttps://nalgebra.org/)
* `CUDABackend` - uses CUDA, which runs on GPUs

```rust,edition2021
# extern crate autd3;
# extern crate autd3_gain_holo;
# use autd3::prelude::*;
use autd3_gain_holo::{LinAlgBackend, NalgebraBackend, GSPAT};

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let x1 = 0.;
# let y1 = 0.;
# let z1 = 0.;
# let x2 = 0.;
# let y2 = 0.;
# let z2 = 0.;
let backend = NalgebraBackend::new()?;

let g = GSPAT::new(backend)
      .add_focus(Vector3::new(x1, y1, z1), 1.)
      .add_focus(Vector3::new(x2, y2, z2), 1.);
# Ok(())
# }
```

```cpp
#include "autd3/gain/holo.hpp"

const auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();

auto g = autd3::gain::holo::GSPAT(backend)
            .add_focus(autd3::Vector3(x1, y1, z1), 1.0)
            .add_focus(autd3::Vector3(x2, y2, z2), 1.0);
```

```cs
var backend = new NalgebraBackend();

var g = new GSPAT<NalgebraBackend>(backend)
            .AddFocus(new Vector3d(x1, y1, z1), 1.0)
            .AddFocus(new Vector3d(x2, y2, z2), 1.0);
```

```python
from pyautd3.gain.holo import GSPAT, NalgebraBackend

backend = NalgebraBackend()

g = GSPAT(backend).add_focus([x1, y1, z1], 1.0).add_focus([x2, y2, z2], 1.0)
```

The constructor argument of each algorithm is `backend`.

The `add_focus` function specifies the position of each focus and the amplitude.

## Amplitude constraint

Each algorithm's calculation result must be limited to the range that the transducer can output.
This can be controlled by `with_constraint`, and one of the following four must be specified.

- DontCare: Do nothing. (This is equivalent to clamping to the range $\[0, 1\]$.)
- Normalize: Divide the amplitude of all transducers by the maximum amplitude and normalize it.
- Uniform: Set the amplitude of all transducers to the specified value. (Values outside the range $\[0, 1\]$ are clamped to the range $\[0, 1\]$.)
- Clamp: Clamp the amplitude to the specified range.

```rust,edition2021
# extern crate autd3;
# extern crate autd3_gain_holo;
# use autd3::prelude::*;
use autd3_gain_holo::{LinAlgBackend, NalgebraBackend, GSPAT, Constraint};

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let x1 = 0.;
# let y1 = 0.;
# let z1 = 0.;
# let x2 = 0.;
# let y2 = 0.;
# let z2 = 0.;
let backend = NalgebraBackend::new()?;

let g = GSPAT::new(backend)
      .with_constraint(Constraint::Uniform(1.));
# Ok(())
# }
```

```cpp
auto g = autd3::gain::holo::GSPAT(backend)
		.with_constraint(autd3::gain::holo::AmplitudeConstraint::uniform(1.));
```

```cs
var g = new GSPAT<NalgebraBackend>(backend).WithConstraint(new Uniform(1.0));
```

```python
from pyautd3.gain.holo import AmplitudeConstraint

g = GSPAT(backend).with_constraint(AmplitudeConstraint.uniform(1.0))
```

## Optimization parameters

Each algorithm has additional parameters.
These are all specified by `with_xxx`.

- e.g.,
    ```rust,edition2021
    # extern crate autd3;
    # extern crate autd3_gain_holo;
    # use autd3::prelude::*;
    # use autd3_gain_holo::{LinAlgBackend, NalgebraBackend, GSPAT};
    # #[allow(unused_variables)]
    # fn main() -> Result<(), Box<dyn std::error::Error>> {
    # let x1 = 0.;
    # let y1 = 0.;
    # let z1 = 0.;
    # let x2 = 0.;
    # let y2 = 0.;
    # let z2 = 0.;
    # let backend = NalgebraBackend::new()?;
    let g = GSPAT::new(backend).with_repeat(100)
    #    .add_focus(Vector3::new(x1, y1, z1), 1.)
    #    .add_focus(Vector3::new(x2, y2, z2), 1.);
    # Ok(())
    # }
    ```

    ```cpp
    autd3::gain::holo::GSPAT g(backend).with_repeat(100);
    ```

    ```cs
    var g = new GSPAT(backend).WithRepeat(100);
    ```

    ```python
    g = GSPAT(backend).with_repeat(100)
    ```

Please refar to each paper for more details.

[^inoue2015]: Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE World Haptics Conference (WHC). IEEE, 2015.

[^long2014]: Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.

[^marzo2019]: Marzo, Asier, and Bruce W. Drinkwater. "Holographic acoustic tweezers." Proceedings of the National Academy of Sciences 116.1 (2019): 84-89.

[^plasencia2020]: Plasencia, Diego Martinez, et al. "GS-PAT: high-speed multi-point sound-fields for phased arrays of transducers." ACM Transactions on Graphics (TOG) 39.4 (2020): 138-1.

[^levenberg1944]: Levenberg, Kenneth. "A method for the solution of certain non-linear problems in least squares." Quarterly of applied mathematics 2.2 (1944): 164-168.

[^marquardt1963]: Marquardt, Donald W. "An algorithm for least-squares estimation of nonlinear parameters." Journal of the society for Industrial and Applied Mathematics 11.2 (1963): 431-441.

[^madsen2004]: Madsen, Kaj, Hans Bruun Nielsen, and Ole Tingleff. "Methods for non-linear least squares problems." (2004).

[^suzuki2021]: Suzuki, Shun, et al. "Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search." IEEE Transactions on Haptics (2021).
