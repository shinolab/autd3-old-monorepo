# Link

Link is an interface to AUTD3 devices.
You need to choose one of the following.

- [TwinCAT/RemoteTwinCAT](./link/twincat.md)
- [SOEM/RemoteSOEM](./link/soem.md)
- [Simulator](./link/simulator.md)

## Link options

### Timeout

Set the default timeout with `with_timeout`.

- The details of the timeout are described in [Controller#send#Timeout](./controller.md#timeout)

```rust,should_panic,edition2021
# extern crate autd3;
# extern crate autd3_link_soem;
# use autd3::prelude::*;
# use autd3_link_soem::SOEM;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., AUTD3::DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#            .open_with(
# SOEM::new()
.with_timeout(std::time::Duration::from_millis(20))
# )?;
# Ok(())
# }
```

```cpp
.with_timeout(std::chrono::milliseconds(20))
```

```cs
.WithTimeout(TimeSpan.FromMilliseconds(20))
```

```python
from datetime import timedelta

.with_timeout(timedelta(milliseconds=20))
```
