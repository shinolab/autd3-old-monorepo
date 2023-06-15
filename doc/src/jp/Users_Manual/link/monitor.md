# Monitor

`Monitor`リンクはデバッグ用の`Link`である.

この`Link`は内部でpythonのmatplotlibライブラリを使用するので, 事前にmatplotlibをインストールする必要がある.

## 位相パターンの可視化

```rust,ignore
# use autd3::prelude::*;
use autd3_link_monitor::{Monitor, PlotConfig};

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Monitor::new())?;

# autd.send(Clear::new())?;
# autd.send(Synchronize::new())?;
let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
let g = Focus::new(center);
autd.send(g)?;

autd.link().save_phase(
    "phase.png",
    PlotConfig {
        figsize: (6, 4),
        dpi: 72,
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

## 変調データの可視化

```rust,ignore
# use autd3::prelude::*;
use autd3_link_monitor::{Monitor, PlotConfig};

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Monitor::new())?;

# autd.send(Clear::new())?;
# autd.send(Synchronize::new())?;
let m = Sine::new(150);
autd.send(m)?;

autd.link().save_modulation(
    "mod.png",
    PlotConfig {
        figsize: (6, 4),
        dpi: 72,
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

## 音場の可視化

```rust,ignore
# use autd3::prelude::*;
use autd3_link_monitor::{Monitor, PlotConfig};

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Monitor::new())?;

# autd.send(Clear::new())?;
# autd.send(Synchronize::new())?;
let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

autd.send(Focus::new(center))?;

autd.link().save_field(
    "xy.png",
    center.x - 20.0..center.x + 20.0,
    center.y - 30.0..center.y + 30.0,
    center.z..center.z,
    1.,
    PlotConfig {
        figsize: (6, 6),
        dpi: 72,
        fontsize: 8,
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


## GPUの有効化

`gpu` featureを有効化することで, 音場の計算をGPU行うことができるようになる.

```shell
cargo add autd3-link-monitor --features gpu
```

```rust,ignore
# use autd3::prelude::*;
use autd3_link_monitor::{Monitor, PlotConfig};

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .open_with(Monitor::new().with_gpu(-1))?;
# autd.close()?;
# Ok(())
# }
```

`with_gpu`の引数にはGPUのIDを指定する. `-1`を指定すると, 適当なGPUが自動的に選択される.
