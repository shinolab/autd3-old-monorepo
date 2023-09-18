# RawPCM

`RawPCM` is a `Modulation` constructed from raw pcm file.

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

```cs
var path = "path/to/foo.dat";
var m = new RawPCM(path, 4000);
```

```python
from pyautd3.modulation.audio_file import RawPCM

path = "path/to/foo.dat"
m = RawPCM(path, 4000)
```

You need to specify the sampling frequency of this data as the second argument of the constructor.

> NOTE: `RawPCM` resamples raw pcm file data to the sampling frequency of Modulation.
> Please refer to [Modulation](../modulation.md) for the setting and constraints of the sampling frequency of `Modulation`.
