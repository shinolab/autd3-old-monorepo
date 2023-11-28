# Wav

`Wav` is a `Modulation` constructed from a wav file.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/modulation/wav_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/modulation/wav_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/modulation/wav_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/modulation/wav_0.py}}
```

> NOTE: `Wav` resamples raw pcm file data to the sampling frequency of Modulation.
> Please refer to [Modulation](../modulation.md) for the setting and constraints of the sampling frequency of `Modulation`.
