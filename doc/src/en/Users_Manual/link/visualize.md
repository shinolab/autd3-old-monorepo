# Visualizer

`Visualizer` link is a link for visualizing phase pattern and sound field.

> NOTE:
> This link is currently available only from Rust.

## Visualize phase pattern

```rust,edition2021
# extern crate autd3;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
use autd3_link_visualizer::{Visualizer, PlotConfig};

use std::path::Path;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Visualizer::default())?;

let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
let g = Focus::new(center);
autd.send(g)?;

autd.link().plot_phase(
    PlotConfig {
        fname: Path::new("phase.png").into(),
        ..PlotConfig::default()
    },
    autd.geometry(),
)?;
# autd.close()?;
# Ok(())
# }
```

<figure>
  <img src="../../fig/Users_Manual/phase.png"/>
</figure>

## Visualize modulation data

```rust,edition2021
# extern crate autd3;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
use autd3_link_visualizer::{Visualizer, PlotConfig};

use std::path::Path;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Visualizer::default())?;

let m = Sine::new(150);
autd.send(m)?;

autd.link().plot_modulation(
    PlotConfig {
        fname: Path::new("mod.png").into(),
        ..PlotConfig::default()
    },
)?;
# autd.close()?;
# Ok(())
# }
```

<figure>
  <img src="../../fig/Users_Manual/mod.png"/>
</figure>

## Visualize sound field

```rust,edition2021
# extern crate autd3;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
use autd3_link_visualizer::*;

use std::path::Path;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Visualizer::default())?;

let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

autd.send(Focus::new(center))?;

autd.link().plot_field(
    PlotRange{ 
        x_range: center.x - 20.0..center.x + 20.0,
        y_range: center.y - 30.0..center.y + 30.0,
        z_range: center.z..center.z,
        resolution: 1.
    },
    PlotConfig {
        fname: Path::new("xy.png").into(),
        ..PlotConfig::default()
    },
    autd.geometry(),
)?;
# autd.close()?;
# Ok(())
# }
```

<figure>
  <img src="../../fig/Users_Manual/xy.png"/>
</figure>

### Calculate sound field

You can calculate sound field without plotting by using `calc_field` function.

```rust,edition2021
# extern crate autd3;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
use autd3_link_visualizer::Visualizer;

use std::path::Path;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Visualizer::default())?;

let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

autd.send(Focus::new(center))?;

let p = autd.link().calc_field([center], autd.geometry());
println!(
    "Acoustic pressure at ({}, {}, {}) = {}",
    center.x, center.y, center.z, p[0]
);

# autd.close()?;
# Ok(())
# }
```

The first argument of `calc_field` is an iterator of positions to calculate sound field.
The return value is a `Vec` of complex acoustic pressure at the specified positions.

## Enable GPU

With `gpu` feature, you can calculate sound field on GPU.

```shell
cargo add autd3-link-visualizer --features gpu
```

```rust,ignore,edition2021
# extern crate autd3;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
# use autd3_link_visualizer::{Visualizer, PlotConfig};

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(
Visualizer::new().with_gpu(-1)
# )?;
# autd.close()?;
# Ok(())
# }
```

The argument of `with_gpu` is GPU id. If you specify `-1`, the most suitable GPU is selected automatically.

## Using Matplotlib

With `python` feature, you can plot using Python and matplotlib.

```shell
cagro add autd3-link-visualizer --features python
```

```rust,ignore,edition2021
# extern crate autd3;
# extern crate autd3_link_visualizer;
# use autd3::prelude::*;
# use autd3_link_visualizer::{Visualizer, PlotConfig};

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(
Visualizer::python()
# )?;
# autd.close()?;
# Ok(())
# }
```
