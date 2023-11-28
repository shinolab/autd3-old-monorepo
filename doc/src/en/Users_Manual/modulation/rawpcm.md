# RawPCM

`RawPCM` is a `Modulation` constructed from raw pcm file.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/modulation/rawpcm_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/modulation/rawpcm_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/modulation/rawpcm_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/modulation/rawpcm_0.py}}
```

You need to specify the sampling frequency of this data as the second argument of the constructor.

> NOTE: `RawPCM` resamples raw pcm file data to the sampling frequency of Modulation.
> Please refer to [Modulation](../modulation.md) for the setting and constraints of the sampling frequency of `Modulation`.
