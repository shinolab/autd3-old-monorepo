# Silencer

AUTD3 has a silencer to mute the output.
The silencer suppresses the rapid change in the drive signal of the transducer and mutes the output.

## Theory

The silencer is based on the paper by Suzuki et al.[^suzuki2020].

As a rough outline,
- Amplitude modulation of ultrasound produces audible sound.
- When driving an ultrasound transducer, phase changes cause amplitude fluctuations.
  - Therefore, audible noise is generated.
- Amplitude fluctuations can be suppressed by linearly interpolating phase changes and changing them stepwise.
  - Therefore, noise can be reduced by doing fine interpolation.
- The silencer is a method to reduce noise by doing fine interpolation.

## Silencer Config

To configure the silencer, send `SilencerConfig` to the controller.

```rust
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
let config = SilencerConfig::default();
autd.send(config)?;
# Ok(())
# }
```

```cpp
autd3::SilencerConfig config;
autd.send(config);
```

```cs
var config = new SilencerConfig();
autd.Send(config);
```

```python
from pyautd3 import SilencerConfig

config = SilencerConfig()
autd.send(config)
```

You can set `step` to `SilencerConfig`.
Refer to the followwing for details.
Roughly, the smaller the `step`, the quieter it becomes.

```rust
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
# let step = 10;
let config = SilencerConfig::new(step);
# Ok(())
# }
```

```cpp
autd3::SilencerConfig config(step);
```

```cs
var config = new SilencerConfig(step);
```

```python
config = SilencerConfig(step)
```

## Disabling Silencer

The silencer is enabled by default.
To disable the silencer, do the following.

```rust
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
let config = SilencerConfig::none();
# Ok(())
# }
```

```cpp
const auto config = autd3::SilencerConfig::none();
```

```cs
var config = SilencerConfig.None();
```

```python
config = SilencerConfig.none()
```

## Phase change by Silencer

Silencer changes the phase $P$ linearly and stepwise to mute the output.
In other words, it is almost equivalent to passing the phase $P$ time series data through a (simple) moving average filter.
However, it differs in that it takes into account the fact that the phase data is periodic.

For example, consider the case where the period $T$ of the ultrasound is $T=12$.
In other words, $P=0$ corresponds to $0\,\mathrm{rad}$ and $P=12$ corresponds to $2\pi\,\mathrm{rad}$.
Here, suppose that the phase changes from $P=2$ to $P=6$ at time $t_s$.
The phase change by Silencer is as follows.

<figure>
  <img src="../fig/Users_Manual/silent/phase.svg"/>
<figcaption>Change of phase $P$</figcaption>
</figure>

On the other hand, suppose that the phase changes from $P=2$ to $P=10$ at time $t_s$.
The phase change by Silencer is as follows.
This is because $P=-2$ is closer to $P=2$ than $P=10$ in terms of the phase.

<figure>
  <img src="../fig/Users_Manual/silent/phase2.svg"/>
<figcaption>Change of phase $P$</figcaption>
</figure>

That is, Silencer updates the phase $P$ as follows for the current $P$ and the target value $P_r$.
$$
    P \leftarrow \begin{cases}
        P + \mathrm{sign}(P_r - P) \min (|P_r - P|, \Delta) & \text{if } |P_r - P| \le T/2\\
        P - \mathrm{sign}(P_r - P) \min (|P_r - P|, \Delta) & \text{(otherwise)}\\
    \end{cases}.
$$
Where $\Delta$ is the update amount per step (`step` of `SilencerConfig`).
And the update frequency is $\ufreq$.

Small $\Delta$ makes the phase change smoother and reduces noise.

<figure>
  <img src="../fig/Users_Manual/silent/duty.svg"/>
<figcaption>Phase change over $\Delta$</figcaption>
</figure>

According to this implementation, the behavior is different from the moving average filter.
One is when the phase change amount shown above is larger than $\pi$, and the other is when the phase changes again in the middle.
Examples of phase changes at this time are shown below.

<figure>
  <img src="../fig/Users_Manual/silent/mean.svg"/>
<figcaption>Comparison against moving average filter</figcaption>
</figure>

## Duty change by Silencer

Amplitude modulation of ultrasound produces audible sound.
So, AM noise can be reduced by applying a filter to the duty ratio $D$.

Unlike the phase, the duty ratio $D$ is not periodic with respect to the period $T$.
Therefore, the duty ratio $D$ is updated as follows for the current $D$ and the target value $D_r$.
$$
    D \leftarrow D + \mathrm{sign}(D_r - D) \min (|D_r - D|, \Delta),
$$

[^suzuki2020]: Suzuki, Shun, et al. "Reducing amplitude fluctuation by gradual phase shift in midair ultrasound haptics." IEEE transactions on haptics 13.1 (2020): 87-93.
