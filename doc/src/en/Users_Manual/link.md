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
