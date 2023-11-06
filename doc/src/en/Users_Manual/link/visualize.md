# Visualizer

`Visualizer` link is a link for visualizing phase pattern and sound field.

> NOTE:
> Currently, this Link is not provided in the distributed C++/C#/Unity/Python libraries for linux-armv7 and linux-aarch64. If you want to use this, you need to build it by yourself.

[[_TOC_]]

## Visualize phase pattern

```rust,edition2021
# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
use autd3_link_visualizer::{Visualizer, PlotConfig};

use std::path::Path;

# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Visualizer::builder()).await?;

let center = autd.geometry.center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
let g = Focus::new(center);
autd.send(g).await?;

autd.link.plot_phase(
    PlotConfig {
        fname: Path::new("phase.png").into(),
        ..PlotConfig::default()
    },
    &autd.geometry,
)?;
# autd.close().await?;
# Ok(())
# }
```

```cpp
#include "autd3/link/visualizer.hpp"

auto autd = autd3::Controller::builder()
                .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                .open_with(autd3::link::Visualizer::builder());

autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0, 0, 150);
autd3::gain::Focus g(center);
autd.send(g);

autd3::link::PlotConfig config;
config.fname = "phase.png";
autd.link<autd3::link::Visualizer>().plot_phase(config, autd.geometry());
```

```cs
var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(Visualizer.Builder());

var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
var g = new Focus(center);
autd.Send(g);

autd.Link<Visualizer>().PlotPhase(new PlotConfig
{
    Fname = "phase.png"
}, autd.Geometry);
```

```python
from pyautd3 import Controller, AUTD3
from pyautd3.link.visualizer import Visualizer, PlotConfig
from pyautd3.gain import Focus

autd = (
    Controller.builder()
    .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
    .open_with(Visualizer.builder())
)

center = autd.geometry.center + np.array([0, 0, 150])
g = Focus(center)
autd.send(g)

autd.link.plot_phase(PyPlotConfig(fname="phase.png"), autd.geometry)
```

<figure>
  <img src="../../fig/Users_Manual/phase.png"/>
</figure>

## Visualize modulation data

```rust,edition2021
# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
use autd3_link_visualizer::{Visualizer, PlotConfig};

use std::path::Path;

# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Visualizer::builder()).await?;

let m = Sine::new(150);
autd.send(m).await?;

autd.link.plot_modulation(
    PlotConfig {
        fname: Path::new("mod.png").into(),
        ..PlotConfig::default()
    },
)?;
# autd.close().await?;
# Ok(())
# }
```

```cpp
#include "autd3/link/visualizer.hpp"

auto autd = autd3::Controller::builder()
                .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                .open_with(autd3::link::Visualizer::builder());

autd3::modulation::Sine m(150);
autd.send(m);

autd3::link::PlotConfig config;
config.fname = "mod.png";
autd.link<autd3::link::Visualizer>().plot_modulation(config);
```

```cs
var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(Visualizer.Builder());

var m = new Sine(150);
autd.Send(m);

autd.Link<Visualizer>().PlotModulation(new PlotConfig
{
    Fname = "mod.png"
});
```

```python
from pyautd3 import Controller, AUTD3
from pyautd3.link.visualizer import Visualizer, PlotConfig
from pyautd3.modulation import Sine

autd = (
    Controller.builder()
    .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
    .open_with(Visualizer.builder())
)

m = Sine(150)
autd.send(m)

autd.link.plot_modulation(PyPlotConfig(fname="mod.png"))
```

<figure>
  <img src="../../fig/Users_Manual/mod.png"/>
</figure>

## Visualize sound field

```rust,edition2021
# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
use autd3_link_visualizer::*;

use std::path::Path;

# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Visualizer::builder()).await?;

let center = autd.geometry.center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

autd.send(Focus::new(center)).await?;

autd.link.plot_field(
    PlotConfig {
        fname: Path::new("xy.png").into(),
        ..PlotConfig::default()
    },
    PlotRange{ 
        x_range: center.x - 20.0..center.x + 20.0,
        y_range: center.y - 30.0..center.y + 30.0,
        z_range: center.z..center.z,
        resolution: 1.
    },
    &autd.geometry,
)?;
# autd.close().await?;
# Ok(())
# }
```

```cpp
#include "autd3/link/visualizer.hpp"

auto autd = autd3::Controller::builder()
                .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                .open_with(autd3::link::Visualizer::builder());

autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0, 0, 150);
autd3::gain::Focus g(center);
autd.send(g);

autd3::link::PlotConfig config;
config.fname = "xy.png";
autd.link<autd3::link::Visualizer>().plot_field(
    config,
    autd3::link::PlotRange(center.x() - 20, center.x() + 20, center.y() - 30, center.y() + 30, center.z(), center.z(), 1)
    autd.geometry());
```

