# RawPCM

`RawPCM`はraw pcmファイルをもとに構成される`Modulation`である.

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

コンストラクタの第2引数で, このデータのサンプリング周波数を指定する必要がある.

> NOTE: `RawPCM`では, raw pcmファイルデータをModulationのサンプリング周波数でリサンプリングするので注意されたい.
> Modulationのサンプリング周波数の設定と制約は[Modulation](../modulation.md)を参照されたい.
