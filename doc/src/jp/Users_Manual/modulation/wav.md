# Wav

`Wav`はWavファイルをもとに構成される`Modulation`である.

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

> NOTE: `Wav`では, wavファイルデータをModulationのサンプリング周波数でリサンプリングするので注意されたい.
> Modulationのサンプリング周波数の設定と制約は[Modulation](../modulation.md)を参照されたい.
