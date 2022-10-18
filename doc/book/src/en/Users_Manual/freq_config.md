# Frequency Configuration

From version 2.0, the frequency of all the transducers can be changed individually.

The conventional $\SI{40}{kHz}$ fixed mode is called `Legacy mode`, and the variable frequency mode is called `Normal mode`.

The default mode is `Legacy`.
To use `Normal` mode, set `mode` to `NormalMode`.

```cpp
  autd.mode() = autd3::NormalMode::create();
```

The frequency of the transducer is accessed from `Geometry` -> `Device` -> `Transducer` and specified with the function `set_frequency` of `Transducer` or `set_cycle`.

The frequency that can be specified is $\SI{163.84}{MHz}/N, N=1,2,.... 8191$.
The `set_cycle` specifies this $N$ directly.
You can also use `set_frequency` to specify the frequency.

> NOTE: The actual frequency does not change from $\SI{40}{kHz}$ even if you change the frequency in Legacy mode.

Note that you need to call the `synchronize` function once after changing the frequency.

```cpp
  for (auto& dev : autd.geometry())
    for (auto& tr : dev) tr.set_frequency(70e3); // actual frequency is 163.84MHz/2341 ~ 69987 Hz
```

## NormalPhase mode

Normal mode has slightly higher communication latency because it requires one frame for each amplitude/phase data.
In practice, the amplitude data is not expected to be updated frequently, so a `NormalPhase` mode is also available, in which only the phase data is transmitted.

```cpp
  autd.mode() = autd3::NormalPhaseMode::create();
```

In this mode, the amplitude is controlled by sending the `Amplitudes`.

```cpp
  autd3::Amplitudes amp(1.0);
  autd.send(amp);
```
