# Wav

`Wav` is a `Modulation` constructed from a wav file.

```rust,should_panic,edition2021
# extern crate autd3_modulation_audio_file;
use autd3_modulation_audio_file::Wav;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let path = "path/to/foo.wav";
let m = Wav::new(&path)?;
# Ok(())
# }
```

```cpp
#include "autd3/modulation/audio_file.hpp"

const auto path = "path/to/foo.wav";
const auto m = autd3::modulation::audio_file::Wav(path);
```

```cs
var path = "path/to/foo.wav";
var m = new Wav(path);
```

```python
from pyautd3.modulation.audio_file import Wav

path = "path/to/foo.wav"
m = Wav(path)
```

> NOTE: `Wav` resamples raw pcm file data to the sampling frequency of Modulation.
> Please refer to [Modulation](../modulation.md) for the setting and constraints of the sampling frequency of `Modulation`.