```cs
var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(Visualizer.Builder());

var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
var g = new Focus(center);
autd.Send(g);

autd.Link<Visualizer>().PlotField(new PlotConfig
{
    Fname = "xy.png"
},
    new PlotRange
    {
        XStart = center.x - 20,
        XEnd = center.x + 20,
        YStart = center.y - 30,
        YEnd = center.y + 30,
        ZStart = center.z,
        ZEnd = center.z,
        Resolution = 1
    },
    autd.Geometry);
```

```python
from pyautd3 import Controller, AUTD3
from pyautd3.link.visualizer import Visualizer, PlotConfig, PlotRange
from pyautd3.gain import Focus

autd = (
    Controller.builder()
    .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
    .open_with(Visualizer.builder())
)

center = autd.geometry.center + np.array([0, 0, 150])
g = Focus(center)
autd.send(g)

autd.link.plot_field(
    PlotConfig(fname="xy.png"),
    PlotRange(
        x_start=center[0] - 20,
        x_end=center[0] + 20,
        y_start=center[1] - 30,
        y_end=center[1] + 30,
        z_start=center[2],
        z_end=center[2],
        resolution=1,
    ),
    autd.geometry,
)
```

<figure>
  <img src="../../fig/Users_Manual/xy.png"/>
</figure>

### Calculate sound field

You can calculate sound field without plotting by using `calc_field` function.

```rust,edition2021
# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
use autd3_link_visualizer::Visualizer;

use std::path::Path;

# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Visualizer::builder()).await?;

let center = autd.geometry.center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

autd.send(Focus::new(center)).await?;

let p = autd.link.calc_field(&[center], &autd.geometry)?;
println!(
    "Acoustic pressure at ({}, {}, {}) = {}",
    center.x, center.y, center.z, p[0]
);

# autd.close().await?;
# Ok(())
# }
```

```cpp
#include "autd3/link/visualizer.hpp"

auto autd = autd3::Controller::builder()
                .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                .open_with(autd3::link::Visualizer::builder());

autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0, 0, 150);
autd3::gain::Focus g(center);
autd.send(g);

std::vector points{center};
const auto p = autd.link<autd3::link::Visualizer>().calc_field(points, autd.geometry());
std::cout << "Acoustic pressure at (" << center.x() << ", " << center.y() << ", " << center.z() << ") = " << p[0] << std::endl;
```

```cs
var autd = Controller.Builder().AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero)).OpenWith(Visualizer.Builder());

var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
var g = new Focus(center);
autd.Send(g);

var points = new List<Vector3d> { center };
var p = autd.Link<Visualizer>().CalcField(points, autd.Geometry);
Console.WriteLine($"Acoustic pressure at ({center.x}, {center.y}, {center.z}) = ({p[0]})");
```

```python
from pyautd3 import Controller, AUTD3
from pyautd3.link.visualizer import Visualizer
from pyautd3.gain import Focus

autd = (
    Controller.builder()
    .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))
    .open_with(Visualizer.builder())
)

center = autd.geometry.center + np.array([0, 0, 150])
g = Focus(center)
autd.send(g)

points = [center]
p = autd.link.calc_field(points, autd.geometry)
print(f"Acoustic pressure at ({center[0]}, {center[1]}, {center[2]}) = {p[0]}")
```

The first argument of `calc_field` is an iterator of positions to calculate sound field.
The return value is a `Vec` of complex acoustic pressure at the specified positions.

## Enable GPU

You can use GPU to calculate sound field.
The argument of `with_gpu` is GPU id. If you specify `-1`, the most suitable GPU is selected automatically.

```rust,ignore,edition2021
# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
# use autd3_link_visualizer::{Visualizer, PlotConfig};

# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(
Visualizer::new().with_gpu(-1)
# ).await?;
# autd.close().await?;
# Ok(())
# }
```

- In rust library, you need to enable `gpu` feature.

    ```shell
    cargo add autd3-link-visualizer --features gpu
    ```

```cpp
autd3::link::Visualizer::builder().with_gpu(-1)
```

```cs
Visualizer.Builder().WithGpu(-1)
```

```python
Visualizer.builder().with_gpu(-1)
```

## Using Matplotlib

You can plot using Python and matplotlib with `PythonBackend`.

```rust,ignore,edition2021
# extern crate autd3;
# extern crate tokio;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
# use autd3_link_visualizer::{Visualizer, PlotConfig};

# #[allow(unused_variables)]
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(
Visualizer::python()
# ).await?;
# autd.close().await?;
# Ok(())
# }
```

- In rust library, you need to enable `python` feature.

    ```shell
    cagro add autd3-link-visualizer --features python
    ```

```cpp
autd3::link::Visualizer::builder().with_backend<autd3::link::PythonBackend>()
```

```cs
Visualizer.Builder().WithBackend<PythonBackend>()
```

```python
from pyautd3.link.visualizer import Visualizer, PythonBackend

Visualizer.builder().with_backend(PythonBackend())
```
