# Null

`Null`は振幅0の`Gain`である.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
let g = autd3::gain::Null::new();
# }
```

```cpp
const auto g = autd3::gain::Null();
```

```cs
var g = new Null();
```

```python
from pyautd3.gain import Null

g = Null()
```
