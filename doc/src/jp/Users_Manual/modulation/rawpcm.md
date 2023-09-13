# RawPCM

`RawPCM`はraw pcmファイルをもとに構成される`Modulation`である.

```rust,should_panic,edition2021
# extern crate autd3_modulation_audio_file;
use autd3_modulation_audio_file::RawPCM;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let path = "path/to/foo.dat";
let m = RawPCM::new(&path, 4000)?;
# Ok(())
# }
```

```cpp
#include "autd3/modulation/audio_file.hpp"

const auto path = "path/to/foo.fat";
const auto m = autd3::modulation::audio_file::RawPCM(path, 4000);
```

コンストラクタの第2引数で, このデータのサンプリング周波数を指定する必要がある.

> NOTE: `RawPCM`では, raw pcmファイルデータをModulationのサンプリング周波数でリサンプリングするので注意されたい.
> Modulationのサンプリング周波数の設定と制約は[Modulation](../modulation.md)を参照されたい.
