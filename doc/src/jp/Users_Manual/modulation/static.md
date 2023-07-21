# Static

変調なし.


```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Static::new();
# }
```

```cpp
const auto m = autd3::modulation::Static();
```

```cs
var m = new Static();
```

```python
from pyautd3.modulation import Static

m = Static()
```

## 振幅の指定

`with_amp`にて, 0-1の規格化された音圧振幅を指定できる.
$\[0, 1\]$の範囲外の値は$\[0, 1\]$にクランプされる (すなわち, $0$未満の値は$0$に, $1$より大きい値は$1$になる).

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Static::new().with_amp(1.);
# }
```

```cpp
const auto m = autd3::modulation::Static().with_amp(1.);
```

```cs
var m = new Static().WithAmp(1);
```

```python
from pyautd3.modulation import Static

m = Static().with_amp(1)
```
