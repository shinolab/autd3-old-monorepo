# Link

LinkはAUTD3デバイスとのインターフェースである.
以下の中から一つを選択する必要がある.

- [TwinCAT/RemoteTwinCAT](./link/twincat.md)
- [SOEM/RemoteSOEM](./link/soem.md)
- [Simulator](./link/simulator.md)

## Linkに共通のオプション

### タイムアウト時間

`with_timeout`でデフォルトのタイムアウト時間を設定する.

* タイムアウト時間の詳細は[Controller#send#タイムアウト](./controller.md#%E3%82%BF%E3%82%A4%E3%83%A0%E3%82%A2%E3%82%A6%E3%83%88)を参照されたい

```rust,edition2021
{{#include ../../codes/Users_Manual/link_0.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/link_0.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/link_0.cs}}
```

```python
{{#include ../../codes/Users_Manual/link_0.py}}
```

デフォルトで各Linkに対して適当な値が設定されている.
