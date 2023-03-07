# Modulation

`Modulation` controls AM modulation.
The `Modulation` is realized by sequentially sampling $\SI{8}{bit}$ data stored in a buffer at a constant sampling rate and multiplying it by the duty ratio.
Currently, `Modulation` has the following restrictions.

* Maximum buffer size is 65536
* Sampling rate is $\SI{163.84}{MHz}/N$, where $N$ is a 32-bit unsigned integer and must be greater than $1160$.
* Modulation is uniform to all devices.
* Modulation loops automatically.

The SDK provides some `Modulation` to generate several types of AM by default.

[[_TOC_]]

## Static

Static is used for an unmodulated ultrasound.

```cpp
  autd3::modulation::Static m;
```

Note that the first argument is a normalized amplitude of 0-1 (1 by default), which can be used to modify the ultrasound output uniformly.

## Sine

`Modulation` for deforming the sound pressure into a sinusoidal shape.

```cpp
  autd3::modulation::Sine m(f, amplitude, offset); 
```

The first argument is the frequency $f$, the second is $amplitude$ (1 by default), and the third is $offset$ (0.5 by default), so that the sound pressure waveform is
$$
    \frac{amplitude}{2} \times \sin(2\pi ft) + offset.
$$

Values exceeding $\[0,1\]$ in the above are clamped into $\[0,1\]$.
The sampling frequency is set to $\SI{4}{kHz}$ ($N=40960$) by default.

## SineSquared

`Modulation` to transform the radiation pressure, i.e., the square of the sound pressure, into a sinusoidal shape.
Arguments are the same as for `Sine`.

## SineLegacy

Compatible with the old version of `Sine`.

You can take `double` value as frequency, but the frequency is not exactly the specified frequency, but the closest frequency among the possible output frequencies.
Also, the duty ratio, not the sound pressure, will be sinusoidal.

## Square

Square wave modulation.

```cpp
  autd3::modulation::Square m(f, low, high); 
```

The first argument is the frequency $f$, the second is the low value (0 by default), and the third is the high value (1 by default), so that the sound pressure waveform repeats low and high with frequency $f$.

You can specify the duty ratio as the fourth argument.
The duty ratio is defined as $t_\text{high}/T = t_\text{high}f$, where $t_\text{high}$ is the time to output high in one cycle $T=1/f$.

## Wav

`Wav` is a `Modulation` based on a wav file.

```cpp
  const std::filesystem::path path("sin150.wav");
  autd3::modulation::Wav m(path);
```

You must compile with the `BUILD_MODULATION_AUDIO_FILE` option turned on to use `Wav`.

## RawPCM

RawPCM` is a `Modulation` built from unsigned 8-bit binary data files.

```cpp
  const std::filesystem::path path = std::filesystem::path("sin150.wav");
  autd3::modulation::RawPCM m(path, 4e3);
```

You must compile with the `BUILD_MODULATION_AUDIO_FILE` option turned on to use `RawPCM`.

## Create Custom Modulation Tutorial

You can create your own `Modulation` as well as `Gain`.
Here, we try to create a `Burst` that outputs only for a certain moment in a cycle[^fn_burst].

The following is a sample of `Burst`.
```cpp
class Burst final : public autd3::Modulation {
 public:
  std::vector<autd3::Amp> calc() override {
    std::vector buffer(_buf_size, autd3::Amp(0));
    buffer()[_buf_size - 1] = autd3::Amp(1);
  }

  explicit Burst(const size_t buf_size = 4000, const uint16_t freq_div = 40960) noexcept : _buf_size(buf_size) 
  {
    _freq_div = freq_div;
  }

 private:
  size_t _buf_size;
};
```

Like `Gain`, `Modulation::calc` method is called inside `Controller::send`.
In this `calc`, you can rewrite the contents of `buffer`.

$N$, which determines the `Modulation` sampling frequency $\SI{163.84}{MHz}/N$, is set to `_freq_div`.
In this example, since $N=40960$ by default, the sampling frequency is $\SI{4}{kHz}$.

Moreover, for example, if `buf_size` is set to 4000, $0$ is sampled $3999$ times, and then $1$ is sampled once.
Thus, AM is such that $\SI{0.25}{ms}=1/\SI{4}{kHz}$ is output in the period $\SI{1}{s}$.

## Modulation common functions

### Sampling frequency division ratio

The `sampling_freq_div_ratio` is used to check and set the division ratio $N$ of the sampling frequency.
The fundamental frequency of sampling frequency is $\SI{163.84}{MHz}$.
The value of `sampling_freq_div_ratio` can be an integer larger than 1160.

```cpp
    m.sampling_frequency_division() = 20480; // 163.84MHz/20480 = 8kHz
```

### sampling frequency

You can get the sampling frequency with `sampling_frequency`.

## Modulation Delay

Modulation works on all transducers simultaneously and does not take propagation delay into account.
Therefore, there is a possibility that the modulation is out of phase with the distance between the transducer and the focal point.

To compensate for this, a `ModDelay` function is provided to delay the sampling index for each transducer.

For example, if you want to delay the $17$-th transducer of the $0$-th device by one sampling index relative to all other transducers, you can do so as follows:

```cpp
  autd.geometry()[0][17].mod_delay() = 1;
  autd.send(autd3::mod_delay_config());
```

Since this is a delay of the sampling index, the actual delay time depends on the sampling frequency.
If `mod_delay` is $1$ and the sampling frequency is $\SI{40}{kHz}$, the delay is $\SI{25}{\text{μ}s}$, and if $\SI{4}{kHz}$, the delay is $\SI{250}{\text{μ}s}$.

Also, the value of `mod_delay` must be less than the modulation length, i.e., `buffer` size.

[^fn_burst]: Such Modulation is not implemented in SDK.
