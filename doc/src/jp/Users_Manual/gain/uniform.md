# Uniform

`Uniform`はすべての振動子に同じ位相/振幅を設定する.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
let g = autd3::gain::Uniform::new(1.0);
# }
```

```cpp
const auto g = autd3::gain::Uniform(1.0);
```

```cs
var g = new Uniform(1.0);
```

```python
from pyautd3.gain import Uniform

g = Uniform(1.0)
```

コンストラクタには振幅を指定する.

## 位相の指定

`with_phase`にて, 位相を指定できる.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
let g = autd3::gain::Uniform::new(1.0).with_phase(0.0);
# }
```

```cpp
const auto g = autd3::gain::Uniform(1.0).with_phase(0.0);
```

```cs
var g = new Uniform(1.0).WithPhase(0.0);
```

```python
from pyautd3.gain import Uniform

g = Uniform(1.0).with_phase(0.0)
```

デフォルトの位相は$0$である.
