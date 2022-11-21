# Frequency Configuration

From version 2.0, the frequency of all the transducers can be changed individually.

The conventional $\SI{40}{kHz}$ fixed mode is called `LegacyMode`, and the variable frequency mode is called `NormalMode`.

The default mode is `Legacy`.

The frequency of the transducer is accessed from `Geometry` -> `Device` -> `Transducer` and specified with the function `set_frequency` of `Transducer` or `set_cycle`.

The frequency that can be specified is $\SI{163.84}{MHz}/N, N=1,2,.... 8191$.
The `set_cycle` specifies this $N$ directly.
You can also use `set_frequency` to specify the frequency.

Note that you must change the frequency before synchronization.

```cpp
  autd << autd3::normal_mode;

  for (auto& dev : autd.geometry())
    for (auto& tr : dev) tr.set_frequency(70e3); // actual frequency is 163.84MHz/2341 ~ 69987 Hz

  autd << autd3::synchronize;
```

## NormalPhase mode

Normal mode has slightly higher communication latency because it requires one frame for each amplitude/phase data.
In practice, the amplitude data is not expected to be updated frequently, so a `NormalPhaseMode` is also available, in which only the phase data is transmitted.

```cpp
  autd << autd3::normal_phase_mode;
```

In this mode, the amplitude is controlled by sending the `Amplitudes`.

```cpp
  autd3::Amplitudes amp(1.0);
  autd << amp;
```
