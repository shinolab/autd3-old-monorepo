[[_TOC_]]

# Simulator

Simulator link is a link used when using [AUTD simulator](../../Simulator/simulator.md).

Before using this link, you need to start AUTD simulator.

## Simulator link API

### Contructor

Simulator link's constructor takes a port number of AUTD simulator.

```rust,should_panic,edition2021
# extern crate autd3;
# extern crate autd3_link_simulator;
# use autd3::prelude::*;
use autd3_link_simulator::Simulator;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., AUTD3::DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#            .open_with(
Simulator::builder(8080)
# )?;
# Ok(())
# }
```

```cpp
#include "autd3/link/simulator.hpp"

autd3::link::Simulator::builder(8080)
```

```cs
Simulator.Builder(8080)
```

```python
from pyautd3.link import Simulator

Simulator.builder(8080)
```

### AUTD simulator server IP address

You can specify the IP address of the server running AUTD simulator with `with_server_ip`.

```rust,should_panic,edition2021
# extern crate autd3;
# extern crate autd3_link_simulator;
# use autd3::prelude::*;
use autd3_link_simulator::Simulator;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., AUTD3::DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#            .open_with(
Simulator::builder(8080)
    .with_server_ip("127.0.0.1".parse()?)
# )?;
# Ok(())
# }
```

```cpp
#include "autd3/link/simulator.hpp"

autd3::link::Simulator::builder(8080)
    .with_server_ip("127.0.0.1")
```

```cs
Simulator.Builder(8080)
    .WithServerIp(IPAddress.Parse("127.0.0.1"))
```

```python
from pyautd3.link import Simulator

Simulator.builder(8080)\
    .with_server_ip("127.0.0.1")
```

The default server IP address is localhost.
