# Simulator

Simulator linkは[AUTDシミュレータ](../../Simulator/simulator.md)を使用する際に使うLinkである.

このlinkの使用の前に, AUTDシミュレータを起動しておく必要がある.

[[_TOC_]]

## SimulatorリンクのAPI

### コンストラクタ

`Simulator`のコンストラクタにはAUTDシミュレータのポート番号を指定する.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/link/simulator_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/link/simulator_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/link/simulator_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/link/simulator_0.py}}
```

### リモート接続

`with_server_ip`でAUTDシミュレータを実行しているサーバのIPアドレスを指定することで, リモートのシミュレータに接続することができる.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/link/simulator_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/link/simulator_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/link/simulator_1.cs}}
```

```python
{{#include ../../../codes/Users_Manual/link/simulator_1.py}}
```


デフォルトはローカルホスト ("127.0.0.1") である.

### `Geometry`の更新

`Geometry`を更新しても, Simulator側の表示は自動的には更新されない.
`Geometry`を更新するには`update_geometry`関数を使用する.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/link/simulator_2.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/link/simulator_2.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/link/simulator_2.cs}}
```

```python
{{#include ../../../codes/Users_Manual/link/simulator_2.py}}
```
