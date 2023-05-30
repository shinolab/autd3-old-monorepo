# Set Mode/Change frequency

You can change the frequency of ultrasound from $\ufreq$.
The conventional mode with $\ufreq$ is called Legacy mode, and the mode with variable frequency is called Advanced mode.

The default mode is Legacy mode.

You can change to the Advanced mode by the following.

```cpp
  auto geometry = autd3::Geometry::Builder()
                      ...
                      .advanced_mode()
                      .build();
```

The frequency is specified by `set_frequency` or by changing `cycle` directly of `Transducer`.
The `Transducer` can be accessed via the `Geometry` iterator or via the indexer.
The frequency can be specified as $\clkf/N,N=2,...,8191$.
The `cycle` represents this $N$.
In the case of `set_frequency`, the closest value of the possible $N$ is chosen.

Note that frequency configuratino must be done before sending `Synchronize`.

```cpp
  for (auto& tr : autd.geometry())
    tr.set_frequency(70e3); // 163.84MHz/2341 ~ 69987 Hz

  autd.send(autd3::Synchronize());
```

## AdvancedPhase mode

The communication latency in Advanced mode is twice as long as in Legacy mode, because the amplitude/phase data must be sent in one frame each.
In practice, amplitude data is not expected to be updated frequently, so the AdvancedPhase mode is provided to send only phase data.

```cpp
  auto geometry = autd3::Geometry::Builder()
                      ...
                      .advanced_phase_mode()
                      .build();
```

In this mode, the amplitude is controlled by sending the `Amplitudes` class in advance.
```cpp
  autd3::Amplitudes amp(1.0);
  autd.send(amp);
```

All `Gain` amplitude parameters are ignored in this mode.
