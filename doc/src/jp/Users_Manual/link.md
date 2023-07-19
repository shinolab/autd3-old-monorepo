# Link

LinkはAUTD3デバイスとのインターフェースである.
以下の中から一つを選択する必要がある.

- [TwinCAT/RemoteTwinCAT](./link/twincat.md)
- [SOEM/RemoteSOEM](./link/soem.md)
- [Simulator](./link/simulator.md)
- [Monitor](./link/monitor.md)
- [Bundle](./link/bundle.md)

## Linkに共通のオプション

### タイムアウト時間

`with_timeout`でデフォルトのタイムアウト時間を設定する.

* タイムアウト時間の詳細は[Controller#send#タイムアウト](./controller.md#%E3%82%BF%E3%82%A4%E3%83%A0%E3%82%A2%E3%82%A6%E3%83%88)を参照されたい

```rust,should_panic,edition2021
# extern crate autd3;
# extern crate autd3_link_soem;
# use autd3::prelude::*;
# use autd3_link_soem::SOEM;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
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

デフォルトは各Linkに対して適当な値が設定されている.


### Log

`Log`リンクを使用すると, ロギングを有効にした`Link`を生成できる.

```rust,should_panic,edition2021
# extern crate autd3;
# extern crate autd3_link_soem;
# use autd3::prelude::*;
use autd3::link::Log;
use autd3_link_soem::SOEM;

# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#            .open_with(
SOEM::new().with_log()
# )?;
# Ok(())
# }
```

```cpp
#include "autd3/link/log.hpp"

// linkは何らかのLink
autd3::link::Log(link)
```
