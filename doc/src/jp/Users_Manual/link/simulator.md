[[_TOC_]]

# Simulator

Simulator linkは[AUTDシミュレータ](../../Simulator/simulator.md)を使用する際に使うLinkである.

このlinkの使用の前に, AUTDシミュレータを起動しておく必要がある.

## SimulatorリンクのAPI

### コンストラクタ

`Simulator`のコンストラクタにはAUTDシミュレータのポート番号を指定する.

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
from pyautd3.link.simulator import Simulator

Simulator.builder(8080)
```

### リモート接続

`with_server_ip`でAUTDシミュレータを実行しているサーバのIPアドレスを指定することで, リモートのシミュレータに接続することができる.

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
from pyautd3.link.simulator import Simulator

Simulator.builder(8080)\
    .with_server_ip("127.0.0.1")
```

デフォルトはローカルホスト ("127.0.0.1") である.
