# Modulation

`Modulation` controls AM modulation.
The `Modulation` is realized by sequentially sampling $\SI{8}{bit}$ data stored in a buffer at a constant sampling rate and multiplying it by the duty ratio.
Currently, `Modulation` has the following restrictions.

* Maximum buffer size is 65536
* Sampling rate is $\SI{163.84}{MHz}/N$, where $N$ is a 32-bit unsigned integer and must be greater than $1160$
* Modulation is uniform to all devices
* Modulation loops automatically
* Modulation start/end timing cannot be controlled

The SDK provides some `Modulation` to generate several types of AM.

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

## Cache

`Cache` is a cache of `Modulation` to store the result of modulation data calculation.
It is used when the calculation of modulation data is heavy and the same `Modulation` is sent more than once.
It can also be used to check or change the modulation data after the modulation data calculation.

To use `Cache`, specify any `Modulation` type as a type argument and pass the constructor arguments of the original type in the constructor.

```cpp
  autd3::modulation::Cache<autd3::modulation::Sine> m(...) ;
```

The modulation data can be accessed with the `buffer` function or with the indexer.
Note that you need to call the `calc` function first.

```cpp
  autd3::modulation::Cache<autd3::modulation::Sine> m(...) ;
  m.calc();
  m[0] = 0;
```
In the above example, the 0-th modulation data is set to 0.

## Transform

`Transform` is a `Modulation` that modifies the result of `Modulation` calculation.

To use `Transform`, specify any `Modulation` type as a type argument.
The first argument of the constructor is a transformation function, the second and subsequent arguments are constructor arguments of the original type.
```cpp
  autd3::modulation::Transform<autd3::modulation::Sine> m([](const double v) {return std::clamp(v, 0.5, 1.0); }, 150);
```
For example, in the above example, the modulation data is like a half-rectified sine wave of $\SI{150}{Hz}$.

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

## Modulation API

### sampling_frequency_division 

The `sampling_frequency_division` is used to check and set the division ratio $N$ of the sampling frequency.
The fundamental frequency of sampling frequency is $\SI{163.84}{MHz}$.
The value of `sampling_frequency_division` can be an integer larger than 1160.

```cpp
    m.sampling_frequency_division() = 20480; // 163.84MHz/20480 = 8kHz
```

### sampling frequency

You can get the sampling frequency with `sampling_frequency`.

### size

You can get the length of the modulation data buffer with `size`.

## Modulation Delay

Modulation works on all transducers simultaneously and does not take propagation delay into account.
Therefore, there is a possibility that the modulation is out of phase with the distance between the transducer and the focal point.

To compensate for this, a `ModDelay` function is provided to delay the sampling index for each transducer.

For example, if you want to delay the $17$-th transducer of the $0$-th device by one sampling index relative to all other transducers, you can do so as follows:

```cpp
  autd.geometry()[0][17].mod_delay() = 1;
  autd.send(autd3::ModDelayConfig());
```

Since this is a delay of the sampling index, the actual delay time depends on the sampling frequency.
If `mod_delay` is $1$ and the sampling frequency is $\SI{40}{kHz}$, the delay is $\SI{25}{\text{μ}s}$, and if $\SI{4}{kHz}$, the delay is $\SI{250}{\text{μ}s}$.

Also, the value of `mod_delay` must be less than the modulation length, i.e., `buffer` size.
