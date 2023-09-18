# Square

`Modulation` to transform sound pressure to a square wave.

Specify the frequency $f$ as an integer in the constructor.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Square::new(150);
# }
```

```cpp
const auto m = autd3::modulation::Square(150);
```

```cs
var m = new Square(150);
```

```python
from pyautd3.modulation import Square

m = Square(150)
```
## Set amplitude

You can set the amplitude of the square wave with `with_low` and `with_high` for low level and high level, respectively.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Square::new(150)
        .with_low(0.)
        .with_high(1.);
# }
```

```cpp
const auto m = autd3::modulation::Square(150)
        .with_low(0.)
        .with_high(1.);
```

```cs
var m = new Square(150)
        .WithLow(0)
        .WithHigh(1);
```

```python
from pyautd3.modulation import Square

m = Square(150).with_low(0).with_high(1)
```

## Set duty ratio

You can set the duty ratio of the square wave with `with_duty`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Square::new(150)
        .with_duty(0.5);
# }
```

```cpp
const auto m = autd3::modulation::Square(150)
        .with_duty(0.5);
```

```cs
var m = new Square(150)
        .WithDuty(0.5);
```

```python
from pyautd3.modulation import Square

m = Square(150).with_duty(0.5)
```