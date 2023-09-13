# RadiationPressure

`RadiationPressure`は`Modulation`を音圧ではなく, 放射圧 (音圧の二乗に比例) に印加するための`Modulation`である.

例えば, $\SI{1}{kHz}$の`Sine`変調に`RadiationPressure`を適用した場合の音圧振幅の放射圧は以下のようになり, 放射圧の包絡線が$\SI{1}{kHz}$のsin波に従う.

<figure>
  <img src="../../fig/Users_Manual/sine_1k_mod_rad.png"/>
</figure>

```rust,edition2021
# extern crate autd3;
use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main() {
let m = Sine::new(150).with_radiation_pressure();
# }
```

```cpp
const auto m = autd3::modulation::Sine(150).with_radiation_pressure();
```

```cs
var m = new RadiationPressure(new Sine(150));
```

```python
from pyautd3.modulation import Sine, RadiationPressure

m = RadiationPressure(Sine(150))
```
