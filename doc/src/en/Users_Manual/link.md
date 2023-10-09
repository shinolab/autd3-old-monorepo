# Link

Link is an interface to AUTD3 devices.
You need to choose one of the following.

- [TwinCAT/RemoteTwinCAT](./link/twincat.md)
- [SOEM/RemoteSOEM](./link/soem.md)
- [Simulator](./link/simulator.md)
- [Visualizer](./link/visualize.md)

## Link options

### Timeout

Set the default timeout with `with_timeout`.

- The details of the timeout are described in [Controller#send#Timeout](./controller.md#timeout)

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() {
# let link = autd3::link::Nop::builder();
// link is some Link
# let link =
link.with_timeout(std::time::Duration::from_millis(20));
# }
```

```cpp
// link is some Link
link.with_timeout(std::chrono::milliseconds(20));
```

```cs
// link is some Link
link.WithTimeout(TimeSpan.FromMilliseconds(20))
```

```python
from datetime import timedelta

# link is some Link
link.with_timeout(timedelta(milliseconds=20))
```
