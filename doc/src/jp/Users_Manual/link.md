# Link

LinkはAUTD3デバイスとのインターフェースである.
以下の中から一つを選択する必要がある.

- [TwinCAT/RemoteTwinCAT](./link/twincat.md)
- [SOEM/RemoteSOEM](./link/soem.md)
- [Simulator](./link/simulator.md)
- [Visualizer](./link/visualize.md)
- [Bundle](./link/bundle.md)

## Linkに共通のオプション

### タイムアウト時間

`with_timeout`でデフォルトのタイムアウト時間を設定する.

* タイムアウト時間の詳細は[Controller#send#タイムアウト](./controller.md#%E3%82%BF%E3%82%A4%E3%83%A0%E3%82%A2%E3%82%A6%E3%83%88)を参照されたい

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() {
# let link = autd3::link::NullLink {};
// linkは何らかのLink
# let link =
link.with_timeout(std::time::Duration::from_millis(20));
# }
```

```cpp
// linkは何らかのLink
link.with_timeout(std::chrono::milliseconds(20));
```

```cs
// linkは何らかのLink
link.WithTimeout(TimeSpan.FromMilliseconds(20))
```

```python
from datetime import timedelta

# linkは何らかのLink
link.with_timeout(timedelta(milliseconds=20))
```

デフォルトで各Linkに対して適当な値が設定されている.

### Log

`Log`リンクを使用すると, ロギングを有効にした`Link`を生成できる.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
use autd3::link::Log;

# #[allow(unused_variables)]
# fn main() {
# let link = autd3::link::NullLink {};
// linkは何らかのLink
# let link: Log<LegacyTransducer, autd3::link::NullLink> =
link.with_log();
# }
```

```cpp
#include "autd3/link/log.hpp"

// linkは何らかのLink
link.with_log()
```

```cs
// linkは何らかのLink
link.WithLog()
```

```python
# linkは何らかのLink
link.with_log()
```
