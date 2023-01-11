# Spatio-Temporal Modulation

`STM` is a function to realize Spatio-Temporal Modulation in Hardware timers.
The SDK provides `FocusSTM`, which supports only a single focus, and `GainSTM`, which supports arbitrary `Gain`.

### FocusSTM

`FocusSTM` has the following restrictions.

* The maximum number of sampling points is 65536
* The sampling frequency is $\SI{163.84}{MHz}/N$. where $N$ is a 32-bit unsigned integer and must be greater than or equal to $1612$.

The usage of `FocusSTM` is as follows.

```cpp
  autd3::FocusSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
  std::vector<size_t> points(points_num);
  std::iota(points.begin(), points.end(), 0);
  std::transform(points.begin(), points.end(), std::back_inserter(stm), [&](const size_t i) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    return autd3::Point(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
  });

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd << stm;
```

Due to constraints on the number of sampling points and sampling period, the specified frequency and the actual frequency may differ.
For example, in the above example, since 200 points are sampled at $\SI{1}{Hz}$, the sampling frequency should be $\SI{200}{Hz}=\SI{163.84}{MHz}/819200$, which satisfies the constraint.
However, if `point_num` is 199, the sampling frequency must be $\SI{199}{Hz}$, but there is no $N$ such that $\SI{199}{Hz}=\SI{163.84}{MHz}/N$, so the closest $N$ is selected.
This results in a discrepancy between the specified frequency and the actual frequency.
The `set_frequency` function returns the actual frequency.

### GainSTM

Unlike `FocusSTM`, `GainSTM` can handle arbitrary `Gain`.
However, the number of `Gain` that can be used is

- 2048 for Legacy mode
- 1024 for Normla mode.

```cpp
  autd3::GainSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
  std::vector<size_t> points(points_num);
  std::iota(points.begin(), points.end(), 0);
  std::for_each(points.begin(), points.end(), [&](const size_t i) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    autd3::gain::Focus g(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0));
    stm.add(g);
  });

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";
  autd << stm;
```

The frequency constraints are also the same as for `FocusSTM`.

Since `GainSTM` sends all phase/amplitude data, the latency is large[^fn_gain_seq].

To cope with this problem, `GainSTM` provides two modes: `PhaseFull` mode, which transmits only the phase, and `PhaseHalf` mode[^phase_half], which compresses the phase to $\SI{4}{bit}$. 
The mode can be switched with the `mode` function.

```cpp
stm.mode() = autd3::Mode::PhaseFull;
```

The default is `PhaseDutyFull` mode, which sends phase and amplitude.

### STM common functions

#### frequency

Get `STM` frequency.

#### sampling_frequency

Get the sampling frequency of `STM`.

#### sampling_frequency_division

Get or set the division ratio of the sampling frequency of `STM`.
The fundamental frequency of sampling frequency is $\SI{163.84}{MHz}$.

```cpp
    stm.sampling_frequency_division() = 20480; // 163.84MHz/20480 = 8kHz
```

# SoftwareSTM

`SoftwareSTM` is a function to realize Spatio-Temporal Modulation with software timers.
There is no AUTD3 hardware restriction, but its accuracy depends on the accuracy of the host PC[^timer_precision].

The usage of `SoftwareSTM` is as follows.
The basic usage is the same as that of `GainSTM`.

```cpp
  autd3::SoftwareSTM stm;

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  constexpr size_t points_num = 200;
  constexpr auto radius = 30.0;
  std::vector<size_t> points(points_num);
  std::iota(points.begin(), points.end(), 0);
  std::for_each(points.begin(), points.end(), [&](const size_t i) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    autd3::gain::Focus g(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0.0));
    stm.add(g);
  });

  const auto actual_freq = stm.set_frequency(1);
  std::cout << "Actual frequency is " << actual_freq << " Hz\n";

  auto handle = stm.start(autd);

  std::cout << "press any key to stop software stm..." << std::endl;
  std::cin.ignore();

  handle.finish();
```

[^fn_gain_seq]: About 60 times the latency of `FocusSTM`.

[^phase_half]: Only available in Legacy mode.

[^timer_precision]: On Windows, for example, the limit is about $\SI{1}{ms}$.
