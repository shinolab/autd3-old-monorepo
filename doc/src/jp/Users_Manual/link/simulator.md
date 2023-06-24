[[_TOC_]]

# Simulator

Simulator linkは[AUTDシミュレータ](../../Simulator/simulator.md)を使用する際に使うLinkである.

このlinkの使用の前に, AUTDシミュレータを起動しておく必要がある.

## SimulatorリンクのAPI

### コンストラクタ

`Simulator`のコンストラクタにはAUTDシミュレータのポート番号を指定する.

```rust,should_panic
# use autd3::prelude::*;
use autd3_link_simulator::Simulator;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#            .open_with(
Simulator::new(8080)
# )?;
# Ok(())
# }
```

```cpp
#include "autd3/link/simulator.hpp"

autd3::link::Simulator(8080)
```

```cs
new Simulator(8080)
```

```python
from pyautd3.link import SOEM

Simulator(8080)
```

### AUTDシミュレータサーバIP

`with_addr`でAUTDシミュレータを実行しているサーバのIPアドレスを指定する.

```rust,should_panic
# use autd3::prelude::*;
use autd3_link_simulator::Simulator;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#            .open_with(
Simulator::new(8080)
    .with_server_ip("127.0.0.1".parse()?)
# )?;
# Ok(())
# }
```

```cpp
#include "autd3/link/simulator.hpp"

autd3::link::Simulator(8080)
    .with_server_ip("127.0.0.1")
```

```cs
new Simulator(8080)
    .WithServerIp(IPAddress.Parse("127.0.0.1"))
```

```python
from pyautd3.link import Simulator

Simulator(8080)\
    .with_server_ip("127.0.0.1")
```

デフォルトはローカルホスト ("127.0.0.1") である.
